//! Causal links between historical events.
//!
//! Provides [`Causality`] structs representing cause-effect relationships
//! between events, a [`CausalStrength`] classification, and traversal
//! functions for exploring causal chains.
//!
//! # Sources
//!
//! Causal relationships derived from Stearns (2001) and Roberts & Westad
//! (2013). Strength classifications are editorial assessments based on
//! historiographic consensus. Full bibliography:
//! [`docs/sources/general.md`](https://github.com/MacCracken/itihas/blob/main/docs/sources/general.md).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

/// Strength of a causal relationship between events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CausalStrength {
    /// Loose or debated connection.
    Weak,
    /// Recognized contributing factor.
    Moderate,
    /// Major cause widely accepted by historians.
    Strong,
    /// Immediate, undisputed trigger.
    Direct,
}

impl fmt::Display for CausalStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weak => f.write_str("Weak"),
            Self::Moderate => f.write_str("Moderate"),
            Self::Strong => f.write_str("Strong"),
            Self::Direct => f.write_str("Direct"),
        }
    }
}

/// A causal link between two historical events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Causality {
    /// Name of the cause event (must match an event name).
    pub cause: Cow<'static, str>,
    /// Name of the effect event (must match an event name).
    pub effect: Cow<'static, str>,
    /// How strong the causal connection is.
    pub strength: CausalStrength,
    /// Brief explanation of the causal mechanism.
    pub description: Cow<'static, str>,
}

impl fmt::Display for Causality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {} ({})", self.cause, self.effect, self.strength)
    }
}

