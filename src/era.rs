//! Historical periods with date ranges and civilizational phases.
//!
//! Provides [`Era`] structs representing major historical periods, an
//! [`EraCategory`] classification enum, and pre-built eras from the Bronze Age
//! to the Information Age.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// Classification of historical periods.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EraCategory {
    /// Earliest civilizations, pre-literacy to early writing (~3300 BCE - 800 BCE).
    Ancient,
    /// Greco-Roman flowering (~800 BCE - 500 CE).
    Classical,
    /// Post-Roman European period (~500 - 1500 CE).
    Medieval,
    /// Renaissance through Enlightenment (~1400 - 1800 CE).
    EarlyModern,
    /// Industrial Revolution through World Wars (~1760 - 1970 CE).
    Modern,
    /// Post-WWII to present (~1945 - present).
    Contemporary,
}

impl fmt::Display for EraCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ancient => f.write_str("Ancient"),
            Self::Classical => f.write_str("Classical"),
            Self::Medieval => f.write_str("Medieval"),
            Self::EarlyModern => f.write_str("Early Modern"),
            Self::Modern => f.write_str("Modern"),
            Self::Contemporary => f.write_str("Contemporary"),
        }
    }
}

/// A historical era with date range and region.
///
/// Years use astronomical year numbering: negative values represent BCE
/// (e.g., -3300 = 3300 BCE), positive values represent CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Era {
    /// Name of the era.
    pub name: Cow<'static, str>,
    /// Start year (negative = BCE).
    pub start_year: i32,
    /// End year (negative = BCE). `i32::MAX` means ongoing.
    pub end_year: i32,
    /// Geographic region or scope.
    pub region: Cow<'static, str>,
    /// Brief description.
    pub description: Cow<'static, str>,
    /// Category classification.
    pub category: EraCategory,
}

impl fmt::Display for Era {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.end_year == i32::MAX {
            write!(f, "{} ({} – present)", self.name, self.start_year)
        } else {
            write!(f, "{} ({} – {})", self.name, self.start_year, self.end_year)
        }
    }
}

impl PartialOrd for Era {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Era {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_year
            .cmp(&other.start_year)
            .then_with(|| self.end_year.cmp(&other.end_year))
    }
}

fn build_eras() -> Vec<Era> {
    vec![
        Era {
            name: Cow::Borrowed("Bronze Age"),
            start_year: -3500,
            end_year: -1100,
            region: Cow::Borrowed("Near East, Mediterranean, South Asia"),
            description: Cow::Borrowed(
                "Emergence of bronze metallurgy, early writing systems, and first cities",
            ),
            category: EraCategory::Ancient,
        },
        Era {
            name: Cow::Borrowed("Iron Age"),
            start_year: -1200,
            end_year: -550,
            region: Cow::Borrowed("Near East, Mediterranean, South Asia"),
            description: Cow::Borrowed(
                "Widespread iron smelting, alphabetic writing, rise of empires",
            ),
            category: EraCategory::Ancient,
        },
        Era {
            name: Cow::Borrowed("Classical Antiquity"),
            start_year: -800,
            end_year: 476,
            region: Cow::Borrowed("Mediterranean, Europe"),
            description: Cow::Borrowed(
                "Greek and Roman civilization, philosophy, democracy, republic",
            ),
            category: EraCategory::Classical,
        },
        Era {
            name: Cow::Borrowed("Middle Ages"),
            start_year: 476,
            end_year: 1453,
            region: Cow::Borrowed("Europe, Near East"),
            description: Cow::Borrowed("Feudalism, monasticism, Islamic Golden Age, Crusades"),
            category: EraCategory::Medieval,
        },
        Era {
            name: Cow::Borrowed("Renaissance"),
            start_year: 1400,
            end_year: 1600,
            region: Cow::Borrowed("Europe"),
            description: Cow::Borrowed(
                "Revival of classical learning, art, science, and exploration",
            ),
            category: EraCategory::EarlyModern,
        },
        Era {
            name: Cow::Borrowed("Age of Enlightenment"),
            start_year: 1600,
            end_year: 1789,
            region: Cow::Borrowed("Europe"),
            description: Cow::Borrowed(
                "Scientific Revolution, rationalism, empiricism, social contract theory",
            ),
            category: EraCategory::EarlyModern,
        },
        Era {
            name: Cow::Borrowed("Industrial Age"),
            start_year: 1760,
            end_year: 1970,
            region: Cow::Borrowed("Global"),
            description: Cow::Borrowed("Mechanization, mass production, urbanization, world wars"),
            category: EraCategory::Modern,
        },
        Era {
            name: Cow::Borrowed("Information Age"),
            start_year: 1970,
            end_year: i32::MAX,
            region: Cow::Borrowed("Global"),
            description: Cow::Borrowed("Digital revolution, internet, AI, globalization"),
            category: EraCategory::Contemporary,
        },
    ]
}

