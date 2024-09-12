use serde_json::Value;

// Define a custom trait
pub(crate) trait JsonSearch {
    fn find_id(&self) -> Option<String>;
}

// Implement the trait for serde_json::Value
impl JsonSearch for Value {
    fn find_id(&self) -> Option<String> {
        if let Some(id) = self.get("String") {
            // Slice off 1 character from both sides
            let raw_id = id.to_string();
            let start_index = 1;
            let end_index = raw_id.len() - 1;
            return Some(raw_id[start_index..end_index].to_string())
        }

        // Search recursively through objects and arrays
        match self {
            Value::Object(map) => {
                for (_, v) in map {
                    if let Some(found) = v.find_id() {
                        return Some(found)
                    }
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    if let Some(found) = v.find_id() {
                        return Some(found)
                    }
                }
            }
            _ => {}
        }

        None
    }
}