fn build_causalities() -> Vec<Causality> {
    vec![
        // Bronze Age chains
        Causality {
            cause: Cow::Borrowed("Invention of Writing"),
            effect: Cow::Borrowed("Code of Hammurabi"),
            strength: CausalStrength::Strong,
            description: Cow::Borrowed(
                "Written law codes required the prior development of writing systems",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Bronze Age Collapse"),
            effect: Cow::Borrowed("Founding of Rome"),
            strength: CausalStrength::Moderate,
            description: Cow::Borrowed(
                "Power vacuum after Bronze Age Collapse allowed new Mediterranean polities to emerge",
            ),
        },
        // Classical chains
        Causality {
            cause: Cow::Borrowed("Battle of Marathon"),
            effect: Cow::Borrowed("Conquests of Alexander the Great"),
            strength: CausalStrength::Moderate,
            description: Cow::Borrowed(
                "Greek survival against Persia enabled the rise of Macedon and eventual counter-conquest",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Conquests of Alexander the Great"),
            effect: Cow::Borrowed("Maurya Empire under Ashoka"),
            strength: CausalStrength::Moderate,
            description: Cow::Borrowed(
                "Alexander's invasion destabilized the Nanda dynasty, enabling Chandragupta's rise",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Fall of the Western Roman Empire"),
            effect: Cow::Borrowed("Rise of Islam"),
            strength: CausalStrength::Weak,
            description: Cow::Borrowed(
                "Western Roman collapse destabilized Mediterranean trade; Byzantine-Sassanid exhaustion was the more direct factor enabling Islam's rapid expansion",
            ),
        },
        // Medieval chains
        Causality {
            cause: Cow::Borrowed("Rise of Islam"),
            effect: Cow::Borrowed("Mongol Conquests"),
            strength: CausalStrength::Weak,
            description: Cow::Borrowed(
                "Islamic caliphates created wealthy urbanized targets that drew Mongol expansion westward",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Mongol Conquests"),
            effect: Cow::Borrowed("Fall of Constantinople"),
            strength: CausalStrength::Moderate,
            description: Cow::Borrowed(
                "Mongol disruption weakened Byzantine trade networks and empowered the Ottoman Turks",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Fall of Constantinople"),
            effect: Cow::Borrowed("French Revolution"),
            strength: CausalStrength::Weak,
            description: Cow::Borrowed(
                "Fall of Byzantium accelerated the Renaissance and Enlightenment, which seeded revolutionary ideals",
            ),
        },
        // Early Modern chains
        Causality {
            cause: Cow::Borrowed("Gutenberg Printing Press"),
            effect: Cow::Borrowed("French Revolution"),
            strength: CausalStrength::Strong,
            description: Cow::Borrowed(
                "Mass printing enabled Enlightenment ideas to spread widely, undermining monarchical legitimacy",
            ),
        },
        Causality {
            cause: Cow::Borrowed("French Revolution"),
            effect: Cow::Borrowed("Invention of the World Wide Web"),
            strength: CausalStrength::Weak,
            description: Cow::Borrowed(
                "Revolutionary ideals of open knowledge exchange influenced long-term development of information-sharing technologies",
            ),
        },
        // Additional cross-era links
        Causality {
            cause: Cow::Borrowed("Invention of Writing"),
            effect: Cow::Borrowed("Gutenberg Printing Press"),
            strength: CausalStrength::Strong,
            description: Cow::Borrowed(
                "Printing mechanized the reproduction of written text, requiring writing as a prerequisite",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Code of Hammurabi"),
            effect: Cow::Borrowed("Founding of Rome"),
            strength: CausalStrength::Weak,
            description: Cow::Borrowed(
                "Legal code traditions influenced later Mediterranean civilizations including Roman law",
            ),
        },
        Causality {
            cause: Cow::Borrowed("Gutenberg Printing Press"),
            effect: Cow::Borrowed("Invention of the World Wide Web"),
            strength: CausalStrength::Strong,
            description: Cow::Borrowed(
                "Printing democratized information; the web is its digital successor",
            ),
        },
    ]
}

/// Returns all pre-built causal links.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_causalities() -> &'static [Causality] {
    static DATA: std::sync::LazyLock<Vec<Causality>> = std::sync::LazyLock::new(build_causalities);
    &DATA
}

/// Returns all pre-built causal links.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_causalities() -> Vec<Causality> {
    build_causalities()
}

/// Returns all causal links where the given event is the **effect**.
///
/// In other words, returns the causes of this event.
#[must_use]
#[inline]
pub fn causes_of(event_name: &str) -> Vec<Causality> {
    tracing::debug!(event_name, "looking up causes of event");
    let lower = event_name.to_lowercase();
    all_causalities()
        .iter()
        .filter(|c| c.effect.to_lowercase() == lower)
        .cloned()
        .collect()
}

/// Returns all causal links where the given event is the **cause**.
///
/// In other words, returns the effects of this event.
#[must_use]
#[inline]
pub fn effects_of(event_name: &str) -> Vec<Causality> {
    tracing::debug!(event_name, "looking up effects of event");
    let lower = event_name.to_lowercase();
    all_causalities()
        .iter()
        .filter(|c| c.cause.to_lowercase() == lower)
        .cloned()
        .collect()
}

/// Follows causal chains forward from an event up to `max_depth` steps.
///
/// Returns all events reachable by following cause → effect links,
/// with each entry paired with its depth from the starting event.
#[must_use]
pub fn chain(event_name: &str, max_depth: usize) -> Vec<(String, usize)> {
    tracing::debug!(event_name, max_depth, "tracing causal chain");
    let mut results: Vec<(String, usize)> = Vec::new();
    let mut frontier: Vec<(String, usize)> = vec![(String::from(event_name), 0)];
    let mut visited: Vec<String> = vec![String::from(event_name)];

    while let Some((current, depth)) = frontier.pop() {
        if depth >= max_depth {
            continue;
        }
        for link in effects_of(&current) {
            let effect = link.effect.to_string();
            if !visited.contains(&effect) {
                visited.push(effect.clone());
                results.push((effect.clone(), depth + 1));
                frontier.push((effect, depth + 1));
            }
        }
    }

    results.sort_by_key(|(_, d)| *d);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_causalities_nonempty() {
        assert!(!all_causalities().is_empty());
    }

    #[test]
    fn test_causes_of_french_revolution() {
        let causes = causes_of("French Revolution");
        assert!(!causes.is_empty());
        assert!(causes.iter().any(|c| c.cause == "Gutenberg Printing Press"));
    }

    #[test]
    fn test_effects_of_writing() {
        let effects = effects_of("Invention of Writing");
        assert!(!effects.is_empty());
        assert!(effects.iter().any(|c| c.effect == "Code of Hammurabi"));
    }

    #[test]
    fn test_chain_from_writing() {
        let ch = chain("Invention of Writing", 3);
        assert!(!ch.is_empty());
        // Should reach Code of Hammurabi at depth 1
        assert!(
            ch.iter()
                .any(|(name, depth)| name == "Code of Hammurabi" && *depth == 1)
        );
    }

    #[test]
    fn test_chain_no_cycles() {
        // Even with max_depth high, should not loop
        let ch = chain("Invention of Writing", 10);
        let names: Vec<_> = ch.iter().map(|(n, _)| n.as_str()).collect();
        // No duplicates
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(names.len(), unique.len());
    }

    #[test]
    fn test_causality_serde_roundtrip() {
        let causalities = all_causalities();
        for c in causalities.iter() {
            let json = serde_json::to_string(c).unwrap();
            let back: Causality = serde_json::from_str(&json).unwrap();
            assert_eq!(c, &back);
        }
    }

    #[test]
    fn test_causal_strength_serde_roundtrip() {
        let strengths = [
            CausalStrength::Weak,
            CausalStrength::Moderate,
            CausalStrength::Strong,
            CausalStrength::Direct,
        ];
        for s in &strengths {
            let json = serde_json::to_string(s).unwrap();
            let back: CausalStrength = serde_json::from_str(&json).unwrap();
            assert_eq!(*s, back);
        }
    }
}
