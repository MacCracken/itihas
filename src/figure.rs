//! Historical figures with era and civilization context.
//!
//! Provides [`Figure`] structs representing major historical figures, a
//! [`FigureDomain`] classification enum, and 52 pre-built figures spanning
//! from Hammurabi to Nikola Tesla.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

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

impl fmt::Display for FigureDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ruler => f.write_str("Ruler"),
            Self::Philosopher => f.write_str("Philosopher"),
            Self::Scientist => f.write_str("Scientist"),
            Self::Artist => f.write_str("Artist"),
            Self::Military => f.write_str("Military"),
            Self::Religious => f.write_str("Religious"),
            Self::Explorer => f.write_str("Explorer"),
            Self::Inventor => f.write_str("Inventor"),
        }
    }
}

/// A historical figure.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.birth_year, self.death_year) {
            (Some(b), Some(d)) => write!(f, "{} ({} – {})", self.name, b, d),
            (Some(b), None) => write!(f, "{} (b. {})", self.name, b),
            (None, Some(d)) => write!(f, "{} (d. {})", self.name, d),
            (None, None) => write!(f, "{}", self.name),
        }
    }
}

fn build_figures() -> Vec<Figure> {
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
            civilization: Cow::Borrowed("Maurya Empire"),
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
        // ── Rulers ──────────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Ramesses II"),
            birth_year: Some(-1303),
            death_year: Some(-1213),
            civilization: Cow::Borrowed("Ancient Egypt"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Pharaoh of the Nineteenth Dynasty, prolific builder and military campaigner",
            ),
        },
        Figure {
            name: Cow::Borrowed("Mansa Musa"),
            birth_year: Some(1280),
            death_year: Some(1337),
            civilization: Cow::Borrowed("Mali Empire"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Emperor of Mali, renowned for extraordinary wealth and pilgrimage to Mecca",
            ),
        },
        Figure {
            name: Cow::Borrowed("Pachacuti"),
            birth_year: Some(1418),
            death_year: Some(1471),
            civilization: Cow::Borrowed("Inca Empire"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Ninth Sapa Inca who transformed Cusco and expanded the Inca Empire",
            ),
        },
        Figure {
            name: Cow::Borrowed("Suleiman the Magnificent"),
            birth_year: Some(1494),
            death_year: Some(1566),
            civilization: Cow::Borrowed("Ottoman Empire"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Longest-reigning Ottoman sultan, presided over the empire's golden age",
            ),
        },
        Figure {
            name: Cow::Borrowed("Montezuma II"),
            birth_year: Some(1466),
            death_year: Some(1520),
            civilization: Cow::Borrowed("Aztec Empire"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Ninth tlatoani of Tenochtitlan at the height of the Aztec Empire",
            ),
        },
        Figure {
            name: Cow::Borrowed("Sundiata Keita"),
            birth_year: Some(1217),
            death_year: Some(1255),
            civilization: Cow::Borrowed("Mali Empire"),
            domain: FigureDomain::Ruler,
            description: Cow::Borrowed(
                "Founder of the Mali Empire and hero of the Mandinka oral epic tradition",
            ),
        },
        // ── Philosophers ────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Confucius"),
            birth_year: Some(-551),
            death_year: Some(-479),
            civilization: Cow::Borrowed("Ancient China"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Chinese thinker whose teachings on ethics and governance shaped East Asian civilization",
            ),
        },
        Figure {
            name: Cow::Borrowed("Chanakya"),
            birth_year: Some(-375),
            death_year: Some(-283),
            civilization: Cow::Borrowed("Maurya Empire"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Author of the Arthashastra, advisor to Chandragupta Maurya, pioneer of political science",
            ),
        },
        Figure {
            name: Cow::Borrowed("Socrates"),
            birth_year: Some(-470),
            death_year: Some(-399),
            civilization: Cow::Borrowed("Ancient Greece"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Athenian philosopher, founder of Western ethics through the Socratic method",
            ),
        },
        Figure {
            name: Cow::Borrowed("Nagarjuna"),
            birth_year: Some(150),
            death_year: Some(250),
            civilization: Cow::Borrowed("Satavahana Dynasty"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Indian philosopher, founder of the Madhyamaka school of Mahayana Buddhism",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ibn Rushd"),
            birth_year: Some(1126),
            death_year: Some(1198),
            civilization: Cow::Borrowed("Almohad Caliphate"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed(
                "Andalusian polymath, foremost commentator on Aristotle in the Islamic world",
            ),
        },
        Figure {
            name: Cow::Borrowed("Zera Yacob"),
            birth_year: Some(1599),
            death_year: Some(1692),
            civilization: Cow::Borrowed("Ethiopian Empire"),
            domain: FigureDomain::Philosopher,
            description: Cow::Borrowed("Ethiopian rationalist philosopher, author of the Hatata"),
        },
        // ── Scientists ──────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Aryabhata"),
            birth_year: Some(476),
            death_year: Some(550),
            civilization: Cow::Borrowed("Gupta Empire"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Indian mathematician and astronomer, pioneered zero and place-value notation",
            ),
        },
        Figure {
            name: Cow::Borrowed("Al-Khwarizmi"),
            birth_year: Some(780),
            death_year: Some(850),
            civilization: Cow::Borrowed("Abbasid Caliphate"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Persian mathematician, father of algebra, introduced Hindu-Arabic numerals to the West",
            ),
        },
        Figure {
            name: Cow::Borrowed("Avicenna"),
            birth_year: Some(980),
            death_year: Some(1037),
            civilization: Cow::Borrowed("Samanid Empire"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Persian polymath, author of The Canon of Medicine, foundational text for centuries",
            ),
        },
        Figure {
            name: Cow::Borrowed("Galileo Galilei"),
            birth_year: Some(1564),
            death_year: Some(1642),
            civilization: Cow::Borrowed("Italian city-states"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Astronomer and physicist, championed heliocentrism and pioneered telescopic observation",
            ),
        },
        Figure {
            name: Cow::Borrowed("Seki Takakazu"),
            birth_year: Some(1642),
            death_year: Some(1708),
            civilization: Cow::Borrowed("Tokugawa Shogunate"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Japanese mathematician who independently developed calculus-like methods and determinants",
            ),
        },
        Figure {
            name: Cow::Borrowed("Jagadish Chandra Bose"),
            birth_year: Some(1858),
            death_year: Some(1937),
            civilization: Cow::Borrowed("British Empire"),
            domain: FigureDomain::Scientist,
            description: Cow::Borrowed(
                "Bengali physicist and biologist, pioneer of radio science and plant physiology",
            ),
        },
        // ── Artists ─────────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Murasaki Shikibu"),
            birth_year: Some(978),
            death_year: Some(1014),
            civilization: Cow::Borrowed("Heian Japan"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Japanese novelist, author of The Tale of Genji, considered the first novel",
            ),
        },
        Figure {
            name: Cow::Borrowed("Rumi"),
            birth_year: Some(1207),
            death_year: Some(1273),
            civilization: Cow::Borrowed("Seljuk Sultanate of Rum"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Persian poet and Sufi mystic, author of the Masnavi, one of the greatest works of Persian literature",
            ),
        },
        Figure {
            name: Cow::Borrowed("Michelangelo"),
            birth_year: Some(1475),
            death_year: Some(1564),
            civilization: Cow::Borrowed("Italian city-states"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Sculptor, painter, and architect; created the Sistine Chapel ceiling and David",
            ),
        },
        Figure {
            name: Cow::Borrowed("William Shakespeare"),
            birth_year: Some(1564),
            death_year: Some(1616),
            civilization: Cow::Borrowed("Kingdom of England"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Playwright and poet, authored works central to the English literary canon",
            ),
        },
        Figure {
            name: Cow::Borrowed("Hokusai"),
            birth_year: Some(1760),
            death_year: Some(1849),
            civilization: Cow::Borrowed("Tokugawa Shogunate"),
            domain: FigureDomain::Artist,
            description: Cow::Borrowed(
                "Japanese ukiyo-e artist, created The Great Wave off Kanagawa and Thirty-six Views of Mount Fuji",
            ),
        },
        // ── Military ────────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Sun Tzu"),
            birth_year: Some(-544),
            death_year: Some(-496),
            civilization: Cow::Borrowed("Ancient China"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed("Chinese strategist, author of The Art of War"),
        },
        Figure {
            name: Cow::Borrowed("Hannibal Barca"),
            birth_year: Some(-247),
            death_year: Some(-183),
            civilization: Cow::Borrowed("Phoenicia"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Carthaginian general who crossed the Alps and defeated Roman armies at Cannae",
            ),
        },
        Figure {
            name: Cow::Borrowed("Saladin"),
            birth_year: Some(1137),
            death_year: Some(1193),
            civilization: Cow::Borrowed("Ayyubid Sultanate"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Sultan who united Egypt and Syria and recaptured Jerusalem from the Crusaders",
            ),
        },
        Figure {
            name: Cow::Borrowed("Yi Sun-sin"),
            birth_year: Some(1545),
            death_year: Some(1598),
            civilization: Cow::Borrowed("Joseon Dynasty"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Korean admiral who defeated the Japanese navy with innovative turtle ships",
            ),
        },
        Figure {
            name: Cow::Borrowed("Shaka Zulu"),
            birth_year: Some(1787),
            death_year: Some(1828),
            civilization: Cow::Borrowed("Zulu Kingdom"),
            domain: FigureDomain::Military,
            description: Cow::Borrowed(
                "Zulu king who revolutionized warfare in southern Africa with new tactics and formations",
            ),
        },
        // ── Religious ───────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Siddhartha Gautama"),
            birth_year: Some(-563),
            death_year: Some(-483),
            civilization: Cow::Borrowed("Shakya Republic"),
            domain: FigureDomain::Religious,
            description: Cow::Borrowed(
                "The Buddha, founder of Buddhism and teacher of the Middle Way",
            ),
        },
        Figure {
            name: Cow::Borrowed("Jesus of Nazareth"),
            birth_year: Some(-4),
            death_year: Some(30),
            civilization: Cow::Borrowed("Roman Empire"),
            domain: FigureDomain::Religious,
            description: Cow::Borrowed(
                "Central figure of Christianity, whose teachings shaped Western civilization",
            ),
        },
        Figure {
            name: Cow::Borrowed("Muhammad"),
            birth_year: Some(570),
            death_year: Some(632),
            civilization: Cow::Borrowed("Rashidun Caliphate"),
            domain: FigureDomain::Religious,
            description: Cow::Borrowed(
                "Prophet of Islam, unified the Arabian Peninsula and established the Muslim community",
            ),
        },
        Figure {
            name: Cow::Borrowed("Guru Nanak"),
            birth_year: Some(1469),
            death_year: Some(1539),
            civilization: Cow::Borrowed("Mughal Empire"),
            domain: FigureDomain::Religious,
            description: Cow::Borrowed(
                "Founder of Sikhism, emphasized devotion, equality, and service",
            ),
        },
        Figure {
            name: Cow::Borrowed("Martin Luther"),
            birth_year: Some(1483),
            death_year: Some(1546),
            civilization: Cow::Borrowed("Holy Roman Empire"),
            domain: FigureDomain::Religious,
            description: Cow::Borrowed(
                "Theologian whose Ninety-five Theses sparked the Protestant Reformation",
            ),
        },
        // ── Explorers ───────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Leif Erikson"),
            birth_year: Some(970),
            death_year: Some(1020),
            civilization: Cow::Borrowed("Viking/Norse"),
            domain: FigureDomain::Explorer,
            description: Cow::Borrowed(
                "Norse explorer, first European known to have reached North America",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ibn Battuta"),
            birth_year: Some(1304),
            death_year: Some(1369),
            civilization: Cow::Borrowed("Marinid Sultanate"),
            domain: FigureDomain::Explorer,
            description: Cow::Borrowed(
                "Moroccan traveller who journeyed over 120,000 km across Africa, Asia, and Europe",
            ),
        },
        Figure {
            name: Cow::Borrowed("Zheng He"),
            birth_year: Some(1371),
            death_year: Some(1433),
            civilization: Cow::Borrowed("Ming China"),
            domain: FigureDomain::Explorer,
            description: Cow::Borrowed(
                "Chinese admiral who led seven naval expeditions across the Indian Ocean",
            ),
        },
        Figure {
            name: Cow::Borrowed("Vasco da Gama"),
            birth_year: Some(1469),
            death_year: Some(1524),
            civilization: Cow::Borrowed("Portuguese Empire"),
            domain: FigureDomain::Explorer,
            description: Cow::Borrowed(
                "Portuguese explorer, first European to reach India by sea around Africa",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ranald MacDonald"),
            birth_year: Some(1824),
            death_year: Some(1894),
            civilization: Cow::Borrowed("United States"),
            domain: FigureDomain::Explorer,
            description: Cow::Borrowed(
                "Half-Chinook adventurer, first native English speaker to teach English in Japan",
            ),
        },
        // ── Inventors ───────────────────────────────────────────────
        Figure {
            name: Cow::Borrowed("Cai Lun"),
            birth_year: Some(50),
            death_year: Some(121),
            civilization: Cow::Borrowed("Ancient China"),
            domain: FigureDomain::Inventor,
            description: Cow::Borrowed(
                "Chinese court official who refined papermaking, enabling widespread literacy",
            ),
        },
        Figure {
            name: Cow::Borrowed("Ismail al-Jazari"),
            birth_year: Some(1136),
            death_year: Some(1206),
            civilization: Cow::Borrowed("Artuqid Dynasty"),
            domain: FigureDomain::Inventor,
            description: Cow::Borrowed(
                "Polymath engineer, built programmable automata and pioneered crankshaft mechanisms",
            ),
        },
        Figure {
            name: Cow::Borrowed("James Watt"),
            birth_year: Some(1736),
            death_year: Some(1819),
            civilization: Cow::Borrowed("British Empire"),
            domain: FigureDomain::Inventor,
            description: Cow::Borrowed(
                "Scottish inventor whose improved steam engine powered the Industrial Revolution",
            ),
        },
        Figure {
            name: Cow::Borrowed("Nikola Tesla"),
            birth_year: Some(1856),
            death_year: Some(1943),
            civilization: Cow::Borrowed("United States"),
            domain: FigureDomain::Inventor,
            description: Cow::Borrowed(
                "Serbian-American inventor of alternating current systems and induction motor",
            ),
        },
    ]
}

