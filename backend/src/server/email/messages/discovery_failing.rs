use super::{Body, Content, Email, EmailCategory, links};

/// Alerts that a discovery's last few runs all failed, with why the newest one did.
pub struct DiscoveryFailing<'a> {
    pub discovery_name: &'a str,
    pub daemon_name: &'a str,
    pub site_name: &'a str,
    pub failed_runs: usize,
    /// The newest failure's reason, as its metadata names it.
    pub reason_name: &'a str,
    /// The newest failure's reason, as its metadata describes it: what happened and what to check.
    pub reason_description: &'a str,
}

impl Email for DiscoveryFailing<'_> {
    fn subject(&self) -> String {
        "Your Scheduled Discovery Keeps Failing".to_string()
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::DaemonAlerts
    }

    fn campaign(&self) -> &'static str {
        "discovery_failing"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(
                Content::new()
                    .heading("Discovery Keeps Failing")
                    .paragraph("Hi there,")
                    .paragraph(&format!(
                        "The last {} runs of <strong>{}</strong> on daemon <strong>{}</strong> (<strong>{}</strong>) failed. Scanopy has no new results from this discovery until a run completes.",
                        self.failed_runs, self.discovery_name, self.daemon_name, self.site_name
                    ))
                    .paragraph(&format!(
                        "The latest run ended with <strong>{}</strong>: {}",
                        self.reason_name, self.reason_description
                    ))
                    .paragraph(&format!(
                        r#"The <a href="{}" style="color: #2563eb; text-decoration: none;">troubleshooting guide</a> covers the common causes."#,
                        links::DOCS_DAEMON_TROUBLESHOOTING
                    )),
            )
            .cta(links::APP_DAEMONS, "View Daemons")
            .render()
    }
}
