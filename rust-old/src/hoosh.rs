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

// ── LLM inference via hoosh (feature: `hoosh-llm`) ──────────────────────

/// Natural language historical queries via LLM inference.
///
/// Uses [`hoosh::HooshClient`] to parse free-form questions into structured
/// tool calls, then resolves them from itihas data. Falls back to pure LLM
/// generation when data is insufficient.
///
/// # Usage
///
/// ```rust,ignore
/// use itihas::hoosh::llm;
///
/// let client = hoosh::HooshClient::new("http://localhost:8080");
/// let response = llm::answer(&client, "What civilizations existed in 500 BCE?", "llama3").await?;
/// ```
#[cfg(feature = "hoosh-llm")]
pub mod llm {
    use hoosh::{HooshClient, InferenceRequest, ToolChoice, ToolDefinition};

    use super::*;

    /// Errors from LLM-assisted historical queries.
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum LlmError {
        /// Hoosh inference failed.
        #[error("inference error: {0}")]
        Inference(#[from] hoosh::HooshError),
        /// Could not parse tool call arguments.
        #[error("failed to parse tool arguments: {0}")]
        ParseArgs(String),
    }

    /// Answer a natural language historical question using LLM inference.
    ///
    /// Strategy:
    /// 1. Try to parse as a structured `HistoryQuery` via LLM tool calling
    /// 2. If the LLM picks a tool, resolve from itihas data
    /// 3. If no tool is called, return the LLM's direct text response
    ///
    /// # Errors
    ///
    /// Returns [`LlmError`] if the hoosh inference call fails.
    pub async fn answer(
        client: &HooshClient,
        question: &str,
        model: &str,
    ) -> Result<QueryResponse, LlmError> {
        tracing::info!(question, model, "answering via hoosh LLM");

        let request = InferenceRequest {
            model: model.to_string(),
            prompt: question.to_string(),
            system: Some(SYSTEM_PROMPT.to_string()),
            messages: vec![],
            max_tokens: Some(1024),
            temperature: Some(0.0),
            top_p: None,
            stream: false,
            tools: tool_defs(),
            tool_choice: Some(ToolChoice::Auto),
        };

        let response = client.infer(&request).await?;

        // If the LLM made a tool call, resolve it from data.
        if let Some(call) = response.tool_calls.first() {
            tracing::debug!(tool = %call.name, args = %call.arguments, "LLM selected tool");

            // era_lookup doesn't map to HistoryQuery — handle directly.
            if call.name == "era_lookup"
                && let Some(mut resp) = resolve_era_lookup(&call.arguments)
            {
                resp.source = ResponseSource::Hybrid;
                return Ok(resp);
            }

            if let Some(query) = parse_tool_call(&call.name, &call.arguments)
                && let Some(mut resp) = answer_from_data(&query)
            {
                resp.source = ResponseSource::Hybrid;
                return Ok(resp);
            }
        }

        // No tool call or data insufficient — return LLM text directly.
        Ok(QueryResponse {
            query_id: None,
            source: ResponseSource::LlmGenerated,
            content: response.text,
            confidence: None,
            structured_data: None,
        })
    }

    const SYSTEM_PROMPT: &str = "\
You are a historical data assistant. When the user asks a historical question, \
use the available tools to look up the answer from structured data. \
Use events_in_range for date-based queries, civilizations_at for \
\"what existed when\" questions, causes_of for causal questions, \
interactions_between for relationship queries, figure_lookup for \
people, and era_lookup for period queries. \
Only respond directly if no tool fits the question.";

    /// Build hoosh tool definitions for itihas lookups.
    #[must_use]
    fn tool_defs() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "events_in_range".into(),
                description: "Find historical events in a year range (negative = BCE)".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "start": {"type": "integer", "description": "Start year"},
                        "end": {"type": "integer", "description": "End year"}
                    },
                    "required": ["start", "end"]
                }),
            },
            ToolDefinition {
                name: "civilizations_at".into(),
                description: "Find civilizations active at a given year (negative = BCE)".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "year": {"type": "integer", "description": "Year to query"}
                    },
                    "required": ["year"]
                }),
            },
            ToolDefinition {
                name: "causes_of".into(),
                description: "Find causal factors of a named historical event".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "event": {"type": "string", "description": "Event name"}
                    },
                    "required": ["event"]
                }),
            },
            ToolDefinition {
                name: "interactions_between".into(),
                description: "Find interactions between two civilizations".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "civ_a": {"type": "string", "description": "First civilization"},
                        "civ_b": {"type": "string", "description": "Second civilization"}
                    },
                    "required": ["civ_a", "civ_b"]
                }),
            },
            ToolDefinition {
                name: "figure_lookup".into(),
                description: "Look up a historical figure by name".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "description": "Figure name"}
                    },
                    "required": ["name"]
                }),
            },
            ToolDefinition {
                name: "era_lookup".into(),
                description: "Look up a historical era by name or find eras containing a year"
                    .into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "description": "Era name"},
                        "year": {"type": "integer", "description": "Year to find containing eras"}
                    }
                }),
            },
        ]
    }

    /// Parse an LLM tool call into a [`HistoryQuery`].
    fn parse_tool_call(name: &str, args: &serde_json::Value) -> Option<HistoryQuery> {
        match name {
            "events_in_range" => {
                let start = args.get("start")?.as_i64()? as i32;
                let end = args.get("end")?.as_i64()? as i32;
                Some(HistoryQuery::EventsInRange { start, end })
            }
            "civilizations_at" => {
                let year = args.get("year")?.as_i64()? as i32;
                Some(HistoryQuery::CivilizationsAt { year })
            }
            "causes_of" => {
                let event = args.get("event")?.as_str()?.to_string();
                Some(HistoryQuery::CausesOf { event })
            }
            "interactions_between" => {
                let civ_a = args.get("civ_a")?.as_str()?.to_string();
                let civ_b = args.get("civ_b")?.as_str()?.to_string();
                Some(HistoryQuery::InteractionsBetween { civ_a, civ_b })
            }
            "figure_lookup" => {
                let name = args.get("name")?.as_str()?.to_string();
                Some(HistoryQuery::FigureLookup { name })
            }
            // era_lookup doesn't map to a HistoryQuery variant;
            // resolved via resolve_era_lookup() instead.
            "era_lookup" => None,
            _ => None,
        }
    }

    /// Resolve an era lookup directly (not covered by [`HistoryQuery`]).
    ///
    /// This handles the `era_lookup` tool call which doesn't map to a
    /// `HistoryQuery` variant.
    #[must_use]
    pub fn resolve_era_lookup(args: &serde_json::Value) -> Option<QueryResponse> {
        if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
            let era = crate::era::by_name(name).ok()?;
            let data = serde_json::to_value(&era).ok()?;
            return Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: format!("{} ({} – {})", era.name, era.start_year, era.end_year),
                confidence: Some(1.0),
                structured_data: Some(data),
            });
        }
        if let Some(year) = args.get("year").and_then(|v| v.as_i64()) {
            let eras = crate::era::eras_containing(year as i32);
            if eras.is_empty() {
                return Some(QueryResponse {
                    query_id: None,
                    source: ResponseSource::ItihasData,
                    content: String::from("No eras found containing that year."),
                    confidence: Some(1.0),
                    structured_data: Some(serde_json::Value::Array(vec![])),
                });
            }
            let data = serde_json::to_value(&eras).ok()?;
            let names: Vec<&str> = eras.iter().map(|e| e.name.as_ref()).collect();
            return Some(QueryResponse {
                query_id: None,
                source: ResponseSource::ItihasData,
                content: names.join(", "),
                confidence: Some(1.0),
                structured_data: Some(data),
            });
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tool_defs_count() {
            assert_eq!(tool_defs().len(), 6);
        }

        #[test]
        fn test_tool_defs_have_schemas() {
            for td in tool_defs() {
                assert!(!td.name.is_empty());
                assert!(!td.description.is_empty());
                assert!(td.parameters.get("type").is_some());
            }
        }

        #[test]
        fn test_parse_events_in_range() {
            let q = parse_tool_call(
                "events_in_range",
                &serde_json::json!({"start": -500, "end": 500}),
            );
            assert!(matches!(
                q,
                Some(HistoryQuery::EventsInRange {
                    start: -500,
                    end: 500
                })
            ));
        }

        #[test]
        fn test_parse_civilizations_at() {
            let q = parse_tool_call("civilizations_at", &serde_json::json!({"year": -500}));
            assert!(matches!(
                q,
                Some(HistoryQuery::CivilizationsAt { year: -500 })
            ));
        }

        #[test]
        fn test_parse_causes_of() {
            let q = parse_tool_call(
                "causes_of",
                &serde_json::json!({"event": "French Revolution"}),
            );
            assert!(matches!(q, Some(HistoryQuery::CausesOf { .. })));
        }

        #[test]
        fn test_parse_interactions_between() {
            let q = parse_tool_call(
                "interactions_between",
                &serde_json::json!({"civ_a": "Rome", "civ_b": "Egypt"}),
            );
            assert!(matches!(q, Some(HistoryQuery::InteractionsBetween { .. })));
        }

        #[test]
        fn test_parse_figure_lookup() {
            let q = parse_tool_call("figure_lookup", &serde_json::json!({"name": "Aristotle"}));
            assert!(matches!(q, Some(HistoryQuery::FigureLookup { .. })));
        }

        #[test]
        fn test_parse_unknown_tool() {
            assert!(parse_tool_call("unknown", &serde_json::json!({})).is_none());
        }

        #[test]
        fn test_parse_missing_args() {
            assert!(parse_tool_call("events_in_range", &serde_json::json!({})).is_none());
            assert!(parse_tool_call("civilizations_at", &serde_json::json!({})).is_none());
        }

        #[test]
        fn test_resolve_era_by_name() {
            let resp = resolve_era_lookup(&serde_json::json!({"name": "Bronze Age"}));
            assert!(resp.is_some());
            let resp = resp.unwrap();
            assert!(resp.content.contains("Bronze Age"));
        }

        #[test]
        fn test_resolve_era_by_year() {
            let resp = resolve_era_lookup(&serde_json::json!({"year": -500}));
            assert!(resp.is_some());
        }

        #[test]
        fn test_resolve_era_unknown() {
            let resp = resolve_era_lookup(&serde_json::json!({"name": "Space Age"}));
            assert!(resp.is_none());
        }
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
