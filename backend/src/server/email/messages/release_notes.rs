use super::{Body, Content, Email, EmailCategory, compose::html_escape, links};
use crate::server::release_notes::catalog::{CHANGELOG_URL, ReleaseNote};

/// Sent to every user when the server starts on a release marked for email.
/// Carries the notes of every release since the last one emailed, newest first.
pub struct ReleaseNotes<'a> {
    /// Newest first, at most `catalog::MAX_SHOWN`.
    pub notes: &'a [&'a ReleaseNote],
    /// Whether older releases were left out for length.
    pub truncated: bool,
}

impl Email for ReleaseNotes<'_> {
    fn subject(&self) -> String {
        match self.notes.first() {
            Some(newest) => format!("Scanopy {}: {}", newest.version, newest.title),
            None => "What's new in Scanopy".to_string(),
        }
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::ProductUpdates
    }

    fn campaign(&self) -> &'static str {
        "release_notes"
    }

    fn body_html(&self) -> String {
        let mut content = Content::new().heading("What's new in Scanopy");
        for note in self.notes {
            content = content
                .subheading(&format!(
                    "{} <span style=\"font-weight: 400; color: #9ca3af;\">v{}</span>",
                    html_escape(&note.title),
                    note.version
                ))
                .raw(&format!(
                    "<div style=\"margin: 0 0 24px 0; font-size: 16px; line-height: 24px; color: #4a4a4a;\">{}</div>\n",
                    email_images(&note.html)
                ));
        }
        if self.truncated {
            content = content.paragraph(&format!(
                "Earlier releases are in the <a href=\"{CHANGELOG_URL}\">changelog</a>."
            ));
        }
        Body::new()
            .content(content)
            .cta(links::APP_HOME, "Open Scanopy")
            .render()
    }
}

/// The width of the email's content column: the 600px card less its 40px padding.
const CONTENT_WIDTH: u32 = 520;

/// Size each image to the content column. Outlook desktop ignores CSS
/// `max-width` and honours the `width` attribute; every other client scales
/// down to a narrow screen with `max-width`. The rendered notes carry no raw
/// HTML, so every `<img ` is a markdown image.
fn email_images(html: &str) -> String {
    html.replace(
        "<img ",
        &format!(
            "<img width=\"{CONTENT_WIDTH}\" style=\"display: block; width: 100%; max-width: {CONTENT_WIDTH}px; height: auto; margin: 16px 0; border: 0; border-radius: 6px;\" "
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::release_notes::catalog::tests::note_with_body;

    #[test]
    fn an_image_fits_the_content_column() {
        let note = ReleaseNote::parse(&note_with_body(
            "![The topology view](https://scanopy.net/changelog/v0.18.0/topology.png)\n\nText with `<img ` in it.",
        ))
        .unwrap();
        let html = ReleaseNotes {
            notes: &[&note],
            truncated: false,
        }
        .body_html();
        assert_eq!(
            html.matches(&format!("width=\"{CONTENT_WIDTH}\"")).count(),
            1
        );
    }
}
