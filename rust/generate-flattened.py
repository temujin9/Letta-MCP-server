#!/usr/bin/env python3
"""
Generate flattened parameter implementations for Turbo MCP tools.
This script reads the Request structs and generates the flattened function signatures.
"""

import re

# Tool definitions with their parameters
tools = {
    "source_manager": {
        "operation_enum": "SourceOperation",
        "params": [
            ("operation", "String", False, "enum"),
            ("source_id", "Option<String>", True, None),
            ("agent_id", "Option<String>", True, None),
            ("name", "Option<String>", True, None),
            ("description", "Option<String>", True, None),
            ("file_id", "Option<String>", True, None),
            ("file_name", "Option<String>", True, None),
            ("file_data", "Option<String>", True, None),
            ("content_type", "Option<String>", True, None),
            ("limit", "Option<i32>", True, None),
            ("include_content", "Option<bool>", True, None),
        ]
    },
    "tool_manager": {
        "operation_enum": "ToolOperation",
        "params": [
            ("operation", "String", False, "enum"),
            ("tool_id", "Option<String>", True, None),
            ("agent_id", "Option<String>", True, None),
            ("agent_ids", "Option<Vec<String>>", True, None),
            ("source_code", "Option<String>", True, None),
            ("source_type", "Option<String>", True, None),
            ("tags", "Option<Vec<String>>", True, None),
            ("description", "Option<String>", True, None),
            ("json_schema", "Option<Value>", True, None),
            ("args_json_schema", "Option<Value>", True, None),
            ("return_char_limit", "Option<u32>", True, None),
            ("args", "Option<Value>", True, None),
            ("env_vars", "Option<std::collections::HashMap<String, String>>", True, None),
            ("name", "Option<String>", True, None),
        ]
    },
    "memory_unified": {
        "operation_enum": "MemoryOperation",
        "params": [
            ("operation", "String", False, "enum"),
            ("agent_id", "Option<String>", True, None),
            ("block_id", "Option<String>", True, None),
            ("block_label", "Option<String>", True, None),
            ("passage_id", "Option<String>", True, None),
            ("label", "Option<String>", True, None),
            ("value", "Option<String>", True, None),
            ("text", "Option<String>", True, None),
            ("query", "Option<String>", True, None),
            ("limit", "Option<i32>", True, None),
            ("offset", "Option<i32>", True, None),
            ("is_template", "Option<bool>", True, None),
        ]
    },
    "agent_advanced": {
        "operation_enum": "AgentOperation",
        "params": [
            ("operation", "String", False, "enum"),
            ("agent_id", "Option<String>", True, None),
            ("name", "Option<String>", True, None),
            ("description", "Option<String>", True, None),
            ("system", "Option<String>", True, None),
            ("llm_config", "Option<Value>", True, None),
            ("embedding_config", "Option<Value>", True, None),
            ("tool_ids", "Option<Value>", True, None),
            ("pagination", "Option<serde_json::Value>", True, "json"),  # Will be parsed as Pagination
            ("messages", "Option<serde_json::Value>", True, "json"),  # Will be parsed as Vec<Message>
            ("stream", "Option<bool>", True, None),
            ("filters", "Option<serde_json::Value>", True, "json"),  # Will be parsed as BulkDeleteFilters
            ("query", "Option<String>", True, None),
            ("search_filters", "Option<serde_json::Value>", True, "json"),  # Will be parsed as SearchFilters
            ("export_data", "Option<Value>", True, None),
            ("update_data", "Option<Value>", True, None),
        ]
    }
}

def generate_tool(tool_name, tool_def):
    """Generate flattened implementation for a tool"""
    operation_enum = tool_def["operation_enum"]
    params = tool_def["params"]

    # Generate parameter list for function signature
    param_list = []
    for name, typ, optional, special in params:
        param_list.append(f"        {name}: {typ},")

    # Generate operation enum parsing (for first parameter)
    # Note: Skipping request_heartbeat since it's ignored

    # Generate request struct construction
    request_fields = []
    for name, typ, optional, special in params:
        if name == "operation":
            request_fields.append(f"            operation: op,")
        elif special == "json":
            # Parse JSON strings to proper types
            base_name = name.replace("_", "")
            request_fields.append(f"            {name}: {name}.and_then(|v| serde_json::from_value(v).ok()),")
        else:
            request_fields.append(f"            {name},")
    request_fields.append(f"            request_heartbeat: None,")

    # Generate function
    func = f'''    #[tool]
    async fn letta_{tool_name}(
        &self,
{chr(10).join(param_list)}
    ) -> McpResult<String> {{
        // Parse operation from string
        let op = serde_json::from_value(serde_json::Value::String(operation))
            .map_err(|e| McpError::invalid_request(format!("Invalid operation: {{}}", e)))?;

        // Create request from individual parameters
        let request = {tool_name}::{tool_name.title().replace("_", "")}Request {{
{chr(10).join(request_fields)}
        }};

        // Call handler
        let response = {tool_name}::handle_{tool_name}(&self.client, request)
            .await
            .map_err(|e| e.into())?;

        // Serialize to JSON string for MCP response
        serde_json::to_string(&response)
            .map_err(|e| McpError::internal(format!("Failed to serialize response: {{}}", e)))
    }}
'''
    return func

# Generate all tools
print("// Generated flattened tool implementations")
print()
for tool_name, tool_def in tools.items():
    print(generate_tool(tool_name, tool_def))
    print()
