//! Which ports a scan probed and got a definite answer from.
//!
//! A discovery payload lists the ports it found open. That says nothing about a stored port it
//! didn't list: the scan may have probed it and found it closed, or never probed it (a light
//! scan's port set, a Docker inventory, an SNMP walk). Coverage is the probed set, so the server
//! can tell the two apart. A port may change hands only at an address where the scan covered it.
//! Daemons that predate coverage send none, and none covers nothing.

use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::ports::r#impl::base::{PortType, TransportProtocol};

/// An inclusive run of port numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

/// The ports of one protocol a scan covered at one address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CoveredPorts {
    #[schema(value_type = String, example = "192.168.1.10")]
    pub ip_address: IpAddr,
    pub protocol: TransportProtocol,
    pub ranges: Vec<PortRange>,
}

/// Every (address, protocol, port) a scan probed and got a definite answer from: open, refused,
/// or silent after every retry. A probe that failed on the scanner's side (file descriptors
/// exhausted, the scan cancelled) is left out, since it learned nothing about the port.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct PortCoverage(pub Vec<CoveredPorts>);

impl PortCoverage {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|c| c.ranges.is_empty())
    }

    /// Record `numbers` as covered at `ip_address`, folding them into ranges.
    pub fn add(
        &mut self,
        ip_address: IpAddr,
        protocol: TransportProtocol,
        numbers: impl IntoIterator<Item = u16>,
    ) {
        let mut numbers: Vec<u16> = numbers.into_iter().collect();
        if numbers.is_empty() {
            return;
        }
        let entry = match self
            .0
            .iter_mut()
            .position(|c| c.ip_address == ip_address && c.protocol == protocol)
        {
            Some(i) => &mut self.0[i],
            None => {
                self.0.push(CoveredPorts {
                    ip_address,
                    protocol,
                    ranges: Vec::new(),
                });
                self.0.last_mut().expect("just pushed")
            }
        };
        numbers.extend(
            entry
                .ranges
                .iter()
                .flat_map(|r| r.start..=r.end),
        );
        numbers.sort_unstable();
        numbers.dedup();

        let mut ranges: Vec<PortRange> = Vec::new();
        for n in numbers {
            match ranges.last_mut() {
                Some(r) if r.end.checked_add(1) == Some(n) => r.end = n,
                _ => ranges.push(PortRange { start: n, end: n }),
            }
        }
        entry.ranges = ranges;
    }

    /// Fold `other` into this coverage.
    pub fn merge(&mut self, other: PortCoverage) {
        for covered in other.0 {
            self.add(
                covered.ip_address,
                covered.protocol,
                covered.ranges.iter().flat_map(|r| r.start..=r.end),
            );
        }
    }

    /// Whether the scan covered `port` at `ip_address`.
    pub fn covers(&self, ip_address: IpAddr, port: &PortType) -> bool {
        let (number, protocol) = (port.number(), port.protocol());
        self.0.iter().any(|c| {
            c.ip_address == ip_address
                && c.protocol == protocol
                && c.ranges.iter().any(|r| r.start <= number && number <= r.end)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lan() -> IpAddr {
        "192.168.4.58".parse().unwrap()
    }

    #[test]
    fn covers_only_the_ports_added_at_that_address_and_protocol() {
        let mut coverage = PortCoverage::default();
        coverage.add(lan(), TransportProtocol::Tcp, [22, 80, 443]);

        assert!(coverage.covers(lan(), &PortType::new_tcp(443)));
        assert!(!coverage.covers(lan(), &PortType::new_tcp(8443)));
        assert!(!coverage.covers(lan(), &PortType::new_udp(443)));
        assert!(!coverage.covers("192.168.4.59".parse().unwrap(), &PortType::new_tcp(443)));
    }

    #[test]
    fn a_full_scan_folds_into_one_range_and_survives_a_round_trip() {
        let mut coverage = PortCoverage::default();
        coverage.add(lan(), TransportProtocol::Tcp, 1..=65535);
        assert_eq!(coverage.0[0].ranges, vec![PortRange { start: 1, end: 65535 }]);

        let json = serde_json::to_string(&coverage).unwrap();
        let back: PortCoverage = serde_json::from_str(&json).unwrap();
        assert!(back.covers(lan(), &PortType::new_tcp(65535)));
    }

    #[test]
    fn merging_two_scans_of_one_address_covers_both() {
        let mut liveness = PortCoverage::default();
        liveness.add(lan(), TransportProtocol::Tcp, [80, 443]);
        let mut deep = PortCoverage::default();
        deep.add(lan(), TransportProtocol::Tcp, 81..=442);
        liveness.merge(deep);

        assert_eq!(liveness.0.len(), 1);
        assert_eq!(liveness.0[0].ranges, vec![PortRange { start: 80, end: 443 }]);
    }
}
