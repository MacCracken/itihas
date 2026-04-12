//! Military campaign timelines with battles, commanders, and outcomes.
//!
//! Provides [`Campaign`] structs representing major military campaigns,
//! [`Battle`] for key engagements, [`CampaignOutcome`] classification,
//! and lookup functions by region, commander, civilization, and period.
//!
//! # Sources
//!
//! General: Keegan (1993), Black (2009), Fuller (1954--1956). Alexander:
//! Arrian (Selincourt trans., 1971). Punic Wars: Goldsworthy (2000). Hundred
//! Years' War: Sumption (1990--2023). Napoleonic: Chandler (1966). American
//! Civil War: McPherson (1988). Russo-Japanese: Connaughton (1988). Full
//! bibliography:
//! [`docs/sources/campaigns.md`](https://github.com/MacCracken/itihas/blob/main/docs/sources/campaigns.md).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// Outcome of a military campaign.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CampaignOutcome {
    /// Decisive victory for the initiating side.
    Victory,
    /// Defeat of the initiating side.
    Defeat,
    /// No clear winner; exhaustion or withdrawal.
    Stalemate,
    /// Ended by formal agreement.
    Treaty,
    /// Campaign ongoing or outcome ambiguous.
    Inconclusive,
}

impl fmt::Display for CampaignOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Victory => f.write_str("Victory"),
            Self::Defeat => f.write_str("Defeat"),
            Self::Stalemate => f.write_str("Stalemate"),
            Self::Treaty => f.write_str("Treaty"),
            Self::Inconclusive => f.write_str("Inconclusive"),
        }
    }
}

/// A key battle within a campaign.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Battle {
    /// Name of the battle.
    pub name: Cow<'static, str>,
    /// Year the battle occurred (negative = BCE).
    pub year: i32,
    /// Location of the battle.
    pub location: Cow<'static, str>,
    /// Brief description and outcome.
    pub description: Cow<'static, str>,
}

impl fmt::Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.year)
    }
}

/// A military campaign or war.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Campaign {
    /// Name of the campaign or war.
    pub name: Cow<'static, str>,
    /// Primary geographic region.
    pub region: Cow<'static, str>,
    /// Start year (negative = BCE).
    pub start_year: i32,
    /// End year (negative = BCE).
    pub end_year: i32,
    /// Civilizations or states on the initiating/attacking side.
    pub belligerents_a: Vec<Cow<'static, str>>,
    /// Civilizations or states on the defending/opposing side.
    pub belligerents_b: Vec<Cow<'static, str>>,
    /// Notable commanders (both sides).
    pub commanders: Vec<Cow<'static, str>>,
    /// Key battles in chronological order.
    pub battles: Vec<Battle>,
    /// Campaign outcome.
    pub outcome: CampaignOutcome,
    /// Brief description.
    pub description: Cow<'static, str>,
}

impl fmt::Display for Campaign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} – {}, {})",
            self.name, self.start_year, self.end_year, self.outcome
        )
    }
}

impl PartialOrd for Campaign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Campaign {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_year
            .cmp(&other.start_year)
            .then_with(|| self.name.cmp(&other.name))
    }
}

