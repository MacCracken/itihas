//! Historical trade routes with endpoints, period, and commodity metadata.
//!
//! Provides [`TradeRoute`] structs representing major historical trade networks,
//! a [`RouteType`] classification enum, and lookup functions by region,
//! period, commodity, and civilization.
//!
//! # Sources
//!
//! General: Curtin (1984). Silk Roads: Liu (2010), Frankopan (2015). Indian
//! Ocean: Beaujard (2019). Trans-Saharan: Austen (2010). Hanseatic: Dollinger
//! (1970). Atlantic: Mintz (1985). Full bibliography:
//! [`docs/sources/trade-routes.md`](https://github.com/MacCracken/itihas/blob/main/docs/sources/trade-routes.md).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

use crate::error::ItihasError;

/// Classification of trade routes by primary medium.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RouteType {
    /// Overland route (caravan, road).
    Land,
    /// Maritime route (sea, ocean).
    Maritime,
    /// River-based route (inland waterway).
    River,
    /// Mixed overland and maritime.
    Mixed,
}

impl fmt::Display for RouteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Land => f.write_str("Land"),
            Self::Maritime => f.write_str("Maritime"),
            Self::River => f.write_str("River"),
            Self::Mixed => f.write_str("Mixed"),
        }
    }
}

/// A historical trade route.
///
/// Years use astronomical year numbering: negative = BCE, positive = CE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeRoute {
    /// Name of the route or network.
    pub name: Cow<'static, str>,
    /// Primary regions connected.
    pub regions: Vec<Cow<'static, str>>,
    /// Route type classification.
    pub route_type: RouteType,
    /// Civilizations that operated or controlled the route.
    pub civilizations: Vec<Cow<'static, str>>,
    /// Primary commodities traded.
    pub commodities: Vec<Cow<'static, str>>,
    /// Approximate start of active use (negative = BCE).
    pub start_year: i32,
    /// Approximate end of active use (negative = BCE). `i32::MAX` for ongoing.
    pub end_year: i32,
    /// Brief description.
    pub description: Cow<'static, str>,
}

impl fmt::Display for TradeRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {} – {})",
            self.name, self.route_type, self.start_year, self.end_year
        )
    }
}

