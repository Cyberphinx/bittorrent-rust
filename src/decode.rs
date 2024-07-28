use std::collections::HashMap;

use eyre::{eyre, Result};

pub struct Decoder;

impl Decoder {
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
        let value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)?;

        Decoder::convert(value)
    }

    pub fn decode_bencoded_bytes(encoded_value: &[u8]) -> Result<serde_json::Value> {
        let value: serde_bencode::value::Value = serde_bencode::from_bytes(encoded_value)?;

        Decoder::convert(value)
    }

    fn convert(value: serde_bencode::value::Value) -> Result<serde_json::Value> {
        match value {
            serde_bencode::value::Value::Bytes(b) => {
                // Decoding bencoded string
                let string = String::from_utf8(b)?;
                Ok(serde_json::Value::String(string))
            }

            serde_bencode::value::Value::Int(i) => {
                // Decoding bencoded integer
                Ok(serde_json::Value::Number(serde_json::Number::from(i)))
            }

            serde_bencode::value::Value::List(l) => {
                // Decoding bencoded list
                let array = l
                    .into_iter()
                    .map(Decoder::convert)
                    .collect::<Result<Vec<serde_json::Value>>>()?;

                Ok(serde_json::Value::Array(array))
            }

            serde_bencode::value::Value::Dict(d) => {
                // Decoding bencoded dictionary
                // Iterating over each key-value pair in the dictionary
                let object = d
                    .into_iter()
                    .map(|(k, v)| {
                        // For each pair, it converts the key to a string and recursively converts the value using this function
                        let key = String::from_utf8(k)?;

                        let value = Decoder::convert(v)?;

                        Ok((key, value))
                    })
                    .collect::<Result<serde_json::Map<String, serde_json::Value>>>()?;

                Ok(serde_json::Value::Object(object))
            }
        }
    }

    pub fn extract_string(
        key: &str,
        d: &HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<String> {
        d.get(key.as_bytes())
            .and_then(|v| match v {
                serde_bencode::value::Value::Bytes(b) => String::from_utf8(b.clone()).ok(),

                _ => None,
            })
            .ok_or(eyre!("Missing field: {}", key))
    }

    pub fn extract_bytes(
        key: &str,
        d: &HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<Vec<u8>> {
        d.get(key.as_bytes())
            .and_then(|v| match v {
                serde_bencode::value::Value::Bytes(b) => Some(b.clone()),

                _ => None,
            })
            .ok_or(eyre!("Missing field: {}", key))
    }

    pub fn extract_dict(
        key: &str,
        d: &HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<HashMap<Vec<u8>, serde_bencode::value::Value>> {
        d.get(key.as_bytes())
            .and_then(|v| match v {
                serde_bencode::value::Value::Dict(d) => Some(d.clone()),

                _ => None,
            })
            .ok_or(eyre!("Missing field: {}", key))
    }

    pub fn extract_int(
        key: &str,
        d: &HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<i64> {
        d.get(key.as_bytes())
            .and_then(|v| match v {
                serde_bencode::value::Value::Int(i) => Some(*i),

                _ => None,
            })
            .ok_or(eyre!("Missing field: {}", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decode_bencoded_string() {
        let encoded_value = "5:hello";
        let decoded_value = Decoder::decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, "hello");
    }

    #[test]
    fn decode_bencoded_integer() {
        let encoded_value = "i42e";
        let decoded_value = Decoder::decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, 42);
    }

    #[test]
    fn decode_bencoded_list() {
        let encoded_value = "l5:helloi42ee";
        let decoded_value = Decoder::decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, json!(("hello", 42)));
    }

    #[test]
    fn decode_bencoded_distionary() {
        let encoded_value = "d3:foo3:bar5:helloi42ee";
        let decoded_value = Decoder::decode_bencoded_value(encoded_value).unwrap();
        assert_eq!(decoded_value, json!({"foo":"bar","hello":42}));
    }
}
