use eyre::{eyre, Result};
use serde_json::{json, Value};

pub fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value> {
    let first_char = encoded_value.chars().next().unwrap();

    if first_char.is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        if let Some(value) = encoded_value.split(':').nth(1) {
            Ok(serde_json::Value::String(value.to_string()))
        } else {
            Err(eyre!(
                "Encoded value doesn't contain valid values: {}",
                encoded_value
            ))
        }
    } else if first_char == 'i' {
        // Example: "i45e" -> 45
        let value = encoded_value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        Ok(Value::String(value))
    } else if first_char == 'l' {
        // Example: "l5:helloi52ee"
        if let Some(value_after_colon) = encoded_value.split(':').nth(1) {
            let mut splitted_value = value_after_colon.split('i');
            let string_value = splitted_value.next();
            let number_value = splitted_value.next();
            if let (Some(string_value), Some(number_value)) = (string_value, number_value) {
                let s_val = string_value.to_string();
                let n_val = number_value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
                Ok(json!((s_val, n_val)))
            } else {
                Err(eyre!(
                    "Array doesn't contain valid values: {}",
                    encoded_value
                ))
            }
        } else {
            Err(eyre!("array does not seem to be a list"))
        }
    } else {
        Err(eyre!("Unhandled encoded value: {}", encoded_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_bencoded_string() {
        let encoded_value = "5:hello";
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, "hello");
    }

    #[test]
    fn decode_bencoded_integer() {
        let encoded_value = "i52e";
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, "52");
    }

    #[test]
    fn decode_bencoded_list() {
        let encoded_value = "l5:helloi52ee";
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, json!(("hello", "52")));
    }
}
