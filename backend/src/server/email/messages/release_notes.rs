use super::{Body, Content, Email, EmailCategory, compose::html_escape, links};
use crate::server::release_notes::catalog::ReleaseNote;

/// Sent to every user when the server starts on a release marked for email.
/// Carries that release's notes only.
pub struct ReleaseNotes<'a> {
    pub note: &'a ReleaseNote,
}

/// The subject both release emails share.
pub(super) fn subject(note: &ReleaseNote) -> String {
    format!("Scanopy {}: {}", note.version, note.title)
}

/// `note`'s title, then each section of its notes closed by a secondary link
/// to `section_cta`, a `(href, label)` pair.
pub(super) fn note_content(note: &ReleaseNote, section_cta: (&str, &str)) -> Content {
    let (href, label) = section_cta;
    let mut content = Content::new()
        .heading("What's new in Scanopy")
        .subheading(&format!(
            "{} <span style=\"font-weight: 400; color: #9ca3af;\">v{}</span>",
            html_escape(&note.title),
            note.version
        ));
    for section in &note.sections {
        content = content
            .raw(&format!(
                "<div style=\"margin: 0 0 12px 0; font-size: 16px; line-height: 24px; color: #4a4a4a;\">{}</div>\n",
                email_images(section)
            ))
            .secondary_cta(href, label);
    }
    content
}

impl Email for ReleaseNotes<'_> {
    fn subject(&self) -> String {
        subject(self.note)
    }

    fn category(&self) -> EmailCategory {
        EmailCategory::ProductUpdates
    }

    fn campaign(&self) -> &'static str {
        "release_notes"
    }

    fn body_html(&self) -> String {
        Body::new()
            .content(note_content(
                self.note,
                (links::APP_HOME, "See it in Scanopy"),
            ))
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

    /// Every section of the notes, the lead-in included, closes with its own
    /// link, in order, before the next section starts.
    #[test]
    fn each_section_closes_with_a_link() {
        let note = ReleaseNote::parse(
            "---\nversion: 0.18.0\ndate: 2026-10-11\ntitle: 'T'\nemail: true\n---\n\nLead-in.\n\n## New Features\n\n- One\n\n## Fixes\n\n- Two\n",
        )
        .unwrap();
        let link = "See it in Scanopy";
        let html = ReleaseNotes { note: &note }.body_html();
        let found = ["Lead-in.", link, "New Features", link, "Fixes", link]
            .iter()
            .scan(0, |from, needle| {
                let at = *from + html[*from..].find(needle)?;
                *from = at + needle.len();
                Some(at)
            })
            .count();
        assert_eq!(found, 6, "sections and links interleave: {html}");
        assert_eq!(html.matches(link).count(), 3);
    }

    #[test]
    fn an_image_fits_the_content_column() {
        let note = ReleaseNote::parse(&note_with_body(
            "![The topology view](https://scanopy.net/changelog/v0.18.0/topology.png)\n\nText with `<img ` in it.",
        ))
        .unwrap();
        let html = ReleaseNotes { note: &note }.body_html();
        assert_eq!(
            html.matches(&format!("width=\"{CONTENT_WIDTH}\"")).count(),
            1
        );
    }
}
