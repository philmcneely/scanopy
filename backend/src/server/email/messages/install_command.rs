use super::{Body, Content, Email, EmailCategory, links};

/// Delivers a ready-to-run daemon install command for the recipient's OS.
pub struct InstallCommand<'a> {
    pub install_command: &'a str,
    pub os: &'a str,
}

impl Email for InstallCommand<'_> {
    fn subject(&self) -> String {
        "Your Daemon Install Command".to_string()
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::DaemonSetup
    }

    fn campaign(&self) -> &'static str {
        "install_command"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(
                Content::new()
                    .heading("Install Command")
                    .paragraph("Hi there,")
                    .paragraph(&format!(
                        "Here's the daemon install command you requested. Run it on your {} machine to set up your Scanopy daemon.",
                        self.os
                    ))
                    .code(self.install_command)
                    .raw(
                        r#"                            <p style="margin: 0 0 10px 0; font-size: 14px; line-height: 20px; color: #6b7280;">Copy and paste this command into your terminal. The daemon will download, install, and start automatically.</p>
"#,
                    ),
            )
            .cta(links::APP_HOME, "Open Scanopy")
            .render()
    }
}
