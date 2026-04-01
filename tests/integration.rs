//! Integration tests for itihas — cross-module behavior.

use itihas::calendar::{CalendarSystem, CalendarType};
use itihas::civilization::{self, Civilization};
use itihas::era::{self, Era, EraCategory, EraScope};
use itihas::event::{self, Event, EventCategory, EventSignificance};
use itihas::figure::{self, Figure, FigureDomain};

// ---------------------------------------------------------------------------
// Serde roundtrips — all types
// ---------------------------------------------------------------------------

#[test]
fn test_all_eras_serde_roundtrip() {
    for era in era::all_eras() {
        let json = serde_json::to_string(era).unwrap();
        let back: Era = serde_json::from_str(&json).unwrap();
        assert_eq!(*era, back);
    }
}

#[test]
fn test_all_civilizations_serde_roundtrip() {
    for civ in civilization::all_civilizations() {
        let json = serde_json::to_string(civ).unwrap();
        let back: Civilization = serde_json::from_str(&json).unwrap();
        assert_eq!(*civ, back);
    }
}

#[test]
fn test_all_events_serde_roundtrip() {
    for event in event::all_events() {
        let json = serde_json::to_string(event).unwrap();
        let back: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(*event, back);
    }
}

#[test]
fn test_all_calendars_serde_roundtrip() {
    for cal in itihas::calendar::all_calendars() {
        let json = serde_json::to_string(cal).unwrap();
        let back: CalendarSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(*cal, back);
    }
}

#[test]
fn test_all_figures_serde_roundtrip() {
    for fig in figure::all_figures() {
        let json = serde_json::to_string(fig).unwrap();
        let back: Figure = serde_json::from_str(&json).unwrap();
        assert_eq!(*fig, back);
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
