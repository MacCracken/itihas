//! Archaeological sites with location, period, and discovery metadata.
//!
//! Provides [`Site`] structs representing major archaeological sites,
//! a [`SiteType`] classification enum, and lookup functions by region,
//! period, and civilization.
//!
//! # Sources
//!
//! Methodology: Renfrew & Bahn (2020). Prehistory: Scarre (2018). UNESCO
//! sites: World Heritage Centre advisory evaluations. Gobekli Tepe: Schmidt
//! (2012). Catalhoyuk: Hodder (2006). Ur: Woolley (1982). Full bibliography:
//! [`docs/sources/sites.md`](https://github.com/MacCracken/itihas/blob/main/docs/sources/sites.md).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// Classification of archaeological sites.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SiteType {
    /// Urban settlement or city ruins.
    Settlement,
    /// Temple, shrine, or sacred precinct.
    Temple,
    /// Burial ground, tomb, or necropolis.
    Burial,
    /// Defensive structure or fortified position.
    Fortress,
    /// Monumental construction (pyramids, megaliths, etc.).
    Monument,
    /// Palace or royal administrative complex.
    Palace,
    /// Workshop, kiln, or production facility.
    Workshop,
    /// Cave with evidence of habitation or art.
    Cave,
    /// Port, harbor, or maritime facility.
    Port,
}

impl fmt::Display for SiteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Settlement => f.write_str("Settlement"),
            Self::Temple => f.write_str("Temple"),
            Self::Burial => f.write_str("Burial"),
            Self::Fortress => f.write_str("Fortress"),
            Self::Monument => f.write_str("Monument"),
            Self::Palace => f.write_str("Palace"),
            Self::Workshop => f.write_str("Workshop"),
            Self::Cave => f.write_str("Cave"),
            Self::Port => f.write_str("Port"),
        }
    }
}

/// An archaeological site.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Site {
    /// Name of the site.
    pub name: Cow<'static, str>,
    /// Modern country or region where the site is located.
    pub location: Cow<'static, str>,
    /// Geographic region.
    pub region: Cow<'static, str>,
    /// Type classification.
    pub site_type: SiteType,
    /// Associated civilization(s).
    pub civilization: Cow<'static, str>,
    /// Approximate earliest date of occupation or construction (negative = BCE).
    pub start_year: i32,
    /// Approximate end of occupation (negative = BCE). `i32::MAX` for ongoing.
    pub end_year: i32,
    /// Year the site was discovered or first excavated by modern archaeology.
    pub discovery_year: i32,
    /// Brief description.
    pub description: Cow<'static, str>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.name, self.location, self.start_year)
    }
}

