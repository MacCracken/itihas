//! Calendar system metadata.
//!
//! Provides [`CalendarSystem`] structs with metadata about historical and
//! modern calendar systems. This module provides descriptive metadata only —
//! actual calendar computation belongs in **sankhya**.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Classification of calendar systems by astronomical basis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CalendarType {
    /// Based on the Earth's orbit around the Sun (~365.25 days/year).
    Solar,
    /// Based on lunar phases (~29.5 days/month, ~354 days/year).
    Lunar,
    /// Combines lunar months with solar year corrections (intercalary months).
    Lunisolar,
    /// Fixed mathematical cycle, not tied to astronomical observation.
    Fixed,
}

/// Metadata about a calendar system.
///
/// Describes the structure and rules of a calendar without performing
/// any date computation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarSystem {
    /// Name of the calendar system.
    pub name: Cow<'static, str>,
    /// Type classification.
    pub calendar_type: CalendarType,
    /// Epoch year in the calendar's own reckoning (e.g., 1 CE for Gregorian).
    pub epoch_year: i32,
    /// Number of months in a standard year.
    pub months: u8,
    /// Human-readable description of the leap rule.
    pub leap_rule_description: Cow<'static, str>,
}

/// Returns all pre-built calendar systems.
#[must_use]
#[inline]
pub fn all_calendars() -> Vec<CalendarSystem> {
    vec![
        CalendarSystem {
            name: Cow::Borrowed("Gregorian"),
            calendar_type: CalendarType::Solar,
            epoch_year: 1,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "Divisible by 4, except centuries unless divisible by 400",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Julian"),
            calendar_type: CalendarType::Solar,
            epoch_year: 1,
            months: 12,
            leap_rule_description: Cow::Borrowed("Every 4th year without exception"),
        },
        CalendarSystem {
            name: Cow::Borrowed("Islamic (Hijri)"),
            calendar_type: CalendarType::Lunar,
            epoch_year: 622,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "11 leap years in a 30-year cycle (years 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29)",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Hebrew"),
            calendar_type: CalendarType::Lunisolar,
            epoch_year: -3760,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "7 leap years in a 19-year Metonic cycle (years 3, 6, 8, 11, 14, 17, 19) add Adar II",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Chinese"),
            calendar_type: CalendarType::Lunisolar,
            epoch_year: -2697,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "7 intercalary months in 19-year cycle; leap month inserted when no major solar term falls within it",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Hindu (Vikram Samvat)"),
            calendar_type: CalendarType::Lunisolar,
            epoch_year: -57,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "Intercalary month (Adhik Maas) added when two new moons occur in the same solar month",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Maya (Long Count)"),
            calendar_type: CalendarType::Fixed,
            epoch_year: -3114,
            months: 18,
            leap_rule_description: Cow::Borrowed(
                "No leap correction; 18 months of 20 days plus 5-day Wayeb period = 365 days exactly",
            ),
        },
        CalendarSystem {
            name: Cow::Borrowed("Egyptian (Civil)"),
            calendar_type: CalendarType::Solar,
            epoch_year: -2781,
            months: 12,
            leap_rule_description: Cow::Borrowed(
                "No leap days in civil calendar; 12 months of 30 days plus 5 epagomenal days = 365 days",
            ),
        },
    ]
}

/// Look up a calendar system by name (case-insensitive substring match).
#[must_use]
#[inline]
pub fn by_name(name: &str) -> Option<CalendarSystem> {
    tracing::debug!(name, "looking up calendar by name");
    let lower = name.to_lowercase();
    all_calendars()
        .into_iter()
        .find(|c| c.name.to_lowercase().contains(&lower))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_calendars_count() {
        assert_eq!(all_calendars().len(), 8);
    }

    #[test]
    fn test_by_name_gregorian() {
        let cal = by_name("gregorian").unwrap();
        assert_eq!(cal.name, "Gregorian");
        assert_eq!(cal.calendar_type, CalendarType::Solar);
    }

    #[test]
    fn test_by_name_hijri() {
        let cal = by_name("hijri").unwrap();
        assert_eq!(cal.name, "Islamic (Hijri)");
        assert_eq!(cal.calendar_type, CalendarType::Lunar);
    }

    #[test]
    fn test_by_name_maya() {
        let cal = by_name("maya").unwrap();
        assert_eq!(cal.calendar_type, CalendarType::Fixed);
        assert_eq!(cal.months, 18);
    }

    #[test]
    fn test_by_name_not_found() {
        assert!(by_name("Martian").is_none());
    }

    #[test]
    fn test_calendar_serde_roundtrip() {
        for cal in all_calendars() {
            let json = serde_json::to_string(&cal).unwrap();
            let back: CalendarSystem = serde_json::from_str(&json).unwrap();
            assert_eq!(cal, back);
        }
    }

    #[test]
    fn test_calendar_type_serde_roundtrip() {
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
}
