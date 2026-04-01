//! Error types for itihas.

use thiserror::Error;

/// Errors that can occur in historical data processing.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ItihasError {
    /// Era not found by name.
    #[error("unknown era: {0}")]
    UnknownEra(String),

    /// Civilization not found by name.
    #[error("unknown civilization: {0}")]
    UnknownCivilization(String),

    /// Calendar system not found by name.
    #[error("unknown calendar: {0}")]
    UnknownCalendar(String),

    /// Year value is out of supported range.
    #[error("invalid year: {0}")]
    InvalidYear(i32),

    /// Event not found by name.
    #[error("event not found: {0}")]
    EventNotFound(String),
}
