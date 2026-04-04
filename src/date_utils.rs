// Re-export all date utilities from rcal-lib
pub use rcal_lib::core::date_utils::{
    get_date_suggestions, validate_date_input, validate_time_input,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

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
        println!("Current date: {current_date}");
        println!("Start date: {start_date}");

        let suggestions = get_date_suggestions("12", start_date, current_date);
        println!("Suggestions for input '12':");
        for (i, (s, v)) in suggestions.iter().enumerate() {
            println!("  {i}: {s} (valid: {v})");
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
            println!("  {i}: {s} (valid: {v})");
        }

        // All three should be valid since they're on or after Jan 1
        assert_eq!(suggestions.len(), 3);
        assert!(suggestions.iter().all(|(_, is_valid)| *is_valid));
    }

    #[test]
    fn test_user_actual_scenario_january_12() {
        // Test with exact user scenario: today is Jan 10, creating event with same start date
        let today = chrono::Utc::now().date_naive();
        println!("Testing with actual date: {today}");
        let start_date = today; // Creating event starting today
        let current_date = today; // Current system date

        let suggestions = get_date_suggestions("12", start_date, current_date);
        println!("Suggestions for input '12' when start_date = today:");
        for (i, (s, v)) in suggestions.iter().enumerate() {
            println!("  {i}: {s} (valid: {v})");
        }

        // Should have 3 suggestions
        assert!(!suggestions.is_empty(), "Should have suggestions");
    }
}
