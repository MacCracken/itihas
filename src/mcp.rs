//! MCP tool definitions and handlers for itihas.
//!
//! Exposes historical data as Model Context Protocol tools for AI agents.
//! Tool handlers invoke core itihas modules and return structured JSON.
//!
//! # Usage
//!
//! ```rust,no_run
//! use bote::{Dispatcher, ToolRegistry};
//! use itihas::mcp;
//!
//! let mut registry = ToolRegistry::new();
//! for def in mcp::tool_definitions() {
//!     registry.register(def);
//! }
//! let mut dispatcher = Dispatcher::new(registry);
//! mcp::register_handlers(&mut dispatcher);
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use alloc::string::String;

use bote::{Dispatcher, ToolAnnotations, ToolDef, ToolSchema};
use serde::{Deserialize, Serialize};

use crate::{civilization, era, event, figure};

/// Result of invoking an MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ToolResult {
    /// Successful invocation with JSON payload.
    Success(serde_json::Value),
    /// Invocation failed with error message.
    Error(String),
}

/// Returns all itihas MCP tool definitions.
///
/// All itihas tools are read-only lookups into static historical data.
#[must_use]
pub fn tool_definitions() -> Vec<ToolDef> {
    vec![
        ToolDef::new(
            "itihas_era",
            "Look up historical eras by name, year, scope, or region",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("name".into(), serde_json::json!({"type": "string", "description": "Era name (case-insensitive)"})),
                    ("year".into(), serde_json::json!({"type": "integer", "description": "Year to find containing eras (negative = BCE)"})),
                    ("region".into(), serde_json::json!({"type": "string", "description": "Region substring filter"})),
                ]),
                vec![],
            ),
        ).with_annotations(ToolAnnotations::read_only()),
        ToolDef::new(
            "itihas_civilization",
            "Look up civilizations by name, region, or active year",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("name".into(), serde_json::json!({"type": "string", "description": "Civilization name (case-insensitive)"})),
                    ("year".into(), serde_json::json!({"type": "integer", "description": "Year to find active civilizations (negative = BCE)"})),
                    ("region".into(), serde_json::json!({"type": "string", "description": "Region substring filter"})),
                ]),
                vec![],
            ),
        ).with_annotations(ToolAnnotations::read_only()),
        ToolDef::new(
            "itihas_event",
            "Look up historical events by name, year range, or category",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("name".into(), serde_json::json!({"type": "string", "description": "Event name (case-insensitive)"})),
                    ("start_year".into(), serde_json::json!({"type": "integer", "description": "Start of year range (negative = BCE)"})),
                    ("end_year".into(), serde_json::json!({"type": "integer", "description": "End of year range (negative = BCE)"})),
                    ("category".into(), serde_json::json!({"type": "string", "description": "Category filter (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse)"})),
                ]),
                vec![],
            ),
        ).with_annotations(ToolAnnotations::read_only()),
        ToolDef::new(
            "itihas_figure",
            "Look up historical figures by name or domain",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("name".into(), serde_json::json!({"type": "string", "description": "Figure name (case-insensitive)"})),
                    ("domain".into(), serde_json::json!({"type": "string", "description": "Domain filter (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor)"})),
                ]),
                vec![],
            ),
        ).with_annotations(ToolAnnotations::read_only()),
        ToolDef::new(
            "itihas_timeline",
            "Get a timeline of events, eras, and civilizations for a year range",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("start_year".into(), serde_json::json!({"type": "integer", "description": "Start year (negative = BCE)"})),
                    ("end_year".into(), serde_json::json!({"type": "integer", "description": "End year (negative = BCE)"})),
                ]),
                vec!["start_year".into(), "end_year".into()],
            ),
        ).with_annotations(ToolAnnotations::read_only()),
    ]
}

/// Register all itihas tool handlers on a bote [`Dispatcher`].
///
/// Call this after registering the tool definitions from [`tool_definitions`].
pub fn register_handlers(dispatcher: &mut Dispatcher) {
    dispatcher.handle("itihas_era", Arc::new(handle_era));
    dispatcher.handle("itihas_civilization", Arc::new(handle_civilization));
    dispatcher.handle("itihas_event", Arc::new(handle_event));
    dispatcher.handle("itihas_figure", Arc::new(handle_figure));
    dispatcher.handle("itihas_timeline", Arc::new(handle_timeline));
}

