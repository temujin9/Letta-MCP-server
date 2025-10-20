use schemars::{JsonSchema, schema_for};
use letta_types::AgentAdvancedRequest;

fn main() {
    let root_schema = schema_for!(AgentAdvancedRequest);
    let schema_json = serde_json::to_value(&root_schema).unwrap();

    println!("Schema keys: {:?}", schema_json.as_object().unwrap().keys().collect::<Vec<_>>());
    println!("\nHas $defs: {}", schema_json.get("$defs").is_some());

    if let Some(defs) = schema_json.get("$defs") {
        println!("$defs keys: {:?}", defs.as_object().unwrap().keys().collect::<Vec<_>>());
    }

    println!("\nFull schema:");
    println!("{}", serde_json::to_string_pretty(&schema_json).unwrap());
}
