//! LLM-powered historical queries via Hoosh.
//!
//! Provides [`HistoryQuery`] variants for structured historical questions,
//! [`QueryResponse`] for results, and [`answer_from_data`] for resolving
//! queries directly from itihas data without LLM inference.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

/// A structured historical query that can be answered by data or LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HistoryQuery {
    /// Look up events in a year range.
    EventsInRange {
        /// Start year (negative = BCE).
        start: i32,
        /// End year (negative = BCE).
        end: i32,
    },
    /// Find civilizations active at a given year.
    CivilizationsAt {
        /// Year to query (negative = BCE).
        year: i32,
    },
    /// Explain what caused a named event.
    CausesOf {
        /// Event name.
        event: String,
    },
    /// Describe interactions between two civilizations.
    InteractionsBetween {
        /// First civilization name.
        civ_a: String,
        /// Second civilization name.
        civ_b: String,
    },
    /// Look up a historical figure by name.
    FigureLookup {
        /// Figure name.
        name: String,
    },
    /// Free-form historical question for LLM inference.
    FreeForm {
        /// The question text.
        question: String,
    },
}

/// How the response was produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ResponseSource {
    /// Answered entirely from itihas static data.
    ItihasData,
    /// Answered by an LLM via Hoosh.
    LlmGenerated,
    /// Combination of data lookup and LLM elaboration.
    Hybrid,
}

/// Response to a historical query.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryResponse {
    /// Optional query identifier for tracking.
    pub query_id: Option<Cow<'static, str>>,
    /// How this response was produced.
    pub source: ResponseSource,
    /// The response content.
    pub content: String,
    /// Confidence score (0.0–1.0), if applicable.
    pub confidence: Option<f64>,
    /// Structured data payload, if available.
    pub structured_data: Option<serde_json::Value>,
}

/// Build an [`EventsInRange`](HistoryQuery::EventsInRange) query.
#[must_use]
pub fn events_in_range(start: i32, end: i32) -> HistoryQuery {
    HistoryQuery::EventsInRange { start, end }
}

/// Build a [`CivilizationsAt`](HistoryQuery::CivilizationsAt) query.
#[must_use]
pub fn civilizations_at(year: i32) -> HistoryQuery {
    HistoryQuery::CivilizationsAt { year }
}

/// Build a [`CausesOf`](HistoryQuery::CausesOf) query.
#[must_use]
pub fn causes_of(event: impl Into<String>) -> HistoryQuery {
    HistoryQuery::CausesOf {
        event: event.into(),
    }
}

/// Build an [`InteractionsBetween`](HistoryQuery::InteractionsBetween) query.
#[must_use]
pub fn interactions_between(civ_a: impl Into<String>, civ_b: impl Into<String>) -> HistoryQuery {
    HistoryQuery::InteractionsBetween {
        civ_a: civ_a.into(),
        civ_b: civ_b.into(),
    }
}

/// Build a [`FigureLookup`](HistoryQuery::FigureLookup) query.
#[must_use]
pub fn figure_lookup(name: impl Into<String>) -> HistoryQuery {
    HistoryQuery::FigureLookup { name: name.into() }
}

/// Build a [`FreeForm`](HistoryQuery::FreeForm) query.
#[must_use]
pub fn free_form(question: impl Into<String>) -> HistoryQuery {
    HistoryQuery::FreeForm {
        question: question.into(),
    }
}

