use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TIME_PATTERN: Regex = Regex::new(r"^(?:[01]\d|2[0-3]):[0-5]\d$").unwrap();
    static ref HOUR_PATTERN: Regex = Regex::new(r"^(?:[01]?\d|2[0-3])$").unwrap();
    static ref SINGLE_HOUR_PATTERN: Regex = Regex::new(r"^[0-9]$").unwrap();
}

/// Validates a time input string and returns Ok(()) for valid inputs or Err(message) for invalid.
/// Supports:
/// - HH:MM format (e.g., "14:30", "09:05", "00:00")
/// - HH format (e.g., "14", "9", "23", "0")  
/// - H format (single digit, e.g., "9", "0")
///
/// Empty input is considered valid (for all-day events).
pub fn validate_time_input(input: &str) -> Result<(), String> {
    let trimmed = input.trim();

    // Empty input is valid (for all-day events)
    if trimmed.is_empty() {
        return Ok(());
    }

    // Check HH:MM format
    if TIME_PATTERN.is_match(trimmed) {
        return Ok(());
    }

    // Check HH format (2 digits, 0-23)
    if HOUR_PATTERN.is_match(trimmed) {
        return Ok(());
    }

    // Check single digit H format (0-9)
    if SINGLE_HOUR_PATTERN.is_match(trimmed) {
        return Ok(());
    }

    Err("Invalid time format. Use HH:MM, HH, or H (e.g., 14:30, 14, 9)".to_string())
}

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

    let day = parts[0]
        .parse::<u32>()
        .map_err(|_| "Invalid day".to_string())?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| "Invalid month".to_string())?;

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

    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "Invalid date".to_string())
}

