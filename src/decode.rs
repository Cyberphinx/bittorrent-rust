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
pub fn decode_bencoded_value(encoded_value: &str) -> Result<Value> {
    let first_char = encoded_value.chars().next().unwrap();

    match first_char {
        c if c.is_ascii_digit() => {
            let (length, value) = Decoder::split_string_by_colon(encoded_value)?;
            Decoder::decode_string(length, value)
        }
        'i' => Decoder::decode_integer(encoded_value),
        'l' => {
            let (length, value) = Decoder::split_string_by_colon(encoded_value)?;
            Decoder::decode_list(length, value)
        }
        _ => Err(eyre!("Unhandled encoded value: {}", encoded_value)),
    }
}

struct Decoder;

impl Decoder {
    fn split_string_by_colon(encoded_value: &str) -> Result<(&str, &str)> {
        encoded_value.split_once(':').ok_or_else(|| {
            eyre!(
                "Could not split the encoded value into two values with a colon in-between: {}",
                encoded_value
            )
        })
    }

    fn decode_string(length: &str, value: &str) -> Result<Value> {
        // Parse the length part to an integer, filtering out non-numeric values
        if let Ok(expected_length) = length
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
        {
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
                length
            ))
        }
    }

    fn decode_integer(encoded_value: &str) -> Result<Value> {
        let value = encoded_value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        Ok(Value::String(value))
    }

    fn decode_list(length: &str, value: &str) -> Result<Value> {
        if let Some((string_value, number_value)) = value.split_once('i') {
            Ok(json!((
                Decoder::decode_string(length, string_value)?,
                Decoder::decode_integer(number_value)?
            )))
        } else {
            Err(eyre!("Array doesn't contain valid values: {}", value))
        }
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
