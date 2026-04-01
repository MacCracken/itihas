//! Structured historical events.
//!
//! Provides [`Event`] structs representing major world events, an
//! [`EventCategory`] classification enum, and 15 pre-built events spanning
//! from the invention of writing to the digital revolution.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// Classification of historical events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EventCategory {
    /// Armed conflict between states or groups.
    War,
    /// Formal agreement between parties.
    Treaty,
    /// New knowledge about the natural world or geography.
    Discovery,
    /// Creation of a new tool, technique, or technology.
    Invention,
    /// Fundamental political or social upheaval.
    Revolution,
    /// Large-scale population movement.
    Migration,
    /// Establishment of a state, city, or institution.
    Founding,
    /// Fall of a state, empire, or civilization.
    Collapse,
}

/// A structured historical event.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Event {
    /// Name of the event.
    pub name: Cow<'static, str>,
    /// Year the event occurred (negative = BCE).
    pub year: i32,
    /// Era name this event belongs to.
    pub era: Cow<'static, str>,
    /// Category classification.
    pub category: EventCategory,
    /// Brief description.
    pub description: Cow<'static, str>,
    /// Civilizations involved in or affected by this event.
    pub civilizations_involved: Vec<Cow<'static, str>>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.year)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then_with(|| self.name.cmp(&other.name))
    }
}

fn build_events() -> Vec<Event> {
    vec![
        Event {
            name: Cow::Borrowed("Invention of Writing"),
            year: -3400,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Development of cuneiform in Sumer, enabling record-keeping and literature",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mesopotamia")],
        },
        Event {
            name: Cow::Borrowed("Construction of the Great Pyramid"),
            year: -2560,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Completion of the Great Pyramid of Giza under Pharaoh Khufu",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient Egypt")],
        },
        Event {
            name: Cow::Borrowed("Code of Hammurabi"),
            year: -1754,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "One of the earliest written legal codes, established by King Hammurabi of Babylon",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mesopotamia")],
        },
        Event {
            name: Cow::Borrowed("Bronze Age Collapse"),
            year: -1177,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Widespread societal collapse across the Eastern Mediterranean",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Mesopotamia"),
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Ancient Greece"),
            ],
        },
        Event {
            name: Cow::Borrowed("Founding of Rome"),
            year: -753,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed("Traditional date of the founding of Rome by Romulus"),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
        },
        Event {
            name: Cow::Borrowed("Battle of Marathon"),
            year: -490,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Greek victory over Persian invasion, preserving Athenian democracy",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Greece"),
                Cow::Borrowed("Persian Empire"),
            ],
        },
        Event {
            name: Cow::Borrowed("Conquests of Alexander the Great"),
            year: -334,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Macedonian conquest from Greece to India, spreading Hellenistic culture",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Greece"),
                Cow::Borrowed("Persian Empire"),
            ],
        },
        Event {
            name: Cow::Borrowed("Maurya Empire under Ashoka"),
            year: -268,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Ashoka's reign unifies most of South Asia, promotes Buddhism and non-violence",
            ),
            civilizations_involved: vec![Cow::Borrowed("Maurya Empire")],
        },
        Event {
            name: Cow::Borrowed("Fall of the Western Roman Empire"),
            year: 476,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Deposition of Romulus Augustulus, traditionally marking the end of antiquity",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
        },
        Event {
            name: Cow::Borrowed("Rise of Islam"),
            year: 622,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Muhammad's Hijra to Medina, founding the Islamic community",
            ),
            civilizations_involved: vec![],
        },
        Event {
            name: Cow::Borrowed("Mongol Conquests"),
            year: 1206,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Genghis Khan unifies Mongol tribes and launches largest contiguous land empire",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Mongol Empire"),
                Cow::Borrowed("Ancient China"),
            ],
        },
        Event {
            name: Cow::Borrowed("Fall of Constantinople"),
            year: 1453,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Ottoman conquest of Constantinople, ending the Byzantine Empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ottoman Empire")],
        },
        Event {
            name: Cow::Borrowed("Gutenberg Printing Press"),
            year: 1440,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Movable type printing revolutionizes information dissemination in Europe",
            ),
            civilizations_involved: vec![],
        },
        Event {
            name: Cow::Borrowed("French Revolution"),
            year: 1789,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Overthrow of the monarchy, establishing principles of liberty and equality",
            ),
            civilizations_involved: vec![],
        },
        Event {
            name: Cow::Borrowed("Invention of the World Wide Web"),
            year: 1989,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed("Tim Berners-Lee proposes the World Wide Web at CERN"),
            civilizations_involved: vec![],
        },
    ]
}

/// Returns all pre-built historical events.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_events() -> &'static [Event] {
    static DATA: std::sync::LazyLock<Vec<Event>> = std::sync::LazyLock::new(build_events);
    &DATA
}

/// Returns all pre-built historical events.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_events() -> Vec<Event> {
    build_events()
}

/// Returns events matching the given category.
#[must_use]
#[inline]
pub fn by_category(category: &EventCategory) -> Vec<Event> {
    tracing::debug!(?category, "looking up events by category");
    all_events()
        .iter()
        .filter(|e| e.category == *category)
        .cloned()
        .collect()
}

/// Returns events that occurred in or near the given year (exact match).
#[must_use]
#[inline]
pub fn at_year(year: i32) -> Vec<Event> {
    tracing::debug!(year, "looking up events at year");
    all_events()
        .iter()
        .filter(|e| e.year == year)
        .cloned()
        .collect()
}

/// Look up an event by exact name (case-insensitive).
///
/// Returns `Err(ItihasError::EventNotFound)` if no event matches.
#[inline]
pub fn by_name(name: &str) -> Result<Event, ItihasError> {
    tracing::debug!(name, "looking up event by name");
    let lower = name.to_lowercase();
    all_events()
        .iter()
        .find(|e| e.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::EventNotFound(String::from(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_events_count() {
        assert_eq!(all_events().len(), 15);
    }

    #[test]
    fn test_by_category_war() {
        let wars = by_category(&EventCategory::War);
        assert!(wars.len() >= 3);
        assert!(wars.iter().all(|e| e.category == EventCategory::War));
    }

    #[test]
    fn test_by_category_invention() {
        let inventions = by_category(&EventCategory::Invention);
        assert!(inventions.len() >= 2);
        assert!(inventions.iter().any(|e| e.name == "Invention of Writing"));
    }

    #[test]
    fn test_at_year_exact() {
        let events = at_year(476);
        assert!(
            events
                .iter()
                .any(|e| e.name == "Fall of the Western Roman Empire")
        );
    }

    #[test]
    fn test_at_year_none() {
        let events = at_year(42);
        assert!(events.is_empty());
    }

    #[test]
    fn test_event_serde_roundtrip() {
        for event in all_events() {
            let json = serde_json::to_string(event).unwrap();
            let back: Event = serde_json::from_str(&json).unwrap();
            assert_eq!(*event, back);
        }
    }

    #[test]
    fn test_event_category_serde_roundtrip() {
        let categories = [
            EventCategory::War,
            EventCategory::Treaty,
            EventCategory::Discovery,
            EventCategory::Invention,
            EventCategory::Revolution,
            EventCategory::Migration,
            EventCategory::Founding,
            EventCategory::Collapse,
        ];
        for cat in &categories {
            let json = serde_json::to_string(cat).unwrap();
            let back: EventCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(*cat, back);
        }
    }
}
