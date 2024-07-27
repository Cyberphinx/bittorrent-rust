use eyre::{eyre, Result};
use serde_json::{json, Value};

/// This function decodes bencoded (bee-encoded) strings
///
/// # Arguents
///
/// * `5:hello`
///
/// # Returns
///
/// The decoded string of `5:hello` is `hello`
///
/// 1. Integers: Encoded as i<integer>e. For example, the integer 42 is encoded as i42e.
///
/// 2. Byte Strings: Encoded as <length>:<contents>. For example, the string "spam" is encoded as 4:spam.
///
/// 3. Lists: Encoded as l<contents>e. For example, a list containing the string "spam" and the integer 42 is encoded as l4:spami42ee.
///
/// 4. Dictionaries: Encoded as d<contents>e. For example, a dictionary with keys "bar" and "foo" and values "spam" and 42, respectively, is encoded as d3:bar4:spam3:fooi42ee
///
pub fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value> {
    let first_char = encoded_value.chars().next().unwrap();

    if first_char.is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        if let Some((len, value)) = encoded_value.split_once(':') {
            // Parse the length part to an integer
            if let Ok(expected_length) = len.parse::<usize>() {
                // Check if the length of the value matches the expected length
                if value.len() == expected_length {
                    Ok(Value::String(value.to_string()))
                } else {
                    Err(eyre::eyre!(
                        "Length of the value '{}' does not match the expected length {}",
                        value,
                        expected_length
                    ))
                }
            } else {
                Err(eyre::eyre!(
                    "Failed to parse the length part '{}' as an integer",
                    len
                ))
            }
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
