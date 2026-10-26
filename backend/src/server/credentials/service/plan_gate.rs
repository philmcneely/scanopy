//! Credential types an organization's plan must include before it can create one (see
//! `CredentialTypeDiscriminants::required_feature`).
//!
//! Only creating one is gated. A credential that already exists keeps being sent to daemons and
//! stays editable on any plan, so an organization that drops below the plan loses nothing it set up.

use super::*;
use crate::server::billing::types::base::BillingPlan;
use crate::server::shared::types::metadata::TypeMetadataProvider;
use crate::server::shared::types::{api::ApiError, error_codes::ErrorCode};
use axum::http::StatusCode;

impl CredentialService {
    /// Refuse saving `credential` as a type its organization's plan does not include, unless it
    /// already had that type (`current`).
    pub(super) async fn refuse_type_outside_plan(
        &self,
        current: Option<CredentialTypeDiscriminants>,
        credential: &Credential,
    ) -> Result<(), Error> {
        let next = credential.base.credential_type.discriminant();
        if next.required_feature().is_none() || current == Some(next) {
            return Ok(());
        }
        let organization = self
            .organization_service
            .get_by_id(&credential.base.organization_id)
            .await?
            .ok_or_else(ApiError::organization_required)?;
        check_type_in_plan(current, next, &organization.base.plan)?;
        Ok(())
    }
}

/// Why an organization on `plan` cannot save a credential as `next` when it was `current`
/// (`None` for a new credential).
pub(crate) fn check_type_in_plan(
    current: Option<CredentialTypeDiscriminants>,
    next: CredentialTypeDiscriminants,
    plan: &BillingPlan,
) -> Result<(), ApiError> {
    if current == Some(next) || next.available_on(plan) {
        return Ok(());
    }
    let feature = next
        .required_feature()
        .map(|f| f.name().to_string())
        .unwrap_or_default();
    Err(ApiError::coded(
        StatusCode::PAYMENT_REQUIRED,
        ErrorCode::BillingFeatureNotAvailable { feature },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::billing::plans::{get_community_plan, get_free_plan};
    use crate::server::billing::types::base::BillingPlanDiscriminants;
    use strum::IntoEnumIterator;

    const INSTANT_ON: CredentialTypeDiscriminants = CredentialTypeDiscriminants::InstantOnAccount;

    fn plan(disc: BillingPlanDiscriminants) -> BillingPlan {
        BillingPlan::default_for_discriminant(disc).expect("plan has a default")
    }

    #[test]
    fn a_new_gated_credential_is_refused_below_the_plan_and_allowed_on_it() {
        let refused = check_type_in_plan(None, INSTANT_ON, &get_free_plan()).unwrap_err();
        assert_eq!(refused.status, StatusCode::PAYMENT_REQUIRED);
        assert!(check_type_in_plan(None, INSTANT_ON, &get_community_plan()).is_err());

        let pro = plan(BillingPlanDiscriminants::Pro);
        assert!(check_type_in_plan(None, INSTANT_ON, &pro).is_ok());
        let standard = plan(BillingPlanDiscriminants::SelfHostedStandard);
        assert!(check_type_in_plan(None, INSTANT_ON, &standard).is_ok());
    }

    /// An organization that drops below the plan keeps editing what it already has.
    #[test]
    fn an_existing_gated_credential_stays_editable_below_the_plan() {
        assert!(check_type_in_plan(Some(INSTANT_ON), INSTANT_ON, &get_free_plan()).is_ok());
    }

    /// Changing an existing credential's type is the same as creating one of the new type.
    #[test]
    fn changing_a_credential_to_a_gated_type_is_refused_below_the_plan() {
        let from = Some(CredentialTypeDiscriminants::SnmpCommunity);
        assert!(check_type_in_plan(from, INSTANT_ON, &get_free_plan()).is_err());
    }

    /// A plan card lists the integrations a plan can create credentials for, so a gated
    /// integration is listed exactly on the plans whose features include its gate.
    #[test]
    fn a_plan_lists_a_gated_integration_only_when_it_has_the_feature() {
        use crate::server::shared::types::metadata::HasId;

        let gated: Vec<_> = CredentialTypeDiscriminants::iter()
            .filter_map(|t| t.required_feature().map(|f| (t.integration().id(), f)))
            .collect();
        assert!(!gated.is_empty());
        for plan in BillingPlan::iter() {
            let disc = plan.discriminant();
            let listed = plan.integration_ids();
            for (integration, feature) in &gated {
                assert_eq!(
                    listed.contains(integration),
                    plan.has_feature(feature.id()),
                    "{integration} on {disc:?}"
                );
            }
        }
    }

    /// A card shows what its plan adds over the tier below; together with that tier's list it is
    /// the plan's whole list, and Pro's addition over Free is the gated integration.
    #[test]
    fn a_plans_incremental_integrations_complete_its_previous_tier() {
        for plan in BillingPlan::iter() {
            let disc = plan.discriminant();
            let Some(previous) = plan
                .previous_tier()
                .and_then(BillingPlan::default_for_discriminant)
            else {
                continue;
            };
            let mut combined = previous.integration_ids();
            combined.extend(plan.incremental_integration_ids());
            combined.sort_unstable();
            let mut own = plan.integration_ids();
            own.sort_unstable();
            assert_eq!(combined, own, "{disc:?}");
        }
        let pro = plan(BillingPlanDiscriminants::Pro);
        assert!(!pro.incremental_integration_ids().is_empty());
    }

    #[test]
    fn ungated_types_are_allowed_on_every_plan() {
        for plan in BillingPlan::iter() {
            let disc = plan.discriminant();
            for t in CredentialTypeDiscriminants::iter().filter(|t| t.required_feature().is_none())
            {
                assert!(
                    check_type_in_plan(None, t, &plan).is_ok(),
                    "{t:?} on {disc:?}"
                );
            }
        }
    }
}
