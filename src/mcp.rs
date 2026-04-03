//! MCP tool definitions for itihas.
//!
//! Exposes historical data as Model Context Protocol tools for AI agents.
//! Tool handlers invoke core itihas modules and return structured JSON.
//!
//! **Status**: Stub — tool definitions are declared but handlers are
//! pending the bote MCP framework integration.

use std::collections::HashMap;

use alloc::string::String;

use bote::{ToolDef, ToolSchema};
use serde::{Deserialize, Serialize};

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
                ]),
                vec![],
            ),
        ),
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
        ),
        ToolDef::new(
            "itihas_event",
            "Look up historical events by name, year range, or category",
            ToolSchema::new(
                "object",
                HashMap::from([
                    ("name".into(), serde_json::json!({"type": "string", "description": "Event name (case-insensitive)"})),
                    ("start_year".into(), serde_json::json!({"type": "integer", "description": "Start of year range (negative = BCE)"})),
                    ("end_year".into(), serde_json::json!({"type": "integer", "description": "End of year range (negative = BCE)"})),
                ]),
                vec![],
            ),
        ),
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
        ),
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
        ),
    ]
}

// Tool invocation is pending bote MCP framework integration.
// When bote is integrated, `invoke()` will dispatch to handlers
// that call into crate::era, crate::civilization, crate::event, etc.

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_tool_definition_serde_roundtrip() {
        for def in tool_definitions() {
            let json = serde_json::to_string(&def).unwrap();
            let back: ToolDef = serde_json::from_str(&json).unwrap();
            assert_eq!(def.name, back.name);
        }
    }
}
