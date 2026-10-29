//! LLMNR detection over TCP with a reverse lookup of the probed address.
//!
//! LLMNR (RFC 4795) carries DNS messages, framed over TCP with the same 2-byte length prefix as
//! DNS. A responder answers only for names it is authoritative for (§2.1.1) and stays silent for
//! the rest, so the root query the DNS probe sends draws nothing. The one name a responder holds
//! that the probe knows is the reverse name of the address it is answering on, so the probe asks
//! for that address's PTR record.
//!
//! systemd-resolved enables LLMNR per link but listens on every address, so an address on a link
//! with LLMNR off stays silent. It is not claimed there, which is right: nothing answers on it.

use std::net::IpAddr;

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::dns_tcp::parse_reply;
use crate::daemon::utils::app_probe::{AppProbe, AppProbeOutcome, ProbeContext, request_response};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// Echoed by the responder, correlating the reply to our query.
const TRANSACTION_ID: u16 = 0x5CA2;
/// QTYPE PTR.
const QTYPE_PTR: u16 = 12;
/// QCLASS IN.
const QCLASS_IN: u16 = 1;

pub struct LlmnrProbe;

#[async_trait]
impl AppProbe for LlmnrProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Llmnr
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, port, &ptr_query(ctx.ip), 1024).await;
        Ok(parse_reply(&reply, TRANSACTION_ID))
    }
}

/// The reverse name of `ip`: `181.4.168.192.in-addr.arpa`, or the nibble form under `ip6.arpa`.
fn reverse_name(ip: IpAddr) -> Vec<String> {
    let mut labels: Vec<String> = match ip {
        IpAddr::V4(v4) => v4.octets().iter().rev().map(u8::to_string).collect(),
        IpAddr::V6(v6) => v6
            .octets()
            .iter()
            .rev()
            .flat_map(|byte| [byte & 0x0f, byte >> 4])
            .map(|nibble| format!("{nibble:x}"))
            .collect(),
    };
    labels.extend(
        match ip {
            IpAddr::V4(_) => ["in-addr", "arpa"],
            IpAddr::V6(_) => ["ip6", "arpa"],
        }
        .map(str::to_string),
    );
    labels
}

/// A PTR query for `ip`'s reverse name, length-prefixed for TCP.
fn ptr_query(ip: IpAddr) -> Vec<u8> {
    let mut message = Vec::new();
    message.extend_from_slice(&TRANSACTION_ID.to_be_bytes());
    message.extend_from_slice(&0x0000u16.to_be_bytes()); // flags: standard query
    message.extend_from_slice(&1u16.to_be_bytes()); // one question
    message.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // no answer, authority or additional records
    for label in reverse_name(ip) {
        message.push(label.len() as u8);
        message.extend_from_slice(label.as_bytes());
    }
    message.push(0x00);
    message.extend_from_slice(&QTYPE_PTR.to_be_bytes());
    message.extend_from_slice(&QCLASS_IN.to_be_bytes());

    let mut framed = (message.len() as u16).to_be_bytes().to_vec();
    framed.extend_from_slice(&message);
    framed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reverse_name_reads_the_address_backwards() {
        assert_eq!(
            reverse_name("192.168.4.181".parse().unwrap()).join("."),
            "181.4.168.192.in-addr.arpa"
        );
        let v6 = reverse_name("2001:db8::1".parse().unwrap()).join(".");
        assert!(v6.starts_with("1.0.0.0.0.0.0.0."), "{v6}");
        assert!(v6.ends_with(".8.b.d.0.1.0.0.2.ip6.arpa"), "{v6}");
    }

    #[test]
    fn a_response_to_the_query_is_llmnr_and_silence_is_not() {
        let query = ptr_query("192.168.4.181".parse().unwrap());
        // The responder echoes the header with the response bit set.
        let mut response = query.clone();
        response[4] |= 0x80;
        assert_eq!(
            parse_reply(&response, TRANSACTION_ID),
            AppProbeOutcome::Answered { identity: None }
        );
        assert_eq!(
            parse_reply(&query, TRANSACTION_ID),
            AppProbeOutcome::NoAnswer
        );
        assert_eq!(parse_reply(&[], TRANSACTION_ID), AppProbeOutcome::NoAnswer);
    }
}
