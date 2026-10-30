use super::{Body, Content, Email, EmailCategory, links};

/// Alerts that a daemon's last few scans all failed, with why the newest one did.
pub struct DaemonScansFailing<'a> {
    pub daemon_name: &'a str,
    pub site_name: &'a str,
    pub failed_runs: usize,
    /// The discoveries the failed scans ran for, newest first, each named once.
    pub discovery_names: &'a [&'a str],
    /// The newest failure's reason, as its metadata names it.
    pub reason_name: &'a str,
    /// The newest failure's reason, as its metadata describes it: what happened and what to check.
    pub reason_description: &'a str,
    /// The command that prints the newest failed scan's log lines on the daemon's host, when the
    /// daemon's logs explain the failure and the scan can be found in them.
    pub log_command: Option<&'a str>,
}

impl Email for DaemonScansFailing<'_> {
    fn subject(&self) -> String {
        "Your Daemon's Scans Keep Failing".to_string()
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::DaemonAlerts
    }

    fn campaign(&self) -> &'static str {
        "daemon_scans_failing"
    }

    fn body_html(&self) -> String {
        let mut content = Content::new()
            .heading("Daemon Scans Keep Failing")
                    .paragraph("Hi there,")
                    .paragraph(&format!(
                        "The last {} scans on your daemon <strong>{}</strong> on <strong>{}</strong> failed ({}). Scanopy has no new results from this daemon until a scan completes.",
                        self.failed_runs,
                        self.daemon_name,
                        self.site_name,
                        self.discovery_names.join(", ")
                    ))
                    .paragraph(&format!(
                        "The latest scan ended with <strong>{}</strong>: {}",
                        self.reason_name, self.reason_description
                    ));
        if let Some(command) = self.log_command {
            content = content
                .paragraph("To see the latest scan's log lines, run this on the daemon's host:")
                .code(command);
        }
        let content = content.paragraph(&format!(
            r#"The <a href="{}" style="color: #2563eb; text-decoration: none;">troubleshooting guide</a> covers the common causes."#,
            links::DOCS_DAEMON_TROUBLESHOOTING
        ));
        Body::new()
            .content(content)
            .cta(links::APP_DAEMONS, "View Daemons")
            .render()
    }
}
