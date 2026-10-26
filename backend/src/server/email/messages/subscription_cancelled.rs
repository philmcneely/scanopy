use super::{Body, Content, Email, EmailCategory, links};

/// Sent when a cancelled cloud subscription reaches its period end: the org
/// moves to Free, with a resubscribe
/// CTA.
pub struct SubscriptionCancelled<'a> {
    pub period_end_date: &'a str,
}

impl Email for SubscriptionCancelled<'_> {
    fn subject(&self) -> String {
        "Your Subscription Has Been Cancelled".to_string()
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::Billing
    }

    fn campaign(&self) -> &'static str {
        "subscription_cancelled"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(
                Content::new()
                    .heading("Subscription Cancelled")
                    .paragraph("Hi there,")
                    .paragraph(&format!(
                        "Your Scanopy subscription ended on {}, and your organization is now on Free. Everything Scanopy found is still here, and your daemons still run the scans you start. The topology view, tagging and scheduled scans come back when you choose a plan.",
                        self.period_end_date
                    ))
                    .paragraph("Choose a paid plan from Settings to resume. Your sites, hosts and schedules are all still there."),
            )
            .cta(links::PLAN_PICKER, "Resubscribe")
            .render()
    }
}
