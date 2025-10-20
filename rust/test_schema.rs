use schemars::{JsonSchema, schema_for};

#[derive(JsonSchema)]
struct TestStruct {
    /// Name field
    name: String,
    /// Age field
    age: Option<i32>,
}

fn main() {
    let schema = schema_for!(TestStruct);
    println!("Type: {:?}", std::any::type_name_of_val(&schema));
    println!("\nSchema JSON:\n{}", serde_json::to_string_pretty(&schema).unwrap());
}
