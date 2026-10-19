use std::net::Ipv4Addr;

use crate::daemon::discovery::integration::snmp::sim::{AgentFaults, Purpose, SimDevice, Tables};
use crate::server::credentials::r#impl::types::CredentialType;

use super::{inline, switch_silent_01};

/// `switch-silent-01`'s tables, with GETBULK dropped on every one of them and GETNEXT answered.
///
/// The candidate cause the customer report suggested first: an agent, or a firewall dropping large
/// or fragmented responses, that serves no GETBULK. The walk already falls back to GETNEXT for the
/// rest of the session once GETBULK goes unanswered, so this device is read in full. It does not
/// produce the customer's warnings, which is how it rules that cause out.
pub fn device() -> SimDevice {
    SimDevice {
        name: "switch-bulkdrop-01",
        ip: Ipv4Addr::UNSPECIFIED,
        purpose: Purpose::Control {
            role: "drops GETBULK on every table and answers GETNEXT, showing the GETNEXT fallback reads such a device in full",
        },
        credential: CredentialType::SnmpV2c {
            community: inline("netdefault"),
        },
        system: switch_silent_01::system("switch-bulkdrop-01"),
        tables: Tables {
            faults: AgentFaults {
                refuse_getbulk: AgentFaults::every_table(),
                ..Default::default()
            },
            ..switch_silent_01::tables(0x5c)
        },
        arp_handler: Default::default(),
        suppresses: Vec::new(),
        rejects_getbulk: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::daemon::discovery::integration::snmp::sim::harness;

    /// Every table arrives through GETNEXT, and there is nothing to warn about.
    #[tokio::test]
    async fn getnext_reads_every_table() {
        let scan = harness::scan("switch-bulkdrop-01").await;

        assert_eq!(scan.if_table.entries.len(), 3);
        assert!(scan.if_table.set_complete);
        assert_eq!(scan.fdb.records.len(), 2);
        assert_eq!(scan.bridge_ports.len(), 3);
        assert!(!scan.ip_addr_table.is_empty());
        assert!(
            harness::walk_warnings("switch-bulkdrop-01")
                .await
                .is_empty(),
            "a device read in full has nothing to report"
        );
    }
}