/// Convenience: register tool definitions + handlers in one call.
pub fn register_all(dispatcher: &mut Dispatcher) {
    for def in tool_definitions() {
        // Use the inner registry via register_tool for dynamic registration,
        // but since we have &mut we can register on the underlying registry
        // before the dispatcher is shared. We re-register via handle().
        dispatcher.handle(
            def.name.clone(),
            Arc::new(match def.name.as_str() {
                "itihas_era" => handle_era as fn(serde_json::Value) -> serde_json::Value,
                "itihas_civilization" => handle_civilization,
                "itihas_event" => handle_event,
                "itihas_figure" => handle_figure,
                "itihas_timeline" => handle_timeline,
                _ => unreachable!(),
            }),
        );
    }
}

fn handle_era(params: serde_json::Value) -> serde_json::Value {
    tracing::debug!(?params, "itihas_era invoked");

    // Name lookup — returns single era.
    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        return match era::by_name(name) {
            Ok(e) => mcp_success(serde_json::to_value(&e).unwrap_or_default()),
            Err(e) => mcp_error(&e.to_string()),
        };
    }

    // Year lookup — returns eras containing that year.
    if let Some(year) = params.get("year").and_then(|v| v.as_i64()) {
        let results = era::eras_containing(year as i32);
        return mcp_success(serde_json::to_value(&results).unwrap_or_default());
    }

    // Region lookup.
    if let Some(region) = params.get("region").and_then(|v| v.as_str()) {
        let results = era::by_region(region);
        return mcp_success(serde_json::to_value(&results).unwrap_or_default());
    }

    // No filter — return all eras.
    let all = era::all_eras();
    mcp_success(serde_json::to_value(all).unwrap_or_default())
}

fn handle_civilization(params: serde_json::Value) -> serde_json::Value {
    tracing::debug!(?params, "itihas_civilization invoked");

    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        return match civilization::by_name(name) {
            Ok(c) => mcp_success(serde_json::to_value(&c).unwrap_or_default()),
            Err(e) => mcp_error(&e.to_string()),
        };
    }

    if let Some(year) = params.get("year").and_then(|v| v.as_i64()) {
        let results = civilization::active_at(year as i32);
        return mcp_success(serde_json::to_value(&results).unwrap_or_default());
    }

    if let Some(region) = params.get("region").and_then(|v| v.as_str()) {
        let results = civilization::by_region(region);
        return mcp_success(serde_json::to_value(&results).unwrap_or_default());
    }

    let all = civilization::all_civilizations();
    mcp_success(serde_json::to_value(all).unwrap_or_default())
}

fn handle_event(params: serde_json::Value) -> serde_json::Value {
    tracing::debug!(?params, "itihas_event invoked");

    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        return match event::by_name(name) {
            Ok(e) => mcp_success(serde_json::to_value(&e).unwrap_or_default()),
            Err(e) => mcp_error(&e.to_string()),
        };
    }

    // Category filter.
    if let Some(cat_str) = params.get("category").and_then(|v| v.as_str()) {
        if let Some(category) = parse_event_category(cat_str) {
            let results = event::by_category(&category);
            return mcp_success(serde_json::to_value(&results).unwrap_or_default());
        }
        return mcp_error(&format!("unknown category: {cat_str}"));
    }

    // Year range filter.
    let start = params.get("start_year").and_then(|v| v.as_i64());
    let end = params.get("end_year").and_then(|v| v.as_i64());
    if let (Some(s), Some(e)) = (start, end) {
        let results = event::events_between(s as i32, e as i32);
        return mcp_success(serde_json::to_value(&results).unwrap_or_default());
    }

    let all = event::all_events();
    mcp_success(serde_json::to_value(all).unwrap_or_default())
}

fn handle_figure(params: serde_json::Value) -> serde_json::Value {
    tracing::debug!(?params, "itihas_figure invoked");

    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        return match figure::by_name(name) {
            Ok(f) => mcp_success(serde_json::to_value(&f).unwrap_or_default()),
            Err(e) => mcp_error(&e.to_string()),
        };
    }

    if let Some(domain_str) = params.get("domain").and_then(|v| v.as_str()) {
        if let Some(domain) = parse_figure_domain(domain_str) {
            let results = figure::by_domain(&domain);
            return mcp_success(serde_json::to_value(&results).unwrap_or_default());
        }
        return mcp_error(&format!("unknown domain: {domain_str}"));
    }

    let all = figure::all_figures();
    mcp_success(serde_json::to_value(all).unwrap_or_default())
}

