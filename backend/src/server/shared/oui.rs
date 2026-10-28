//! MAC vendor lookup from the IEEE registries.
//!
//! IEEE assigns MAC blocks at three sizes: MA-L (24-bit prefix, `oui.csv`), MA-M (28-bit,
//! `mam.csv`) and MA-S (36-bit, `oui36.csv`), plus the retired IAB blocks (36-bit, `iab.csv`). The
//! MA-L rows that hold the smaller blocks name "IEEE Registration Authority", not the maker, so a
//! lookup takes the longest assigned prefix. CIDs are not loaded: they sit in locally administered
//! space, which never names a vendor.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Each registry, longest prefix first, which is the order a lookup tries them in.
const REGISTRIES: [&str; 4] = [
    include_str!("../../../assets/oui36.csv"),
    include_str!("../../../assets/iab.csv"),
    include_str!("../../../assets/mam.csv"),
    include_str!("../../../assets/oui.csv"),
];

/// Assignment lengths in hex digits: MA-S and IAB, MA-M, MA-L.
const PREFIX_LENGTHS: [usize; 3] = [9, 7, 6];

#[derive(Debug, Clone, Deserialize)]
struct OuiRecord {
    #[serde(rename = "Registry")]
    _registry: String,
    #[serde(rename = "Assignment")]
    assignment: String,
    #[serde(rename = "Organization Name")]
    organization_name: String,
    #[serde(rename = "Organization Address")]
    _organization_address: String,
}

#[derive(Debug, Clone)]
pub struct OuiEntry {
    pub company_name: String,
}

/// Keyed by the assignment's uppercase hex digits, whose length says which registry it came from.
type OuiDatabase = HashMap<String, OuiEntry>;

static OUI_DB: OnceLock<OuiDatabase> = OnceLock::new();

/// An assignment as uppercase hex, if it is one of the lengths IEEE assigns.
fn parse_assignment(hex: &str) -> Option<String> {
    let hex = hex.trim();
    (PREFIX_LENGTHS.contains(&hex.len()) && hex.chars().all(|c| c.is_ascii_hexdigit()))
        .then(|| hex.to_ascii_uppercase())
}

fn load_database() -> OuiDatabase {
    let mut db = HashMap::new();

    for csv in REGISTRIES {
        let mut reader = csv::Reader::from_reader(csv.as_bytes());
        for result in reader.deserialize::<OuiRecord>() {
            let Ok(record) = result else {
                continue;
            };
            let Some(assignment) = parse_assignment(&record.assignment) else {
                continue;
            };
            db.insert(
                assignment,
                OuiEntry {
                    company_name: record.organization_name,
                },
            );
        }
    }

    db
}

fn get_database() -> &'static OuiDatabase {
    OUI_DB.get_or_init(load_database)
}

/// Look up the vendor for a MAC address: the holder of the longest IEEE block that contains it.
/// Supports formats: "AA:BB:CC:DD:EE:FF", "AA-BB-CC-DD-EE-FF", "AABBCCDDEEFF", "AA:BB:CC"
pub fn lookup_by_mac(mac: &str) -> Option<&'static OuiEntry> {
    let hex: String = mac
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    let db = get_database();
    PREFIX_LENGTHS
        .iter()
        .filter(|&&len| hex.len() >= len)
        .find_map(|&len| db.get(&hex[..len]))
}

/// Return the number of entries in the OUI database.
pub fn entry_count() -> usize {
    get_database().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oui_database_loads() {
        let count = entry_count();
        assert!(
            count >= 30_000,
            "Expected at least 30,000 OUI entries, got {count}"
        );
    }

    #[test]
    fn test_known_mac_lookups() {
        // Apple owns many OUI prefixes; 3C:22:FB is one
        let entry = lookup_by_mac("3C:22:FB:00:00:00");
        assert!(entry.is_some(), "Expected to find Apple OUI");
        assert!(
            entry.unwrap().company_name.to_lowercase().contains("apple"),
            "Expected Apple, got: {}",
            entry.unwrap().company_name
        );
    }

    #[test]
    fn test_vendor_constants_in_database() {
        use crate::server::services::r#impl::patterns::Vendor;

        let normalize = |s: &str| -> String {
            s.trim()
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect()
        };

        let db = get_database();

        let vendors = [
            Vendor::PHILIPS,
            Vendor::HP,
            Vendor::EERO,
            Vendor::TPLINK,
            Vendor::UBIQUITI,
            Vendor::GOOGLE,
            Vendor::NEST,
            Vendor::AMAZON,
            Vendor::SONOS,
            Vendor::ECOBEE,
            Vendor::ROKU,
            Vendor::ROBOROCK,
        ];

        for vendor in vendors {
            let normalized_vendor = normalize(vendor);
            let found = db
                .values()
                .any(|entry| normalize(&entry.company_name) == normalized_vendor);
            assert!(
                found,
                "Vendor constant '{}' not found in OUI database",
                vendor
            );
        }
    }

    #[test]
    fn a_mac_in_a_smaller_block_resolves_to_the_block_holder() {
        // C8:5C:E2 is an MA-L that IEEE holds and sells on as MA-M blocks; 8C:1F:64 the same as
        // MA-S blocks. Each MAC names the maker that bought the block, not the registrar.
        for mac in ["C8:5C:E2:70:00:01", "8C:1F:64:AF:A0:01"] {
            let entry = lookup_by_mac(mac).expect("assigned block");
            assert_ne!(entry.company_name, "IEEE Registration Authority", "{mac}");
        }
    }

    #[test]
    fn test_unknown_mac_returns_none() {
        // These prefixes may or may not be assigned; just verify no panic
        let _ = lookup_by_mac("00:00:00:00:00:00");
        let _ = lookup_by_mac("FF:FF:FF:FF:FF:FF");
        // Test with too-short input
        assert!(lookup_by_mac("AA:BB").is_none());
        assert!(lookup_by_mac("").is_none());
    }

    #[test]
    fn test_mac_format_handling() {
        // All these formats should parse to the same prefix and return the same result
        let colon = lookup_by_mac("3C:22:FB:00:00:00");
        let dash = lookup_by_mac("3C-22-FB-00-00-00");
        let bare = lookup_by_mac("3C22FB000000");
        let short = lookup_by_mac("3C:22:FB");

        assert!(colon.is_some());
        assert_eq!(
            colon.unwrap().company_name,
            dash.unwrap().company_name,
            "Dash format should match colon format"
        );
        assert_eq!(
            colon.unwrap().company_name,
            bare.unwrap().company_name,
            "Bare hex format should match colon format"
        );
        assert_eq!(
            colon.unwrap().company_name,
            short.unwrap().company_name,
            "Short format should match colon format"
        );
    }
}
