//! Integration tests for itihas — cross-module behavior.

use itihas::calendar::{CalendarSystem, CalendarType};
use itihas::civilization::{self, Civilization};
use itihas::era::{self, Era, EraCategory};
use itihas::event::{self, Event, EventCategory};
use itihas::figure::{self, Figure, FigureDomain};

// ---------------------------------------------------------------------------
// Serde roundtrips — all types
// ---------------------------------------------------------------------------

#[test]
fn test_all_eras_serde_roundtrip() {
    for era in era::all_eras() {
        let json = serde_json::to_string(&era).unwrap();
        let back: Era = serde_json::from_str(&json).unwrap();
        assert_eq!(era, back);
    }
}

#[test]
fn test_all_civilizations_serde_roundtrip() {
    for civ in civilization::all_civilizations() {
        let json = serde_json::to_string(&civ).unwrap();
        let back: Civilization = serde_json::from_str(&json).unwrap();
        assert_eq!(civ, back);
    }
}

#[test]
fn test_all_events_serde_roundtrip() {
    for event in event::all_events() {
        let json = serde_json::to_string(&event).unwrap();
        let back: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(event, back);
    }
}

#[test]
fn test_all_calendars_serde_roundtrip() {
    for cal in itihas::calendar::all_calendars() {
        let json = serde_json::to_string(&cal).unwrap();
        let back: CalendarSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(cal, back);
    }
}

#[test]
fn test_all_figures_serde_roundtrip() {
    for fig in figure::all_figures() {
        let json = serde_json::to_string(&fig).unwrap();
        let back: Figure = serde_json::from_str(&json).unwrap();
        assert_eq!(fig, back);
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
fn test_events_civilizations_are_known() {
    let civ_names: Vec<_> = civilization::all_civilizations()
        .iter()
        .map(|c| c.name.clone())
        .collect();
    for event in event::all_events() {
        for civ in &event.civilizations_involved {
            assert!(
                civ_names.contains(civ),
                "event '{}' references unknown civilization '{}'",
                event.name,
                civ
            );
        }
    }
}

#[test]
fn test_figures_reference_known_civilizations_or_eras() {
    let civ_names: Vec<_> = civilization::all_civilizations()
        .iter()
        .map(|c| c.name.clone())
        .collect();
    let era_names: Vec<_> = era::all_eras().iter().map(|e| e.name.clone()).collect();

    for fig in figure::all_figures() {
        let known = civ_names.contains(&fig.civilization) || era_names.contains(&fig.civilization);
        assert!(
            known,
            "figure '{}' references unknown civilization/era '{}'",
            fig.name, fig.civilization
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
        assert!(
            cal.months > 0,
            "calendar '{}' has 0 months",
            cal.name
        );
    }
}
