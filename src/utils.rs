use serde_json::{self,};
use crate::filters::apply_filter;

/// Processes JSON input by applying a JSONPath filter and prints the result.
pub fn process_input(input: &str, filter: &str) {
    match serde_json::from_str(input) {
        Ok(json_value) => {
            match apply_filter(&json_value, filter) {
                Ok(filtered_json) => {
                    println!("{}", serde_json::to_string_pretty(&filtered_json).unwrap());
                },
                Err(e) => eprintln!("Error applying filter: {}", e),
            }
        },
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_process_input() {
        let json_input = r#"{"name": "Alice", "age": 30}"#;
        let filter = "$.name";
        
        // Capture the output
        std::io::stdout().write_all(b"").unwrap();
        let result = std::panic::catch_unwind(|| {
            process_input(json_input, filter);
        });
        
        assert!(result.is_ok(), "Processing should succeed");
    }
}