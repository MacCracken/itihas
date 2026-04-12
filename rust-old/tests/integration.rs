//! Integration tests for itihas — cross-module behavior.

use itihas::calendar::{CalendarSystem, CalendarType};
use itihas::campaign::{self, Campaign, CampaignOutcome};
use itihas::causality;
use itihas::civilization::{self, Civilization};
use itihas::era::{self, Era, EraCategory, EraScope};
use itihas::event::{self, Event, EventCategory, EventSignificance};
use itihas::figure::{self, Figure, FigureDomain};
use itihas::interaction;
use itihas::site::{self, Site, SiteType};
use itihas::trade::{self, RouteType, TradeRoute};

// ---------------------------------------------------------------------------
// Serde roundtrips — all types
// ---------------------------------------------------------------------------

#[test]
fn test_all_eras_serde_roundtrip() {
    let eras = era::all_eras();
    for era in eras.iter() {
        let json = serde_json::to_string(era).unwrap();
        let back: Era = serde_json::from_str(&json).unwrap();
        assert_eq!(era, &back);
    }
}

#[test]
fn test_all_civilizations_serde_roundtrip() {
    let civs = civilization::all_civilizations();
    for civ in civs.iter() {
        let json = serde_json::to_string(civ).unwrap();
        let back: Civilization = serde_json::from_str(&json).unwrap();
        assert_eq!(civ, &back);
    }
}

#[test]
fn test_all_events_serde_roundtrip() {
    let events = event::all_events();
    for event in events.iter() {
        let json = serde_json::to_string(event).unwrap();
        let back: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(event, &back);
    }
}

#[test]
fn test_all_calendars_serde_roundtrip() {
    let cals = itihas::calendar::all_calendars();
    for cal in cals.iter() {
        let json = serde_json::to_string(cal).unwrap();
        let back: CalendarSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(cal, &back);
    }
}

#[test]
fn test_all_figures_serde_roundtrip() {
    let figs = figure::all_figures();
    for fig in figs.iter() {
        let json = serde_json::to_string(fig).unwrap();
        let back: Figure = serde_json::from_str(&json).unwrap();
        assert_eq!(fig, &back);
    }
}

// ---------------------------------------------------------------------------
// Enum serde roundtrips
// ---------------------------------------------------------------------------

#[test]
fn test_era_category_all_variants_roundtrip() {
    let cats = [
        EraCategory::Ancient,
        EraCategory::Classical,
        EraCategory::Medieval,
        EraCategory::EarlyModern,
        EraCategory::Modern,
        EraCategory::Contemporary,
    ];
    for cat in &cats {
        let json = serde_json::to_string(cat).unwrap();
        let back: EraCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(*cat, back);
    }
}

#[test]
fn test_era_scope_all_variants_roundtrip() {
    let scopes = [EraScope::Global, EraScope::Regional];
    for s in &scopes {
        let json = serde_json::to_string(s).unwrap();
        let back: EraScope = serde_json::from_str(&json).unwrap();
        assert_eq!(*s, back);
    }
}

#[test]
fn test_event_category_all_variants_roundtrip() {
    let cats = [
        EventCategory::War,
        EventCategory::Treaty,
        EventCategory::Discovery,
        EventCategory::Invention,
        EventCategory::Revolution,
        EventCategory::Migration,
        EventCategory::Founding,
        EventCategory::Collapse,
    ];
    for cat in &cats {
        let json = serde_json::to_string(cat).unwrap();
        let back: EventCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(*cat, back);
    }
}

#[test]
fn test_calendar_type_all_variants_roundtrip() {
    let types = [
        CalendarType::Solar,
        CalendarType::Lunar,
        CalendarType::Lunisolar,
        CalendarType::Fixed,
    ];
    for t in &types {
        let json = serde_json::to_string(t).unwrap();
        let back: CalendarType = serde_json::from_str(&json).unwrap();
        assert_eq!(*t, back);
    }
}