fn handle_timeline(params: serde_json::Value) -> serde_json::Value {
    tracing::debug!(?params, "itihas_timeline invoked");

    let start = match params.get("start_year").and_then(|v| v.as_i64()) {
        Some(y) => y as i32,
        None => return mcp_error("missing required field: start_year"),
    };
    let end = match params.get("end_year").and_then(|v| v.as_i64()) {
        Some(y) => y as i32,
        None => return mcp_error("missing required field: end_year"),
    };

    let eras = era::eras_containing(start)
        .into_iter()
        .chain(era::eras_containing(end))
        .collect::<Vec<_>>();
    // Deduplicate by name.
    let mut seen = std::collections::HashSet::new();
    let eras: Vec<_> = eras
        .into_iter()
        .filter(|e| seen.insert(e.name.clone()))
        .collect();

    let events = event::events_between(start, end);
    let civilizations = civilization::active_at(start)
        .into_iter()
        .chain(civilization::active_at(end))
        .collect::<Vec<_>>();
    let mut seen = std::collections::HashSet::new();
    let civilizations: Vec<_> = civilizations
        .into_iter()
        .filter(|c| seen.insert(c.name.clone()))
        .collect();

    mcp_success(serde_json::json!({
        "start_year": start,
        "end_year": end,
        "eras": serde_json::to_value(&eras).unwrap_or_default(),
        "events": serde_json::to_value(&events).unwrap_or_default(),
        "civilizations": serde_json::to_value(&civilizations).unwrap_or_default(),
    }))
}

/// Format a successful MCP tool response.
#[inline]
fn mcp_success(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "content": [{"type": "text", "text": data.to_string()}]
    })
}

/// Format an error MCP tool response.
#[inline]
fn mcp_error(message: &str) -> serde_json::Value {
    serde_json::json!({
        "content": [{"type": "text", "text": message}],
        "isError": true
    })
}

fn parse_event_category(s: &str) -> Option<event::EventCategory> {
    match s.to_ascii_lowercase().as_str() {
        "war" => Some(event::EventCategory::War),
        "treaty" => Some(event::EventCategory::Treaty),
        "discovery" => Some(event::EventCategory::Discovery),
        "invention" => Some(event::EventCategory::Invention),
        "revolution" => Some(event::EventCategory::Revolution),
        "migration" => Some(event::EventCategory::Migration),
        "founding" => Some(event::EventCategory::Founding),
        "collapse" => Some(event::EventCategory::Collapse),
        _ => None,
    }
}

