//! Structured historical events.
//!
//! Provides [`Event`] structs representing major world events, an
//! [`EventCategory`] classification enum, and 105 pre-built events spanning
//! from the invention of writing to the digital revolution.
//!
//! # Sources
//!
//! Dates and classifications: Stearns (2001), Grun (2005). Ancient: Bauer
//! (2007), Bickerman (1980). Medieval: Wickham (2009), Abulafia (2011).
//! Modern: Hobsbawm (1962--1994). Full bibliography:
//! [`docs/sources/events.md`](https://github.com/MacCracken/itihas/blob/main/docs/sources/events.md).

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

impl fmt::Display for EventCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::War => f.write_str("War"),
            Self::Treaty => f.write_str("Treaty"),
            Self::Discovery => f.write_str("Discovery"),
            Self::Invention => f.write_str("Invention"),
            Self::Revolution => f.write_str("Revolution"),
            Self::Migration => f.write_str("Migration"),
            Self::Founding => f.write_str("Founding"),
            Self::Collapse => f.write_str("Collapse"),
        }
    }
}

/// Geographic scope of an event's impact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EventSignificance {
    /// Affects a single city, state, or small territory.
    Local,
    /// Affects a multi-state region or large territory.
    Regional,
    /// Affects an entire continent or major civilizational sphere.
    Continental,
    /// Reshapes the trajectory of world history.
    Global,
}

