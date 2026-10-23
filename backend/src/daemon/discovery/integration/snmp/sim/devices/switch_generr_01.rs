use std::net::Ipv4Addr;

use crate::daemon::discovery::integration::snmp::sim::{AgentFaults, Purpose, SimDevice, Tables};
use crate::server::credentials::r#impl::types::CredentialType;

use super::{inline, switch_silent_01};

/// `switch-silent-01`'s tables, answered with `genErr` instead of silence.
///
/// The other shape that fits the customer's warning set: the device answers every table request,
/// GETBULK and GETNEXT, with an error status. Until the cause travelled with the warning this read
/// as "stopped answering", which is the one thing this device never did.
use crate::daemon::discovery::integration::snmp::sim::CommunityVersions;

pub fn device() -> SimDevice {
    SimDevice {
        name: "switch-generr-01",
        ip: Ipv4Addr::UNSPECIFIED,
        purpose: Purpose::Regression {
            issue: "the silent-devices customer report",
            defect: "a device answering every table request with genErr was reported as having stopped answering",
        },
        credential: CredentialType::SnmpCommunity {
            community: inline("netdefault"),
        },
        community_versions: CommunityVersions::Both,
        system: switch_silent_01::system("switch-generr-01"),
        tables: Tables {
            faults: AgentFaults {
                reject: AgentFaults::every_table(),
                error_status: 5,
                ..Default::default()
            },
            ..switch_silent_01::tables(0x5b)
        },
        arp_handler: Default::default(),
        suppresses: Vec::new(),
        rejects_getbulk: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::daemon::discovery::integration::snmp::sim::harness;
    use crate::daemon::discovery::types::warnings::{DiscoveryWarning, SnmpWalkCause};

    /// The line names genErr for every table, and says nothing about the device going quiet.
    #[tokio::test]
    async fn every_table_is_reported_as_answering_generr() {
        let warnings = harness::walk_warnings("switch-generr-01").await;

        let [
            DiscoveryWarning::SnmpCollectedNothing {
                failures: Some(failures),
                ..
            },
        ] = warnings.as_slice()
        else {
            panic!("expected the one whole-device line, got {warnings:?}");
        };
        assert_eq!(failures.len(), 8, "{failures:?}");
        assert!(
            failures.iter().all(|f| f.cause == SnmpWalkCause::GenErr),
            "{failures:?}"
        );
    }
}
