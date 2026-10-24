//! Where every logo Scanopy draws comes from, and on what terms.
//!
//! The sources are listed here once; which logos each one supplies is read from the `logo_url`s
//! of the service definitions, OS families and identity providers. `generate-fixtures` writes the
//! result to `logo-credits.json`, which Settings > System lists. A logo from a source missing here
//! fails `every_logo_has_a_credited_source`.

use crate::server::auth::r#impl::oidc::IdentityProviderLogo;
use crate::server::hosts::r#impl::os::HostOsFamily;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::shared::types::metadata::TypeMetadataProvider;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize)]
pub struct LogoSource {
    pub name: &'static str,
    pub homepage: &'static str,
    pub copyright: &'static str,
    /// The license the logos are used under. None for a company's own mark, used under no license
    /// to name its product: the UI says it is a trademark of its owner.
    pub license: Option<&'static str>,
    pub license_url: Option<&'static str>,
    /// `logo_url` prefixes this source serves.
    #[serde(skip)]
    prefixes: &'static [&'static str],
    /// What it supplies that no `logo_url` names: static files and email images.
    #[serde(skip)]
    fixed_uses: &'static [&'static str],
}

/// Scanopy's own logo, which needs no credit.
const OWN_LOGOS: &[&str] = &[
    "https://cdn.jsdelivr.net/gh/scanopy/scanopy@main/media/",
    "/logos/scanopy-logo.png",
];

