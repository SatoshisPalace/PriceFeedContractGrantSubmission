use cosmwasm_std::{Decimal, from_binary, Binary, StdError, };
use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct PriceData {
    time: u64,
    close: Decimal,
}

impl PriceData {
    /// Deserialize a JSON string into a vector of PriceData.
    pub fn from_json(json_str: &str) -> Result<Vec<PriceData>, StdError> {
        // Clean the input JSON string by replacing escaped double quotes
        let clean_quotes = json_str.replace("\\\"", "\"");

        let cleaned_json = clean_json_string(&clean_quotes);
        // Using Binary for compatibility with potential CosmWasm usage
        let binary = Binary::from(cleaned_json.as_bytes());

        // Attempt to deserialize the binary data into Vec<PriceData>
        from_binary(&binary).map_err(|err| {
            StdError::generic_err(format!("Failed to parse price data: {}", err))
        })
    }
}

// Note: Regex is impossible due to upload inability, speak to Kenny or Caitlyn for details
// Function to modify a JSON string to ensure all floating point numbers are treated as strings
fn clean_json_string(json_str: &str) -> String {
    // Initialize an empty String to build the result
    let mut result = String::new();
    // Track whether the current sequence of characters being processed is a number
    let mut in_number = false;
    // Track whether the current number being processed contains a decimal point
    let mut has_decimal = false;
    // Store the start index of the current number in the result string
    let mut start_index = 0;

    // Iterate over each character in the JSON string with its index
    for (i, c) in json_str.chars().enumerate() {
        match c {
            // Handle digits when not currently in a number
            '0'..='9' if !in_number => {
                // Start a new number
                in_number = true;
                has_decimal = false;
                start_index = result.len(); // Note where this number starts in the result
                result.push(c);
            },
            // Handle digits when already in a number
            '0'..='9' if in_number => {
                result.push(c);
            },
            // Handle the decimal point if it's part of a number and no decimal point has been added yet
            '.' if in_number && !has_decimal => {
                has_decimal = true;
                result.push(c);
            },
            // Handle the end of a number sequence
            _ if in_number => {
                // If the number has a decimal, treat it as a floating-point number
                if has_decimal {
                    result.insert(start_index, '"'); // Insert opening quote at start of number
                    result.push('"'); // Append closing quote
                }
                result.push(c); // Push the current character which ended the number
                in_number = false; // Reset flags
                has_decimal = false;
            },
            // Default case for any character that's not part of a number
            _ => result.push(c),
        }

        // Handle the final character if it ends a number
        if in_number && i == json_str.len() - 1 {
            if has_decimal {
                result.insert(start_index, '"'); // Ensure the number is enclosed in quotes
                result.push('"');
            }
        }
    }

    // Return the modified JSON string
    result
}