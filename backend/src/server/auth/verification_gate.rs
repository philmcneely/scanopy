//! The license-plan email verification gate.
//!
//! An org on a self-hosted license plan reaches Scanopy Cloud for its license
//! key, which is a lasting credential, and the trial and renewal emails that
//! keep the key working go to the account's address. So on the cloud, a user
//! of such an org verifies their email before the API serves their session
//! (`middleware::session_gate::session_gate_middleware`), whatever their role.
//! Everywhere else, verification gates only the actions marked
//! [`RequireVerified`](crate::server::auth::middleware::permissions::RequireVerified).
//!
//! A server with no email transport can't deliver the link, so it never gates,
//! matching [`AuthenticatedEntity::meets_email_verification`](crate::server::auth::middleware::auth::AuthenticatedEntity::meets_email_verification).

use crate::server::{billing::types::base::BillingPlan, config::DeploymentType};

/// Whether a user in an org on `plan` must verify their email before using
/// the app.
pub fn verification_required(
    deployment_type: DeploymentType,
    plan: Option<&BillingPlan>,
    email_verified: bool,
    has_email_service: bool,
) -> bool {
    deployment_type == DeploymentType::Cloud
        && plan.is_some_and(|plan| plan.license_plan().is_some())
        && has_email_service
        && !email_verified
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::billing::plans::{get_enterprise_plan, get_self_hosted_standard_plan};

    #[test]
    fn only_an_unverified_license_plan_user_on_the_cloud_with_email_is_gated() {
        let license = get_self_hosted_standard_plan();
        let cloud_plan = get_enterprise_plan();
        let cloud = DeploymentType::Cloud;

        assert!(verification_required(cloud, Some(&license), false, true));

        assert!(!verification_required(cloud, Some(&license), true, true));
        assert!(!verification_required(cloud, Some(&license), false, false));
        assert!(!verification_required(
            cloud,
            Some(&cloud_plan),
            false,
            true
        ));
        assert!(!verification_required(cloud, None, false, true));
        for self_hosted in [DeploymentType::Commercial, DeploymentType::Community] {
            assert!(!verification_required(
                self_hosted,
                Some(&license),
                false,
                true
            ));
        }
    }
}
