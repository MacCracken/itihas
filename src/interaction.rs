//! Civilization interaction graph.
//!
//! Provides [`CivInteraction`] structs representing relationships between
//! civilizations, an [`InteractionType`] classification, and query functions
//! for exploring the interaction network.

use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

/// Type of interaction between two civilizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InteractionType {
    /// Commercial exchange of goods, services, or currency.
    Trade,
    /// Armed conflict between civilizations.
    War,
    /// Exchange of ideas, art, religion, or technology.
    CulturalExchange,
    /// Formal political or military partnership.
    Alliance,
    /// One civilization subjugating or absorbing another.
    Conquest,
    /// Peaceful exchange of diplomatic missions or treaties.
    Diplomacy,
}

impl fmt::Display for InteractionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trade => f.write_str("Trade"),
            Self::War => f.write_str("War"),
            Self::CulturalExchange => f.write_str("Cultural Exchange"),
            Self::Alliance => f.write_str("Alliance"),
            Self::Conquest => f.write_str("Conquest"),
            Self::Diplomacy => f.write_str("Diplomacy"),
        }
    }
}

/// A recorded interaction between two civilizations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CivInteraction {
    /// First civilization in the interaction.
    pub civ_a: Cow<'static, str>,
    /// Second civilization in the interaction.
    pub civ_b: Cow<'static, str>,
    /// Type of interaction.
    pub interaction_type: InteractionType,
    /// Approximate start year of the interaction (negative = BCE).
    pub start_year: i32,
    /// Approximate end year of the interaction (negative = BCE).
    pub end_year: i32,
    /// Brief description.
    pub description: Cow<'static, str>,
}

impl fmt::Display for CivInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ↔ {} ({}, {} – {})",
            self.civ_a, self.civ_b, self.interaction_type, self.start_year, self.end_year
        )
    }
}

