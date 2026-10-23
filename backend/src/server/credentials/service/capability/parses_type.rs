//! A daemon too old to parse a credential type is sent none of it.

use super::DaemonCapability;
use crate::server::credentials::r#impl::capability::CapabilityGap;
use crate::server::credentials::r#impl::types::CredentialTypeDiscriminants;
use crate::server::credentials::service::TypedCredentialMapping;
use semver::Version;

/// Parsing the credential type at all. Every credential relies on it.
///
/// Gated on the 7-way `CredentialType` discriminant kept beside each mapping, not the collapsed
/// wire tag, since SnmpV1 and SnmpV2c share one `Snmp` tag with different gates. Older daemons
/// predate `serde(other)`, so sending them a type they can't parse would fail the whole request.
pub(crate) struct ParsesCredentialType;

/// The credential's type, for its gate.
pub(crate) struct ParsesCredentialTypeGap(CredentialTypeDiscriminants);

impl From<ParsesCredentialTypeGap> for CapabilityGap {
    fn from(_: ParsesCredentialTypeGap) -> Self {
        CapabilityGap::ParsesCredentialType
    }
}

impl DaemonCapability for ParsesCredentialType {
    type Context = ();
    type Gap = ParsesCredentialTypeGap;

    fn gap(_: &(), credential: &TypedCredentialMapping) -> Option<Self::Gap> {
        Some(ParsesCredentialTypeGap(credential.discriminant))
    }

    fn last_without(gap: &Self::Gap) -> Version {
        gap.0.last_unsupported_daemon_version()
    }

    fn withhold(_: &Self::Gap, credential: &mut TypedCredentialMapping) {
        credential.mapping.default_credential = None;
        credential.mapping.ip_overrides.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::discovery::types::warnings::DiscoveryWarning;
    use crate::server::credentials::service::capability::retain_capable;
    use crate::server::credentials::service::mapping_for;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    /// SnmpV2c and SnmpV1 share the `Snmp` wire tag with different gates, so a 0.16.2 daemon gets
    /// the v2c credential and loses only the v1 one, with one warning naming it.
    #[test]
    fn only_the_type_the_daemon_cannot_read_is_withheld() {
        let mut map = BTreeMap::new();
        let v2c_id = Uuid::from_u128(1);
        let v1_id = Uuid::from_u128(2);
        for (id, disc) in [
            (v2c_id, CredentialTypeDiscriminants::SnmpV2c),
            (v1_id, CredentialTypeDiscriminants::SnmpV1),
        ] {
            let ct = disc.to_credential_type();
            mapping_for(&mut map, id, &ct).default_credential = Some(ct.to_query_payload());
        }

        let withheld =
            retain_capable::<ParsesCredentialType>(&(), &mut map, Some(&Version::new(0, 16, 2)));

        assert!(map.contains_key(&v2c_id));
        assert!(!map.contains_key(&v1_id));
        assert!(matches!(
            withheld.as_slice(),
            [DiscoveryWarning::CredentialNeedsNewerDaemon {
                credential_id: Some(id),
                gap: CapabilityGap::ParsesCredentialType,
                ..
            }] if *id == v1_id
        ));
    }
}