impl fmt::Display for EventSignificance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => f.write_str("Local"),
            Self::Regional => f.write_str("Regional"),
            Self::Continental => f.write_str("Continental"),
            Self::Global => f.write_str("Global"),
        }
    }
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
    /// Geographic scope of impact.
    pub significance: EventSignificance,
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
            significance: EventSignificance::Global,
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
            significance: EventSignificance::Regional,
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
            significance: EventSignificance::Global,
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
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of Rome"),
            year: -753,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed("Traditional date of the founding of Rome by Romulus"),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Local,
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
            significance: EventSignificance::Continental,
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
            significance: EventSignificance::Global,
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
            significance: EventSignificance::Continental,
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
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Rise of Islam"),
            year: 622,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Muhammad's Hijra to Medina, founding the Islamic community",
            ),
            civilizations_involved: vec![Cow::Borrowed("Arab Caliphates")],
            significance: EventSignificance::Global,
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
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Fall of Constantinople"),
            year: 1453,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Ottoman conquest of Constantinople, ending the Byzantine Empire",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ottoman Empire"),
                Cow::Borrowed("Byzantine Empire"),
            ],
            significance: EventSignificance::Continental,
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
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("French Revolution"),
            year: 1789,
            era: Cow::Borrowed("Age of Enlightenment"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Overthrow of the monarchy, establishing principles of liberty and equality",
            ),
            civilizations_involved: vec![Cow::Borrowed("Holy Roman Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Invention of the World Wide Web"),
            year: 1989,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed("Tim Berners-Lee proposes the World Wide Web at CERN"),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        // ── Bronze Age (8 events) ──────────────────────────────────────
        Event {
            name: Cow::Borrowed("Battle of Kadesh"),
            year: -1274,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Major chariot battle between Egypt and the Hittites in modern Syria",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Hittite Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Trojan War"),
            year: -1180,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Legendary conflict between Mycenaean Greeks and the city of Troy",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient Greece")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Unification of Upper and Lower Egypt"),
            year: -3100,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Narmer unifies Upper and Lower Egypt, founding the First Dynasty",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient Egypt")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Egyptian-Hittite Peace Treaty"),
            year: -1259,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Earliest known international peace treaty, between Ramesses II and Hattusili III",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Hittite Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of the Shang Dynasty"),
            year: -1600,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Earliest archaeologically attested Chinese dynasty, with oracle bone writing",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient China")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Invention of the Wheel"),
            year: -3500,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Development of the potter's wheel and wheeled vehicles in Mesopotamia",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mesopotamia")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Migration of the Sea Peoples"),
            year: -1200,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Migration,
            description: Cow::Borrowed(
                "Mass migration of maritime raiders destabilizing eastern Mediterranean civilizations",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Hittite Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Decline of the Indus Valley Civilization"),
            year: -1900,
            era: Cow::Borrowed("Bronze Age"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Gradual abandonment of major Indus cities such as Mohenjo-daro and Harappa",
            ),
            civilizations_involved: vec![Cow::Borrowed("Indus Valley")],
            significance: EventSignificance::Regional,
        },
        // ── Iron Age (7 events) ────────────────────────────────────────
        Event {
            name: Cow::Borrowed("Founding of Carthage"),
            year: -814,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed("Phoenician colonists establish Carthage in North Africa"),
            civilizations_involved: vec![Cow::Borrowed("Phoenicia")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("First Olympic Games"),
            year: -776,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Traditional date of the first Olympic Games at Olympia, Greece",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient Greece")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Birth of Buddhism"),
            year: -528,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Siddhartha Gautama attains enlightenment, founding one of the world's great religions",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Assyrian Conquest of Egypt"),
            year: -671,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Esarhaddon's Assyrian army conquers Memphis and controls Egypt",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Assyrian Empire"),
                Cow::Borrowed("Ancient Egypt"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Invention of Iron Smelting"),
            year: -1100,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Widespread adoption of iron smelting, replacing bronze tools and weapons",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Rise of the Kingdom of Kush"),
            year: -1070,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Kush gains independence from Egypt and becomes a major Nubian power",
            ),
            civilizations_involved: vec![Cow::Borrowed("Kingdom of Kush")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Phoenician Alphabet Spreads"),
            year: -1050,
            era: Cow::Borrowed("Iron Age"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Phoenician consonantal alphabet adopted by Greeks and other Mediterranean cultures",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Phoenicia"),
                Cow::Borrowed("Ancient Greece"),
            ],
            significance: EventSignificance::Global,
        },
        // ── Classical Antiquity (15 events) ────────────────────────────
        Event {
            name: Cow::Borrowed("Punic Wars"),
            year: -264,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Series of wars between Rome and Carthage for Mediterranean supremacy",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire"), Cow::Borrowed("Carthage")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Assassination of Julius Caesar"),
            year: -44,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Roman senators assassinate Caesar, triggering civil wars and the end of the Republic",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Crucifixion of Jesus"),
            year: 30,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Execution of Jesus of Nazareth, catalyzing the spread of Christianity",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Eruption of Vesuvius"),
            year: 79,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Volcanic eruption buries Pompeii and Herculaneum under ash",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Local,
        },
        Event {
            name: Cow::Borrowed("Construction of the Great Wall"),
            year: -221,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Qin Shi Huang links existing walls into a unified defensive barrier",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient China")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Invention of Paper"),
            year: 105,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Cai Lun improves papermaking in Han China, revolutionizing writing",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient China")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Edict of Milan"),
            year: 313,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Constantine and Licinius legalize Christianity across the Roman Empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Sack of Rome by the Visigoths"),
            year: 410,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Alaric I leads the Visigoths in sacking Rome, shocking the Roman world",
            ),
            civilizations_involved: vec![Cow::Borrowed("Roman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Persian Royal Road"),
            year: -500,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Darius I builds an extensive road and postal system spanning the Persian Empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Persian Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of the Aksumite Empire"),
            year: 100,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Aksum rises as a major trading power in the Horn of Africa",
            ),
            civilizations_involved: vec![Cow::Borrowed("Aksumite Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Battle of Thermopylae"),
            year: -480,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Spartan-led Greek force delays the Persian invasion at a narrow coastal pass",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient Greece"),
                Cow::Borrowed("Persian Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Discovery of the Silk Road"),
            year: -130,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Zhang Qian's missions open trade routes connecting China to Central Asia and beyond",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Ancient China"),
                Cow::Borrowed("Parthian Empire"),
            ],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Olmec Civilization Decline"),
            year: -400,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Decline of the Olmec, the earliest major Mesoamerican civilization",
            ),
            civilizations_involved: vec![Cow::Borrowed("Olmec")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Bantu Migration"),
            year: -500,
            era: Cow::Borrowed("Classical Antiquity"),
            category: EventCategory::Migration,
            description: Cow::Borrowed(
                "Bantu-speaking peoples begin large-scale migration across sub-Saharan Africa",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        // ── Middle Ages (20 events) ────────────────────────────────────
        Event {
            name: Cow::Borrowed("Construction of Angkor Wat"),
            year: 1113,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Suryavarman II builds the largest religious monument in the world",
            ),
            civilizations_involved: vec![Cow::Borrowed("Khmer Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Magna Carta"),
            year: 1215,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "English barons force King John to accept limits on royal power",
            ),
            civilizations_involved: vec![Cow::Borrowed("Kingdom of England")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Black Death Reaches Europe"),
            year: 1347,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Bubonic plague pandemic kills an estimated third of Europe's population",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Viking Raids Begin"),
            year: 793,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Norse raiders attack Lindisfarne, marking the start of the Viking Age",
            ),
            civilizations_involved: vec![Cow::Borrowed("Viking/Norse")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Coronation of Charlemagne"),
            year: 800,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Pope Leo III crowns Charlemagne Emperor, reviving the concept of a Western Empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Carolingian Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of the Ghana Empire"),
            year: 830,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Ghana Empire becomes the first major West African trading state",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ghana Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("First Crusade"),
            year: 1096,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "European Christians capture Jerusalem and establish Crusader states",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Byzantine Empire"),
                Cow::Borrowed("Holy Roman Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of the Mali Empire"),
            year: 1235,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Sundiata Keita defeats Sumanguru and establishes the Mali Empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mali Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Invention of Gunpowder Weapons"),
            year: 900,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Chinese alchemists develop gunpowder, later adapted for military use",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient China")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Great Zimbabwe Constructed"),
            year: 1100,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Stone city of Great Zimbabwe becomes center of a powerful trading kingdom",
            ),
            civilizations_involved: vec![Cow::Borrowed("Great Zimbabwe")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Norman Conquest of England"),
            year: 1066,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "William the Conqueror defeats Harold II at Hastings and takes the English crown",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Kingdom of England"),
                Cow::Borrowed("Normandy"),
            ],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Mansa Musa's Pilgrimage to Mecca"),
            year: 1324,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Migration,
            description: Cow::Borrowed(
                "Mali emperor's lavish hajj destabilizes gold prices across the Mediterranean",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mali Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Founding of Tenochtitlan"),
            year: 1325,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "The Mexica found their capital city on an island in Lake Texcoco",
            ),
            civilizations_involved: vec![Cow::Borrowed("Aztec Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Polynesian Settlement of New Zealand"),
            year: 1280,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Migration,
            description: Cow::Borrowed(
                "Polynesian navigators reach and settle Aotearoa, becoming the Maori",
            ),
            civilizations_involved: vec![Cow::Borrowed("Polynesia")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Srivijaya Maritime Dominance"),
            year: 683,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Srivijaya controls the Strait of Malacca, dominating Southeast Asian trade",
            ),
            civilizations_involved: vec![Cow::Borrowed("Srivijaya")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Invention of the Compass"),
            year: 1040,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Chinese navigators develop the magnetic compass for maritime use",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ancient China")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Mongol Sack of Baghdad"),
            year: 1258,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Mongol siege destroys the Abbasid Caliphate's capital, ending the Islamic Golden Age",
            ),
            civilizations_involved: vec![Cow::Borrowed("Mongol Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Founding of the Ottoman Empire"),
            year: 1299,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Osman I establishes a small Anatolian beylik that grows into a world empire",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ottoman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Chola Naval Expeditions"),
            year: 1025,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Rajendra Chola I launches naval raids against Srivijaya, projecting Indian power into Southeast Asia",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Chola Dynasty"),
                Cow::Borrowed("Srivijaya"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Ethiopian Zagwe Dynasty"),
            year: 1137,
            era: Cow::Borrowed("Middle Ages"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Zagwe dynasty comes to power and later builds the rock-hewn churches of Lalibela",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ethiopian Empire")],
            significance: EventSignificance::Local,
        },
        // ── Renaissance (5 events) ─────────────────────────────────────
        Event {
            name: Cow::Borrowed("Spanish Inquisition"),
            year: 1478,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Ferdinand and Isabella establish the Inquisition to enforce Catholic orthodoxy",
            ),
            civilizations_involved: vec![Cow::Borrowed("Spanish Empire")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Columbus Reaches the Americas"),
            year: 1492,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Christopher Columbus lands in the Caribbean, initiating sustained European contact with the Americas",
            ),
            civilizations_involved: vec![Cow::Borrowed("Spanish Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Vasco da Gama Reaches India"),
            year: 1498,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Portuguese navigator reaches Calicut, opening a direct sea route to Asia",
            ),
            civilizations_involved: vec![Cow::Borrowed("Portuguese Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Protestant Reformation"),
            year: 1517,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Martin Luther's Ninety-Five Theses spark religious upheaval across Europe",
            ),
            civilizations_involved: vec![Cow::Borrowed("Holy Roman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Fall of the Aztec Empire"),
            year: 1521,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Hernan Cortes and indigenous allies conquer Tenochtitlan, ending the Aztec Empire",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Aztec Empire"),
                Cow::Borrowed("Spanish Empire"),
            ],
            significance: EventSignificance::Continental,
        },
        // ── Age of Enlightenment (5 events) ────────────────────────────
        Event {
            name: Cow::Borrowed("Treaty of Westphalia"),
            year: 1648,
            era: Cow::Borrowed("Age of Enlightenment"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Ends the Thirty Years' War and establishes the principle of state sovereignty",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("Holy Roman Empire"),
                Cow::Borrowed("Kingdom of France"),
            ],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("American Revolution"),
            year: 1776,
            era: Cow::Borrowed("Age of Enlightenment"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Thirteen colonies declare independence from Britain, creating the United States",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Newton's Principia Published"),
            year: 1687,
            era: Cow::Borrowed("Age of Enlightenment"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Isaac Newton publishes laws of motion and universal gravitation",
            ),
            civilizations_involved: vec![Cow::Borrowed("Kingdom of England")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Tokugawa Shogunate Established"),
            year: 1603,
            era: Cow::Borrowed("Renaissance"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Tokugawa Ieyasu unifies Japan and begins over two centuries of stability",
            ),
            civilizations_involved: vec![Cow::Borrowed("Tokugawa Shogunate")],
            significance: EventSignificance::Regional,
        },
        Event {
            name: Cow::Borrowed("Founding of the Maratha Empire"),
            year: 1674,
            era: Cow::Borrowed("Age of Enlightenment"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "Shivaji Maharaj crowned Chhatrapati, establishing a major Indian power",
            ),
            civilizations_involved: vec![Cow::Borrowed("Maratha Empire")],
            significance: EventSignificance::Regional,
        },
        // ── Industrial Age (20 events) ─────────────────────────────────
        Event {
            name: Cow::Borrowed("Abolition of the Atlantic Slave Trade"),
            year: 1807,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "British Parliament passes the Slave Trade Act, banning the transatlantic slave trade",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Opening of the Suez Canal"),
            year: 1869,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Artificial waterway connects the Mediterranean and Red Sea, transforming global trade",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("World War I"),
            year: 1914,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Global conflict between the Allied and Central Powers, reshaping the political order",
            ),
            civilizations_involved: vec![
                Cow::Borrowed("British Empire"),
                Cow::Borrowed("Ottoman Empire"),
            ],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("World War II"),
            year: 1939,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Deadliest conflict in human history, ending with the atomic bomb and the UN's founding",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Moon Landing"),
            year: 1969,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed("Apollo 11 lands the first humans on the Moon"),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Haitian Revolution"),
            year: 1791,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Enslaved people in Saint-Domingue revolt, creating the first free Black republic",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Latin American Wars of Independence"),
            year: 1810,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Simon Bolivar and others lead independence movements across South America",
            ),
            civilizations_involved: vec![Cow::Borrowed("Spanish Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Meiji Restoration"),
            year: 1868,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Japan restores imperial rule and begins rapid industrialization and modernization",
            ),
            civilizations_involved: vec![Cow::Borrowed("Tokugawa Shogunate")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Scramble for Africa"),
            year: 1884,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Berlin Conference partitions Africa among European colonial powers",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Russian Revolution"),
            year: 1917,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Bolsheviks overthrow the Tsar, establishing the Soviet Union",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Indian Independence"),
            year: 1947,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "India gains independence from Britain, partitioning into India and Pakistan",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Chinese Revolution"),
            year: 1949,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Mao Zedong proclaims the People's Republic of China after civil war",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Treaty of Versailles"),
            year: 1919,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Peace treaty ending World War I, imposing terms on Germany and redrawing borders",
            ),
            civilizations_involved: vec![Cow::Borrowed("British Empire")],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Invention of the Telegraph"),
            year: 1837,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Samuel Morse develops the electric telegraph, enabling near-instant long-distance communication",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Invention of the Telephone"),
            year: 1876,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Alexander Graham Bell patents the telephone, transforming human communication",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Panama Canal Opens"),
            year: 1914,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Artificial waterway across Panama connects the Atlantic and Pacific oceans",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Founding of the United Nations"),
            year: 1945,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Founding,
            description: Cow::Borrowed(
                "International organization established to maintain peace and promote cooperation",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("African Decolonization Wave"),
            year: 1960,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Seventeen African nations gain independence in the 'Year of Africa'",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Collapse of the Ottoman Empire"),
            year: 1922,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "The Ottoman sultanate is abolished, ending over six centuries of rule",
            ),
            civilizations_involved: vec![Cow::Borrowed("Ottoman Empire")],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Discovery of Penicillin"),
            year: 1928,
            era: Cow::Borrowed("Industrial Age"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "Alexander Fleming discovers penicillin, launching the antibiotic era",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        // ── Information Age (10 events) ────────────────────────────────
        Event {
            name: Cow::Borrowed("Fall of the Berlin Wall"),
            year: 1989,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Opening of the Berlin Wall symbolizes the end of the Cold War division of Europe",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("September 11 Attacks"),
            year: 2001,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Terrorist attacks on the United States reshape global security and foreign policy",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Dissolution of the Soviet Union"),
            year: 1991,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "The USSR dissolves into fifteen independent states, ending the Cold War",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Apartheid Ends in South Africa"),
            year: 1994,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Nelson Mandela elected president in South Africa's first multiracial elections",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Human Genome Project Completed"),
            year: 2003,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Discovery,
            description: Cow::Borrowed(
                "First complete sequencing of the human genome, transforming biology and medicine",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Arab Spring"),
            year: 2011,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Revolution,
            description: Cow::Borrowed(
                "Wave of pro-democracy protests and uprisings across the Arab world",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Paris Climate Agreement"),
            year: 2015,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Nearly 200 nations agree to limit global warming to well below 2 degrees Celsius",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("COVID-19 Pandemic"),
            year: 2020,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Collapse,
            description: Cow::Borrowed(
                "Global pandemic disrupts economies, health systems, and daily life worldwide",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("Invention of the Smartphone"),
            year: 2007,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Invention,
            description: Cow::Borrowed(
                "Apple iPhone launches the modern smartphone era, reshaping communication and commerce",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Global,
        },
        Event {
            name: Cow::Borrowed("European Union Established"),
            year: 1993,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::Treaty,
            description: Cow::Borrowed(
                "Maastricht Treaty creates the European Union, deepening political and economic integration",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
        },
        Event {
            name: Cow::Borrowed("Rwandan Genocide"),
            year: 1994,
            era: Cow::Borrowed("Information Age"),
            category: EventCategory::War,
            description: Cow::Borrowed(
                "Mass killing of Tutsi in Rwanda claims over 800,000 lives in approximately 100 days",
            ),
            civilizations_involved: vec![],
            significance: EventSignificance::Continental,
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

/// Returns events matching the given significance level.
#[must_use]
#[inline]
pub fn by_significance(significance: &EventSignificance) -> Vec<Event> {
    tracing::debug!(?significance, "looking up events by significance");
    all_events()
        .iter()
        .filter(|e| e.significance == *significance)
        .cloned()
        .collect()
}

/// Returns events that occurred between `start` and `end` years (inclusive).
///
/// Results are sorted chronologically.
#[must_use]
#[inline]
pub fn events_between(start: i32, end: i32) -> Vec<Event> {
    tracing::debug!(start, end, "looking up events between years");
    let mut results: Vec<Event> = all_events()
        .iter()
        .filter(|e| e.year >= start && e.year <= end)
        .cloned()
        .collect();
    results.sort();
    results
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
        assert_eq!(all_events().len(), 105);
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
        let events = all_events();
        for event in events.iter() {
            let json = serde_json::to_string(event).unwrap();
            let back: Event = serde_json::from_str(&json).unwrap();
            assert_eq!(event, &back);
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

    #[test]
    fn test_event_significance_serde_roundtrip() {
        let levels = [
            EventSignificance::Local,
            EventSignificance::Regional,
            EventSignificance::Continental,
            EventSignificance::Global,
        ];
        for s in &levels {
            let json = serde_json::to_string(s).unwrap();
            let back: EventSignificance = serde_json::from_str(&json).unwrap();
            assert_eq!(*s, back);
        }
    }

    #[test]
    fn test_by_significance_global() {
        let global = by_significance(&EventSignificance::Global);
        assert!(!global.is_empty());
        assert!(
            global
                .iter()
                .all(|e| e.significance == EventSignificance::Global)
        );
    }

    #[test]
    fn test_events_between() {
        let events = events_between(-500, 500);
        assert!(!events.is_empty());
        for e in &events {
            assert!(e.year >= -500 && e.year <= 500);
        }
        // Should be sorted
        for w in events.windows(2) {
            assert!(w[0].year <= w[1].year);
        }
    }

    #[test]
    fn test_events_between_empty() {
        let events = events_between(-100_000, -99_000);
        assert!(events.is_empty());
    }
}