fn build_interactions() -> Vec<CivInteraction> {
    vec![
        // Bronze Age interactions
        CivInteraction {
            civ_a: Cow::Borrowed("Mesopotamia"),
            civ_b: Cow::Borrowed("Ancient Egypt"),
            interaction_type: InteractionType::Trade,
            start_year: -3000,
            end_year: -1200,
            description: Cow::Borrowed(
                "Extensive trade in lapis lazuli, copper, and grain via overland and maritime routes",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Mesopotamia"),
            civ_b: Cow::Borrowed("Indus Valley"),
            interaction_type: InteractionType::Trade,
            start_year: -2600,
            end_year: -1900,
            description: Cow::Borrowed(
                "Maritime trade via the Persian Gulf; Indus seals found in Mesopotamian sites",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Ancient Egypt"),
            civ_b: Cow::Borrowed("Hittite Empire"),
            interaction_type: InteractionType::War,
            start_year: -1400,
            end_year: -1259,
            description: Cow::Borrowed(
                "Military conflict culminating in the Battle of Kadesh and the first peace treaty",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Ancient Egypt"),
            civ_b: Cow::Borrowed("Hittite Empire"),
            interaction_type: InteractionType::Diplomacy,
            start_year: -1259,
            end_year: -1178,
            description: Cow::Borrowed(
                "Egyptian-Hittite peace treaty, royal marriages, and diplomatic correspondence",
            ),
        },
        // Classical interactions
        CivInteraction {
            civ_a: Cow::Borrowed("Ancient Greece"),
            civ_b: Cow::Borrowed("Persian Empire"),
            interaction_type: InteractionType::War,
            start_year: -499,
            end_year: -449,
            description: Cow::Borrowed(
                "Greco-Persian Wars: Marathon, Thermopylae, Salamis, Plataea",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Ancient Greece"),
            civ_b: Cow::Borrowed("Ancient Egypt"),
            interaction_type: InteractionType::CulturalExchange,
            start_year: -650,
            end_year: -30,
            description: Cow::Borrowed(
                "Greek adoption of Egyptian astronomical and mathematical knowledge; Ptolemaic rule",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Roman Empire"),
            civ_b: Cow::Borrowed("Ancient China"),
            interaction_type: InteractionType::Trade,
            start_year: -130,
            end_year: 200,
            description: Cow::Borrowed("Silk Road trade; Roman demand for Chinese silk and spices"),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Roman Empire"),
            civ_b: Cow::Borrowed("Ancient Greece"),
            interaction_type: InteractionType::Conquest,
            start_year: -146,
            end_year: -146,
            description: Cow::Borrowed(
                "Roman conquest of Greece; subsequent adoption of Greek culture and philosophy",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Roman Empire"),
            civ_b: Cow::Borrowed("Phoenicia"),
            interaction_type: InteractionType::Conquest,
            start_year: -264,
            end_year: -146,
            description: Cow::Borrowed(
                "Punic Wars: Rome defeats Carthage, gains control of the western Mediterranean",
            ),
        },
        // Medieval interactions
        CivInteraction {
            civ_a: Cow::Borrowed("Byzantine Empire"),
            civ_b: Cow::Borrowed("Ottoman Empire"),
            interaction_type: InteractionType::War,
            start_year: 1299,
            end_year: 1453,
            description: Cow::Borrowed(
                "Gradual Ottoman conquest of Byzantine territory, culminating in the fall of Constantinople",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Mongol Empire"),
            civ_b: Cow::Borrowed("Ancient China"),
            interaction_type: InteractionType::Conquest,
            start_year: 1211,
            end_year: 1279,
            description: Cow::Borrowed(
                "Mongol conquest of Jin and Song dynasties, establishing the Yuan dynasty",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Mongol Empire"),
            civ_b: Cow::Borrowed("Persian Empire"),
            interaction_type: InteractionType::Conquest,
            start_year: 1219,
            end_year: 1258,
            description: Cow::Borrowed(
                "Mongol invasion of Khwarezmia and the Abbasid Caliphate, sack of Baghdad",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Mali Empire"),
            civ_b: Cow::Borrowed("Ancient Egypt"),
            interaction_type: InteractionType::Trade,
            start_year: 1235,
            end_year: 1500,
            description: Cow::Borrowed(
                "Trans-Saharan gold and salt trade; Mansa Musa's legendary pilgrimage through Cairo",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Viking/Norse"),
            civ_b: Cow::Borrowed("Byzantine Empire"),
            interaction_type: InteractionType::Trade,
            start_year: 800,
            end_year: 1100,
            description: Cow::Borrowed(
                "Varangian trade routes from Scandinavia to Constantinople; Varangian Guard service",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Khmer Empire"),
            civ_b: Cow::Borrowed("Ancient China"),
            interaction_type: InteractionType::CulturalExchange,
            start_year: 802,
            end_year: 1431,
            description: Cow::Borrowed(
                "Chinese diplomatic missions and cultural influence; adoption of Indian and Chinese elements",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Chola Dynasty"),
            civ_b: Cow::Borrowed("Srivijaya"),
            interaction_type: InteractionType::War,
            start_year: 1025,
            end_year: 1025,
            description: Cow::Borrowed(
                "Chola naval raids on Srivijaya to secure Indian Ocean trade routes",
            ),
        },
        // Early Modern / Colonial interactions
        CivInteraction {
            civ_a: Cow::Borrowed("Spanish Empire"),
            civ_b: Cow::Borrowed("Aztec Empire"),
            interaction_type: InteractionType::Conquest,
            start_year: 1519,
            end_year: 1521,
            description: Cow::Borrowed("Spanish conquest of the Aztec Empire under Hernán Cortés"),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Spanish Empire"),
            civ_b: Cow::Borrowed("Inca Empire"),
            interaction_type: InteractionType::Conquest,
            start_year: 1532,
            end_year: 1572,
            description: Cow::Borrowed(
                "Spanish conquest of the Inca Empire under Francisco Pizarro",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Portuguese Empire"),
            civ_b: Cow::Borrowed("Kingdom of Kongo"),
            interaction_type: InteractionType::Trade,
            start_year: 1483,
            end_year: 1700,
            description: Cow::Borrowed(
                "Trade in ivory, copper, and enslaved people; Portuguese missionary activity",
            ),
        },
        CivInteraction {
            civ_a: Cow::Borrowed("Ottoman Empire"),
            civ_b: Cow::Borrowed("Republic of Venice"),
            interaction_type: InteractionType::Trade,
            start_year: 1400,
            end_year: 1700,
            description: Cow::Borrowed(
                "Mediterranean trade in spices, textiles, and luxury goods despite periodic wars",
            ),
        },
    ]
}

/// Returns all pre-built civilization interactions.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_interactions() -> &'static [CivInteraction] {
    static DATA: std::sync::LazyLock<Vec<CivInteraction>> =
        std::sync::LazyLock::new(build_interactions);
    &DATA
}

/// Returns all pre-built civilization interactions.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_interactions() -> Vec<CivInteraction> {
    build_interactions()
}

/// Returns all interactions involving a given civilization (case-insensitive).
#[must_use]
#[inline]
pub fn interactions_for(civ_name: &str) -> Vec<CivInteraction> {
    tracing::debug!(civ_name, "looking up interactions for civilization");
    let lower = civ_name.to_lowercase();
    all_interactions()
        .iter()
        .filter(|i| i.civ_a.to_lowercase() == lower || i.civ_b.to_lowercase() == lower)
        .cloned()
        .collect()
}

/// Returns all interactions of a given type.
#[must_use]
#[inline]
pub fn by_type(interaction_type: &InteractionType) -> Vec<CivInteraction> {
    tracing::debug!(?interaction_type, "looking up interactions by type");
    all_interactions()
        .iter()
        .filter(|i| i.interaction_type == *interaction_type)
        .cloned()
        .collect()
}

/// Returns all interactions between two specific civilizations (case-insensitive, order-independent).
#[must_use]
#[inline]
pub fn between(civ_a: &str, civ_b: &str) -> Vec<CivInteraction> {
    tracing::debug!(
        civ_a,
        civ_b,
        "looking up interactions between civilizations"
    );
    let a = civ_a.to_lowercase();
    let b = civ_b.to_lowercase();
    all_interactions()
        .iter()
        .filter(|i| {
            let ia = i.civ_a.to_lowercase();
            let ib = i.civ_b.to_lowercase();
            (ia == a && ib == b) || (ia == b && ib == a)
        })
        .cloned()
        .collect()
}

/// Returns all civilizations that interacted with the given civilization.
///
/// Returns a deduplicated, sorted list of civilization names.
#[must_use]
pub fn neighbors(civ_name: &str) -> Vec<Cow<'static, str>> {
    tracing::debug!(civ_name, "looking up civilization neighbors");
    let lower = civ_name.to_lowercase();
    let mut result: Vec<Cow<'static, str>> = Vec::new();
    for i in all_interactions() {
        let ia = i.civ_a.to_lowercase();
        let ib = i.civ_b.to_lowercase();
        if ia == lower && !result.contains(&i.civ_b) {
            result.push(i.civ_b.clone());
        } else if ib == lower && !result.contains(&i.civ_a) {
            result.push(i.civ_a.clone());
        }
    }
    result.sort();
    result
}

/// Influence score between two civilizations based on interaction count and type.
///
/// Returns 0 if no interactions exist. Higher scores indicate more significant
/// relationships. Weights: Conquest=4, War=3, Alliance=3, Trade=2,
/// CulturalExchange=2, Diplomacy=1.
#[must_use]
pub fn influence_score(civ_a: &str, civ_b: &str) -> u32 {
    tracing::debug!(civ_a, civ_b, "computing influence score");
    between(civ_a, civ_b)
        .iter()
        .map(|i| match i.interaction_type {
            InteractionType::Conquest => 4,
            InteractionType::War => 3,
            InteractionType::Alliance => 3,
            InteractionType::Trade => 2,
            InteractionType::CulturalExchange => 2,
            InteractionType::Diplomacy => 1,
        })
        .sum()
}

/// Geographic proximity score between two civilizations based on shared regions.
///
/// Compares the `region` fields of both civilizations. Returns a score from 0 to 100:
/// - 100: identical region strings
/// - 50–99: share at least one region component (e.g., both mention "Near East")
/// - 0: no region overlap
///
/// Requires civilization data, so this calls into the civilization module.
#[must_use]
pub fn region_proximity(civ_a: &str, civ_b: &str) -> u32 {
    tracing::debug!(civ_a, civ_b, "computing region proximity");
    let civs = crate::civilization::all_civilizations();
    let find = |name: &str| -> Option<&crate::civilization::Civilization> {
        let lower = name.to_lowercase();
        civs.iter().find(|c| c.name.to_lowercase() == lower)
    };

    let (Some(a), Some(b)) = (find(civ_a), find(civ_b)) else {
        return 0;
    };

    let regions_a: Vec<&str> = a.region.split(", ").collect();
    let regions_b: Vec<&str> = b.region.split(", ").collect();

    if a.region == b.region {
        return 100;
    }

    let shared = regions_a.iter().filter(|r| regions_b.contains(r)).count();

    if shared == 0 {
        return 0;
    }

    let total = regions_a.len().max(regions_b.len());

    // Scale shared overlap to 50–99 range
    let ratio = (shared as f64) / (total as f64);
    50 + (ratio * 49.0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_interactions_nonempty() {
        assert!(all_interactions().len() >= 20);
    }

    #[test]
    fn test_interactions_for_rome() {
        let rome = interactions_for("Roman Empire");
        assert!(!rome.is_empty());
        assert!(
            rome.iter()
                .any(|i| i.civ_a == "Roman Empire" || i.civ_b == "Roman Empire")
        );
    }

    #[test]
    fn test_by_type_war() {
        let wars = by_type(&InteractionType::War);
        assert!(!wars.is_empty());
        assert!(
            wars.iter()
                .all(|i| i.interaction_type == InteractionType::War)
        );
    }

    #[test]
    fn test_between_egypt_hittite() {
        let interactions = between("Ancient Egypt", "Hittite Empire");
        assert!(interactions.len() >= 2); // War + Diplomacy
    }

    #[test]
    fn test_between_order_independent() {
        let ab = between("Ancient Egypt", "Hittite Empire");
        let ba = between("Hittite Empire", "Ancient Egypt");
        assert_eq!(ab.len(), ba.len());
    }

    #[test]
    fn test_neighbors_rome() {
        let n = neighbors("Roman Empire");
        assert!(n.iter().any(|c| c == "Ancient China"));
        assert!(n.iter().any(|c| c == "Ancient Greece"));
    }

    #[test]
    fn test_interaction_serde_roundtrip() {
        for i in all_interactions() {
            let json = serde_json::to_string(i).unwrap();
            let back: CivInteraction = serde_json::from_str(&json).unwrap();
            assert_eq!(*i, back);
        }
    }

    #[test]
    fn test_interaction_type_serde_roundtrip() {
        let types = [
            InteractionType::Trade,
            InteractionType::War,
            InteractionType::CulturalExchange,
            InteractionType::Alliance,
            InteractionType::Conquest,
            InteractionType::Diplomacy,
        ];
        for t in &types {
            let json = serde_json::to_string(t).unwrap();
            let back: InteractionType = serde_json::from_str(&json).unwrap();
            assert_eq!(*t, back);
        }
    }

    #[test]
    fn test_interaction_date_ordering() {
        for i in all_interactions() {
            assert!(
                i.start_year <= i.end_year,
                "interaction '{}' has start > end",
                i
            );
        }
    }

    #[test]
    fn test_influence_score_nonzero() {
        let score = influence_score("Ancient Egypt", "Hittite Empire");
        assert!(score > 0);
    }

    #[test]
    fn test_influence_score_zero_for_unrelated() {
        let score = influence_score("Inca Empire", "Ancient China");
        assert_eq!(score, 0);
    }

    #[test]
    fn test_region_proximity_same_region() {
        // Mesopotamia and Assyrian Empire are both "Near East"
        let score = region_proximity("Mesopotamia", "Assyrian Empire");
        assert_eq!(score, 100);
    }

    #[test]
    fn test_region_proximity_partial_overlap() {
        // Roman Empire is "Mediterranean, Europe"; Ancient Greece is "Mediterranean"
        let score = region_proximity("Roman Empire", "Ancient Greece");
        assert!(score >= 50);
    }

    #[test]
    fn test_region_proximity_no_overlap() {
        let score = region_proximity("Inca Empire", "Ancient China");
        assert_eq!(score, 0);
    }

    #[test]
    fn test_region_proximity_unknown_civ() {
        let score = region_proximity("Atlantis", "Ancient Greece");
        assert_eq!(score, 0);
    }
}