#[test]
fn test_event_significance_all_variants_roundtrip() {
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
fn test_figure_domain_all_variants_roundtrip() {
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

// ---------------------------------------------------------------------------
// Cross-module lookups
// ---------------------------------------------------------------------------

#[test]
fn test_eras_and_civilizations_at_500_bce() {
    let eras = era::eras_containing(-500);
    let civs = civilization::active_at(-500);

    // Both should return results for this well-known date
    assert!(!eras.is_empty(), "no eras found for 500 BCE");
    assert!(!civs.is_empty(), "no civilizations found for 500 BCE");

    // Classical Antiquity should be an active era
    assert!(eras.iter().any(|e| e.name == "Classical Antiquity"));

    // Greece and Persia should both be active
    assert!(civs.iter().any(|c| c.name == "Ancient Greece"));
    assert!(civs.iter().any(|c| c.name == "Persian Empire"));
}

#[test]
fn test_events_reference_known_eras() {
    let era_names: Vec<_> = era::all_eras().iter().map(|e| e.name.clone()).collect();
    for event in event::all_events() {
        assert!(
            era_names.contains(&event.era),
            "event '{}' references unknown era '{}'",
            event.name,
            event.era
        );
    }
}

#[test]
fn test_events_civilizations_nonempty() {
    for event in event::all_events() {
        for civ in &event.civilizations_involved {
            assert!(
                !civ.is_empty(),
                "event '{}' has empty civilization reference",
                event.name,
            );
        }
    }
}

#[test]
fn test_events_civilizations_not_era_names() {
    let era_names: Vec<_> = era::all_eras().iter().map(|e| e.name.clone()).collect();
    for event in event::all_events() {
        for civ in &event.civilizations_involved {
            assert!(
                !era_names.contains(civ),
                "event '{}' uses era name '{}' as civilization",
                event.name,
                civ,
            );
        }
    }
}

#[test]
fn test_figures_have_nonempty_civilization() {
    for fig in figure::all_figures() {
        assert!(
            !fig.civilization.is_empty(),
            "figure '{}' has empty civilization field",
            fig.name
        );
    }
}

#[test]
fn test_figures_civilization_is_not_era_name() {
    let era_names: Vec<_> = era::all_eras().iter().map(|e| e.name.clone()).collect();

    for fig in figure::all_figures() {
        assert!(
            !era_names.contains(&fig.civilization),
            "figure '{}' uses era name '{}' as civilization — should be a political entity",
            fig.name,
            fig.civilization
        );
    }
}

// ---------------------------------------------------------------------------
// Error display
// ---------------------------------------------------------------------------

#[test]
fn test_error_display() {
    let err = itihas::ItihasError::UnknownEra("FutureAge".into());
    assert_eq!(err.to_string(), "unknown era: FutureAge");

    let err = itihas::ItihasError::UnknownCivilization("Atlantis".into());
    assert_eq!(err.to_string(), "unknown civilization: Atlantis");
}

#[test]
fn test_all_error_variants_display() {
    let cases: Vec<(itihas::ItihasError, &str)> = vec![
        (
            itihas::ItihasError::UnknownEra("X".into()),
            "unknown era: X",
        ),
        (
            itihas::ItihasError::UnknownCivilization("Y".into()),
            "unknown civilization: Y",
        ),
        (
            itihas::ItihasError::UnknownCalendar("Z".into()),
            "unknown calendar: Z",
        ),
        (
            itihas::ItihasError::InvalidYear(999_999),
            "invalid year: 999999",
        ),
        (
            itihas::ItihasError::EventNotFound("W".into()),
            "event not found: W",
        ),
        (
            itihas::ItihasError::FigureNotFound("V".into()),
            "figure not found: V",
        ),
    ];
    for (err, expected) in cases {
        assert_eq!(err.to_string(), expected);
    }
}

// ---------------------------------------------------------------------------
// Data integrity
// ---------------------------------------------------------------------------

#[test]
fn test_era_date_ordering() {
    for era in era::all_eras() {
        assert!(
            era.start_year < era.end_year,
            "era '{}' has start_year >= end_year",
            era.name
        );
    }
}

#[test]
fn test_civilization_date_ordering() {
    for civ in civilization::all_civilizations() {
        assert!(
            civ.founding_year < civ.end_year,
            "civilization '{}' has founding_year >= end_year",
            civ.name
        );
    }
}

#[test]
fn test_figure_birth_before_death() {
    for fig in figure::all_figures() {
        if let (Some(birth), Some(death)) = (fig.birth_year, fig.death_year) {
            assert!(
                birth < death,
                "figure '{}' has birth_year >= death_year",
                fig.name
            );
        }
    }
}

#[test]
fn test_calendar_months_positive() {
    for cal in itihas::calendar::all_calendars() {
        assert!(cal.months > 0, "calendar '{}' has 0 months", cal.name);
    }
}

#[test]
fn test_civilization_peak_era_is_known() {
    let era_names: Vec<_> = era::all_eras().iter().map(|e| e.name.clone()).collect();
    for civ in civilization::all_civilizations() {
        assert!(
            era_names.contains(&civ.peak_era),
            "civilization '{}' references unknown peak_era '{}'",
            civ.name,
            civ.peak_era
        );
    }
}

#[test]
fn test_event_year_within_era_range() {
    for event in event::all_events() {
        let era = era::by_name(&event.era).unwrap_or_else(|_| {
            panic!(
                "event '{}' references unknown era '{}'",
                event.name, event.era
            )
        });
        assert!(
            event.year >= era.start_year && event.year <= era.end_year,
            "event '{}' (year {}) falls outside era '{}' ({} – {})",
            event.name,
            event.year,
            era.name,
            era.start_year,
            era.end_year
        );
    }
}

// ---------------------------------------------------------------------------
// Query function integration tests
// ---------------------------------------------------------------------------

#[test]
fn test_by_region_returns_subset() {
    let all = civilization::all_civilizations();
    let near_east = civilization::by_region("Near East");
    assert!(!near_east.is_empty());
    assert!(near_east.len() < all.len());
    for civ in &near_east {
        assert!(
            civ.region.to_lowercase().contains("near east"),
            "civilization '{}' region '{}' does not contain 'near east'",
            civ.name,
            civ.region
        );
    }
}

#[test]
fn test_active_at_boundary_years() {
    // Founding year should be inclusive
    let civs = civilization::active_at(-3500);
    assert!(civs.iter().any(|c| c.name == "Mesopotamia"));

    // End year should be inclusive
    let civs = civilization::active_at(476);
    assert!(civs.iter().any(|c| c.name == "Roman Empire"));
}

#[test]
fn test_events_by_category_returns_correct_category() {
    let wars = event::by_category(&EventCategory::War);
    assert!(!wars.is_empty());
    for w in &wars {
        assert_eq!(w.category, EventCategory::War);
    }
}

#[test]
fn test_events_at_year_returns_correct_year() {
    let events = event::at_year(-753);
    assert!(!events.is_empty());
    for e in &events {
        assert_eq!(e.year, -753);
    }
}

#[test]
fn test_events_between_classical_antiquity() {
    let events = event::events_between(-800, 476);
    assert!(!events.is_empty());
    for e in &events {
        assert!(e.year >= -800 && e.year <= 476);
    }
    // Verify chronological order
    for w in events.windows(2) {
        assert!(w[0].year <= w[1].year);
    }
}

#[test]
fn test_calendar_by_name_found_and_not_found() {
    assert!(itihas::calendar::by_name("gregorian").is_ok());
    assert!(itihas::calendar::by_name("Martian").is_err());
}

#[test]
fn test_figures_by_domain_returns_correct_domain() {
    let rulers = figure::by_domain(&FigureDomain::Ruler);
    assert!(!rulers.is_empty());
    for r in &rulers {
        assert_eq!(r.domain, FigureDomain::Ruler);
    }
}

// ---------------------------------------------------------------------------
// by_name lookups (Result-returning)
// ---------------------------------------------------------------------------

#[test]
fn test_era_by_name_found() {
    let era = era::by_name("bronze age").unwrap();
    assert_eq!(era.name, "Bronze Age");
}

#[test]
fn test_era_by_name_not_found() {
    assert!(era::by_name("Space Age").is_err());
}

#[test]
fn test_civilization_by_name_found() {
    let civ = civilization::by_name("ancient greece").unwrap();
    assert_eq!(civ.name, "Ancient Greece");
}

#[test]
fn test_civilization_by_name_not_found() {
    assert!(civilization::by_name("Atlantis").is_err());
}

#[test]
fn test_event_by_name_found() {
    let event = event::by_name("fall of constantinople").unwrap();
    assert_eq!(event.year, 1453);
}

#[test]
fn test_event_by_name_not_found() {
    assert!(event::by_name("Battle of Endor").is_err());
}

#[test]
fn test_figure_by_name_found() {
    let fig = figure::by_name("aristotle").unwrap();
    assert_eq!(fig.domain, FigureDomain::Philosopher);
}

#[test]
fn test_figure_by_name_not_found() {
    assert!(figure::by_name("Gandalf").is_err());
}

// ---------------------------------------------------------------------------
// Display impls
// ---------------------------------------------------------------------------

#[test]
fn test_era_display() {
    let era = era::by_name("Bronze Age").unwrap();
    let s = era.to_string();
    assert!(s.contains("Bronze Age"));
    assert!(s.contains("-3500"));

    // Ongoing era should display "present" instead of i32::MAX
    let info = era::by_name("Information Age").unwrap();
    let s = info.to_string();
    assert!(s.contains("present"));
    assert!(!s.contains("2147483647"));
}

#[test]
fn test_civilization_display() {
    let civ = civilization::by_name("Roman Empire").unwrap();
    let s = civ.to_string();
    assert!(s.contains("Roman Empire"));
    assert!(s.contains("Mediterranean"));
}

#[test]
fn test_event_display() {
    let event = event::by_name("French Revolution").unwrap();
    let s = event.to_string();
    assert!(s.contains("French Revolution"));
    assert!(s.contains("1789"));
}

#[test]
fn test_figure_display() {
    let fig = figure::by_name("Aristotle").unwrap();
    let s = fig.to_string();
    assert!(s.contains("Aristotle"));
    assert!(s.contains("-384"));
}

#[test]
fn test_calendar_display() {
    let cal = itihas::calendar::by_name("gregorian").unwrap();
    let s = cal.to_string();
    assert!(s.contains("Gregorian"));
    assert!(s.contains("Solar"));
}

// ---------------------------------------------------------------------------
// Ord impls
// ---------------------------------------------------------------------------

#[test]
fn test_eras_sort_chronologically() {
    let mut eras = era::all_eras().to_vec();
    eras.sort();
    for w in eras.windows(2) {
        assert!(w[0].start_year <= w[1].start_year);
    }
}

#[test]
fn test_events_sort_chronologically() {
    let mut events = event::all_events().to_vec();
    events.sort();
    for w in events.windows(2) {
        assert!(w[0].year <= w[1].year);
    }
}

// ---------------------------------------------------------------------------
// EraScope tests
// ---------------------------------------------------------------------------

#[test]
fn test_by_scope_partitions_all_eras() {
    let all = era::all_eras();
    let global = era::by_scope(&EraScope::Global);
    let regional = era::by_scope(&EraScope::Regional);
    assert!(!global.is_empty());
    assert!(!regional.is_empty());
    assert_eq!(global.len() + regional.len(), all.len());
    for e in &global {
        assert_eq!(e.scope, EraScope::Global);
    }
    for e in &regional {
        assert_eq!(e.scope, EraScope::Regional);
    }
}

#[test]
fn test_regional_eras_have_specific_regions() {
    let regional = era::by_scope(&EraScope::Regional);
    for e in &regional {
        assert!(
            e.region != "Global",
            "regional era '{}' has region 'Global'",
            e.name
        );
    }
}

#[test]
fn test_era_by_region_east_asia() {
    let east_asia = era::by_region("East Asia");
    assert!(!east_asia.is_empty());
    // Should include Chinese dynasties
    assert!(east_asia.iter().any(|e| e.name == "Tang Dynasty"));
}

#[test]
fn test_era_by_region_south_asia() {
    let south_asia = era::by_region("South Asia");
    assert!(!south_asia.is_empty());
    assert!(south_asia.iter().any(|e| e.name == "Vedic Period"));
}

#[test]
fn test_era_by_region_mesoamerica() {
    let meso = era::by_region("Mesoamerica");
    assert_eq!(meso.len(), 3);
    assert!(meso.iter().any(|e| e.name == "Mesoamerican Classic"));
}

// ---------------------------------------------------------------------------
// Causality tests
// ---------------------------------------------------------------------------

#[test]
fn test_causality_references_known_events() {
    let event_names: Vec<_> = event::all_events().iter().map(|e| e.name.clone()).collect();
    for c in causality::all_causalities() {
        assert!(
            event_names.contains(&c.cause),
            "causality references unknown cause event '{}'",
            c.cause
        );
        assert!(
            event_names.contains(&c.effect),
            "causality references unknown effect event '{}'",
            c.effect
        );
    }
}

#[test]
fn test_causality_cause_precedes_effect() {
    for c in causality::all_causalities() {
        let cause_event = event::by_name(&c.cause).unwrap();
        let effect_event = event::by_name(&c.effect).unwrap();
        assert!(
            cause_event.year <= effect_event.year,
            "cause '{}' (year {}) occurs after effect '{}' (year {})",
            c.cause,
            cause_event.year,
            c.effect,
            effect_event.year
        );
    }
}

#[test]
fn test_chain_follows_correct_depth() {
    let ch = causality::chain("Invention of Writing", 2);
    for (_, depth) in &ch {
        assert!(*depth <= 2);
    }
}

// ---------------------------------------------------------------------------
// Interaction tests
// ---------------------------------------------------------------------------

#[test]
fn test_interaction_civs_are_known() {
    let civ_names: Vec<_> = civilization::all_civilizations()
        .iter()
        .map(|c| c.name.clone())
        .collect();
    for i in interaction::all_interactions() {
        assert!(
            civ_names.contains(&i.civ_a),
            "interaction references unknown civ_a '{}'",
            i.civ_a
        );
        assert!(
            civ_names.contains(&i.civ_b),
            "interaction references unknown civ_b '{}'",
            i.civ_b
        );
    }
}

#[test]
fn test_interaction_neighbors_are_known_civs() {
    let civ_names: Vec<_> = civilization::all_civilizations()
        .iter()
        .map(|c| c.name.clone())
        .collect();
    let rome_neighbors = interaction::neighbors("Roman Empire");
    for n in &rome_neighbors {
        assert!(
            civ_names.contains(n),
            "neighbor '{}' is not a known civilization",
            n
        );
    }
}

#[test]
fn test_influence_score_symmetric() {
    let ab = interaction::influence_score("Ancient Egypt", "Hittite Empire");
    let ba = interaction::influence_score("Hittite Empire", "Ancient Egypt");
    assert_eq!(ab, ba);
}

#[test]
fn test_region_proximity_symmetric() {
    let ab = interaction::region_proximity("Roman Empire", "Ancient Greece");
    let ba = interaction::region_proximity("Ancient Greece", "Roman Empire");
    assert_eq!(ab, ba);
}

// ---------------------------------------------------------------------------
// Site tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_sites_serde_roundtrip() {
    for site in site::all_sites().iter() {
        let json = serde_json::to_string(site).unwrap();
        let back: Site = serde_json::from_str(&json).unwrap();
        assert_eq!(site, &back);
    }
}

#[test]
fn test_site_date_ordering() {
    for site in site::all_sites() {
        assert!(
            site.start_year <= site.end_year,
            "site '{}' has start_year ({}) > end_year ({})",
            site.name,
            site.start_year,
            site.end_year
        );
    }
}

#[test]
fn test_site_by_type_returns_correct_type() {
    let temples = site::by_type(&SiteType::Temple);
    assert!(!temples.is_empty());
    for s in &temples {
        assert_eq!(s.site_type, SiteType::Temple);
    }
}

#[test]
fn test_site_type_all_variants_roundtrip() {
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

#[test]
fn test_site_by_name_found() {
    let site = site::by_name("pompeii").unwrap();
    assert_eq!(site.name, "Pompeii");
}

#[test]
fn test_site_by_name_not_found() {
    assert!(site::by_name("Atlantis").is_err());
}

#[test]
fn test_site_display() {
    let s = site::by_name("Giza").unwrap();
    let display = s.to_string();
    assert!(display.contains("Giza"));
    assert!(display.contains("Egypt"));
}

// ---------------------------------------------------------------------------
// Trade route tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_routes_serde_roundtrip() {
    for route in trade::all_routes().iter() {
        let json = serde_json::to_string(route).unwrap();
        let back: TradeRoute = serde_json::from_str(&json).unwrap();
        assert_eq!(route, &back);
    }
}

#[test]
fn test_route_date_ordering() {
    for route in trade::all_routes() {
        assert!(
            route.start_year <= route.end_year,
            "route '{}' has start_year ({}) > end_year ({})",
            route.name,
            route.start_year,
            route.end_year
        );
    }
}

#[test]
fn test_route_has_regions() {
    for route in trade::all_routes() {
        assert!(
            !route.regions.is_empty(),
            "route '{}' has no regions",
            route.name
        );
    }
}

#[test]
fn test_route_has_commodities() {
    for route in trade::all_routes() {
        assert!(
            !route.commodities.is_empty(),
            "route '{}' has no commodities",
            route.name
        );
    }
}

#[test]
fn test_route_by_type_returns_correct_type() {
    let maritime = trade::by_type(&RouteType::Maritime);
    assert!(!maritime.is_empty());
    for r in &maritime {
        assert_eq!(r.route_type, RouteType::Maritime);
    }
}

#[test]
fn test_route_type_all_variants_roundtrip() {
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

#[test]
fn test_route_by_name_found() {
    let route = trade::by_name("silk road").unwrap();
    assert_eq!(route.name, "Silk Road");
}

#[test]
fn test_route_by_name_not_found() {
    assert!(trade::by_name("Route 66").is_err());
}

#[test]
fn test_route_by_commodity_returns_matching() {
    let silk_routes = trade::by_commodity("silk");
    assert!(!silk_routes.is_empty());
    for r in &silk_routes {
        assert!(
            r.commodities
                .iter()
                .any(|c| c.to_lowercase().contains("silk")),
            "route '{}' does not trade silk",
            r.name
        );
    }
}

#[test]
fn test_route_display() {
    let r = trade::by_name("Silk Road").unwrap();
    let display = r.to_string();
    assert!(display.contains("Silk Road"));
    assert!(display.contains("Land"));
}

// ---------------------------------------------------------------------------
// Campaign tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_campaigns_serde_roundtrip() {
    for campaign in campaign::all_campaigns().iter() {
        let json = serde_json::to_string(campaign).unwrap();
        let back: Campaign = serde_json::from_str(&json).unwrap();
        assert_eq!(campaign, &back);
    }
}

#[test]
fn test_campaign_date_ordering() {
    for campaign in campaign::all_campaigns() {
        assert!(
            campaign.start_year <= campaign.end_year,
            "campaign '{}' has start_year ({}) > end_year ({})",
            campaign.name,
            campaign.start_year,
            campaign.end_year
        );
    }
}

#[test]
fn test_campaign_battles_within_dates() {
    for campaign in campaign::all_campaigns() {
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
fn test_campaign_has_belligerents() {
    for campaign in campaign::all_campaigns() {
        assert!(
            !campaign.belligerents_a.is_empty(),
            "campaign '{}' has no belligerents_a",
            campaign.name
        );
        assert!(
            !campaign.belligerents_b.is_empty(),
            "campaign '{}' has no belligerents_b",
            campaign.name
        );
    }
}

#[test]
fn test_campaign_has_commanders() {
    for campaign in campaign::all_campaigns() {
        assert!(
            !campaign.commanders.is_empty(),
            "campaign '{}' has no commanders",
            campaign.name
        );
    }
}

#[test]
fn test_campaign_has_battles() {
    for campaign in campaign::all_campaigns() {
        assert!(
            !campaign.battles.is_empty(),
            "campaign '{}' has no battles",
            campaign.name
        );
    }
}

#[test]
fn test_campaign_by_outcome_returns_correct_outcome() {
    let victories = campaign::by_outcome(&CampaignOutcome::Victory);
    assert!(!victories.is_empty());
    for c in &victories {
        assert_eq!(c.outcome, CampaignOutcome::Victory);
    }
}

#[test]
fn test_campaign_outcome_all_variants_roundtrip() {
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
fn test_campaign_by_name_found() {
    let c = campaign::by_name("napoleonic wars").unwrap();
    assert_eq!(c.name, "Napoleonic Wars");
}

#[test]
fn test_campaign_by_name_not_found() {
    assert!(campaign::by_name("Star Wars").is_err());
}

#[test]
fn test_campaign_sort_chronologically() {
    let mut campaigns = campaign::all_campaigns().to_vec();
    campaigns.sort();
    for w in campaigns.windows(2) {
        assert!(w[0].start_year <= w[1].start_year);
    }
}

#[test]
fn test_campaign_display() {
    let c = campaign::by_name("Napoleonic Wars").unwrap();
    let display = c.to_string();
    assert!(display.contains("Napoleonic Wars"));
    assert!(display.contains("Defeat"));
}

#[test]
fn test_campaigns_between_overlapping() {
    let campaigns = campaign::campaigns_between(-500, -200);
    assert!(!campaigns.is_empty());
    for c in &campaigns {
        assert!(c.start_year <= -200 && c.end_year >= -500);
    }
}
