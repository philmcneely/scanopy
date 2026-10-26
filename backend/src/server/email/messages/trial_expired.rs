use super::{Body, Content, Email, EmailCategory, links};

/// Sent when a cloud trial ends without conversion: the org moves to Free,
/// with a CTA to the plan picker.
pub struct TrialExpired<'a> {
    pub plan_name: &'a str,
    pub billing_period: &'a str,
}

impl Email for TrialExpired<'_> {
    fn subject(&self) -> String {
        "Your Trial Has Ended".to_string()
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::Billing
    }

    fn campaign(&self) -> &'static str {
        "trial_expired"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(
                Content::new()
                    .heading("Your Trial Has Ended")
                    .paragraph("Hi there,")
                    .paragraph(&format!(
                        "Your {} {} trial has ended, and your organization is now on Free. Everything Scanopy found is still here, and your daemons still run the scans you start. The topology view, tagging and scheduled scans come back when you choose a plan.",
                        self.plan_name, self.billing_period
                    ))
                    .paragraph("Choose a paid plan from Settings to pick up where the trial left off. Your sites, hosts and schedules are all still there."),
            )
            .cta(links::PLAN_PICKER, "Choose a plan")
            .render()
    }
}
