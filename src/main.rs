use core::panic;
use std::env;

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        println!("Decode command started");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1]);
    }
}

fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let first_char = encoded_value.chars().next().unwrap();
    // If encoded_value starts with a digit, it's a number
    if first_char.is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        serde_json::Value::String(string.to_string())
    } else if first_char == 'i' {
        // Example: "i45e" -> 45
        let value: String = encoded_value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        Value::String(value)
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}