pub const LOGO_SOURCES: &[LogoSource] = &[
    LogoSource {
        name: "Dashboard Icons",
        homepage: "https://github.com/homarr-labs/dashboard-icons",
        copyright: "2024 Bjorn Lammers, Meier Lukas, Thomas Camlong and Homarr Labs",
        license: Some("Apache License 2.0"),
        license_url: Some(
            "https://github.com/scanopy/scanopy/blob/main/backend/assets/dashboard-icons/LICENSE",
        ),
        prefixes: &["https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "selfh.st/icons",
        homepage: "https://github.com/selfhst/icons",
        copyright: "selfh.st",
        license: Some("CC BY 4.0"),
        license_url: Some("https://github.com/selfhst/icons/blob/main/LICENSE"),
        prefixes: &["https://cdn.jsdelivr.net/gh/selfhst/icons"],
        // The Discord and GitHub links in emails (`email/messages`).
        fixed_uses: &["Discord", "GitHub"],
    },
    LogoSource {
        name: "Simple Icons",
        homepage: "https://github.com/simple-icons/simple-icons",
        copyright: "Simple Icons contributors",
        license: Some("CC0 1.0"),
        license_url: Some("https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md"),
        prefixes: &[
            "https://simpleicons.org/icons/",
            "https://cdn.jsdelivr.net/npm/simple-icons",
        ],
        fixed_uses: &[],
    },
    LogoSource {
        name: "libvirt",
        homepage: "https://libvirt.org/logos/",
        copyright: "The libvirt project authors",
        license: Some("GNU LGPL 2.1 or later"),
        license_url: Some(
            "https://github.com/scanopy/scanopy/blob/main/backend/assets/libvirt/LICENSE",
        ),
        prefixes: &["https://libvirt.org/logos/"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "Tux",
        homepage: "https://commons.wikimedia.org/wiki/File:Tux.svg",
        copyright: "Larry Ewing, Simon Budig and Garrett LeSage",
        license: Some("Use with acknowledgement"),
        license_url: Some("https://commons.wikimedia.org/wiki/File:Tux.svg"),
        // `ui/static/logos/tux.png`, the Linux icon (`HostOsFamily::icon_origin_url`).
        prefixes: &["https://commons.wikimedia.org/wiki/File:Tux.svg"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "Windows logo",
        homepage: "https://commons.wikimedia.org/wiki/File:Windows_logo_-_2021.svg",
        copyright: "Microsoft",
        license: Some("Public domain"),
        license_url: Some("https://commons.wikimedia.org/wiki/File:Windows_logo_-_2021.svg"),
        // `ui/static/logos/windows.svg`, the Windows icon (`HostOsFamily::icon_origin_url`).
        prefixes: &["https://commons.wikimedia.org/wiki/File:Windows_logo_-_2021.svg"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "VectorLogoZone",
        homepage: "https://www.vectorlogo.zone/",
        copyright: "The respective trademark owners",
        license: None,
        license_url: None,
        prefixes: &["https://www.vectorlogo.zone/logos/"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "Rollease Acmeda",
        homepage: "https://www.rolleaseacmeda.com/",
        copyright: "Rollease Acmeda",
        license: None,
        license_url: None,
        prefixes: &["https://www.rolleaseacmeda.com/"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "Chamberlain Group (myQ)",
        homepage: "https://www.myq.com/",
        copyright: "The Chamberlain Group",
        license: None,
        license_url: None,
        prefixes: &["https://www.myq.com/"],
        fixed_uses: &[],
    },
    LogoSource {
        name: "Roborock",
        homepage: "https://us.roborock.com/",
        copyright: "Roborock",
        license: None,
        license_url: None,
        // Roborock's site serves its assets from this Webflow bucket.
        prefixes: &["https://cdn.prod.website-files.com/6720fa7290112f642d0a83ac/"],
        fixed_uses: &[],
    },
];

/// The source a logo URL comes from. None for Scanopy's own logo and for a URL no source serves.
pub fn logo_source(url: &str) -> Option<&'static LogoSource> {
    LOGO_SOURCES
        .iter()
        .find(|source| source.prefixes.iter().any(|p| url.starts_with(p)))
}

fn is_own_logo(url: &str) -> bool {
    OWN_LOGOS.iter().any(|p| url.starts_with(p))
}

/// Every logo Scanopy draws from a URL, by the name of what it stands for.
fn named_logo_urls() -> Vec<(&'static str, &'static str)> {
    let services = ServiceDefinitionRegistry::all_service_definitions()
        .into_iter()
        .map(|s| (ServiceDefinition::name(&s), s.logo_url()));
    let os_families =
        HostOsFamily::iter().map(|f| (TypeMetadataProvider::name(&f), f.icon_origin_url()));
    let identity_providers = IdentityProviderLogo::iter().map(|p| (p.name(), p.logo_url()));
    services
        .chain(os_families)
        .chain(identity_providers)
        .filter(|(_, url)| !url.is_empty() && !is_own_logo(url))
        .collect()
}

#[derive(Debug, Serialize)]
pub struct LogoCredit {
    #[serde(flatten)]
    pub source: &'static LogoSource,
    /// What its logos stand for, sorted and without repeats.
    pub used_for: Vec<&'static str>,
}

/// Each source with what it supplies, in `LOGO_SOURCES` order.
pub fn logo_credits() -> Vec<LogoCredit> {
    let logos = named_logo_urls();
    LOGO_SOURCES
        .iter()
        .map(|source| {
            let mut used_for: Vec<&'static str> = logos
                .iter()
                .filter(|(_, url)| logo_source(url).is_some_and(|s| std::ptr::eq(s, source)))
                .map(|(name, _)| *name)
                .chain(source.fixed_uses.iter().copied())
                .collect();
            used_for.sort_unstable_by_key(|name| name.to_lowercase());
            used_for.dedup();
            LogoCredit { source, used_for }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_logo_has_a_credited_source() {
        let uncredited: Vec<_> = named_logo_urls()
            .into_iter()
            .filter(|(_, url)| logo_source(url).is_none())
            .collect();
        assert!(
            uncredited.is_empty(),
            "logos from a source missing from LOGO_SOURCES: {uncredited:?}"
        );
    }

    #[test]
    fn every_listed_source_supplies_a_logo() {
        let unused: Vec<&str> = logo_credits()
            .iter()
            .filter(|credit| credit.used_for.is_empty())
            .map(|credit| credit.source.name)
            .collect();
        assert!(unused.is_empty(), "sources no logo comes from: {unused:?}");
    }
}