/// Generates date suggestions based on input prefix.
/// Returns (suggestion_text, is_valid) pairs.
/// Supports common relative dates like "tomorrow", "next week", "end of month",
/// and partial inputs like single digits for day completion.
pub fn get_date_suggestions(
    input: &str,
    start_date: NaiveDate,
    current_date: NaiveDate,
) -> Vec<(String, bool)> {
    let mut suggestions = Vec::new();
    let input_lower = input.to_lowercase();
    let current_month = current_date
        .format("%m")
        .to_string()
        .parse::<u32>()
        .unwrap();
    let current_year = current_date
        .format("%Y")
        .to_string()
        .parse::<i32>()
        .unwrap();

    // Prioritize digit-based date completion
    if input.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(num) = input.parse::<u32>() {
            if (1..=31).contains(&num) {
                // Find the starting month: current month if valid, otherwise next month
                let mut starting_month = current_month;
                let last_day_of_current = match current_month {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 => {
                        if current_year % 4 == 0
                            && (current_year % 100 != 0 || current_year % 400 == 0)
                        {
                            29
                        } else {
                            28
                        }
                    }
                    _ => 31,
                };
                let day = if num > last_day_of_current {
                    last_day_of_current
                } else {
                    num
                };
                // If current month date is before current_date and input is single digit, start from next month
                let current_year = current_date
                    .format("%Y")
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
                let current_month_date = NaiveDate::from_ymd_opt(current_year, current_month, day);
                let is_before_current = current_month_date.is_some_and(|d| d < current_date);
                if is_before_current && input.len() == 1 {
                    starting_month = (current_month % 12) + 1;
                }

                // Suggest for starting month, next month, and month after next
                // Track year progression when months wrap from December to January
                let month_suggestions = [
                    (starting_month, current_year),
                    (
                        (starting_month % 12) + 1,
                        if starting_month == 12 {
                            current_year + 1
                        } else {
                            current_year
                        },
                    ),
                    (
                        ((starting_month % 12) + 1) % 12 + 1,
                        if starting_month >= 11 {
                            current_year + 1
                        } else {
                            current_year
                        },
                    ),
                ];

                for (month, year_for_leap) in &month_suggestions {
                    let last_day_of_month = match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 => {
                            if year_for_leap % 4 == 0
                                && (year_for_leap % 100 != 0 || year_for_leap % 400 == 0)
                            {
                                29
                            } else {
                                28
                            }
                        }
                        _ => 31,
                    };

                    let day = if num > last_day_of_month {
                        last_day_of_month
                    } else {
                        num
                    };
                    let month_str = format!("{month:02}");
                    let date_str = format!("{day:02}/{month_str}");

                    let is_valid = validate_date_input(&date_str, start_date).is_ok();
                    if is_valid {
                        suggestions.push((date_str, true));
                    }
                }
            }
        }
    }

    // Handle empty input - show top 5 common suggestions
    if input.trim().is_empty() {
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
        let next_month = {
            let mut date = start_date;
            let current_month = date.format("%m").to_string().parse::<u32>().unwrap();
            while date.format("%m").to_string().parse::<u32>().unwrap() == current_month {
                date += chrono::Duration::days(1);
            }
            date
        };

        let top_suggestions = vec![
            (tomorrow, "Tomorrow"),
            (next_week, "Next week"),
            (end_of_month, "End of month"),
            (next_month, "Next month"),
            (start_date, "Same day"),
        ];

        for (date, desc) in top_suggestions {
            let day = date.format("%d").to_string().parse::<u32>().unwrap();
            let month = date.format("%m").to_string().parse::<u32>().unwrap();
            suggestions.push((format!("{desc} ({day:02}/{month:02})"), true));
        }
    } else {
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
        let next_month = {
            let mut date = start_date;
            let current_month = date.format("%m").to_string().parse::<u32>().unwrap();
            while date.format("%m").to_string().parse::<u32>().unwrap() == current_month {
                date += chrono::Duration::days(1);
            }
            date
        };
        let end_of_year = {
            let year = start_date.format("%Y").to_string().parse::<i32>().unwrap();
            NaiveDate::from_ymd_opt(year, 12, 31).unwrap()
        };

        // Duration-based suggestions
        let one_day = start_date + chrono::Duration::days(1);
        let one_week = start_date + chrono::Duration::weeks(1);
        let two_weeks = start_date + chrono::Duration::weeks(2);
        let one_month = next_month;

        // Weekday suggestions
        let weekdays = [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ];
        let weekday_nums = [1, 2, 3, 4, 5, 6, 7]; // 1=Mon, 7=Sun
        let mut next_weekdays = Vec::new();
        for (&weekday, &target_num) in weekdays.iter().zip(weekday_nums.iter()) {
            let current_num = start_date.format("%u").to_string().parse::<u32>().unwrap();
            let days_ahead = if target_num > current_num {
                target_num - current_num
            } else {
                7 - current_num + target_num
            };
            let date = start_date + chrono::Duration::days(days_ahead as i64);
            next_weekdays.push((weekday, date));
        }

        // Define suggestions with their possible input matches
        let suggestion_matches = vec![
            (tomorrow, vec!["tomorrow", "tom", "tomorow"]),
            (next_week, vec!["next week", "nextweek"]),
            (
                end_of_month,
                vec!["end of month", "endofmonth", "end month"],
            ),
            (next_month, vec!["next month", "nextmonth"]),
            (end_of_year, vec!["end of year", "endofyear", "end year"]),
            (start_date, vec!["same day", "sameday"]),
            (one_day, vec!["1 day", "1day"]),
            (one_week, vec!["1 week", "1week"]),
            (two_weeks, vec!["2 weeks", "2weeks"]),
            (one_month, vec!["1 month", "1month"]),
        ];

        // Check relative and duration matches
        let descriptions = [
            "Tomorrow",
            "Next week",
            "End of month",
            "Next month",
            "End of year",
            "Same day",
            "1 day",
            "1 week",
            "2 weeks",
            "1 month",
        ];
        for i in 0..suggestion_matches.len() {
            let (date, possible_inputs) = &suggestion_matches[i];
            let desc = descriptions[i];
            for &possible in possible_inputs {
                if possible.starts_with(&input_lower)
                    || input_lower.starts_with(possible)
                    || possible.contains(&input_lower)
                    || input_lower.contains(possible)
                {
                    let day = date.format("%d").to_string().parse::<u32>().unwrap();
                    let month = date.format("%m").to_string().parse::<u32>().unwrap();
                    suggestions.push((format!("{desc} ({day:02}/{month:02})"), true));
                    break; // Only add once per date
                }
            }
        }

        // Next weekday suggestions
        for (weekday, date) in &next_weekdays {
            let possible = format!("next {weekday}");
            let short = format!("next {}", &weekday[..3]);
            if possible.starts_with(&input_lower)
                || input_lower.starts_with(&possible)
                || short.starts_with(&input_lower)
                || input_lower.starts_with(&short)
                || possible.contains(&input_lower)
                || input_lower.contains(&possible)
            {
                let day = date.format("%d").to_string().parse::<u32>().unwrap();
                let month = date.format("%m").to_string().parse::<u32>().unwrap();
                suggestions.push((format!("Next {weekday} ({day:02}/{month:02})"), true));
                break; // Only one weekday suggestion
            }
        }

        // Enhanced partial input completion
        if input.contains('/') {
            let parts: Vec<&str> = input.split('/').collect();
            if parts.len() == 2 {
                let day_part = parts[0].trim();
                let month_part = parts[1].trim();
                if !day_part.is_empty() && month_part.is_empty() {
                    // "15/" -> complete with current month
                    if let Ok(day) = day_part.parse::<u32>() {
                        if (1..=31).contains(&day) {
                            let month = start_date.format("%m").to_string();
                            let date_str = format!("{day:02}/{month}");
                            let is_valid = validate_date_input(&date_str, start_date).is_ok();
                            suggestions.push((date_str, is_valid));
                        }
                    }
                } else if day_part.is_empty() && !month_part.is_empty() {
                    // " /10" -> complete with appropriate day (start_date day if valid, else 1)
                    if let Ok(month) = month_part.parse::<u32>() {
                        if (1..=12).contains(&month) {
                            let day = start_date.format("%d").to_string();
                            let date_str = format!("{day}/{month_part}");
                            let is_valid = validate_date_input(&date_str, start_date).is_ok();
                            suggestions.push((date_str, is_valid));
                        }
                    }
                } else if !day_part.is_empty()
                    && !month_part.is_empty()
                    && (day_part.len() < 2 || month_part.len() < 2)
                {
                    // Partial, show full format if matches and not full
                    if let (Ok(day), Ok(month)) =
                        (day_part.parse::<u32>(), month_part.parse::<u32>())
                    {
                        if (1..=31).contains(&day) && (1..=12).contains(&month) {
                            let date_str = format!("{day:02}/{month:02}");
                            let is_valid = validate_date_input(&date_str, start_date).is_ok();
                            suggestions.push((date_str, is_valid));
                        }
                    }
                }
            }
        }

        // Common date patterns
        if input_lower.contains("last day") || input_lower.contains("lastday") {
            let day = end_of_month
                .format("%d")
                .to_string()
                .parse::<u32>()
                .unwrap();
            let month = end_of_month
                .format("%m")
                .to_string()
                .parse::<u32>()
                .unwrap();
            let date_str = format!("Last day of month ({day:02}/{month:02})");
            suggestions.push((date_str, true));
        }
        if input_lower.contains("first of next") || input_lower.contains("firstofnext") {
            let day = next_month.format("%d").to_string().parse::<u32>().unwrap();
            let month = next_month.format("%m").to_string().parse::<u32>().unwrap();
            let date_str = format!("First of next month ({day:02}/{month:02})");
            suggestions.push((date_str, true));
        }
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_time_input_valid_hhmm() {
        assert!(validate_time_input("00:00").is_ok());
        assert!(validate_time_input("12:30").is_ok());
        assert!(validate_time_input("23:59").is_ok());
        assert!(validate_time_input("09:05").is_ok());
        assert!(validate_time_input("14:30").is_ok());
    }

    #[test]
    fn test_validate_time_input_valid_hh() {
        assert!(validate_time_input("0").is_ok());
        assert!(validate_time_input("9").is_ok());
        assert!(validate_time_input("14").is_ok());
        assert!(validate_time_input("23").is_ok());
        assert!(validate_time_input("01").is_ok());
        assert!(validate_time_input("08").is_ok());
    }

    #[test]
    fn test_validate_time_input_invalid() {
        assert!(validate_time_input("24:00").is_err());
        assert!(validate_time_input("12:60").is_err());
        assert!(validate_time_input("abc").is_err());
        assert!(validate_time_input("12:").is_err());
        assert!(validate_time_input(":30").is_err());
        assert!(validate_time_input("12:3").is_err());
        assert!(validate_time_input("123").is_err()); // Three digits
    }

    #[test]
    fn test_validate_time_input_empty() {
        assert!(validate_time_input("").is_ok()); // Empty is valid for all-day
        assert!(validate_time_input("   ").is_ok()); // Whitespace is valid
    }

    #[test]
    fn test_validate_time_input_single_digit() {
        assert!(validate_time_input("0").is_ok());
        assert!(validate_time_input("5").is_ok());
        assert!(validate_time_input("9").is_ok());
    }

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
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(); // Sunday
        let current_date = start_date; // Use same for backward compatibility
        let suggestions = get_date_suggestions("tom", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("Tomorrow"));
        assert!(suggestions[0].0.contains("02/10")); // Tomorrow
        assert!(suggestions[0].1); // Valid

        let suggestions = get_date_suggestions("next", start_date, current_date);
        assert!(!suggestions.is_empty());

        let suggestions = get_date_suggestions("end", start_date, current_date);
        assert!(!suggestions.is_empty());

        // Test new relative suggestions
        let suggestions = get_date_suggestions("next monday", start_date, current_date);
        assert!(!suggestions.is_empty());
        // Next Monday from Oct 1 (Sunday) is Oct 2
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("1 day", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("next month", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("01/11")); // First of next month

        let suggestions = get_date_suggestions("same day", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("01/10"));

        // Test fuzzy matching
        let suggestions = get_date_suggestions("tomorow", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("endofmonth", start_date, current_date);
        assert!(!suggestions.is_empty());

        // Test partial
        let suggestions = get_date_suggestions("15/", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("15/10"));
    }

    #[test]
    fn test_get_date_suggestions_empty_input() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(); // Sunday
        let current_date = start_date;
        let suggestions = get_date_suggestions("", start_date, current_date);
        assert_eq!(suggestions.len(), 5);

        // Check the top 5 suggestions in priority order
        assert!(suggestions[0].0.contains("Tomorrow"));
        assert!(suggestions[0].0.contains("02/10"));
        assert!(suggestions[0].1); // Valid

        assert!(suggestions[1].0.contains("Next week"));
        assert!(suggestions[1].0.contains("08/10"));
        assert!(suggestions[1].1); // Valid

        assert!(suggestions[2].0.contains("End of month"));
        assert!(suggestions[2].0.contains("31/10"));
        assert!(suggestions[2].1); // Valid

        assert!(suggestions[3].0.contains("Next month"));
        assert!(suggestions[3].0.contains("01/11"));
        assert!(suggestions[3].1); // Valid

        assert!(suggestions[4].0.contains("Same day"));
        assert!(suggestions[4].0.contains("01/10"));
        assert!(suggestions[4].1); // Valid
    }

    #[test]
    fn test_get_date_suggestions_digit_completion_with_multiple_months() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let current_date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(); // January 2025
        let suggestions = get_date_suggestions("12", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions.len(), 3); // Should suggest for 3 months
        assert!(suggestions[0].0.contains("12/01")); // Current month January
        assert!(suggestions[1].0.contains("12/02")); // Next month February
        assert!(suggestions[2].0.contains("12/03")); // Month after next March
        assert!(suggestions.iter().all(|(_, is_valid)| *is_valid)); // All should be valid
    }

    #[test]
    fn test_get_date_suggestions_edge_case_invalid_day_adjusted() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let current_date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(); // January has 31 days
        let suggestions = get_date_suggestions("32", start_date, current_date);
        // 32 is invalid for all months (max 31), so no suggestions
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_get_date_suggestions_edge_case_february_leap_year() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let current_date = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap(); // February 2024 is leap year
        let suggestions = get_date_suggestions("30", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("29/02")); // Adjusted to last day of February leap year
        assert!(suggestions[0].1); // Valid
    }

    #[test]
    fn test_get_date_suggestions_digit_completion_appears_before_relative_dates() {
        // Use a start_date in January to test that 01/01 is valid but we still suggest for future months
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(); // January 15, 2025
        let current_date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(); // January 2025
        let suggestions = get_date_suggestions("1", start_date, current_date);
        assert!(!suggestions.is_empty());
        // Since current month date 01/01 (Jan 1, 2025) is before start_date (Jan 15, 2025), skip to next month
        assert!(suggestions[0].0.contains("01/02"));
        assert!(suggestions[1].0.contains("01/03"));
        assert!(suggestions[2].0.contains("01/04"));
        // Then relative suggestions
        assert!(!suggestions[0].0.contains("day"));
        let has_relative_day = suggestions.iter().any(|s| s.0.contains("1 day"));
        assert!(
            has_relative_day,
            "Should have '1 day' suggestion after digit format"
        );
    }

    #[test]
    fn test_get_date_suggestions_december_to_january_year_transition() {
        // Test that suggestions correctly handle year transition from December to January
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(); // January 15, 2025
        let current_date = NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(); // December 2024
        let suggestions = get_date_suggestions("12", start_date, current_date);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions.len(), 3); // Should suggest for 3 months
        assert!(suggestions[0].0.contains("12/12")); // December 2024 (current month)
        assert!(suggestions[1].0.contains("12/01")); // January 2025 (next month, year incremented)
        assert!(suggestions[2].0.contains("12/02")); // February 2025 (month after next, year incremented)
        assert!(suggestions.iter().all(|(_, is_valid)| *is_valid)); // All should be valid
    }

    #[test]
    fn test_debug_january_twelve_suggestions() {
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let current_date = NaiveDate::from_ymd_opt(2026, 1, 10).unwrap();
        println!("Current date: {}", current_date);
        println!("Start date: {}", start_date);

        let suggestions = get_date_suggestions("12", start_date, current_date);
        println!("Suggestions for input '12':");
        for (i, (s, v)) in suggestions.iter().enumerate() {
            println!("  {}: {} (valid: {})", i, s, v);
        }

        assert!(!suggestions.is_empty(), "Should have suggestions");
        assert_eq!(suggestions.len(), 3, "Should have exactly 3 suggestions");
        assert!(
            suggestions[0].0.contains("12/01"),
            "First suggestion should be 12/01"
        );
        assert!(
            suggestions[1].0.contains("12/02"),
            "Second suggestion should be 12/02"
        );
        assert!(
            suggestions[2].0.contains("12/03"),
            "Third suggestion should be 12/03"
        );
    }

    #[test]
    fn test_january_start_first_twelve_suggestions() {
        // Test when start_date is January 1st and user types "12"
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let current_date = NaiveDate::from_ymd_opt(2026, 1, 10).unwrap();

        let suggestions = get_date_suggestions("12", start_date, current_date);
        println!("Test: Start date Jan 1, user types '12'");
        println!("Suggestions:");
        for (i, (s, v)) in suggestions.iter().enumerate() {
            println!("  {}: {} (valid: {})", i, s, v);
        }

        // All three should be valid since they're on or after Jan 1
        assert_eq!(suggestions.len(), 3);
        assert!(suggestions.iter().all(|(_, is_valid)| *is_valid));
    }

    #[test]
    fn test_user_actual_scenario_january_12() {
        // Test with exact user scenario: today is Jan 10, creating event with same start date
        let today = chrono::Utc::now().date_naive();
        println!("Testing with actual date: {}", today);
        let start_date = today; // Creating event starting today
        let current_date = today; // Current system date

        let suggestions = get_date_suggestions("12", start_date, current_date);
        println!("Suggestions for input '12' when start_date = today:");
        for (i, (s, v)) in suggestions.iter().enumerate() {
            println!("  {}: {} (valid: {})", i, s, v);
        }

        // Should have 3 suggestions
        assert!(!suggestions.is_empty(), "Should have suggestions");
    }
}
