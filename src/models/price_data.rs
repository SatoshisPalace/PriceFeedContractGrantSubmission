use cosmwasm_std::{Decimal, from_binary, Binary, StdError, };
use getset::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct Quote {
    timestamp: String,
    #[getset(get = "pub")]
    quote: QuoteDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
#[allow(non_snake_case)]
pub struct QuoteDetails {
    #[getset(get = "pub")]
    USD: PriceData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
pub struct PriceData {
    #[getset(get = "pub")]
    price: Decimal,
    #[getset(get = "pub")]
    timestamp: String,
}
#[derive(Serialize, Deserialize, Debug, Getters, PartialEq, JsonSchema)]
#[getset(get = "pub")]
pub struct PriceInfo {
    #[getset(get = "pub")]
    quotes: Vec<Quote>,
    id: u32,
    name: String,
    symbol: String,
    is_active: u8,
    is_fiat: u8,
}

impl PriceInfo {
    /// Deserialize a JSON string into a PriceInfo struct.
    pub fn from_json(json_str: &str) -> Result<Self, StdError> {
        
        // Clean the JSON string to ensure floating point numbers are treated as strings
        let cleaned_json = clean_json_string(&json_str);
        // Using Binary for compatibility with potential CosmWasm usage
        let binary = Binary::from(cleaned_json.as_bytes());

        // Attempt to deserialize the binary data into PriceInfo
        from_binary(&binary).map_err(|err| {
            StdError::generic_err(format!("Failed to parse price data: {}", err))
        })
    }
}

// Note: Regex is impossible due to upload inability, speak to Kenny or Caitlyn for details
// Function to modify a JSON string to ensure all floating point numbers are treated as strings
fn clean_json_string(input_str: &str) -> String {
    let json_str = &input_str.replace("\\\"", "\"");

    let mut result = String::new();
    let mut in_number = false;
    let mut has_decimal = false;
    let mut start_index = 0;
    let mut prev_char = '\0'; // Track the previous character

    for (i, c) in json_str.chars().enumerate() {
        match c {
            // Handle digits when not currently in a number
            '0'..='9' if !in_number && prev_char == ':' => {
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
                // If the number has a decimal and ends with a comma
                if has_decimal && c == ',' {
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

        prev_char = c; // Update the previous character

        // Handle the final character if it ends a number
        if in_number && i == json_str.len() - 1 {
            if has_decimal {
                result.insert(start_index, '"'); // Ensure the number is enclosed in quotes
                result.push('"');
            }
        }
    }

    result
}