fn build_sites() -> Vec<Site> {
    vec![
        // --- Near East ---
        Site {
            name: Cow::Borrowed("Göbekli Tepe"),
            location: Cow::Borrowed("Turkey"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Pre-Pottery Neolithic"),
            start_year: -9500,
            end_year: -8000,
            discovery_year: 1963,
            description: Cow::Borrowed(
                "Oldest known monumental sanctuary; massive T-shaped pillars with animal reliefs",
            ),
        },
        Site {
            name: Cow::Borrowed("Çatalhöyük"),
            location: Cow::Borrowed("Turkey"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Neolithic Anatolia"),
            start_year: -7500,
            end_year: -5700,
            discovery_year: 1958,
            description: Cow::Borrowed(
                "Large Neolithic proto-city; densely packed mudbrick houses entered through rooftops",
            ),
        },
        Site {
            name: Cow::Borrowed("Ur"),
            location: Cow::Borrowed("Iraq"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Mesopotamia"),
            start_year: -3800,
            end_year: -500,
            discovery_year: 1853,
            description: Cow::Borrowed(
                "Sumerian city-state; Royal Tombs with rich grave goods, Great Ziggurat",
            ),
        },
        Site {
            name: Cow::Borrowed("Babylon"),
            location: Cow::Borrowed("Iraq"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Mesopotamia"),
            start_year: -2300,
            end_year: -275,
            discovery_year: 1899,
            description: Cow::Borrowed(
                "Capital of Babylonian Empire; Ishtar Gate, Hanging Gardens tradition",
            ),
        },
        Site {
            name: Cow::Borrowed("Persepolis"),
            location: Cow::Borrowed("Iran"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Palace,
            civilization: Cow::Borrowed("Persian Empire"),
            start_year: -518,
            end_year: -330,
            discovery_year: 1620,
            description: Cow::Borrowed(
                "Ceremonial capital of the Achaemenid Empire; Apadana reliefs of tribute-bearers",
            ),
        },
        Site {
            name: Cow::Borrowed("Petra"),
            location: Cow::Borrowed("Jordan"),
            region: Cow::Borrowed("Near East"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Nabataean Kingdom"),
            start_year: -312,
            end_year: 663,
            discovery_year: 1812,
            description: Cow::Borrowed(
                "Rock-cut Nabataean capital; Al-Khazneh treasury façade, water conduit system",
            ),
        },
        // --- North Africa ---
        Site {
            name: Cow::Borrowed("Giza"),
            location: Cow::Borrowed("Egypt"),
            region: Cow::Borrowed("North Africa"),
            site_type: SiteType::Monument,
            civilization: Cow::Borrowed("Ancient Egypt"),
            start_year: -2580,
            end_year: -2510,
            discovery_year: 0, // known since antiquity
            description: Cow::Borrowed(
                "Great Pyramid of Khufu, Sphinx; Old Kingdom royal necropolis",
            ),
        },
        Site {
            name: Cow::Borrowed("Valley of the Kings"),
            location: Cow::Borrowed("Egypt"),
            region: Cow::Borrowed("North Africa"),
            site_type: SiteType::Burial,
            civilization: Cow::Borrowed("Ancient Egypt"),
            start_year: -1539,
            end_year: -1075,
            discovery_year: 1738,
            description: Cow::Borrowed(
                "New Kingdom royal tombs including Tutankhamun's tomb (KV62)",
            ),
        },
        Site {
            name: Cow::Borrowed("Carthage"),
            location: Cow::Borrowed("Tunisia"),
            region: Cow::Borrowed("North Africa"),
            site_type: SiteType::Port,
            civilization: Cow::Borrowed("Phoenicia"),
            start_year: -814,
            end_year: -146,
            discovery_year: 1830,
            description: Cow::Borrowed(
                "Phoenician maritime capital; circular military harbor, tophet sanctuary",
            ),
        },
        Site {
            name: Cow::Borrowed("Leptis Magna"),
            location: Cow::Borrowed("Libya"),
            region: Cow::Borrowed("North Africa"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Roman Empire"),
            start_year: -1000,
            end_year: 647,
            discovery_year: 1920,
            description: Cow::Borrowed("Roman city; Severan basilica, market, theater, and harbor"),
        },
        // --- Mediterranean ---
        Site {
            name: Cow::Borrowed("Knossos"),
            location: Cow::Borrowed("Greece"),
            region: Cow::Borrowed("Mediterranean"),
            site_type: SiteType::Palace,
            civilization: Cow::Borrowed("Minoan Civilization"),
            start_year: -2000,
            end_year: -1375,
            discovery_year: 1878,
            description: Cow::Borrowed(
                "Minoan palace complex; labyrinthine layout, bull-leap frescoes, Linear A/B tablets",
            ),
        },
        Site {
            name: Cow::Borrowed("Mycenae"),
            location: Cow::Borrowed("Greece"),
            region: Cow::Borrowed("Mediterranean"),
            site_type: SiteType::Fortress,
            civilization: Cow::Borrowed("Mycenaean Civilization"),
            start_year: -1600,
            end_year: -1100,
            discovery_year: 1876,
            description: Cow::Borrowed(
                "Mycenaean citadel; Lion Gate, Treasury of Atreus, Schliemann's gold masks",
            ),
        },
        Site {
            name: Cow::Borrowed("Pompeii"),
            location: Cow::Borrowed("Italy"),
            region: Cow::Borrowed("Mediterranean"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Roman Empire"),
            start_year: -600,
            end_year: 79,
            discovery_year: 1748,
            description: Cow::Borrowed(
                "Roman city preserved by Vesuvius eruption; complete urban snapshot of 79 CE",
            ),
        },
        // --- South Asia ---
        Site {
            name: Cow::Borrowed("Mohenjo-daro"),
            location: Cow::Borrowed("Pakistan"),
            region: Cow::Borrowed("South Asia"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Indus Valley"),
            start_year: -2500,
            end_year: -1900,
            discovery_year: 1922,
            description: Cow::Borrowed(
                "Indus Valley city; Great Bath, grid-plan streets, standardized brick construction",
            ),
        },
        Site {
            name: Cow::Borrowed("Harappa"),
            location: Cow::Borrowed("Pakistan"),
            region: Cow::Borrowed("South Asia"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Indus Valley"),
            start_year: -2600,
            end_year: -1900,
            discovery_year: 1826,
            description: Cow::Borrowed(
                "Indus Valley city; granaries, worker quarters, and extensive craft production",
            ),
        },
        Site {
            name: Cow::Borrowed("Sanchi"),
            location: Cow::Borrowed("India"),
            region: Cow::Borrowed("South Asia"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Maurya Empire"),
            start_year: -300,
            end_year: 1200,
            discovery_year: 1818,
            description: Cow::Borrowed(
                "Buddhist stupa complex; Great Stupa with carved gateways (toranas)",
            ),
        },
        Site {
            name: Cow::Borrowed("Ajanta Caves"),
            location: Cow::Borrowed("India"),
            region: Cow::Borrowed("South Asia"),
            site_type: SiteType::Cave,
            civilization: Cow::Borrowed("Gupta Empire"),
            start_year: -200,
            end_year: 650,
            discovery_year: 1819,
            description: Cow::Borrowed(
                "Rock-cut Buddhist monasteries and prayer halls; masterwork murals and sculpture",
            ),
        },
        // --- East Asia ---
        Site {
            name: Cow::Borrowed("Terracotta Army"),
            location: Cow::Borrowed("China"),
            region: Cow::Borrowed("East Asia"),
            site_type: SiteType::Burial,
            civilization: Cow::Borrowed("Ancient China"),
            start_year: -246,
            end_year: -208,
            discovery_year: 1974,
            description: Cow::Borrowed(
                "Mausoleum guard of Qin Shi Huang; 8,000+ life-size clay warriors",
            ),
        },
        Site {
            name: Cow::Borrowed("Longmen Grottoes"),
            location: Cow::Borrowed("China"),
            region: Cow::Borrowed("East Asia"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Ancient China"),
            start_year: 493,
            end_year: 1127,
            discovery_year: 0, // known since creation
            description: Cow::Borrowed(
                "Buddhist rock-cut shrines; 100,000+ statues across 2,345 caves",
            ),
        },
        Site {
            name: Cow::Borrowed("Nara Heijō Palace"),
            location: Cow::Borrowed("Japan"),
            region: Cow::Borrowed("East Asia"),
            site_type: SiteType::Palace,
            civilization: Cow::Borrowed("Japan"),
            start_year: 710,
            end_year: 784,
            discovery_year: 1852,
            description: Cow::Borrowed(
                "Imperial capital; modeled on Tang Chang'an, wooden palace complex",
            ),
        },
        // --- Southeast Asia ---
        Site {
            name: Cow::Borrowed("Angkor Wat"),
            location: Cow::Borrowed("Cambodia"),
            region: Cow::Borrowed("Southeast Asia"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Khmer Empire"),
            start_year: 1113,
            end_year: 1431,
            discovery_year: 1860,
            description: Cow::Borrowed(
                "Largest religious monument; Hindu-Buddhist temple with bas-relief galleries",
            ),
        },
        Site {
            name: Cow::Borrowed("Borobudur"),
            location: Cow::Borrowed("Indonesia"),
            region: Cow::Borrowed("Southeast Asia"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Sailendra Dynasty"),
            start_year: 780,
            end_year: 1006,
            discovery_year: 1814,
            description: Cow::Borrowed(
                "Largest Buddhist temple; 2,672 relief panels, 504 Buddha statues",
            ),
        },
        // --- Mesoamerica ---
        Site {
            name: Cow::Borrowed("Teotihuacan"),
            location: Cow::Borrowed("Mexico"),
            region: Cow::Borrowed("Mesoamerica"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Teotihuacan Civilization"),
            start_year: -200,
            end_year: 550,
            discovery_year: 0, // known to Aztecs
            description: Cow::Borrowed(
                "Largest pre-Columbian city in the Americas; Pyramid of the Sun, Avenue of the Dead",
            ),
        },
        Site {
            name: Cow::Borrowed("Tikal"),
            location: Cow::Borrowed("Guatemala"),
            region: Cow::Borrowed("Mesoamerica"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Maya"),
            start_year: -600,
            end_year: 900,
            discovery_year: 1848,
            description: Cow::Borrowed(
                "Major Maya city; towering temple pyramids, stelae with Long Count dates",
            ),
        },
        Site {
            name: Cow::Borrowed("Chichén Itzá"),
            location: Cow::Borrowed("Mexico"),
            region: Cow::Borrowed("Mesoamerica"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Maya"),
            start_year: 600,
            end_year: 1200,
            discovery_year: 1843,
            description: Cow::Borrowed(
                "Maya-Toltec city; El Castillo pyramid with equinox serpent shadow",
            ),
        },
        Site {
            name: Cow::Borrowed("Machu Picchu"),
            location: Cow::Borrowed("Peru"),
            region: Cow::Borrowed("South America"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Inca Empire"),
            start_year: 1450,
            end_year: 1572,
            discovery_year: 1911,
            description: Cow::Borrowed(
                "Inca royal estate; dry-stone terraces, Intihuatana sundial, cloud-forest setting",
            ),
        },
        // --- Sub-Saharan Africa ---
        Site {
            name: Cow::Borrowed("Great Zimbabwe"),
            location: Cow::Borrowed("Zimbabwe"),
            region: Cow::Borrowed("Sub-Saharan Africa"),
            site_type: SiteType::Settlement,
            civilization: Cow::Borrowed("Kingdom of Zimbabwe"),
            start_year: 1100,
            end_year: 1450,
            discovery_year: 1871,
            description: Cow::Borrowed(
                "Medieval stone city; Great Enclosure with drystone walls up to 11m high",
            ),
        },
        Site {
            name: Cow::Borrowed("Lalibela"),
            location: Cow::Borrowed("Ethiopia"),
            region: Cow::Borrowed("Sub-Saharan Africa"),
            site_type: SiteType::Temple,
            civilization: Cow::Borrowed("Zagwe Dynasty"),
            start_year: 1181,
            end_year: 1221,
            discovery_year: 1520,
            description: Cow::Borrowed(
                "11 monolithic rock-hewn churches; carved top-down from living rock",
            ),
        },
        // --- Europe ---
        Site {
            name: Cow::Borrowed("Stonehenge"),
            location: Cow::Borrowed("England"),
            region: Cow::Borrowed("Europe"),
            site_type: SiteType::Monument,
            civilization: Cow::Borrowed("Neolithic Britain"),
            start_year: -3000,
            end_year: -1500,
            discovery_year: 0, // known since antiquity
            description: Cow::Borrowed(
                "Megalithic stone circle; sarsen trilithons aligned to solstice sunrise",
            ),
        },
        Site {
            name: Cow::Borrowed("Lascaux"),
            location: Cow::Borrowed("France"),
            region: Cow::Borrowed("Europe"),
            site_type: SiteType::Cave,
            civilization: Cow::Borrowed("Upper Paleolithic"),
            start_year: -17000,
            end_year: -15000,
            discovery_year: 1940,
            description: Cow::Borrowed(
                "Paleolithic cave paintings; Hall of Bulls, polychrome animal figures",
            ),
        },
        Site {
            name: Cow::Borrowed("Altamira"),
            location: Cow::Borrowed("Spain"),
            region: Cow::Borrowed("Europe"),
            site_type: SiteType::Cave,
            civilization: Cow::Borrowed("Upper Paleolithic"),
            start_year: -36000,
            end_year: -13000,
            discovery_year: 1868,
            description: Cow::Borrowed(
                "Paleolithic cave art; polychrome bison ceiling, earliest known artistic masterwork",
            ),
        },
        Site {
            name: Cow::Borrowed("Newgrange"),
            location: Cow::Borrowed("Ireland"),
            region: Cow::Borrowed("Europe"),
            site_type: SiteType::Burial,
            civilization: Cow::Borrowed("Neolithic Ireland"),
            start_year: -3200,
            end_year: -2900,
            discovery_year: 1699,
            description: Cow::Borrowed(
                "Passage tomb; roof box aligned to winter solstice sunrise illuminating chamber",
            ),
        },
    ]
}

/// Returns all pre-built archaeological sites.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_sites() -> &'static [Site] {
    static DATA: std::sync::LazyLock<Vec<Site>> = std::sync::LazyLock::new(build_sites);
    &DATA
}

/// Returns all pre-built archaeological sites.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_sites() -> Vec<Site> {
    build_sites()
}

/// Returns sites whose region contains the given substring (case-insensitive).
#[must_use]
#[inline]
pub fn by_region(region: &str) -> Vec<Site> {
    tracing::debug!(region, "looking up sites by region");
    let lower = region.to_lowercase();
    all_sites()
        .iter()
        .filter(|s| s.region.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

/// Returns sites associated with a given civilization (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_civilization(civilization: &str) -> Vec<Site> {
    tracing::debug!(civilization, "looking up sites by civilization");
    let lower = civilization.to_lowercase();
    all_sites()
        .iter()
        .filter(|s| s.civilization.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

/// Returns sites of the given type.
#[must_use]
#[inline]
pub fn by_type(site_type: &SiteType) -> Vec<Site> {
    tracing::debug!(?site_type, "looking up sites by type");
    all_sites()
        .iter()
        .filter(|s| s.site_type == *site_type)
        .cloned()
        .collect()
}

/// Returns sites that were occupied during the given year.
#[must_use]
#[inline]
pub fn active_at(year: i32) -> Vec<Site> {
    tracing::debug!(year, "looking up sites active at year");
    all_sites()
        .iter()
        .filter(|s| year >= s.start_year && year <= s.end_year)
        .cloned()
        .collect()
}

/// Look up a site by exact name (case-insensitive).
///
/// # Errors
///
/// Returns [`ItihasError::SiteNotFound`] if no site matches.
pub fn by_name(name: &str) -> Result<Site, ItihasError> {
    tracing::debug!(name, "looking up site by name");
    let lower = name.to_lowercase();
    all_sites()
        .iter()
        .find(|s| s.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::SiteNotFound(String::from(name)))
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_all_sites_count() {
        assert_eq!(all_sites().len(), 32);
    }

    #[test]
    fn test_by_region_near_east() {
        let sites = by_region("Near East");
        assert!(sites.iter().any(|s| s.name == "Göbekli Tepe"));
        assert!(sites.iter().any(|s| s.name == "Ur"));
        assert!(sites.iter().any(|s| s.name == "Persepolis"));
    }

    #[test]
    fn test_by_region_case_insensitive() {
        let sites = by_region("mediterranean");
        assert!(sites.iter().any(|s| s.name == "Pompeii"));
        assert!(sites.iter().any(|s| s.name == "Knossos"));
    }

    #[test]
    fn test_by_region_no_match() {
        assert!(by_region("Antarctica").is_empty());
    }

    #[test]
    fn test_by_civilization() {
        let sites = by_civilization("Maya");
        assert!(sites.iter().any(|s| s.name == "Tikal"));
        assert!(sites.iter().any(|s| s.name == "Chichén Itzá"));
    }

    #[test]
    fn test_by_type_temple() {
        let sites = by_type(&SiteType::Temple);
        assert!(sites.iter().any(|s| s.name == "Göbekli Tepe"));
        assert!(sites.iter().any(|s| s.name == "Angkor Wat"));
    }

    #[test]
    fn test_by_type_cave() {
        let sites = by_type(&SiteType::Cave);
        assert!(sites.iter().any(|s| s.name == "Lascaux"));
        assert!(sites.iter().any(|s| s.name == "Ajanta Caves"));
    }

    #[test]
    fn test_active_at_2000_bce() {
        let sites = active_at(-2000);
        let names: Vec<_> = sites.iter().map(|s| s.name.as_ref()).collect();
        assert!(names.contains(&"Ur"));
        assert!(names.contains(&"Mohenjo-daro"));
    }

    #[test]
    fn test_active_at_none() {
        assert!(active_at(-200_000).is_empty());
    }

    #[test]
    fn test_by_name_found() {
        let site = by_name("Pompeii").unwrap();
        assert_eq!(site.location.as_ref(), "Italy");
    }

    #[test]
    fn test_by_name_case_insensitive() {
        assert!(by_name("pompeii").is_ok());
    }

    #[test]
    fn test_by_name_not_found() {
        assert!(by_name("Atlantis").is_err());
    }

    #[test]
    fn test_site_display() {
        let site = by_name("Giza").unwrap();
        let display = format!("{site}");
        assert!(display.contains("Giza"));
        assert!(display.contains("Egypt"));
    }

    #[test]
    fn test_site_type_display() {
        assert_eq!(format!("{}", SiteType::Temple), "Temple");
        assert_eq!(format!("{}", SiteType::Settlement), "Settlement");
    }

    #[test]
    fn test_site_serde_roundtrip() {
        for site in all_sites().iter() {
            let json = serde_json::to_string(site).unwrap();
            let back: Site = serde_json::from_str(&json).unwrap();
            assert_eq!(site, &back);
        }
    }

    #[test]
    fn test_site_type_serde_roundtrip() {
        let types = [
            SiteType::Settlement,
            SiteType::Temple,
            SiteType::Burial,
            SiteType::Fortress,
            SiteType::Monument,
            SiteType::Palace,
            SiteType::Workshop,
            SiteType::Cave,
            SiteType::Port,
        ];
        for st in &types {
            let json = serde_json::to_string(st).unwrap();
            let back: SiteType = serde_json::from_str(&json).unwrap();
            assert_eq!(st, &back);
        }
    }
}