fn parse_figure_domain(s: &str) -> Option<figure::FigureDomain> {
    match s.to_ascii_lowercase().as_str() {
        "ruler" => Some(figure::FigureDomain::Ruler),
        "philosopher" => Some(figure::FigureDomain::Philosopher),
        "scientist" => Some(figure::FigureDomain::Scientist),
        "artist" => Some(figure::FigureDomain::Artist),
        "military" => Some(figure::FigureDomain::Military),
        "religious" => Some(figure::FigureDomain::Religious),
        "explorer" => Some(figure::FigureDomain::Explorer),
        "inventor" => Some(figure::FigureDomain::Inventor),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Assert that a handler result is not an error.
    #[track_caller]
    fn assert_ok(result: &serde_json::Value) {
        assert!(
            result
                .get("isError")
                .is_none_or(|v| v.as_bool() != Some(true)),
            "expected success, got error: {result}"
        );
    }

    /// Assert that a handler result is an error.
    #[track_caller]
    fn assert_err(result: &serde_json::Value) {
        assert_eq!(
            result.get("isError").and_then(|v| v.as_bool()),
            Some(true),
            "expected error, got success: {result}"
        );
    }

    #[test]
    fn test_tool_definitions_count() {
        assert_eq!(tool_definitions().len(), 5);
    }

    #[test]
    fn test_tool_names() {
        let defs = tool_definitions();
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"itihas_era"));
        assert!(names.contains(&"itihas_civilization"));
        assert!(names.contains(&"itihas_event"));
        assert!(names.contains(&"itihas_figure"));
        assert!(names.contains(&"itihas_timeline"));
    }

    #[test]
    fn test_tool_schemas_are_valid() {
        for tool in tool_definitions() {
            assert_eq!(tool.input_schema.schema_type, "object");
            assert!(!tool.description.is_empty());
        }
    }

    #[test]
    fn test_all_tools_are_read_only() {
        for tool in tool_definitions() {
            let ann = tool
                .annotations
                .as_ref()
                .expect("annotations should be set");
            assert_eq!(ann.read_only_hint, Some(true));
            assert_eq!(ann.destructive_hint, Some(false));
        }
    }

    #[test]
    fn test_tool_definition_serde_roundtrip() {
        for def in tool_definitions() {
            let json = serde_json::to_string(&def).unwrap();
            let back: ToolDef = serde_json::from_str(&json).unwrap();
            assert_eq!(def.name, back.name);
        }
    }

    #[test]
    fn test_handle_era_by_name() {
        let result = handle_era(serde_json::json!({"name": "Classical Antiquity"}));
        assert_ok(&result);
        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Classical"));
    }

    #[test]
    fn test_handle_era_by_year() {
        let result = handle_era(serde_json::json!({"year": 500}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_era_unknown_name() {
        let result = handle_era(serde_json::json!({"name": "Nonexistent Era"}));
        assert_err(&result);
    }

    #[test]
    fn test_handle_era_no_params() {
        let result = handle_era(serde_json::json!({}));
        assert_ok(&result);
        let text = result["content"][0]["text"].as_str().unwrap();
        let eras: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
        assert!(eras.len() >= 25);
    }

    #[test]
    fn test_handle_civilization_by_name() {
        let result = handle_civilization(serde_json::json!({"name": "Roman Empire"}));
        assert_ok(&result);
        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Roman"));
    }

    #[test]
    fn test_handle_civilization_by_year() {
        let result = handle_civilization(serde_json::json!({"year": 100}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_event_by_name() {
        let result = handle_event(serde_json::json!({"name": "Fall of the Western Roman Empire"}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_event_by_category() {
        let result = handle_event(serde_json::json!({"category": "War"}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_event_unknown_category() {
        let result = handle_event(serde_json::json!({"category": "Dance"}));
        assert_err(&result);
    }

    #[test]
    fn test_handle_event_year_range() {
        let result = handle_event(serde_json::json!({"start_year": -500, "end_year": 0}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_figure_by_name() {
        let result = handle_figure(serde_json::json!({"name": "Julius Caesar"}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_figure_by_domain() {
        let result = handle_figure(serde_json::json!({"domain": "Philosopher"}));
        assert_ok(&result);
    }

    #[test]
    fn test_handle_figure_unknown_domain() {
        let result = handle_figure(serde_json::json!({"domain": "Chef"}));
        assert_err(&result);
    }

    #[test]
    fn test_handle_timeline() {
        let result = handle_timeline(serde_json::json!({"start_year": -500, "end_year": 500}));
        assert_ok(&result);
        let text = result["content"][0]["text"].as_str().unwrap();
        let timeline: serde_json::Value = serde_json::from_str(text).unwrap();
        assert!(timeline.get("eras").is_some());
        assert!(timeline.get("events").is_some());
        assert!(timeline.get("civilizations").is_some());
    }

    #[test]
    fn test_handle_timeline_missing_start() {
        let result = handle_timeline(serde_json::json!({"end_year": 500}));
        assert_err(&result);
    }

    #[test]
    fn test_handle_timeline_missing_end() {
        let result = handle_timeline(serde_json::json!({"start_year": -500}));
        assert_err(&result);
    }

    #[test]
    fn test_mcp_success_format() {
        let result = mcp_success(serde_json::json!({"foo": "bar"}));
        assert!(result["content"][0]["type"].as_str() == Some("text"));
        assert!(result["content"][0]["text"].is_string());
    }

    #[test]
    fn test_mcp_error_format() {
        let result = mcp_error("something went wrong");
        assert!(result["content"][0]["text"].as_str() == Some("something went wrong"));
        assert_eq!(result["isError"], true);
    }

    #[test]
    fn test_parse_event_category_case_insensitive() {
        assert!(parse_event_category("WAR").is_some());
        assert!(parse_event_category("war").is_some());
        assert!(parse_event_category("War").is_some());
        assert!(parse_event_category("nope").is_none());
    }

    #[test]
    fn test_parse_figure_domain_case_insensitive() {
        assert!(parse_figure_domain("RULER").is_some());
        assert!(parse_figure_domain("ruler").is_some());
        assert!(parse_figure_domain("Ruler").is_some());
        assert!(parse_figure_domain("nope").is_none());
    }
}