/// Attempt to answer a query using only itihas static data.
///
/// Returns `Some(QueryResponse)` if the query can be fully resolved from
/// built-in data, or `None` if LLM inference is needed (e.g., for free-form
/// questions or when data is insufficient).
#[must_use]
pub fn answer_from_data(query: &HistoryQuery) -> Option<QueryResponse> {
    tracing::debug!(?query, "attempting to answer from itihas data");
    match query {
        HistoryQuery::EventsInRange { start, end } => {
            let events = crate::event::events_between(*start, *end);
            if events.is_empty() {
                return Some(QueryResponse {
                    query_id: None,
                    source: ResponseSource::ItihasData,
                    content: String::from("No events found in the specified range."),
                    confidence: Some(1.0),
                    structured_data: Some(serde_json::Value::Array(vec![])),
                });
            }
            let data = serde_json::to_value(&events).ok()?;
            let names: Vec<&str> = events.iter().map(|e| e.name.as_ref()).collect();
            Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: names.join(", "),
                confidence: Some(1.0),
                structured_data: Some(data),
            })
        }
        HistoryQuery::CivilizationsAt { year } => {
            let civs = crate::civilization::active_at(*year);
            if civs.is_empty() {
                return Some(QueryResponse {
                    query_id: None,
                    source: ResponseSource::ItihasData,
                    content: String::from("No civilizations found active at that year."),
                    confidence: Some(1.0),
                    structured_data: Some(serde_json::Value::Array(vec![])),
                });
            }
            let data = serde_json::to_value(&civs).ok()?;
            let names: Vec<&str> = civs.iter().map(|c| c.name.as_ref()).collect();
            Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: names.join(", "),
                confidence: Some(1.0),
                structured_data: Some(data),
            })
        }
        HistoryQuery::CausesOf { event } => {
            let causes = crate::causality::causes_of(event);
            if causes.is_empty() {
                return None; // No data — needs LLM
            }
            let data = serde_json::to_value(&causes).ok()?;
            let descs: Vec<&str> = causes.iter().map(|c| c.description.as_ref()).collect();
            Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: descs.join("; "),
                confidence: Some(0.8),
                structured_data: Some(data),
            })
        }
        HistoryQuery::InteractionsBetween { civ_a, civ_b } => {
            let interactions = crate::interaction::between(civ_a, civ_b);
            if interactions.is_empty() {
                return None; // No data — needs LLM
            }
            let data = serde_json::to_value(&interactions).ok()?;
            let descs: Vec<&str> = interactions
                .iter()
                .map(|i| i.description.as_ref())
                .collect();
            Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: descs.join("; "),
                confidence: Some(0.9),
                structured_data: Some(data),
            })
        }
        HistoryQuery::FigureLookup { name } => {
            let fig = crate::figure::by_name(name).ok()?;
            let data = serde_json::to_value(&fig).ok()?;
            Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: fig.description.to_string(),
                confidence: Some(1.0),
                structured_data: Some(data),
            })
        }
        HistoryQuery::FreeForm { .. } => None, // Always needs LLM
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_in_range_returns_data() {
        let q = events_in_range(-500, 500);
        let resp = answer_from_data(&q).unwrap();
        assert_eq!(resp.source, ResponseSource::ItihasData);
        assert!(!resp.content.is_empty());
        assert!(resp.structured_data.is_some());
    }

    #[test]
    fn test_civilizations_at_returns_data() {
        let q = civilizations_at(-500);
        let resp = answer_from_data(&q).unwrap();
        assert!(resp.content.contains("Ancient Greece"));
    }

    #[test]
    fn test_causes_of_known_event() {
        let q = causes_of("French Revolution");
        let resp = answer_from_data(&q).unwrap();
        assert_eq!(resp.source, ResponseSource::ItihasData);
    }

    #[test]
    fn test_causes_of_unknown_returns_none() {
        let q = causes_of("Battle of Endor");
        assert!(answer_from_data(&q).is_none());
    }

    #[test]
    fn test_interactions_between_known() {
        let q = interactions_between("Ancient Egypt", "Hittite Empire");
        let resp = answer_from_data(&q).unwrap();
        assert!(!resp.content.is_empty());
    }

    #[test]
    fn test_figure_lookup_found() {
        let q = figure_lookup("Aristotle");
        let resp = answer_from_data(&q).unwrap();
        assert!(resp.content.contains("philosopher"));
    }

    #[test]
    fn test_figure_lookup_not_found() {
        let q = figure_lookup("Gandalf");
        assert!(answer_from_data(&q).is_none());
    }

    #[test]
    fn test_free_form_returns_none() {
        let q = free_form("Why did Rome fall?");
        assert!(answer_from_data(&q).is_none());
    }

    #[test]
    fn test_query_serde_roundtrip() {
        let queries = [
            events_in_range(-500, 500),
            civilizations_at(-500),
            causes_of("French Revolution"),
            interactions_between("Rome", "Egypt"),
            figure_lookup("Aristotle"),
            free_form("Why?"),
        ];
        for q in &queries {
            let json = serde_json::to_string(q).unwrap();
            let back: HistoryQuery = serde_json::from_str(&json).unwrap();
            let json2 = serde_json::to_string(&back).unwrap();
            assert_eq!(json, json2);
        }
    }

    #[test]
    fn test_response_source_serde_roundtrip() {
        let sources = [
            ResponseSource::ItihasData,
            ResponseSource::LlmGenerated,
            ResponseSource::Hybrid,
        ];
        for s in &sources {
            let json = serde_json::to_string(s).unwrap();
            let back: ResponseSource = serde_json::from_str(&json).unwrap();
            assert_eq!(*s, back);
        }
    }
}