/// Returns all pre-built historical figures.
///
/// Data is computed once and cached for the lifetime of the process.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_figures() -> &'static [Figure] {
    static DATA: std::sync::LazyLock<Vec<Figure>> = std::sync::LazyLock::new(build_figures);
    &DATA
}

/// Returns all pre-built historical figures.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_figures() -> Vec<Figure> {
    build_figures()
}

/// Returns figures matching the given domain.
#[must_use]
#[inline]
pub fn by_domain(domain: &FigureDomain) -> Vec<Figure> {
    tracing::debug!(?domain, "looking up figures by domain");
    all_figures()
        .iter()
        .filter(|f| f.domain == *domain)
        .cloned()
        .collect()
}

/// Look up a figure by exact name (case-insensitive).
///
/// Returns `Err(ItihasError::FigureNotFound)` if no figure matches.
#[inline]
pub fn by_name(name: &str) -> Result<Figure, ItihasError> {
    tracing::debug!(name, "looking up figure by name");
    let lower = name.to_lowercase();
    all_figures()
        .iter()
        .find(|f| f.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::FigureNotFound(String::from(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_figures_count() {
        assert_eq!(all_figures().len(), 52);
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
    fn test_by_domain_religious() {
        let religious = by_domain(&FigureDomain::Religious);
        assert!(!religious.is_empty());
        assert!(religious.iter().any(|f| f.name == "Siddhartha Gautama"));
    }

    #[test]
    fn test_figure_serde_roundtrip() {
        for fig in all_figures() {
            let json = serde_json::to_string(fig).unwrap();
            let back: Figure = serde_json::from_str(&json).unwrap();
            assert_eq!(*fig, back);
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