/// Returns all pre-built eras.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_eras() -> &'static [Era] {
    static DATA: std::sync::LazyLock<Vec<Era>> = std::sync::LazyLock::new(build_eras);
    &DATA
}

/// Returns all pre-built eras.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_eras() -> Vec<Era> {
    build_eras()
}

/// Returns all eras that contain the given year.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[must_use]
#[inline]
pub fn eras_containing(year: i32) -> Vec<Era> {
    tracing::debug!(year, "looking up eras containing year");
    all_eras()
        .iter()
        .filter(|e| year >= e.start_year && year <= e.end_year)
        .cloned()
        .collect()
}

/// Look up an era by exact name (case-insensitive).
///
/// Returns `Err(ItihasError::UnknownEra)` if no era matches.
#[inline]
pub fn by_name(name: &str) -> Result<Era, ItihasError> {
    tracing::debug!(name, "looking up era by name");
    let lower = name.to_lowercase();
    all_eras()
        .iter()
        .find(|e| e.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::UnknownEra(String::from(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_eras_count() {
        assert_eq!(all_eras().len(), 8);
    }

    #[test]
    fn test_eras_containing_bronze_age() {
        let eras = eras_containing(-2000);
        assert!(eras.iter().any(|e| e.name == "Bronze Age"));
    }

    #[test]
    fn test_eras_containing_classical() {
        let eras = eras_containing(100);
        assert!(eras.iter().any(|e| e.name == "Classical Antiquity"));
    }

    #[test]
    fn test_eras_containing_overlap() {
        // 1500 CE is in both Middle Ages (ends 1453) and Renaissance (starts 1400)
        let eras = eras_containing(1450);
        let names: Vec<_> = eras.iter().map(|e| e.name.as_ref()).collect();
        assert!(names.contains(&"Middle Ages"));
        assert!(names.contains(&"Renaissance"));
    }

    #[test]
    fn test_eras_containing_modern() {
        let eras = eras_containing(2025);
        assert!(eras.iter().any(|e| e.name == "Information Age"));
    }

    #[test]
    fn test_eras_containing_none() {
        // Year far in the past
        let eras = eras_containing(-50000);
        assert!(eras.is_empty());
    }

    #[test]
    fn test_era_serde_roundtrip() {
        for era in all_eras() {
            let json = serde_json::to_string(era).unwrap();
            let back: Era = serde_json::from_str(&json).unwrap();
            assert_eq!(*era, back);
        }
    }

    #[test]
    fn test_era_category_serde_roundtrip() {
        let categories = [
            EraCategory::Ancient,
            EraCategory::Classical,
            EraCategory::Medieval,
            EraCategory::EarlyModern,
            EraCategory::Modern,
            EraCategory::Contemporary,
        ];
        for cat in &categories {
            let json = serde_json::to_string(cat).unwrap();
            let back: EraCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(*cat, back);
        }
    }
}