fn build_routes() -> Vec<TradeRoute> {
    vec![
        TradeRoute {
            name: Cow::Borrowed("Silk Road"),
            regions: vec![
                Cow::Borrowed("East Asia"),
                Cow::Borrowed("Central Asia"),
                Cow::Borrowed("Near East"),
                Cow::Borrowed("Mediterranean"),
            ],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Ancient China"),
                Cow::Borrowed("Persian Empire"),
                Cow::Borrowed("Roman Empire"),
                Cow::Borrowed("Mongol Empire"),
            ],
            commodities: vec![
                Cow::Borrowed("silk"),
                Cow::Borrowed("spices"),
                Cow::Borrowed("jade"),
                Cow::Borrowed("glass"),
                Cow::Borrowed("paper"),
            ],
            start_year: -130,
            end_year: 1453,
            description: Cow::Borrowed(
                "Trans-Eurasian overland network linking Chang'an to Constantinople; primary conduit for silk, ideas, and religions",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Maritime Silk Road"),
            regions: vec![
                Cow::Borrowed("East Asia"),
                Cow::Borrowed("Southeast Asia"),
                Cow::Borrowed("South Asia"),
                Cow::Borrowed("Near East"),
                Cow::Borrowed("East Africa"),
            ],
            route_type: RouteType::Maritime,
            civilizations: vec![
                Cow::Borrowed("Ancient China"),
                Cow::Borrowed("Gupta Empire"),
                Cow::Borrowed("Khmer Empire"),
                Cow::Borrowed("Arab Caliphates"),
            ],
            commodities: vec![
                Cow::Borrowed("silk"),
                Cow::Borrowed("porcelain"),
                Cow::Borrowed("spices"),
                Cow::Borrowed("incense"),
                Cow::Borrowed("ivory"),
            ],
            start_year: -200,
            end_year: 1500,
            description: Cow::Borrowed(
                "Sea route from South China Sea through Indian Ocean to Red Sea; monsoon-driven seasonal trade",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Incense Route"),
            regions: vec![
                Cow::Borrowed("Arabian Peninsula"),
                Cow::Borrowed("Near East"),
                Cow::Borrowed("Mediterranean"),
            ],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Nabataean Kingdom"),
                Cow::Borrowed("Kingdom of Saba"),
            ],
            commodities: vec![
                Cow::Borrowed("frankincense"),
                Cow::Borrowed("myrrh"),
                Cow::Borrowed("spices"),
            ],
            start_year: -1000,
            end_year: 200,
            description: Cow::Borrowed(
                "Caravan route from southern Arabia (Dhofar, Hadramaut) to Gaza and Petra; frankincense and myrrh trade",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Amber Road"),
            regions: vec![Cow::Borrowed("Europe"), Cow::Borrowed("Mediterranean")],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Roman Empire"),
                Cow::Borrowed("Germanic Tribes"),
            ],
            commodities: vec![
                Cow::Borrowed("amber"),
                Cow::Borrowed("furs"),
                Cow::Borrowed("salt"),
            ],
            start_year: -1600,
            end_year: 500,
            description: Cow::Borrowed(
                "Baltic-to-Mediterranean overland route; Baltic amber traded south for Roman manufactured goods",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Trans-Saharan Trade"),
            regions: vec![
                Cow::Borrowed("North Africa"),
                Cow::Borrowed("Sub-Saharan Africa"),
            ],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Ghana Empire"),
                Cow::Borrowed("Mali Empire"),
                Cow::Borrowed("Songhai Empire"),
                Cow::Borrowed("Arab Caliphates"),
            ],
            commodities: vec![
                Cow::Borrowed("gold"),
                Cow::Borrowed("salt"),
                Cow::Borrowed("slaves"),
                Cow::Borrowed("ivory"),
                Cow::Borrowed("textiles"),
            ],
            start_year: 300,
            end_year: 1600,
            description: Cow::Borrowed(
                "Camel caravan routes across Sahara linking West African gold to Mediterranean markets; salt-gold exchange powered by camel domestication",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Spice Route"),
            regions: vec![
                Cow::Borrowed("Southeast Asia"),
                Cow::Borrowed("South Asia"),
                Cow::Borrowed("Near East"),
                Cow::Borrowed("Mediterranean"),
                Cow::Borrowed("Europe"),
            ],
            route_type: RouteType::Maritime,
            civilizations: vec![
                Cow::Borrowed("Gupta Empire"),
                Cow::Borrowed("Arab Caliphates"),
                Cow::Borrowed("Portuguese Empire"),
                Cow::Borrowed("Dutch Republic"),
            ],
            commodities: vec![
                Cow::Borrowed("pepper"),
                Cow::Borrowed("cinnamon"),
                Cow::Borrowed("cloves"),
                Cow::Borrowed("nutmeg"),
                Cow::Borrowed("cardamom"),
            ],
            start_year: -300,
            end_year: 1800,
            description: Cow::Borrowed(
                "Indian Ocean maritime network for spice trade; Malabar Coast, Strait of Malacca, Moluccas",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Tin Route"),
            regions: vec![Cow::Borrowed("Europe"), Cow::Borrowed("Mediterranean")],
            route_type: RouteType::Mixed,
            civilizations: vec![
                Cow::Borrowed("Phoenicia"),
                Cow::Borrowed("Ancient Greece"),
                Cow::Borrowed("Roman Empire"),
            ],
            commodities: vec![
                Cow::Borrowed("tin"),
                Cow::Borrowed("copper"),
                Cow::Borrowed("bronze goods"),
            ],
            start_year: -2500,
            end_year: -100,
            description: Cow::Borrowed(
                "Cornish and Iberian tin shipped to Mediterranean bronze-producing centers; essential for Bronze Age metallurgy",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Via Maris"),
            regions: vec![Cow::Borrowed("Near East"), Cow::Borrowed("North Africa")],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Mesopotamia"),
                Cow::Borrowed("Persian Empire"),
            ],
            commodities: vec![
                Cow::Borrowed("grain"),
                Cow::Borrowed("olive oil"),
                Cow::Borrowed("textiles"),
                Cow::Borrowed("metals"),
            ],
            start_year: -2000,
            end_year: -300,
            description: Cow::Borrowed(
                "Ancient coastal road from Egypt through Levant to Mesopotamia; major military and commercial artery",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Hanseatic Trade Network"),
            regions: vec![Cow::Borrowed("Europe")],
            route_type: RouteType::Maritime,
            civilizations: vec![Cow::Borrowed("Hanseatic League")],
            commodities: vec![
                Cow::Borrowed("herring"),
                Cow::Borrowed("timber"),
                Cow::Borrowed("furs"),
                Cow::Borrowed("grain"),
                Cow::Borrowed("wool"),
            ],
            start_year: 1200,
            end_year: 1669,
            description: Cow::Borrowed(
                "North and Baltic Sea trade network; merchant guild alliance controlling northern European commerce",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Nile Trade Route"),
            regions: vec![
                Cow::Borrowed("North Africa"),
                Cow::Borrowed("Sub-Saharan Africa"),
            ],
            route_type: RouteType::River,
            civilizations: vec![
                Cow::Borrowed("Ancient Egypt"),
                Cow::Borrowed("Kingdom of Kush"),
            ],
            commodities: vec![
                Cow::Borrowed("gold"),
                Cow::Borrowed("ivory"),
                Cow::Borrowed("ebony"),
                Cow::Borrowed("grain"),
                Cow::Borrowed("papyrus"),
            ],
            start_year: -3100,
            end_year: 640,
            description: Cow::Borrowed(
                "Nile River corridor linking Upper and Lower Egypt to Nubia and Kush; annual flood cycle enabling agriculture and transport",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Grand Trunk Road"),
            regions: vec![Cow::Borrowed("South Asia"), Cow::Borrowed("Central Asia")],
            route_type: RouteType::Land,
            civilizations: vec![
                Cow::Borrowed("Maurya Empire"),
                Cow::Borrowed("Gupta Empire"),
                Cow::Borrowed("Mughal Empire"),
            ],
            commodities: vec![
                Cow::Borrowed("textiles"),
                Cow::Borrowed("spices"),
                Cow::Borrowed("gemstones"),
                Cow::Borrowed("horses"),
            ],
            start_year: -300,
            end_year: i32::MAX,
            description: Cow::Borrowed(
                "South Asia's great trunk road from Kabul to Chittagong; rebuilt by Sher Shah Suri, still in use",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Austronesian Maritime Network"),
            regions: vec![
                Cow::Borrowed("Southeast Asia"),
                Cow::Borrowed("Oceania"),
                Cow::Borrowed("East Africa"),
            ],
            route_type: RouteType::Maritime,
            civilizations: vec![Cow::Borrowed("Austronesian Peoples")],
            commodities: vec![
                Cow::Borrowed("obsidian"),
                Cow::Borrowed("pottery"),
                Cow::Borrowed("cinnamon"),
                Cow::Borrowed("coconut"),
            ],
            start_year: -3000,
            end_year: 1500,
            description: Cow::Borrowed(
                "Open-ocean trade spanning Taiwan to Madagascar; earliest long-distance maritime exchange network",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Varangian Trade Routes"),
            regions: vec![Cow::Borrowed("Europe"), Cow::Borrowed("Near East")],
            route_type: RouteType::River,
            civilizations: vec![
                Cow::Borrowed("Viking Civilization"),
                Cow::Borrowed("Byzantine Empire"),
                Cow::Borrowed("Arab Caliphates"),
            ],
            commodities: vec![
                Cow::Borrowed("furs"),
                Cow::Borrowed("slaves"),
                Cow::Borrowed("amber"),
                Cow::Borrowed("silver"),
                Cow::Borrowed("swords"),
            ],
            start_year: 750,
            end_year: 1100,
            description: Cow::Borrowed(
                "Norse river routes via Dnieper and Volga connecting Scandinavia to Constantinople and Baghdad",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Triangular Trade"),
            regions: vec![
                Cow::Borrowed("Europe"),
                Cow::Borrowed("West Africa"),
                Cow::Borrowed("Americas"),
            ],
            route_type: RouteType::Maritime,
            civilizations: vec![
                Cow::Borrowed("British Empire"),
                Cow::Borrowed("Portuguese Empire"),
                Cow::Borrowed("Spanish Empire"),
            ],
            commodities: vec![
                Cow::Borrowed("slaves"),
                Cow::Borrowed("sugar"),
                Cow::Borrowed("rum"),
                Cow::Borrowed("cotton"),
                Cow::Borrowed("manufactured goods"),
            ],
            start_year: 1500,
            end_year: 1867,
            description: Cow::Borrowed(
                "Atlantic triangular exchange: European goods to Africa, enslaved people to Americas, plantation commodities to Europe",
            ),
        },
        TradeRoute {
            name: Cow::Borrowed("Tea-Horse Road"),
            regions: vec![Cow::Borrowed("East Asia"), Cow::Borrowed("South Asia")],
            route_type: RouteType::Land,
            civilizations: vec![Cow::Borrowed("Ancient China"), Cow::Borrowed("Tibet")],
            commodities: vec![
                Cow::Borrowed("tea"),
                Cow::Borrowed("horses"),
                Cow::Borrowed("salt"),
                Cow::Borrowed("medicinal herbs"),
            ],
            start_year: 600,
            end_year: 1950,
            description: Cow::Borrowed(
                "Sichuan-to-Tibet caravan trail; Chinese tea exchanged for Tibetan war horses across Himalayan passes",
            ),
        },
    ]
}

/// Returns all pre-built trade routes.
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn all_routes() -> &'static [TradeRoute] {
    static DATA: std::sync::LazyLock<Vec<TradeRoute>> = std::sync::LazyLock::new(build_routes);
    &DATA
}

/// Returns all pre-built trade routes.
#[cfg(not(feature = "std"))]
#[must_use]
#[inline]
pub fn all_routes() -> Vec<TradeRoute> {
    build_routes()
}

/// Returns routes that connect the given region (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_region(region: &str) -> Vec<TradeRoute> {
    tracing::debug!(region, "looking up trade routes by region");
    let lower = region.to_lowercase();
    all_routes()
        .iter()
        .filter(|r| {
            r.regions
                .iter()
                .any(|reg| reg.to_lowercase().contains(&lower))
        })
        .cloned()
        .collect()
}

/// Returns routes that were active during the given year.
#[must_use]
#[inline]
pub fn active_at(year: i32) -> Vec<TradeRoute> {
    tracing::debug!(year, "looking up trade routes active at year");
    all_routes()
        .iter()
        .filter(|r| year >= r.start_year && year <= r.end_year)
        .cloned()
        .collect()
}

/// Returns routes that traded the given commodity (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_commodity(commodity: &str) -> Vec<TradeRoute> {
    tracing::debug!(commodity, "looking up trade routes by commodity");
    let lower = commodity.to_lowercase();
    all_routes()
        .iter()
        .filter(|r| {
            r.commodities
                .iter()
                .any(|c| c.to_lowercase().contains(&lower))
        })
        .cloned()
        .collect()
}

/// Returns routes of the given type.
#[must_use]
#[inline]
pub fn by_type(route_type: &RouteType) -> Vec<TradeRoute> {
    tracing::debug!(?route_type, "looking up trade routes by type");
    all_routes()
        .iter()
        .filter(|r| r.route_type == *route_type)
        .cloned()
        .collect()
}

/// Returns routes involving a given civilization (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_civilization(civilization: &str) -> Vec<TradeRoute> {
    tracing::debug!(civilization, "looking up trade routes by civilization");
    let lower = civilization.to_lowercase();
    all_routes()
        .iter()
        .filter(|r| {
            r.civilizations
                .iter()
                .any(|c| c.to_lowercase().contains(&lower))
        })
        .cloned()
        .collect()
}

/// Look up a trade route by exact name (case-insensitive).
///
/// # Errors
///
/// Returns [`ItihasError::RouteNotFound`] if no route matches.
pub fn by_name(name: &str) -> Result<TradeRoute, ItihasError> {
    tracing::debug!(name, "looking up trade route by name");
    let lower = name.to_lowercase();
    all_routes()
        .iter()
        .find(|r| r.name.to_lowercase() == lower)
        .cloned()
        .ok_or_else(|| ItihasError::RouteNotFound(String::from(name)))
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_all_routes_count() {
        assert_eq!(all_routes().len(), 15);
    }

    #[test]
    fn test_by_region_east_asia() {
        let routes = by_region("East Asia");
        assert!(routes.iter().any(|r| r.name == "Silk Road"));
        assert!(routes.iter().any(|r| r.name == "Maritime Silk Road"));
    }

    #[test]
    fn test_by_region_case_insensitive() {
        let routes = by_region("europe");
        assert!(routes.iter().any(|r| r.name == "Amber Road"));
        assert!(routes.iter().any(|r| r.name == "Hanseatic Trade Network"));
    }

    #[test]
    fn test_by_region_no_match() {
        assert!(by_region("Antarctica").is_empty());
    }

    #[test]
    fn test_active_at_100_bce() {
        let routes = active_at(-100);
        let names: Vec<_> = routes.iter().map(|r| r.name.as_ref()).collect();
        assert!(names.contains(&"Silk Road"));
        assert!(names.contains(&"Incense Route"));
    }

    #[test]
    fn test_active_at_none() {
        assert!(active_at(-100_000).is_empty());
    }

    #[test]
    fn test_by_commodity_silk() {
        let routes = by_commodity("silk");
        assert!(routes.iter().any(|r| r.name == "Silk Road"));
        assert!(routes.iter().any(|r| r.name == "Maritime Silk Road"));
    }

    #[test]
    fn test_by_commodity_case_insensitive() {
        let routes = by_commodity("GOLD");
        assert!(routes.iter().any(|r| r.name == "Trans-Saharan Trade"));
    }

    #[test]
    fn test_by_type_maritime() {
        let routes = by_type(&RouteType::Maritime);
        assert!(routes.iter().any(|r| r.name == "Maritime Silk Road"));
        assert!(routes.iter().any(|r| r.name == "Spice Route"));
    }

    #[test]
    fn test_by_type_river() {
        let routes = by_type(&RouteType::River);
        assert!(routes.iter().any(|r| r.name == "Nile Trade Route"));
        assert!(routes.iter().any(|r| r.name == "Varangian Trade Routes"));
    }

    #[test]
    fn test_by_civilization() {
        let routes = by_civilization("Roman");
        assert!(routes.iter().any(|r| r.name == "Silk Road"));
        assert!(routes.iter().any(|r| r.name == "Amber Road"));
    }

    #[test]
    fn test_by_name_found() {
        let route = by_name("Silk Road").unwrap();
        assert!(route.commodities.iter().any(|c| c == "silk"));
    }

    #[test]
    fn test_by_name_case_insensitive() {
        assert!(by_name("silk road").is_ok());
    }

    #[test]
    fn test_by_name_not_found() {
        assert!(by_name("Route 66").is_err());
    }

    #[test]
    fn test_route_display() {
        let route = by_name("Silk Road").unwrap();
        let display = format!("{route}");
        assert!(display.contains("Silk Road"));
        assert!(display.contains("Land"));
    }

    #[test]
    fn test_route_type_display() {
        assert_eq!(format!("{}", RouteType::Land), "Land");
        assert_eq!(format!("{}", RouteType::Maritime), "Maritime");
        assert_eq!(format!("{}", RouteType::River), "River");
        assert_eq!(format!("{}", RouteType::Mixed), "Mixed");
    }

    #[test]
    fn test_route_serde_roundtrip() {
        for route in all_routes().iter() {
            let json = serde_json::to_string(route).unwrap();
            let back: TradeRoute = serde_json::from_str(&json).unwrap();
            assert_eq!(route, &back);
        }
    }

    #[test]
    fn test_route_type_serde_roundtrip() {
        let types = [
            RouteType::Land,
            RouteType::Maritime,
            RouteType::River,
            RouteType::Mixed,
        ];
        for rt in &types {
            let json = serde_json::to_string(rt).unwrap();
            let back: RouteType = serde_json::from_str(&json).unwrap();
            assert_eq!(rt, &back);
        }
    }
}
