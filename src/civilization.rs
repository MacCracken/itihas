//! Major civilizations with geographic extent, peak period, and key traits.
//!
//! Provides [`Civilization`] structs, pre-built data for 52 major world
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
        if self.end_year == i32::MAX {
            write!(
                f,
                "{} ({}, {} – present)",
                self.name, self.region, self.founding_year
            )
        } else {
            write!(
                f,
                "{} ({}, {} – {})",
                self.name, self.region, self.founding_year, self.end_year
            )
        }
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
        // ── Sub-Saharan Africa ──────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Kingdom of Kush"),
            region: Cow::Borrowed("East Africa"),
            peak_era: Cow::Borrowed("Iron Age"),
            founding_year: -1070,
            end_year: 350,
            traits: vec![
                Cow::Borrowed("iron smelting"),
                Cow::Borrowed("pyramid construction"),
                Cow::Borrowed("archery"),
                Cow::Borrowed("gold trade"),
            ],
            script: Cow::Borrowed("Meroitic script"),
            language_codes: vec![],
        },
        Civilization {
            name: Cow::Borrowed("Aksumite Empire"),
            region: Cow::Borrowed("East Africa"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: 100,
            end_year: 940,
            traits: vec![
                Cow::Borrowed("monumental stelae"),
                Cow::Borrowed("coinage"),
                Cow::Borrowed("Red Sea trade"),
                Cow::Borrowed("early Christianity"),
            ],
            script: Cow::Borrowed("Ge'ez script"),
            language_codes: vec![Cow::Borrowed("gez")],
        },
        Civilization {
            name: Cow::Borrowed("Ghana Empire"),
            region: Cow::Borrowed("West Africa"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 300,
            end_year: 1200,
            traits: vec![
                Cow::Borrowed("gold trade"),
                Cow::Borrowed("trans-Saharan commerce"),
                Cow::Borrowed("ironworking"),
                Cow::Borrowed("centralized governance"),
            ],
            script: Cow::Borrowed("Arabic script"),
            language_codes: vec![Cow::Borrowed("snk")],
        },
        Civilization {
            name: Cow::Borrowed("Mali Empire"),
            region: Cow::Borrowed("West Africa"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1235,
            end_year: 1600,
            traits: vec![
                Cow::Borrowed("Islamic scholarship"),
                Cow::Borrowed("Timbuktu universities"),
                Cow::Borrowed("gold trade"),
                Cow::Borrowed("oral tradition"),
            ],
            script: Cow::Borrowed("N'ko script"),
            language_codes: vec![Cow::Borrowed("bm")],
        },
        Civilization {
            name: Cow::Borrowed("Songhai Empire"),
            region: Cow::Borrowed("West Africa"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1430,
            end_year: 1591,
            traits: vec![
                Cow::Borrowed("riverine trade"),
                Cow::Borrowed("Islamic jurisprudence"),
                Cow::Borrowed("bureaucratic administration"),
            ],
            script: Cow::Borrowed("Arabic script"),
            language_codes: vec![Cow::Borrowed("son")],
        },
        Civilization {
            name: Cow::Borrowed("Great Zimbabwe"),
            region: Cow::Borrowed("Southern Africa"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1100,
            end_year: 1450,
            traits: vec![
                Cow::Borrowed("stone architecture"),
                Cow::Borrowed("cattle herding"),
                Cow::Borrowed("gold mining"),
                Cow::Borrowed("Indian Ocean trade"),
            ],
            script: Cow::Borrowed("oral tradition"),
            language_codes: vec![Cow::Borrowed("sn")],
        },
        Civilization {
            name: Cow::Borrowed("Kingdom of Kongo"),
            region: Cow::Borrowed("Central Africa"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1390,
            end_year: 1914,
            traits: vec![
                Cow::Borrowed("textile production"),
                Cow::Borrowed("metalworking"),
                Cow::Borrowed("diplomacy"),
                Cow::Borrowed("agriculture"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("kg")],
        },
        Civilization {
            name: Cow::Borrowed("Ethiopian Empire"),
            region: Cow::Borrowed("East Africa"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1270,
            end_year: 1974,
            traits: vec![
                Cow::Borrowed("rock-hewn churches"),
                Cow::Borrowed("Solomonic dynasty"),
                Cow::Borrowed("coffee cultivation"),
                Cow::Borrowed("manuscript tradition"),
            ],
            script: Cow::Borrowed("Ge'ez script"),
            language_codes: vec![Cow::Borrowed("am")],
        },
        // ── Southeast Asia ──────────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Khmer Empire"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 802,
            end_year: 1431,
            traits: vec![
                Cow::Borrowed("temple architecture"),
                Cow::Borrowed("hydraulic engineering"),
                Cow::Borrowed("rice agriculture"),
                Cow::Borrowed("Hindu-Buddhist syncretism"),
            ],
            script: Cow::Borrowed("Khmer script"),
            language_codes: vec![Cow::Borrowed("km")],
        },
        Civilization {
            name: Cow::Borrowed("Srivijaya"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 650,
            end_year: 1377,
            traits: vec![
                Cow::Borrowed("maritime trade"),
                Cow::Borrowed("Buddhist scholarship"),
                Cow::Borrowed("thalassocracy"),
            ],
            script: Cow::Borrowed("Pallava script"),
            language_codes: vec![Cow::Borrowed("ms")],
        },
        Civilization {
            name: Cow::Borrowed("Majapahit"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1293,
            end_year: 1527,
            traits: vec![
                Cow::Borrowed("maritime empire"),
                Cow::Borrowed("literature"),
                Cow::Borrowed("Hindu-Buddhist culture"),
                Cow::Borrowed("spice trade"),
            ],
            script: Cow::Borrowed("Kawi script"),
            language_codes: vec![Cow::Borrowed("jv")],
        },
        Civilization {
            name: Cow::Borrowed("Pagan Kingdom"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 849,
            end_year: 1297,
            traits: vec![
                Cow::Borrowed("pagoda construction"),
                Cow::Borrowed("Theravada Buddhism"),
                Cow::Borrowed("irrigation systems"),
            ],
            script: Cow::Borrowed("Burmese script"),
            language_codes: vec![Cow::Borrowed("my")],
        },
        Civilization {
            name: Cow::Borrowed("Dai Viet"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 968,
            end_year: 1802,
            traits: vec![
                Cow::Borrowed("rice cultivation"),
                Cow::Borrowed("Confucian bureaucracy"),
                Cow::Borrowed("military resistance"),
                Cow::Borrowed("poetry"),
            ],
            script: Cow::Borrowed("Chu Nom script"),
            language_codes: vec![Cow::Borrowed("vi")],
        },
        Civilization {
            name: Cow::Borrowed("Ayutthaya Kingdom"),
            region: Cow::Borrowed("Southeast Asia"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1351,
            end_year: 1767,
            traits: vec![
                Cow::Borrowed("diplomacy"),
                Cow::Borrowed("international trade"),
                Cow::Borrowed("Buddhist art"),
                Cow::Borrowed("hydraulic management"),
            ],
            script: Cow::Borrowed("Thai script"),
            language_codes: vec![Cow::Borrowed("th")],
        },
        // ── Pacific Islands ─────────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Tonga Empire"),
            region: Cow::Borrowed("Oceania"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 950,
            end_year: 1865,
            traits: vec![
                Cow::Borrowed("ocean navigation"),
                Cow::Borrowed("tapa cloth"),
                Cow::Borrowed("chieftain hierarchy"),
            ],
            script: Cow::Borrowed("oral tradition"),
            language_codes: vec![Cow::Borrowed("to")],
        },
        Civilization {
            name: Cow::Borrowed("Hawaiian Kingdom"),
            region: Cow::Borrowed("Oceania"),
            peak_era: Cow::Borrowed("Industrial Age"),
            founding_year: 1795,
            end_year: 1893,
            traits: vec![
                Cow::Borrowed("ahupuaa land management"),
                Cow::Borrowed("wayfinding navigation"),
                Cow::Borrowed("featherwork"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("haw")],
        },
        // ── Pre-Columbian Americas ──────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Olmec"),
            region: Cow::Borrowed("Mesoamerica"),
            peak_era: Cow::Borrowed("Mesoamerican Preclassic"),
            founding_year: -1500,
            end_year: -400,
            traits: vec![
                Cow::Borrowed("colossal stone heads"),
                Cow::Borrowed("ceremonial centers"),
                Cow::Borrowed("rubber processing"),
                Cow::Borrowed("jaguar worship"),
            ],
            script: Cow::Borrowed("Olmec script"),
            language_codes: vec![],
        },
        Civilization {
            name: Cow::Borrowed("Aztec Empire"),
            region: Cow::Borrowed("Mesoamerica"),
            peak_era: Cow::Borrowed("Mesoamerican Postclassic"),
            founding_year: 1428,
            end_year: 1521,
            traits: vec![
                Cow::Borrowed("chinampas agriculture"),
                Cow::Borrowed("tribute system"),
                Cow::Borrowed("monumental architecture"),
                Cow::Borrowed("calendar systems"),
            ],
            script: Cow::Borrowed("Aztec pictographs"),
            language_codes: vec![Cow::Borrowed("nah")],
        },
        Civilization {
            name: Cow::Borrowed("Inca Empire"),
            region: Cow::Borrowed("South America"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1438,
            end_year: 1533,
            traits: vec![
                Cow::Borrowed("road network"),
                Cow::Borrowed("terrace farming"),
                Cow::Borrowed("quipu record-keeping"),
                Cow::Borrowed("masonry"),
            ],
            script: Cow::Borrowed("Quipu"),
            language_codes: vec![Cow::Borrowed("qu")],
        },
        Civilization {
            name: Cow::Borrowed("Mississippian Culture"),
            region: Cow::Borrowed("North America"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 800,
            end_year: 1600,
            traits: vec![
                Cow::Borrowed("mound building"),
                Cow::Borrowed("maize agriculture"),
                Cow::Borrowed("chiefdom organization"),
                Cow::Borrowed("shell trade"),
            ],
            script: Cow::Borrowed("oral tradition"),
            language_codes: vec![],
        },
        Civilization {
            name: Cow::Borrowed("Muisca Confederation"),
            region: Cow::Borrowed("South America"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 600,
            end_year: 1541,
            traits: vec![
                Cow::Borrowed("goldworking"),
                Cow::Borrowed("emerald mining"),
                Cow::Borrowed("salt trade"),
            ],
            script: Cow::Borrowed("oral tradition"),
            language_codes: vec![],
        },
        Civilization {
            name: Cow::Borrowed("Tiwanaku"),
            region: Cow::Borrowed("South America"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 550,
            end_year: 1000,
            traits: vec![
                Cow::Borrowed("raised-field agriculture"),
                Cow::Borrowed("monolithic architecture"),
                Cow::Borrowed("high-altitude adaptation"),
            ],
            script: Cow::Borrowed("oral tradition"),
            language_codes: vec![Cow::Borrowed("ay")],
        },
        // ── Europe ──────────────────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Byzantine Empire"),
            region: Cow::Borrowed("Mediterranean, Europe"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 330,
            end_year: 1453,
            traits: vec![
                Cow::Borrowed("Roman law codification"),
                Cow::Borrowed("Orthodox Christianity"),
                Cow::Borrowed("mosaic art"),
                Cow::Borrowed("Greek fire"),
            ],
            script: Cow::Borrowed("Greek alphabet"),
            language_codes: vec![Cow::Borrowed("grc")],
        },
        Civilization {
            name: Cow::Borrowed("Viking/Norse"),
            region: Cow::Borrowed("Northern Europe"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 793,
            end_year: 1066,
            traits: vec![
                Cow::Borrowed("longship navigation"),
                Cow::Borrowed("Norse mythology"),
                Cow::Borrowed("exploration"),
                Cow::Borrowed("saga literature"),
            ],
            script: Cow::Borrowed("Runic/Futhark"),
            language_codes: vec![Cow::Borrowed("non")],
        },
        Civilization {
            name: Cow::Borrowed("Carolingian Empire"),
            region: Cow::Borrowed("Europe"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 751,
            end_year: 888,
            traits: vec![
                Cow::Borrowed("Carolingian Renaissance"),
                Cow::Borrowed("feudalism"),
                Cow::Borrowed("manuscript production"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("la")],
        },
        Civilization {
            name: Cow::Borrowed("Kingdom of France"),
            region: Cow::Borrowed("Europe"),
            peak_era: Cow::Borrowed("Age of Enlightenment"),
            founding_year: 843,
            end_year: 1792,
            traits: vec![
                Cow::Borrowed("Gothic architecture"),
                Cow::Borrowed("Enlightenment philosophy"),
                Cow::Borrowed("courtly culture"),
                Cow::Borrowed("centralized monarchy"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("fr")],
        },
        Civilization {
            name: Cow::Borrowed("Holy Roman Empire"),
            region: Cow::Borrowed("Central Europe"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 800,
            end_year: 1806,
            traits: vec![
                Cow::Borrowed("imperial governance"),
                Cow::Borrowed("printing press"),
                Cow::Borrowed("Hanseatic trade"),
                Cow::Borrowed("Reformation"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("de")],
        },
        Civilization {
            name: Cow::Borrowed("Republic of Venice"),
            region: Cow::Borrowed("Mediterranean"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 697,
            end_year: 1797,
            traits: vec![
                Cow::Borrowed("maritime trade"),
                Cow::Borrowed("glassmaking"),
                Cow::Borrowed("republican governance"),
                Cow::Borrowed("Renaissance art"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("vec")],
        },
        Civilization {
            name: Cow::Borrowed("Spanish Empire"),
            region: Cow::Borrowed("Europe, Americas"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1469,
            end_year: 1975,
            traits: vec![
                Cow::Borrowed("colonial expansion"),
                Cow::Borrowed("silver trade"),
                Cow::Borrowed("Reconquista"),
                Cow::Borrowed("Catholic missions"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("es")],
        },
        Civilization {
            name: Cow::Borrowed("Portuguese Empire"),
            region: Cow::Borrowed("Europe, Africa, Asia"),
            peak_era: Cow::Borrowed("Renaissance"),
            founding_year: 1415,
            end_year: 1999,
            traits: vec![
                Cow::Borrowed("maritime exploration"),
                Cow::Borrowed("cartography"),
                Cow::Borrowed("spice trade"),
                Cow::Borrowed("colonial administration"),
            ],
            script: Cow::Borrowed("Latin alphabet"),
            language_codes: vec![Cow::Borrowed("pt")],
        },
        // ── Near East / Central Asia ────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Hittite Empire"),
            region: Cow::Borrowed("Near East"),
            peak_era: Cow::Borrowed("Bronze Age"),
            founding_year: -1600,
            end_year: -1178,
            traits: vec![
                Cow::Borrowed("iron metallurgy"),
                Cow::Borrowed("chariot warfare"),
                Cow::Borrowed("treaty diplomacy"),
                Cow::Borrowed("bilingual governance"),
            ],
            script: Cow::Borrowed("Cuneiform/Hieroglyphic Luwian"),
            language_codes: vec![Cow::Borrowed("hit")],
        },
        Civilization {
            name: Cow::Borrowed("Assyrian Empire"),
            region: Cow::Borrowed("Near East"),
            peak_era: Cow::Borrowed("Iron Age"),
            founding_year: -2500,
            end_year: -609,
            traits: vec![
                Cow::Borrowed("military engineering"),
                Cow::Borrowed("library of Ashurbanipal"),
                Cow::Borrowed("siege warfare"),
                Cow::Borrowed("imperial roads"),
            ],
            script: Cow::Borrowed("Cuneiform"),
            language_codes: vec![Cow::Borrowed("akk")],
        },
        Civilization {
            name: Cow::Borrowed("Phoenicia"),
            region: Cow::Borrowed("Near East, Mediterranean"),
            peak_era: Cow::Borrowed("Iron Age"),
            founding_year: -1500,
            end_year: -300,
            traits: vec![
                Cow::Borrowed("alphabet invention"),
                Cow::Borrowed("maritime trade"),
                Cow::Borrowed("purple dye production"),
                Cow::Borrowed("colonization"),
            ],
            script: Cow::Borrowed("Phoenician alphabet"),
            language_codes: vec![Cow::Borrowed("phn")],
        },
        Civilization {
            name: Cow::Borrowed("Parthian Empire"),
            region: Cow::Borrowed("Near East, Central Asia"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: -247,
            end_year: 224,
            traits: vec![
                Cow::Borrowed("horse archery"),
                Cow::Borrowed("Silk Road trade"),
                Cow::Borrowed("feudal governance"),
                Cow::Borrowed("Hellenistic synthesis"),
            ],
            script: Cow::Borrowed("Aramaic script"),
            language_codes: vec![Cow::Borrowed("xpr")],
        },
        Civilization {
            name: Cow::Borrowed("Sassanid Empire"),
            region: Cow::Borrowed("Near East"),
            peak_era: Cow::Borrowed("Classical Antiquity"),
            founding_year: 224,
            end_year: 651,
            traits: vec![
                Cow::Borrowed("Zoroastrian state religion"),
                Cow::Borrowed("academy of Gondishapur"),
                Cow::Borrowed("rock reliefs"),
                Cow::Borrowed("heavy cavalry"),
            ],
            script: Cow::Borrowed("Pahlavi script"),
            language_codes: vec![Cow::Borrowed("pal")],
        },
        Civilization {
            name: Cow::Borrowed("Timurid Empire"),
            region: Cow::Borrowed("Central Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1370,
            end_year: 1507,
            traits: vec![
                Cow::Borrowed("Timurid Renaissance"),
                Cow::Borrowed("astronomical observatories"),
                Cow::Borrowed("miniature painting"),
                Cow::Borrowed("Persian literature"),
            ],
            script: Cow::Borrowed("Persian/Arabic script"),
            language_codes: vec![Cow::Borrowed("fa")],
        },
        // ── South Asia ──────────────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Chola Dynasty"),
            region: Cow::Borrowed("South Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 300,
            end_year: 1279,
            traits: vec![
                Cow::Borrowed("naval power"),
                Cow::Borrowed("bronze sculpture"),
                Cow::Borrowed("temple architecture"),
                Cow::Borrowed("maritime trade"),
            ],
            script: Cow::Borrowed("Tamil script"),
            language_codes: vec![Cow::Borrowed("ta")],
        },
        Civilization {
            name: Cow::Borrowed("Vijayanagara Empire"),
            region: Cow::Borrowed("South Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 1336,
            end_year: 1646,
            traits: vec![
                Cow::Borrowed("Hindu patronage"),
                Cow::Borrowed("Hampi architecture"),
                Cow::Borrowed("diamond trade"),
                Cow::Borrowed("irrigation systems"),
            ],
            script: Cow::Borrowed("Kannada script"),
            language_codes: vec![Cow::Borrowed("kn")],
        },
        Civilization {
            name: Cow::Borrowed("Maratha Empire"),
            region: Cow::Borrowed("South Asia"),
            peak_era: Cow::Borrowed("Age of Enlightenment"),
            founding_year: 1674,
            end_year: 1818,
            traits: vec![
                Cow::Borrowed("guerrilla warfare"),
                Cow::Borrowed("naval innovation"),
                Cow::Borrowed("decentralized governance"),
            ],
            script: Cow::Borrowed("Modi script"),
            language_codes: vec![Cow::Borrowed("mr")],
        },
        Civilization {
            name: Cow::Borrowed("Sikh Empire"),
            region: Cow::Borrowed("South Asia"),
            peak_era: Cow::Borrowed("Industrial Age"),
            founding_year: 1799,
            end_year: 1849,
            traits: vec![
                Cow::Borrowed("military modernization"),
                Cow::Borrowed("religious pluralism"),
                Cow::Borrowed("Khalsa governance"),
            ],
            script: Cow::Borrowed("Gurmukhi script"),
            language_codes: vec![Cow::Borrowed("pa")],
        },
        // ── East Asia ───────────────────────────────────────────────────
        Civilization {
            name: Cow::Borrowed("Goryeo Dynasty"),
            region: Cow::Borrowed("East Asia"),
            peak_era: Cow::Borrowed("Middle Ages"),
            founding_year: 918,
            end_year: 1392,
            traits: vec![
                Cow::Borrowed("celadon pottery"),
                Cow::Borrowed("movable type printing"),
                Cow::Borrowed("Buddhist scholarship"),
                Cow::Borrowed("civil service examinations"),
            ],
            script: Cow::Borrowed("Hanja/Korean"),
            language_codes: vec![Cow::Borrowed("ko")],
        },
        Civilization {
            name: Cow::Borrowed("Tokugawa Shogunate"),
            region: Cow::Borrowed("East Asia"),
            peak_era: Cow::Borrowed("Age of Enlightenment"),
            founding_year: 1603,
            end_year: 1868,
            traits: vec![
                Cow::Borrowed("sakoku isolationism"),
                Cow::Borrowed("ukiyo-e art"),
                Cow::Borrowed("bushido culture"),
                Cow::Borrowed("urbanization"),
            ],
            script: Cow::Borrowed("Japanese script"),
            language_codes: vec![Cow::Borrowed("ja")],
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
        assert_eq!(all_civilizations().len(), 52);
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
