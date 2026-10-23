use super::{Body, Email, EmailCategory, release_notes};
use crate::server::release_notes::catalog::ReleaseNote;

/// How to upgrade a self-hosted server.
const UPGRADE_DOCS: &str = "https://scanopy.net/docs/self-hosted-server/upgrading/";

/// Sent by Scanopy Cloud to the owners of a current self-hosted licence when a
/// release is emailed, in place of [`super::ReleaseNotes`]. Their own servers
/// run elsewhere and are still on the previous release, so every link leads
/// to the upgrade guide.
pub struct LicensedServerUpgrade<'a> {
    pub note: &'a ReleaseNote,
}

impl Email for LicensedServerUpgrade<'_> {
    fn subject(&self) -> String {
        release_notes::subject(self.note)
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::ProductUpdates
    }

    fn campaign(&self) -> &'static str {
        "licensed_server_upgrade"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(release_notes::note_content(
                self.note,
                (UPGRADE_DOCS, "Upgrade to get this"),
            ))
            .cta(UPGRADE_DOCS, "Upgrade your server")
            .alt_link(UPGRADE_DOCS)
            .render()
    }
}
