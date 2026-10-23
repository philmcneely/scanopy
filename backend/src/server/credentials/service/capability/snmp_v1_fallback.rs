//! Falling back to SNMPv1 when a community credential gets no answer over v2c, which daemons do
//! only after `last_without_snmp_v1_fallback`.
//!
//! A community credential goes on the wire as v2c for every daemon. An older daemon parses it and
//! tries v2c only, so it still reads every device that answers v2c, and the credential is sent to
//! it unchanged. What it misses is a device that answers only v1, which a separate SNMPv1
//! credential used to reach. The warning says so, since the server can no longer tell which
//! credentials those were.

use super::DaemonCapability;
use crate::server::credentials::r#impl::capability::CapabilityGap;
use crate::server::credentials::r#impl::types::CredentialTypeDiscriminants;
use crate::server::credentials::service::TypedCredentialMapping;
use crate::server::daemons::r#impl::version::last_without_snmp_v1_fallback;
use semver::Version;

/// A community credential, which a daemon tries over v2c and then v1.
pub(crate) struct FallsBackToSnmpV1;

/// A community credential the daemon tries over v2c only.
pub(crate) struct SnmpV1FallbackGap;

impl From<SnmpV1FallbackGap> for CapabilityGap {
    fn from(_: SnmpV1FallbackGap) -> Self {
        CapabilityGap::FallsBackToSnmpV1
    }
}

impl DaemonCapability for FallsBackToSnmpV1 {
    type Context = ();
    type Gap = SnmpV1FallbackGap;

    fn gap(_: &(), credential: &TypedCredentialMapping) -> Option<Self::Gap> {
        (credential.discriminant == CredentialTypeDiscriminants::SnmpCommunity)
            .then_some(SnmpV1FallbackGap)
    }

    fn last_without(_: &Self::Gap) -> Version {
        last_without_snmp_v1_fallback()
    }

    /// Nothing to strip: the v2c attempt the daemon makes is the whole credential it can use.
    fn withhold(_: &Self::Gap, _: &mut TypedCredentialMapping) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::discovery::types::warnings::DiscoveryWarning;
    use crate::server::credentials::service::capability::retain_capable;
    use crate::server::credentials::service::mapping_for;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    fn community_mapping(id: Uuid) -> BTreeMap<Uuid, TypedCredentialMapping> {
        let mut map = BTreeMap::new();
        let ct = CredentialTypeDiscriminants::SnmpCommunity.to_credential_type();
        mapping_for(&mut map, id, &ct).default_credential = Some(ct.to_query_payload());
        map
    }

    /// A daemon without the fallback still gets the credential, since it reads every v2c device,
    /// and one warning naming what it misses. A daemon with the fallback gets it with no warning.
    #[test]
    fn an_older_daemon_keeps_the_credential_and_is_warned() {
        let id = Uuid::from_u128(1);
        let last_without = last_without_snmp_v1_fallback();
        let newer = Version::new(
            last_without.major,
            last_without.minor,
            last_without.patch + 1,
        );

        let mut old = community_mapping(id);
        let warned = retain_capable::<FallsBackToSnmpV1>(&(), &mut old, Some(&last_without));
        assert!(old[&id].mapping.default_credential.is_some());
        assert!(matches!(
            warned.as_slice(),
            [DiscoveryWarning::CredentialNeedsNewerDaemon {
                credential_id: Some(warned_id),
                gap: CapabilityGap::FallsBackToSnmpV1,
                ..
            }] if *warned_id == id
        ));

        let mut new = community_mapping(id);
        assert!(retain_capable::<FallsBackToSnmpV1>(&(), &mut new, Some(&newer)).is_empty());
        assert!(new[&id].mapping.default_credential.is_some());
    }
}
