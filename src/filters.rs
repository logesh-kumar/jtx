use serde_json_path::JsonPath;
use serde_json::{self, Value};


/// Applies a JSONPath filter to the given JSON input and returns the result.
pub fn apply_filter(json_value: &Value, filter: &str) -> Result<Value, serde_json::Error> {    
    match JsonPath::parse(filter) {
        Ok(path) => {
            let nodes = path.query(json_value).all();
            Ok(serde_json::to_value(nodes).unwrap())
        },
        Err(e) => Err(serde_json::Error::custom(format!("Invalid JSON Path: {}", e)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_apply_filter_valid() {
        let json_input = json!({
            "name": "Alice",
            "age": 30
        });
        let filter = "$.name";
        let expected = json!(["Alice"]);
        let result = apply_filter(&json_input, filter).unwrap();
        assert_eq!(result, expected, "Filtering for $.name should return Alice");
    }

    #[test]
    fn test_apply_filter_invalid_path() {
        let json_input = json!({
            "name": "Alice",
            "age": 30
        });
        let filter = "$.nonexistent";
        let expected = json!([]); // Expect an empty array for nonexistent paths
        let result = apply_filter(&json_input, filter).unwrap();
        assert_eq!(result, expected, "Should return an empty array for invalid path");
    }
    
}