fn build_campaigns() -> Vec<Campaign> {
    vec![
        // ── Ancient ──────────────────────────────────────────────────
        Campaign {
            name: Cow::Borrowed("Greco-Persian Wars"),
            region: Cow::Borrowed("Mediterranean"),
            start_year: -499,
            end_year: -449,
            belligerents_a: vec![Cow::Borrowed("Persian Empire")],
            belligerents_b: vec![Cow::Borrowed("Ancient Greece")],
            commanders: vec![
                Cow::Borrowed("Darius I"),
                Cow::Borrowed("Xerxes I"),
                Cow::Borrowed("Themistocles"),
                Cow::Borrowed("Leonidas I"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Marathon"),
                    year: -490,
                    location: Cow::Borrowed("Marathon, Greece"),
                    description: Cow::Borrowed(
                        "Athenian hoplites defeat Persian invasion force on the plain of Marathon",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Thermopylae"),
                    year: -480,
                    location: Cow::Borrowed("Thermopylae, Greece"),
                    description: Cow::Borrowed(
                        "Leonidas and 300 Spartans delay Xerxes' army at the narrow pass",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Salamis"),
                    year: -480,
                    location: Cow::Borrowed("Salamis, Greece"),
                    description: Cow::Borrowed(
                        "Greek fleet destroys Persian navy in the straits; turning point of the war",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Plataea"),
                    year: -479,
                    location: Cow::Borrowed("Plataea, Greece"),
                    description: Cow::Borrowed(
                        "Combined Greek army defeats Persian land forces, ending the invasion",
                    ),
                },
            ],
            outcome: CampaignOutcome::Defeat,
            description: Cow::Borrowed(
                "Persian attempts to conquer Greece repelled; secured Greek independence and Classical golden age",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Campaigns of Alexander the Great"),
            region: Cow::Borrowed("Near East"),
            start_year: -334,
            end_year: -323,
            belligerents_a: vec![Cow::Borrowed("Macedon")],
            belligerents_b: vec![Cow::Borrowed("Persian Empire")],
            commanders: vec![
                Cow::Borrowed("Alexander the Great"),
                Cow::Borrowed("Darius III"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Granicus"),
                    year: -334,
                    location: Cow::Borrowed("Granicus River, Anatolia"),
                    description: Cow::Borrowed(
                        "Alexander's first major victory; opens Anatolia to Macedonian conquest",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Issus"),
                    year: -333,
                    location: Cow::Borrowed("Issus, Cilicia"),
                    description: Cow::Borrowed(
                        "Alexander defeats Darius III; captures the Persian royal family",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Gaugamela"),
                    year: -331,
                    location: Cow::Borrowed("Gaugamela, Mesopotamia"),
                    description: Cow::Borrowed(
                        "Decisive defeat of Darius III; Alexander becomes master of the Persian Empire",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of the Hydaspes"),
                    year: -326,
                    location: Cow::Borrowed("Hydaspes River, Punjab"),
                    description: Cow::Borrowed(
                        "Alexander defeats King Porus; easternmost extent of the campaign",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Macedonian conquest of the Persian Empire; created Hellenistic world from Greece to India",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Second Punic War"),
            region: Cow::Borrowed("Mediterranean"),
            start_year: -218,
            end_year: -201,
            belligerents_a: vec![Cow::Borrowed("Carthage")],
            belligerents_b: vec![Cow::Borrowed("Roman Republic")],
            commanders: vec![
                Cow::Borrowed("Hannibal Barca"),
                Cow::Borrowed("Scipio Africanus"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Cannae"),
                    year: -216,
                    location: Cow::Borrowed("Cannae, Italy"),
                    description: Cow::Borrowed(
                        "Hannibal's double envelopment destroys 8 Roman legions; worst Roman defeat",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Zama"),
                    year: -202,
                    location: Cow::Borrowed("Zama, North Africa"),
                    description: Cow::Borrowed(
                        "Scipio defeats Hannibal in Africa; ends the war and Carthaginian power",
                    ),
                },
            ],
            outcome: CampaignOutcome::Defeat,
            description: Cow::Borrowed(
                "Hannibal's invasion of Italy via the Alps; despite tactical genius, Carthage ultimately lost to Roman strategic depth",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Gallic Wars"),
            region: Cow::Borrowed("Europe"),
            start_year: -58,
            end_year: -50,
            belligerents_a: vec![Cow::Borrowed("Roman Republic")],
            belligerents_b: vec![Cow::Borrowed("Gaul")],
            commanders: vec![
                Cow::Borrowed("Julius Caesar"),
                Cow::Borrowed("Vercingetorix"),
            ],
            battles: vec![Battle {
                name: Cow::Borrowed("Battle of Alesia"),
                year: -52,
                location: Cow::Borrowed("Alesia, Gaul"),
                description: Cow::Borrowed(
                    "Caesar's double circumvallation traps Vercingetorix; Gaul surrenders",
                ),
            }],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Roman conquest of Gaul; extended Roman territory to the Rhine and English Channel",
            ),
        },
        // ── Medieval ─────────────────────────────────────────────────
        Campaign {
            name: Cow::Borrowed("Muslim Conquest of Iberia"),
            region: Cow::Borrowed("Europe"),
            start_year: 711,
            end_year: 718,
            belligerents_a: vec![Cow::Borrowed("Arab Caliphates")],
            belligerents_b: vec![Cow::Borrowed("Visigothic Kingdom")],
            commanders: vec![
                Cow::Borrowed("Tariq ibn Ziyad"),
                Cow::Borrowed("Musa ibn Nusayr"),
                Cow::Borrowed("Roderic"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Guadalete"),
                    year: 711,
                    location: Cow::Borrowed("Guadalete, Iberia"),
                    description: Cow::Borrowed(
                        "Umayyad forces defeat Visigothic King Roderic; opens Iberia to Muslim rule",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Covadonga"),
                    year: 718,
                    location: Cow::Borrowed("Covadonga, Asturias"),
                    description: Cow::Borrowed(
                        "Pelayo's victory; seeds of the Reconquista and the Kingdom of Asturias",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Umayyad conquest of Visigothic Iberia; established Al-Andalus for nearly 800 years",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Mongol Invasion of Khwarezmia"),
            region: Cow::Borrowed("Central Asia"),
            start_year: 1219,
            end_year: 1221,
            belligerents_a: vec![Cow::Borrowed("Mongol Empire")],
            belligerents_b: vec![Cow::Borrowed("Khwarezmian Empire")],
            commanders: vec![
                Cow::Borrowed("Genghis Khan"),
                Cow::Borrowed("Muhammad II of Khwarezm"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Siege of Bukhara"),
                    year: 1220,
                    location: Cow::Borrowed("Bukhara, Central Asia"),
                    description: Cow::Borrowed(
                        "Mongols capture and sack Bukhara; demonstrates speed of Mongol advance",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Siege of Samarkand"),
                    year: 1220,
                    location: Cow::Borrowed("Samarkand, Central Asia"),
                    description: Cow::Borrowed(
                        "Largest city in Khwarezmia falls; Mongol Empire expands into Islamic world",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Mongol destruction of the Khwarezmian Empire; opened Central Asia and Persia to Mongol rule",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Hundred Years' War"),
            region: Cow::Borrowed("Europe"),
            start_year: 1337,
            end_year: 1453,
            belligerents_a: vec![Cow::Borrowed("Kingdom of England")],
            belligerents_b: vec![Cow::Borrowed("Kingdom of France")],
            commanders: vec![
                Cow::Borrowed("Edward III"),
                Cow::Borrowed("Henry V"),
                Cow::Borrowed("Joan of Arc"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Crécy"),
                    year: 1346,
                    location: Cow::Borrowed("Crécy, France"),
                    description: Cow::Borrowed(
                        "English longbowmen devastate French cavalry; demonstrates ranged warfare supremacy",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Agincourt"),
                    year: 1415,
                    location: Cow::Borrowed("Agincourt, France"),
                    description: Cow::Borrowed(
                        "Henry V's outnumbered English army defeats French; peak of English success",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Siege of Orléans"),
                    year: 1429,
                    location: Cow::Borrowed("Orléans, France"),
                    description: Cow::Borrowed(
                        "Joan of Arc lifts the siege; turning point leading to French recovery",
                    ),
                },
            ],
            outcome: CampaignOutcome::Defeat,
            description: Cow::Borrowed(
                "English dynastic claim to the French throne; ended with French victory and English expulsion from continental France",
            ),
        },
        // ── Early Modern ─────────────────────────────────────────────
        Campaign {
            name: Cow::Borrowed("Ottoman Conquest of Constantinople"),
            region: Cow::Borrowed("Near East"),
            start_year: 1453,
            end_year: 1453,
            belligerents_a: vec![Cow::Borrowed("Ottoman Empire")],
            belligerents_b: vec![Cow::Borrowed("Byzantine Empire")],
            commanders: vec![Cow::Borrowed("Mehmed II"), Cow::Borrowed("Constantine XI")],
            battles: vec![Battle {
                name: Cow::Borrowed("Siege of Constantinople"),
                year: 1453,
                location: Cow::Borrowed("Constantinople"),
                description: Cow::Borrowed(
                    "53-day siege; Mehmed's massive cannons breach the Theodosian Walls; end of the Roman state",
                ),
            }],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Ottoman capture of Constantinople; ended the Byzantine Empire and established Ottoman control of the Bosphorus",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Spanish Conquest of the Aztec Empire"),
            region: Cow::Borrowed("Mesoamerica"),
            start_year: 1519,
            end_year: 1521,
            belligerents_a: vec![Cow::Borrowed("Spanish Empire")],
            belligerents_b: vec![Cow::Borrowed("Aztec Empire")],
            commanders: vec![
                Cow::Borrowed("Hernán Cortés"),
                Cow::Borrowed("Montezuma II"),
                Cow::Borrowed("Cuauhtémoc"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("La Noche Triste"),
                    year: 1520,
                    location: Cow::Borrowed("Tenochtitlan, Mexico"),
                    description: Cow::Borrowed(
                        "Spanish retreat from Tenochtitlan with heavy losses; Aztec tactical victory",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Siege of Tenochtitlan"),
                    year: 1521,
                    location: Cow::Borrowed("Tenochtitlan, Mexico"),
                    description: Cow::Borrowed(
                        "75-day siege; Spanish and indigenous allies destroy the Aztec capital",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Spanish conquest of the Aztec Empire with indigenous allies; smallpox and siege warfare destroyed Tenochtitlan",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Thirty Years' War"),
            region: Cow::Borrowed("Europe"),
            start_year: 1618,
            end_year: 1648,
            belligerents_a: vec![
                Cow::Borrowed("Holy Roman Empire"),
                Cow::Borrowed("Spanish Empire"),
            ],
            belligerents_b: vec![Cow::Borrowed("Kingdom of France"), Cow::Borrowed("Sweden")],
            commanders: vec![
                Cow::Borrowed("Gustavus Adolphus"),
                Cow::Borrowed("Albrecht von Wallenstein"),
                Cow::Borrowed("Cardinal Richelieu"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Breitenfeld"),
                    year: 1631,
                    location: Cow::Borrowed("Breitenfeld, Saxony"),
                    description: Cow::Borrowed(
                        "Gustavus Adolphus' Swedish army defeats Imperial forces; demonstrates mobile artillery tactics",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Lützen"),
                    year: 1632,
                    location: Cow::Borrowed("Lützen, Saxony"),
                    description: Cow::Borrowed(
                        "Swedish victory but Gustavus Adolphus killed; shifted momentum of the war",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Rocroi"),
                    year: 1643,
                    location: Cow::Borrowed("Rocroi, France"),
                    description: Cow::Borrowed(
                        "French victory ends Spanish military supremacy; beginning of French dominance",
                    ),
                },
            ],
            outcome: CampaignOutcome::Treaty,
            description: Cow::Borrowed(
                "Religious and political war devastating Central Europe; ended by the Peace of Westphalia establishing state sovereignty",
            ),
        },
        // ── Modern ───────────────────────────────────────────────────
        Campaign {
            name: Cow::Borrowed("Napoleonic Wars"),
            region: Cow::Borrowed("Europe"),
            start_year: 1803,
            end_year: 1815,
            belligerents_a: vec![Cow::Borrowed("French Empire")],
            belligerents_b: vec![
                Cow::Borrowed("British Empire"),
                Cow::Borrowed("Russian Empire"),
                Cow::Borrowed("Austrian Empire"),
            ],
            commanders: vec![
                Cow::Borrowed("Napoleon Bonaparte"),
                Cow::Borrowed("Duke of Wellington"),
                Cow::Borrowed("Mikhail Kutuzov"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Austerlitz"),
                    year: 1805,
                    location: Cow::Borrowed("Austerlitz, Moravia"),
                    description: Cow::Borrowed(
                        "Napoleon's masterpiece; defeats combined Austro-Russian army on the anniversary of his coronation",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Trafalgar"),
                    year: 1805,
                    location: Cow::Borrowed("Cape Trafalgar, Spain"),
                    description: Cow::Borrowed(
                        "Nelson destroys Franco-Spanish fleet; secures British naval supremacy",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Borodino"),
                    year: 1812,
                    location: Cow::Borrowed("Borodino, Russia"),
                    description: Cow::Borrowed(
                        "Bloodiest single day of the Napoleonic Wars; Pyrrhic French victory before Moscow",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Waterloo"),
                    year: 1815,
                    location: Cow::Borrowed("Waterloo, Belgium"),
                    description: Cow::Borrowed(
                        "Wellington and Blücher defeat Napoleon; ends the Napoleonic era",
                    ),
                },
            ],
            outcome: CampaignOutcome::Defeat,
            description: Cow::Borrowed(
                "French bid for European hegemony; reshaped borders, spread revolutionary ideals, ended with Napoleon's exile",
            ),
        },
        Campaign {
            name: Cow::Borrowed("American Civil War"),
            region: Cow::Borrowed("North America"),
            start_year: 1861,
            end_year: 1865,
            belligerents_a: vec![Cow::Borrowed("United States (Union)")],
            belligerents_b: vec![Cow::Borrowed("Confederate States")],
            commanders: vec![
                Cow::Borrowed("Ulysses S. Grant"),
                Cow::Borrowed("William T. Sherman"),
                Cow::Borrowed("Robert E. Lee"),
                Cow::Borrowed("Stonewall Jackson"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Gettysburg"),
                    year: 1863,
                    location: Cow::Borrowed("Gettysburg, Pennsylvania"),
                    description: Cow::Borrowed(
                        "Turning point; Lee's invasion of the North repelled with massive casualties",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Siege of Vicksburg"),
                    year: 1863,
                    location: Cow::Borrowed("Vicksburg, Mississippi"),
                    description: Cow::Borrowed(
                        "Grant captures the last Confederate stronghold on the Mississippi; splits the Confederacy",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Union victory preserving the United States and abolishing slavery; deadliest American conflict",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Zulu War"),
            region: Cow::Borrowed("Sub-Saharan Africa"),
            start_year: 1879,
            end_year: 1879,
            belligerents_a: vec![Cow::Borrowed("British Empire")],
            belligerents_b: vec![Cow::Borrowed("Zulu Kingdom")],
            commanders: vec![
                Cow::Borrowed("Lord Chelmsford"),
                Cow::Borrowed("Cetshwayo kaMpande"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Battle of Isandlwana"),
                    year: 1879,
                    location: Cow::Borrowed("Isandlwana, Zululand"),
                    description: Cow::Borrowed(
                        "Zulu army overruns British camp; worst British defeat against an indigenous force",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Rorke's Drift"),
                    year: 1879,
                    location: Cow::Borrowed("Rorke's Drift, Zululand"),
                    description: Cow::Borrowed(
                        "150 British soldiers repel 3,000–4,000 Zulu warriors at a mission station",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Ulundi"),
                    year: 1879,
                    location: Cow::Borrowed("Ulundi, Zululand"),
                    description: Cow::Borrowed(
                        "British defeat the Zulu army and burn the royal kraal; ends Zulu independence",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "British conquest of the Zulu Kingdom; ended Zulu military power despite early British defeats",
            ),
        },
        Campaign {
            name: Cow::Borrowed("Russo-Japanese War"),
            region: Cow::Borrowed("East Asia"),
            start_year: 1904,
            end_year: 1905,
            belligerents_a: vec![Cow::Borrowed("Japan")],
            belligerents_b: vec![Cow::Borrowed("Russian Empire")],
            commanders: vec![
                Cow::Borrowed("Tōgō Heihachirō"),
                Cow::Borrowed("Nogi Maresuke"),
                Cow::Borrowed("Aleksei Kuropatkin"),
            ],
            battles: vec![
                Battle {
                    name: Cow::Borrowed("Siege of Port Arthur"),
                    year: 1904,
                    location: Cow::Borrowed("Port Arthur, Manchuria"),
                    description: Cow::Borrowed(
                        "Prolonged Japanese siege of Russian naval base; foreshadows trench warfare",
                    ),
                },
                Battle {
                    name: Cow::Borrowed("Battle of Tsushima"),
                    year: 1905,
                    location: Cow::Borrowed("Tsushima Strait"),
                    description: Cow::Borrowed(
                        "Japanese fleet annihilates Russian Baltic Fleet; first modern naval victory by an Asian power over a European one",
                    ),
                },
            ],
            outcome: CampaignOutcome::Victory,
            description: Cow::Borrowed(
                "Decisive Japanese victory over Russia; Treaty of Portsmouth; first modern Asian victory over a European empire",
            ),
        },
    ]
}

/// Returns all pre-built campaigns.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_campaigns() -> &'static [Campaign] {
    static DATA: std::sync::LazyLock<Vec<Campaign>> = std::sync::LazyLock::new(build_campaigns);
    &DATA
}

/// Returns all pre-built campaigns.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_campaigns() -> Vec<Campaign> {
    build_campaigns()
}

/// Returns campaigns in the given region (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_region(region: &str) -> Vec<Campaign> {
    tracing::debug!(region, "looking up campaigns by region");
    let lower = region.to_lowercase();
    all_campaigns()
        .iter()
        .filter(|c| c.region.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

/// Returns campaigns that were active during the given year.
#[must_use]
#[inline]
pub fn active_at(year: i32) -> Vec<Campaign> {
    tracing::debug!(year, "looking up campaigns active at year");
    all_campaigns()
        .iter()
        .filter(|c| year >= c.start_year && year <= c.end_year)
        .cloned()
        .collect()
}

/// Returns campaigns involving a given civilization (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_civilization(civilization: &str) -> Vec<Campaign> {
    tracing::debug!(civilization, "looking up campaigns by civilization");
    let lower = civilization.to_lowercase();
    all_campaigns()
        .iter()
        .filter(|c| {
            c.belligerents_a
                .iter()
                .chain(c.belligerents_b.iter())
                .any(|b| b.to_lowercase().contains(&lower))
        })
        .cloned()
        .collect()
}

/// Returns campaigns featuring a given commander (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_commander(commander: &str) -> Vec<Campaign> {
    tracing::debug!(commander, "looking up campaigns by commander");
    let lower = commander.to_lowercase();
    all_campaigns()
        .iter()
        .filter(|c| {
            c.commanders
                .iter()
                .any(|cmd| cmd.to_lowercase().contains(&lower))
        })
        .cloned()
        .collect()
}

/// Returns campaigns with the given outcome.
#[must_use]
#[inline]
pub fn by_outcome(outcome: &CampaignOutcome) -> Vec<Campaign> {
    tracing::debug!(?outcome, "looking up campaigns by outcome");
    all_campaigns()
        .iter()
        .filter(|c| c.outcome == *outcome)
        .cloned()
        .collect()
}

/// Returns campaigns between two years (inclusive).
#[must_use]
pub fn campaigns_between(start: i32, end: i32) -> Vec<Campaign> {
    tracing::debug!(start, end, "looking up campaigns between years");
    let mut results: Vec<Campaign> = all_campaigns()
        .iter()
        .filter(|c| c.start_year <= end && c.end_year >= start)
        .cloned()
        .collect();
    results.sort();
    results
}

/// Look up a campaign by exact name (case-insensitive).
///
/// # Errors
///
/// Returns [`ItihasError::CampaignNotFound`] if no campaign matches.
pub fn by_name(name: &str) -> Result<Campaign, ItihasError> {
    tracing::debug!(name, "looking up campaign by name");
    let lower = name.to_lowercase();
    all_campaigns()
        .iter()
        .find(|c| c.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::CampaignNotFound(String::from(name)))
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_all_campaigns_count() {
        assert_eq!(all_campaigns().len(), 14);
    }

    #[test]
    fn test_by_region_europe() {
        let campaigns = by_region("Europe");
        assert!(campaigns.iter().any(|c| c.name == "Hundred Years' War"));
        assert!(campaigns.iter().any(|c| c.name == "Napoleonic Wars"));
    }

    #[test]
    fn test_by_region_case_insensitive() {
        let campaigns = by_region("mediterranean");
        assert!(campaigns.iter().any(|c| c.name == "Greco-Persian Wars"));
    }

    #[test]
    fn test_by_region_no_match() {
        assert!(by_region("Antarctica").is_empty());
    }

    #[test]
    fn test_active_at_1812() {
        let campaigns = active_at(1812);
        assert!(campaigns.iter().any(|c| c.name == "Napoleonic Wars"));
    }

    #[test]
    fn test_active_at_none() {
        assert!(active_at(-100_000).is_empty());
    }

    #[test]
    fn test_by_civilization_roman() {
        let campaigns = by_civilization("Roman");
        assert!(campaigns.iter().any(|c| c.name == "Gallic Wars"));
        assert!(campaigns.iter().any(|c| c.name == "Second Punic War"));
    }

    #[test]
    fn test_by_commander_napoleon() {
        let campaigns = by_commander("Napoleon");
        assert_eq!(campaigns.len(), 1);
        assert_eq!(campaigns[0].name, "Napoleonic Wars");
    }

    #[test]
    fn test_by_commander_case_insensitive() {
        let campaigns = by_commander("alexander");
        assert!(
            campaigns
                .iter()
                .any(|c| c.name == "Campaigns of Alexander the Great")
        );
    }

    #[test]
    fn test_by_outcome_victory() {
        let campaigns = by_outcome(&CampaignOutcome::Victory);
        assert!(!campaigns.is_empty());
        for c in &campaigns {
            assert_eq!(c.outcome, CampaignOutcome::Victory);
        }
    }

    #[test]
    fn test_by_outcome_treaty() {
        let campaigns = by_outcome(&CampaignOutcome::Treaty);
        assert!(campaigns.iter().any(|c| c.name == "Thirty Years' War"));
    }

    #[test]
    fn test_campaigns_between() {
        let campaigns = campaigns_between(-500, -200);
        assert!(campaigns.iter().any(|c| c.name == "Greco-Persian Wars"));
        assert!(campaigns.iter().any(|c| c.name == "Second Punic War"));
        // Verify sorted
        for w in campaigns.windows(2) {
            assert!(w[0].start_year <= w[1].start_year);
        }
    }

    #[test]
    fn test_by_name_found() {
        let c = by_name("Napoleonic Wars").unwrap();
        assert_eq!(c.start_year, 1803);
    }

    #[test]
    fn test_by_name_case_insensitive() {
        assert!(by_name("napoleonic wars").is_ok());
    }

    #[test]
    fn test_by_name_not_found() {
        assert!(by_name("Star Wars").is_err());
    }

    #[test]
    fn test_campaign_display() {
        let c = by_name("Napoleonic Wars").unwrap();
        let display = format!("{c}");
        assert!(display.contains("Napoleonic Wars"));
        assert!(display.contains("Defeat"));
    }

    #[test]
    fn test_battle_display() {
        let c = by_name("Napoleonic Wars").unwrap();
        let b = &c.battles[0];
        let display = format!("{b}");
        assert!(display.contains("Austerlitz"));
    }

    #[test]
    fn test_campaign_outcome_display() {
        assert_eq!(format!("{}", CampaignOutcome::Victory), "Victory");
        assert_eq!(format!("{}", CampaignOutcome::Treaty), "Treaty");
        assert_eq!(format!("{}", CampaignOutcome::Stalemate), "Stalemate");
    }

    #[test]
    fn test_campaigns_sort_chronologically() {
        let mut campaigns = all_campaigns().to_vec();
        campaigns.sort();
        for w in campaigns.windows(2) {
            assert!(w[0].start_year <= w[1].start_year);
        }
    }

    #[test]
    fn test_battles_within_campaign_dates() {
        for campaign in all_campaigns() {
            for battle in &campaign.battles {
                assert!(
                    battle.year >= campaign.start_year && battle.year <= campaign.end_year,
                    "battle '{}' (year {}) outside campaign '{}' ({} – {})",
                    battle.name,
                    battle.year,
                    campaign.name,
                    campaign.start_year,
                    campaign.end_year
                );
            }
        }
    }

    #[test]
    fn test_campaign_serde_roundtrip() {
        for campaign in all_campaigns().iter() {
            let json = serde_json::to_string(campaign).unwrap();
            let back: Campaign = serde_json::from_str(&json).unwrap();
            assert_eq!(campaign, &back);
        }
    }

    #[test]
    fn test_campaign_outcome_serde_roundtrip() {
        let outcomes = [
            CampaignOutcome::Victory,
            CampaignOutcome::Defeat,
            CampaignOutcome::Stalemate,
            CampaignOutcome::Treaty,
            CampaignOutcome::Inconclusive,
        ];
        for o in &outcomes {
            let json = serde_json::to_string(o).unwrap();
            let back: CampaignOutcome = serde_json::from_str(&json).unwrap();
            assert_eq!(o, &back);
        }
    }

    #[test]
    fn test_battle_serde_roundtrip() {
        for campaign in all_campaigns().iter() {
            for battle in &campaign.battles {
                let json = serde_json::to_string(battle).unwrap();
                let back: Battle = serde_json::from_str(&json).unwrap();
                assert_eq!(battle, &back);
            }
        }
    }
}
