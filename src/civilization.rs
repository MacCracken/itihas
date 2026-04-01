//! Major civilizations with geographic extent, peak period, and key traits.
//!
//! Provides [`Civilization`] structs, pre-built data for 10 major world
//! civilizations, and lookup functions by region and active year.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// A historical civilization.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Civilization {
    /// Name of the civilization.
    pub name: Cow<'static, str>,
    /// Primary geographic region.
    pub region: Cow<'static, str>,
    /// Era name during peak influence.
    pub peak_era: Cow<'static, str>,
    /// Approximate founding year (negative = BCE).
    pub founding_year: i32,
    /// Approximate end year (negative = BCE). `i32::MAX` for ongoing influence.
    pub end_year: i32,
    /// Key cultural/technological traits.
    pub traits: Vec<Cow<'static, str>>,
    /// Primary writing system or script.
    pub script: Cow<'static, str>,
    /// ISO 639 language codes associated with this civilization.
    pub language_codes: Vec<Cow<'static, str>>,
}

impl fmt::Display for Civilization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {} – {})",
            self.name, self.region, self.founding_year, self.end_year
        )
    }
}

fn build_civilizations() -> Vec<Civilization> {
    vec![
        Civilization {
            name: Cow::Borrowed("Mesopotamia"),
            region: Cow::Borrowed("Near East"),
            peak_era: Cow::Borrowed("Bronze Age"),
            founding_year: -3500,
            end_year: -539,
            traits: vec![
                Cow::Borrowed("cuneiform writing"),
                Cow::Borrowed("irrigation"),
                Cow::Borrowed("law codes"),
                Cow::Borrowed("astronomy"),
            ],
            script: Cow::Borrowed("Cuneiform"),
            language_codes: vec![Cow::Borrowed("akk"), Cow::Borrowed("sux")],
        },
        Civilization {
            name: Cow::Borrowed("Ancient Egypt"),
            region: Cow::Borrowed("North Africa"),
            peak_era: Cow::Borrowed("Bronze Age"),
            founding_year: -3100,
            end_year: -30,
            traits: vec![
                Cow::Borrowed("hieroglyphic writing"),
                Cow::Borrowed("pyramid construction"),
                Cow::Borrowed("mummification"),
                Cow::Borrowed("astronomy"),
            ],
            script: Cow::Borrowed("Hieroglyphic"),
            language_codes: vec![Cow::Borrowed("egy")],
        },
        Civilization {
            name: Cow::Borrowed("Indus Valley"),
            region: Cow::Borrowed("South Asia"),
            peak_era: Cow::Borrowed("Bronze Age"),
            founding_year: -3300,
            end_year: -1300,
            traits: vec![
                Cow::Borrowed("urban planning"),
                Cow::Borrowed("standardized weights"),
                Cow::Borrowed("drainage systems"),
                Cow::Borrowed("trade networks"),
            ],
            script: Cow::Borrowed("Indus script"),
            language_codes: vec![],
        },
        Civilization {
            name: Cow::Borrowed("Ancient China"),
            region: Cow::Borrowed("East Asia"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -2070,
            end_year: 1912,
            traits: vec![
                Cow::Borrowed("paper"),
                Cow::Borrowed("gunpowder"),
                Cow::Borrowed("compass"),
                Cow::Borrowed("printing"),
            ],
            script: Cow::Borrowed("Chinese characters"),
            language_codes: vec![Cow::Borrowed("zh")],
        },
        Civilization {
            name: Cow::Borrowed("Ancient Greece"),
            region: Cow::Borrowed("Mediterranean"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -800,
            end_year: -146,
            traits: vec![
                Cow::Borrowed("democracy"),
                Cow::Borrowed("philosophy"),
                Cow::Borrowed("theater"),
                Cow::Borrowed("mathematics"),
            ],
            script: Cow::Borrowed("Greek alphabet"),
            language_codes: vec![Cow::Borrowed("grc")],
        },
        Civilization {
            name: Cow::Borrowed("Roman Empire"),
            region: Cow::Borrowed("Mediterranean, Europe"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -753,
            end_year: 476,
            traits: vec![
                Cow::Borrowed("engineering"),
                Cow::Borrowed("law"),
                Cow::Borrowed("roads"),
                Cow::Borrowed("aqueducts"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("la")],
        },
        Civilization {
            name: Cow::Borrowed("Maya"),
            region: Cow::Borrowed("Mesoamerica"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -2000,
            end_year: 1500,
            traits: vec![
                Cow::Borrowed("hieroglyphic writing"),
                Cow::Borrowed("calendar systems"),
                Cow::Borrowed("astronomy"),
                Cow::Borrowed("pyramids"),
            ],
            script: Cow::Borrowed("Maya script"),
            language_codes: vec![Cow::Borrowed("yua")],
        },
        Civilization {
            name: Cow::Borrowed("Persian Empire"),
            region: Cow::Borrowed("Near East, Central Asia"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -550,
            end_year: -330,
            traits: vec![
                Cow::Borrowed("postal system"),
                Cow::Borrowed("road network"),
                Cow::Borrowed("religious tolerance"),
                Cow::Borrowed("administration"),
            ],
            script: Cow::Borrowed("Old Persian cuneiform"),
            language_codes: vec![Cow::Borrowed("peo")],
        },
        Civilization {
            name: Cow::Borrowed("Mongol Empire"),
            region: Cow::Borrowed("Central Asia, East Asia, Europe"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1206,
            end_year: 1368,
            traits: vec![
                Cow::Borrowed("cavalry warfare"),
                Cow::Borrowed("Silk Road trade"),
                Cow::Borrowed("religious tolerance"),
                Cow::Borrowed("postal relay"),
            ],
            script: Cow::Borrowed("Mongolian script"),
            language_codes: vec![Cow::Borrowed("mn")],
        },
        Civilization {
            name: Cow::Borrowed("Ottoman Empire"),
            region: Cow::Borrowed("Near East, Southeast Europe, North Africa"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1299,
            end_year: 1922,
            traits: vec![
                Cow::Borrowed("millet system"),
                Cow::Borrowed("architecture"),
                Cow::Borrowed("military innovation"),
                Cow::Borrowed("trade networks"),
            ],
            script: Cow::Borrowed("Ottoman Turkish script"),
            language_codes: vec![Cow::Borrowed("ota")],
        },
    ]
}

/// Returns all pre-built civilizations.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_civilizations() -> &'static [Civilization] {
    static DATA: std::sync::LazyLock<Vec<Civilization>> =
        std::sync::LazyLock::new(build_civilizations);
    &DATA
}

/// Returns all pre-built civilizations.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_civilizations() -> Vec<Civilization> {
    build_civilizations()
}

/// Returns civilizations whose region contains the given substring (case-insensitive).
#[must_use]
#[inline]
pub fn by_region(region: &str) -> Vec<Civilization> {
    tracing::debug!(region, "looking up civilizations by region");
    let lower = region.to_lowercase();
    all_civilizations()
        .iter()
        .filter(|c| c.region.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

/// Returns civilizations that were active during the given year.
///
/// A civilization is considered active if `founding_year <= year <= end_year`.
#[must_use]
#[inline]
pub fn active_at(year: i32) -> Vec<Civilization> {
    tracing::debug!(year, "looking up civilizations active at year");
    all_civilizations()
        .iter()
        .filter(|c| year >= c.founding_year && year <= c.end_year)
        .cloned()
        .collect()
}

/// Look up a civilization by exact name (case-insensitive).
///
/// Returns `Err(ItihasError::UnknownCivilization)` if no civilization matches.
#[inline]
pub fn by_name(name: &str) -> Result<Civilization, ItihasError> {
    tracing::debug!(name, "looking up civilization by name");
    let lower = name.to_lowercase();
    all_civilizations()
        .iter()
        .find(|c| c.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::UnknownCivilization(String::from(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_civilizations_count() {
        assert_eq!(all_civilizations().len(), 10);
    }

    #[test]
    fn test_by_region_near_east() {
        let civs = by_region("Near East");
        assert!(civs.iter().any(|c| c.name == "Mesopotamia"));
        assert!(civs.iter().any(|c| c.name == "Persian Empire"));
    }

    #[test]
    fn test_by_region_case_insensitive() {
        let civs = by_region("mediterranean");
        assert!(civs.iter().any(|c| c.name == "Ancient Greece"));
        assert!(civs.iter().any(|c| c.name == "Roman Empire"));
    }

    #[test]
    fn test_by_region_no_match() {
        let civs = by_region("Antarctica");
        assert!(civs.is_empty());
    }

    #[test]
    fn test_active_at_500_bce() {
        let civs = active_at(-500);
        let names: Vec<_> = civs.iter().map(|c| c.name.as_ref()).collect();
        assert!(names.contains(&"Ancient Egypt"));
        assert!(names.contains(&"Ancient Greece"));
        assert!(names.contains(&"Persian Empire"));
        assert!(names.contains(&"Ancient China"));
    }

    #[test]
    fn test_active_at_1300_ce() {
        let civs = active_at(1300);
        let names: Vec<_> = civs.iter().map(|c| c.name.as_ref()).collect();
        assert!(names.contains(&"Mongol Empire"));
        assert!(names.contains(&"Ottoman Empire"));
        assert!(names.contains(&"Maya"));
    }

    #[test]
    fn test_active_at_none() {
        let civs = active_at(-100_000);
        assert!(civs.is_empty());
    }

    #[test]
    fn test_civilization_serde_roundtrip() {
        for civ in all_civilizations() {
            let json = serde_json::to_string(civ).unwrap();
            let back: Civilization = serde_json::from_str(&json).unwrap();
            assert_eq!(*civ, back);
        }
    }
}
