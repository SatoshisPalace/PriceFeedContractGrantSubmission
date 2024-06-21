use crate::{constants::DAYS_IN_MONTHS, error::price_posting_error::PricePostingError};
use urlencoding::decode;

pub fn convert_timestamp_to_unix(timestamp: &str) -> Result<u64, PricePostingError> {
    let decoded_timestamp = decode(timestamp)
        .map_err(|e| format!("Failed to decode timestamp: {}", e))
        .unwrap()
        .into_owned();

    // Remove the trailing 'Z' character
    let datetime = decoded_timestamp.strip_suffix('Z').unwrap();

    // Split the datetime into date and time parts
    let date_time_parts: Vec<&str> = datetime.split('T').collect();
    if date_time_parts.len() != 2 {
        return Err(PricePostingError::PriceTypeError {
            error: "Timestamp formatted incorrectly".to_string(),
        });
    }

    // Split the date part into year, month, and day
    let date_parts: Vec<&str> = date_time_parts[0].split('-').collect();
    // Split the time part into hour, minute, and second (ignored in this case)
    let time_parts: Vec<&str> = date_time_parts[1].split(':').collect();

    // Check if the date and time parts have the correct lengths
    if date_parts.len() != 3 || time_parts.len() != 3 {
        return Err(PricePostingError::PriceTypeError {
            error: "Timestamp formatted incorrectly".to_string(),
        });
    }

    // Parse the year, month, and day from the date parts
    let year: u64 = date_parts[0].parse().ok().unwrap();
    let month: u64 = date_parts[1].parse().ok().unwrap();
    let day: u64 = date_parts[2].parse().ok().unwrap();

    // Parse the hour and minute from the time parts
    let hour: u64 = time_parts[0].parse().ok().unwrap();
    let minute: u64 = time_parts[1].parse().ok().unwrap();
    // Ignore seconds as our time is always on the minute and seconds are in float: 00.000

    // Calculate the number of days in each month
    let days_in_month = DAYS_IN_MONTHS;

    let mut days_since_epoch = 0;

    // Add days for the years since 1970
    for year in 1970..year {
        days_since_epoch += if is_leap_year(year) { 366 } else { 365 };
    }

    // Add days for the months in the current year
    for month in 0..(month as usize - 1) {
        days_since_epoch += days_in_month[month];
        if month == 1 && is_leap_year(year) {
            days_since_epoch += 1;
        }
    }

    // Add days for the current month
    days_since_epoch += day - 1;

    // Calculate total seconds since the Unix epoch
    let total_seconds = days_since_epoch * 86400 + hour * 3600 + minute * 60; // + second as u64;

    // Return the Unix timestamp as Some(total_seconds)
    Ok(total_seconds)
}

// Determine if a year is a leap year
pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
