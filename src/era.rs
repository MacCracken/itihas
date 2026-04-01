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

/// Whether an era represents a global periodization or a regional tradition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EraScope {
    /// Standard world-history periodization (Bronze Age, Classical, etc.).
    Global,
    /// Region-specific periodization (Chinese dynasties, Indian yugas, etc.).
    Regional,
}

impl fmt::Display for EraScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => f.write_str("Global"),
            Self::Regional => f.write_str("Regional"),
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
    /// Whether this is a global or regional periodization.
    pub scope: EraScope,
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
        // ---- Global periodization ----
        Era {
            name: Cow::Borrowed("Bronze Age"),
            start_year: -3500,
            end_year: -1100,
            region: Cow::Borrowed("Near East, Mediterranean, South Asia"),
            description: Cow::Borrowed(
                "Emergence of bronze metallurgy, early writing systems, and first cities",
            ),
            category: EraCategory::Ancient,
            scope: EraScope::Global,
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
            scope: EraScope::Global,
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
            scope: EraScope::Global,
        },
        Era {
            name: Cow::Borrowed("Middle Ages"),
            start_year: 476,
            end_year: 1453,
            region: Cow::Borrowed("Europe, Near East"),
            description: Cow::Borrowed("Feudalism, monasticism, Islamic Golden Age, Crusades"),
            category: EraCategory::Medieval,
            scope: EraScope::Global,
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
            scope: EraScope::Global,
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
            scope: EraScope::Global,
        },
        Era {
            name: Cow::Borrowed("Industrial Age"),
            start_year: 1760,
            end_year: 1970,
            region: Cow::Borrowed("Global"),
            description: Cow::Borrowed("Mechanization, mass production, urbanization, world wars"),
            category: EraCategory::Modern,
            scope: EraScope::Global,
        },
        Era {
            name: Cow::Borrowed("Information Age"),
            start_year: 1970,
            end_year: i32::MAX,
            region: Cow::Borrowed("Global"),
            description: Cow::Borrowed("Digital revolution, internet, AI, globalization"),
            category: EraCategory::Contemporary,
            scope: EraScope::Global,
        },
        // ---- Chinese dynasties ----
        Era {
            name: Cow::Borrowed("Xia Dynasty"),
            start_year: -2070,
            end_year: -1600,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Semi-legendary first dynasty of China, Yellow River valley",
            ),
            category: EraCategory::Ancient,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Shang Dynasty"),
            start_year: -1600,
            end_year: -1046,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed("Oracle bones, bronze casting, earliest Chinese writing"),
            category: EraCategory::Ancient,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Zhou Dynasty"),
            start_year: -1046,
            end_year: -256,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Mandate of Heaven, Confucius, Laozi, Hundred Schools of Thought",
            ),
            category: EraCategory::Ancient,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Qin Dynasty"),
            start_year: -221,
            end_year: -206,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "First unification of China, Great Wall begun, Legalism, terracotta army",
            ),
            category: EraCategory::Classical,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Han Dynasty"),
            start_year: -206,
            end_year: 220,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Silk Road, Confucian state, paper invention, civil service exams",
            ),
            category: EraCategory::Classical,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Tang Dynasty"),
            start_year: 618,
            end_year: 907,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Golden age of Chinese poetry, cosmopolitan culture, woodblock printing",
            ),
            category: EraCategory::Medieval,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Song Dynasty"),
            start_year: 960,
            end_year: 1279,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Movable type, gunpowder weapons, compass navigation, Neo-Confucianism",
            ),
            category: EraCategory::Medieval,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Ming Dynasty"),
            start_year: 1368,
            end_year: 1644,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Great Wall reconstruction, Forbidden City, Zheng He voyages, porcelain",
            ),
            category: EraCategory::EarlyModern,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Qing Dynasty"),
            start_year: 1644,
            end_year: 1912,
            region: Cow::Borrowed("East Asia"),
            description: Cow::Borrowed(
                "Last imperial dynasty, Manchu rule, territorial expansion, Opium Wars",
            ),
            category: EraCategory::EarlyModern,
            scope: EraScope::Regional,
        },
        // ---- Indian periods ----
        Era {
            name: Cow::Borrowed("Vedic Period"),
            start_year: -1500,
            end_year: -500,
            region: Cow::Borrowed("South Asia"),
            description: Cow::Borrowed(
                "Composition of the Vedas, varna system, early Hinduism, Sanskrit literature",
            ),
            category: EraCategory::Ancient,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Maurya Period"),
            start_year: -322,
            end_year: -185,
            region: Cow::Borrowed("South Asia"),
            description: Cow::Borrowed(
                "First pan-Indian empire under Chandragupta and Ashoka, spread of Buddhism",
            ),
            category: EraCategory::Classical,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Gupta Period"),
            start_year: 320,
            end_year: 550,
            region: Cow::Borrowed("South Asia"),
            description: Cow::Borrowed(
                "Golden age of India: zero, decimal system, Kalidasa, Aryabhata, Nalanda",
            ),
            category: EraCategory::Classical,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Delhi Sultanate"),
            start_year: 1206,
            end_year: 1526,
            region: Cow::Borrowed("South Asia"),
            description: Cow::Borrowed(
                "Turkic and Afghan Muslim rule in northern India, Indo-Islamic architecture",
            ),
            category: EraCategory::Medieval,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Mughal Period"),
            start_year: 1526,
            end_year: 1857,
            region: Cow::Borrowed("South Asia"),
            description: Cow::Borrowed(
                "Babur to Bahadur Shah, Taj Mahal, Akbar's religious tolerance, Urdu synthesis",
            ),
            category: EraCategory::EarlyModern,
            scope: EraScope::Regional,
        },
        // ---- Mesoamerican periods ----
        Era {
            name: Cow::Borrowed("Mesoamerican Preclassic"),
            start_year: -2000,
            end_year: 250,
            region: Cow::Borrowed("Mesoamerica"),
            description: Cow::Borrowed(
                "Olmec civilization, early Maya settlements, development of writing and calendars",
            ),
            category: EraCategory::Ancient,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Mesoamerican Classic"),
            start_year: 250,
            end_year: 900,
            region: Cow::Borrowed("Mesoamerica"),
            description: Cow::Borrowed(
                "Maya golden age, Teotihuacan, Zapotec, monumental architecture and astronomy",
            ),
            category: EraCategory::Classical,
            scope: EraScope::Regional,
        },
        Era {
            name: Cow::Borrowed("Mesoamerican Postclassic"),
            start_year: 900,
            end_year: 1521,
            region: Cow::Borrowed("Mesoamerica"),
            description: Cow::Borrowed(
                "Toltec and Aztec empires, Chichen Itza, Tenochtitlan, Spanish contact",
            ),
            category: EraCategory::Medieval,
            scope: EraScope::Regional,
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

/// Returns eras matching the given scope (Global or Regional).
#[must_use]
#[inline]
pub fn by_scope(scope: &EraScope) -> Vec<Era> {
    tracing::debug!(?scope, "looking up eras by scope");
    all_eras()
        .iter()
        .filter(|e| e.scope == *scope)
        .cloned()
        .collect()
}

/// Returns eras whose region contains the given substring (case-insensitive).
#[must_use]
#[inline]
pub fn by_region(region: &str) -> Vec<Era> {
    tracing::debug!(region, "looking up eras by region");
    let lower = region.to_lowercase();
    all_eras()
        .iter()
        .filter(|e| e.region.to_lowercase().contains(&lower))
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
        assert_eq!(all_eras().len(), 25);
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
        let eras = all_eras();
        for era in eras.iter() {
            let json = serde_json::to_string(era).unwrap();
            let back: Era = serde_json::from_str(&json).unwrap();
            assert_eq!(era, &back);
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
