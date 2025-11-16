use chrono::NaiveDate;

/// Validates a date input string in DD/MM format and returns a NaiveDate.
/// Automatically assumes the year based on the start_date: if the input date
/// is before or on the same day as start_date, it assumes the next year.
/// Returns an error string for invalid formats or dates.
pub fn validate_date_input(input: &str, start_date: NaiveDate) -> Result<NaiveDate, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(start_date);
    }

    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Use DD/MM".to_string());
    }

    let day = parts[0].parse::<u32>().map_err(|_| "Invalid day".to_string())?;
    let month = parts[1].parse::<u32>().map_err(|_| "Invalid month".to_string())?;

    if day == 0 || day > 31 {
        return Err("Day must be between 1 and 31".to_string());
    }
    if month == 0 || month > 12 {
        return Err("Month must be between 1 and 12".to_string());
    }

    let mut year = start_date.format("%Y").to_string().parse::<i32>().unwrap();
    let start_month = start_date.format("%m").to_string().parse::<u32>().unwrap();
    let start_day = start_date.format("%d").to_string().parse::<u32>().unwrap();
    if month < start_month || (month == start_month && day < start_day) {
        year += 1;
    }

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| "Invalid date".to_string())
}

/// Generates date suggestions based on input prefix.
/// Supports common relative dates like "tomorrow", "next week", "end of month",
/// and partial inputs like single digits for day completion.
pub fn get_date_suggestions(input: &str, start_date: NaiveDate) -> Vec<String> {
    let mut suggestions = Vec::new();
    let input_lower = input.to_lowercase();

    // Common relative dates
    let tomorrow = start_date + chrono::Duration::days(1);
    let next_week = start_date + chrono::Duration::weeks(1);
    let start_month = start_date.format("%m").to_string().parse::<u32>().unwrap();
    let end_of_month = {
        let mut date = start_date;
        while date.format("%m").to_string().parse::<u32>().unwrap() == start_month {
            date += chrono::Duration::days(1);
        }
        date - chrono::Duration::days(1)
    };

    if "tomorrow".starts_with(&input_lower) || input_lower.starts_with("tom") {
        let day = tomorrow.format("%d").to_string().parse::<u32>().unwrap();
        let month = tomorrow.format("%m").to_string().parse::<u32>().unwrap();
        suggestions.push(format!("{:02}/{:02}", day, month));
    }
    if "next week".starts_with(&input_lower) || input_lower.starts_with("next") {
        let day = next_week.format("%d").to_string().parse::<u32>().unwrap();
        let month = next_week.format("%m").to_string().parse::<u32>().unwrap();
        suggestions.push(format!("{:02}/{:02}", day, month));
    }
    if "end of month".starts_with(&input_lower) || input_lower.starts_with("end") {
        let day = end_of_month.format("%d").to_string().parse::<u32>().unwrap();
        let month = end_of_month.format("%m").to_string().parse::<u32>().unwrap();
        suggestions.push(format!("{:02}/{:02}", day, month));
    }

    // If input looks like partial DD/MM, suggest completion
    if input.contains('/') {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 2 {
            if parts[0].len() == 1 && parts[1].is_empty() {
                // Single digit day, suggest current month
                suggestions.push(format!("{}0/{}", parts[0], start_date.format("%m").to_string()));
            }
        }
    } else if input.len() == 1 && input.chars().all(|c| c.is_ascii_digit()) {
        // Single digit, assume day, suggest with current month
        suggestions.push(format!("{}0/{}", input, start_date.format("%m").to_string()));
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;
use chrono::prelude::*;

    #[test]
    fn test_validate_date_input_valid() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        assert!(validate_date_input("15/10", start_date).is_ok());
        assert!(validate_date_input("01/11", start_date).is_ok()); // Next month
    }

    #[test]
    fn test_validate_date_input_invalid_format() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        assert!(validate_date_input("15-10", start_date).is_err());
        assert!(validate_date_input("15/10/23", start_date).is_err());
        assert!(validate_date_input("abc", start_date).is_err());
    }

    #[test]
    fn test_validate_date_input_invalid_day() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        assert!(validate_date_input("32/10", start_date).is_err());
        assert!(validate_date_input("0/10", start_date).is_err());
    }

    #[test]
    fn test_validate_date_input_invalid_month() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        assert!(validate_date_input("15/13", start_date).is_err());
        assert!(validate_date_input("15/0", start_date).is_err());
    }

    #[test]
    fn test_validate_date_input_year_assumption() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 15).unwrap();
        // Before start date, should be next year
        let result = validate_date_input("10/10", start_date).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 10, 10).unwrap());
        // After start date, same year
        let result = validate_date_input("20/10", start_date).unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 10, 20).unwrap());
    }

    #[test]
    fn test_validate_date_input_empty() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        assert_eq!(validate_date_input("", start_date).unwrap(), start_date);
        assert_eq!(validate_date_input("   ", start_date).unwrap(), start_date);
    }

    #[test]
    fn test_get_date_suggestions() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let suggestions = get_date_suggestions("tom", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("02/10")); // Tomorrow

        let suggestions = get_date_suggestions("next", start_date);
        assert!(!suggestions.is_empty());

        let suggestions = get_date_suggestions("end", start_date);
        assert!(!suggestions.is_empty());
    }
}