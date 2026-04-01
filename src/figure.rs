//! Historical figures with era and civilization context.
//!
//! Provides [`Figure`] structs representing major historical figures, a
//! [`FigureDomain`] classification enum, and 10 pre-built figures spanning
//! from Hammurabi to Ada Lovelace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Domain of a historical figure's primary contribution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FigureDomain {
    /// Head of state, monarch, emperor.
    Ruler,
    /// Thinker, logician, ethicist.
    Philosopher,
    /// Natural philosopher, researcher.
    Scientist,
    /// Visual artist, musician, writer.
    Artist,
    /// General, strategist, warrior.
    Military,
    /// Founder or leader of a religious tradition.
    Religious,
    /// Navigator, cartographer, traveller.
    Explorer,
    /// Creator of tools, machines, or techniques.
    Inventor,
}

/// A historical figure.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Figure {
    /// Name of the figure.
    pub name: Cow<'static, str>,
    /// Birth year (negative = BCE). `None` if unknown.
    pub birth_year: Option<i32>,
    /// Death year (negative = BCE). `None` if unknown.
    pub death_year: Option<i32>,
    /// Civilization associated with this figure.
    pub civilization: Cow<'static, str>,
    /// Primary domain of contribution.
    pub domain: FigureDomain,
    /// Brief description.
    pub description: Cow<'static, str>,
}

/// Returns all pre-built historical figures.
#[must_use]
#[inline]
pub fn all_figures() -> Vec<Figure> {
    vec![
        Figure {
            name: Cow::Borrowed("Hammurabi"),
            birth_year: Some(-1810),
            death_year: Some(-1750),
            civilization: Cow::Borrowed("Mesopotamia"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "King of Babylon, author of one of the earliest written law codes",
            ),
        },
        Figure {
            name: Cow::Borrowed("Aristotle"),
            birth_year: Some(-384),
            death_year: Some(-322),
            civilization: Cow::Borrowed("Ancient Greece"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Greek philosopher, student of Plato, tutor of Alexander, founder of the Lyceum",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ashoka"),
            birth_year: Some(-304),
            death_year: Some(-232),
            civilization: Cow::Borrowed("Indus Valley"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Maurya emperor who unified most of South Asia and promoted Buddhism",
            ),
        },
        Figure {
            name: Cow::Borrowed("Julius Caesar"),
            birth_year: Some(-100),
            death_year: Some(-44),
            civilization: Cow::Borrowed("Roman Empire"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Roman general and statesman, conquered Gaul, crossed the Rubicon",
            ),
        },
        Figure {
            name: Cow::Borrowed("Hypatia"),
            birth_year: Some(360),
            death_year: Some(415),
            civilization: Cow::Borrowed("Roman Empire"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Alexandrian mathematician, astronomer, and Neoplatonist philosopher",
            ),
        },
        Figure {
            name: Cow::Borrowed("Genghis Khan"),
            birth_year: Some(1162),
            death_year: Some(1227),
            civilization: Cow::Borrowed("Mongol Empire"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Founder of the Mongol Empire, largest contiguous land empire in history",
            ),
        },
        Figure {
            name: Cow::Borrowed("Johannes Gutenberg"),
            birth_year: Some(1400),
            death_year: Some(1468),
            civilization: Cow::Borrowed("Holy Roman Empire"),
            domain: FigureDomain::Inventor,
            description: Cow::Borrowed(
                "Inventor of movable type printing in Europe, revolutionizing information access",
            ),
        },
        Figure {
            name: Cow::Borrowed("Leonardo da Vinci"),
            birth_year: Some(1452),
            death_year: Some(1519),
            civilization: Cow::Borrowed("Italian city-states"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Polymath: painter, sculptor, architect, scientist, engineer, inventor",
            ),
        },
        Figure {
            name: Cow::Borrowed("Isaac Newton"),
            birth_year: Some(1643),
            death_year: Some(1727),
            civilization: Cow::Borrowed("Kingdom of England"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Mathematician and physicist, laws of motion and universal gravitation",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ada Lovelace"),
            birth_year: Some(1815),
            death_year: Some(1852),
            civilization: Cow::Borrowed("British Empire"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Mathematician, recognized as the first computer programmer for her work on Babbage's Analytical Engine",
            ),
        },
    ]
}

/// Returns figures matching the given domain.
#[must_use]
#[inline]
pub fn by_domain(domain: &FigureDomain) -> Vec<Figure> {
    tracing::debug!(?domain, "looking up figures by domain");
    all_figures()
        .into_iter()
        .filter(|f| f.domain == *domain)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_figures_count() {
        assert_eq!(all_figures().len(), 10);
    }

    #[test]
    fn test_by_domain_ruler() {
        let rulers = by_domain(&FigureDomain::Ruler);
        assert!(rulers.iter().any(|f| f.name == "Hammurabi"));
        assert!(rulers.iter().any(|f| f.name == "Ashoka"));
    }

    #[test]
    fn test_by_domain_scientist() {
        let scientists = by_domain(&FigureDomain::Scientist);
        assert!(scientists.iter().any(|f| f.name == "Hypatia"));
        assert!(scientists.iter().any(|f| f.name == "Isaac Newton"));
        assert!(scientists.iter().any(|f| f.name == "Ada Lovelace"));
    }

    #[test]
    fn test_by_domain_military() {
        let mil = by_domain(&FigureDomain::Military);
        assert!(mil.iter().any(|f| f.name == "Julius Caesar"));
        assert!(mil.iter().any(|f| f.name == "Genghis Khan"));
    }

    #[test]
    fn test_by_domain_empty() {
        let religious = by_domain(&FigureDomain::Religious);
        assert!(religious.is_empty());
    }

    #[test]
    fn test_figure_serde_roundtrip() {
        for fig in all_figures() {
            let json = serde_json::to_string(&fig).unwrap();
            let back: Figure = serde_json::from_str(&json).unwrap();
            assert_eq!(fig, back);
        }
    }

    #[test]
    fn test_figure_domain_serde_roundtrip() {
        let domains = [
            FigureDomain::Ruler,
            FigureDomain::Philosopher,
            FigureDomain::Scientist,
            FigureDomain::Artist,
            FigureDomain::Military,
            FigureDomain::Religious,
            FigureDomain::Explorer,
            FigureDomain::Inventor,
        ];
        for d in &domains {
            let json = serde_json::to_string(d).unwrap();
            let back: FigureDomain = serde_json::from_str(&json).unwrap();
            assert_eq!(*d, back);
        }
    }
}
