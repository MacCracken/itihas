//! MCP tool definitions for itihas.
//!
//! Exposes historical data as Model Context Protocol tools for AI agents.
//! Tool handlers invoke core itihas modules and return structured JSON.
//!
//! **Status**: Stub — tool definitions are declared but handlers are
//! pending the bote MCP framework integration.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

/// An MCP tool parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParameterDef {
    /// Parameter name.
    pub name: Cow<'static, str>,
    /// JSON Schema type (e.g., "string", "integer").
    pub param_type: Cow<'static, str>,
    /// Human-readable description.
    pub description: Cow<'static, str>,
    /// Whether this parameter is required.
    pub required: bool,
}

/// An MCP tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ToolDefinition {
    /// Tool name (e.g., "itihas_era").
    pub name: Cow<'static, str>,
    /// Human-readable description.
    pub description: Cow<'static, str>,
    /// Parameter definitions.
    pub parameters: Vec<ParameterDef>,
}

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
pub fn tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: Cow::Borrowed("itihas_era"),
            description: Cow::Borrowed("Look up historical eras by name, year, scope, or region"),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("name"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Era name (case-insensitive)"),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed("Year to find containing eras (negative = BCE)"),
                    required: false,
                },
            ],
        },
        ToolDefinition {
            name: Cow::Borrowed("itihas_civilization"),
            description: Cow::Borrowed("Look up civilizations by name, region, or active year"),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("name"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Civilization name (case-insensitive)"),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed(
                        "Year to find active civilizations (negative = BCE)",
                    ),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("region"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Region substring filter"),
                    required: false,
                },
            ],
        },
        ToolDefinition {
            name: Cow::Borrowed("itihas_event"),
            description: Cow::Borrowed(
                "Look up historical events by name, year range, or category",
            ),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("name"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Event name (case-insensitive)"),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("start_year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed("Start of year range (negative = BCE)"),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("end_year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed("End of year range (negative = BCE)"),
                    required: false,
                },
            ],
        },
        ToolDefinition {
            name: Cow::Borrowed("itihas_figure"),
            description: Cow::Borrowed("Look up historical figures by name or domain"),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("name"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Figure name (case-insensitive)"),
                    required: false,
                },
                ParameterDef {
                    name: Cow::Borrowed("domain"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed(
                        "Domain filter (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor)",
                    ),
                    required: false,
                },
            ],
        },
        ToolDefinition {
            name: Cow::Borrowed("itihas_timeline"),
            description: Cow::Borrowed(
                "Get a timeline of events, eras, and civilizations for a year range",
            ),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("start_year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed("Start year (negative = BCE)"),
                    required: true,
                },
                ParameterDef {
                    name: Cow::Borrowed("end_year"),
                    param_type: Cow::Borrowed("integer"),
                    description: Cow::Borrowed("End year (negative = BCE)"),
                    required: true,
                },
            ],
        },
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
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_ref()).collect();
        assert!(names.contains(&"itihas_era"));
        assert!(names.contains(&"itihas_civilization"));
        assert!(names.contains(&"itihas_event"));
        assert!(names.contains(&"itihas_figure"));
        assert!(names.contains(&"itihas_timeline"));
    }

    #[test]
    fn test_tool_definition_serde_roundtrip() {
        for def in tool_definitions() {
            let json = serde_json::to_string(&def).unwrap();
            let back: ToolDefinition = serde_json::from_str(&json).unwrap();
            assert_eq!(def.name, back.name);
        }
    }
}
