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
pub fn get_date_suggestions(input: &str, start_date: NaiveDate) -> Vec<(String, bool)> {
    let mut suggestions = Vec::new();
    let input_lower = input.to_lowercase();

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
            suggestions.push((format!("{} ({:02}/{:02})", desc, day, month), true));
        }

        return suggestions;
    }

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
                suggestions.push((format!("{} ({:02}/{:02})", desc, day, month), true));
                break; // Only add once per date
            }
        }
    }

    // Next weekday suggestions
    for (weekday, date) in &next_weekdays {
        let possible = format!("next {}", weekday);
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
            suggestions.push((format!("Next {} ({:02}/{:02})", weekday, day, month), true));
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
                    if day >= 1 && day <= 31 {
                        let month = start_date.format("%m").to_string();
                        let date_str = format!("{:02}/{}", day, month);
                        let is_valid = validate_date_input(&date_str, start_date).is_ok();
                        suggestions.push((date_str, is_valid));
                    }
                }
            } else if day_part.is_empty() && !month_part.is_empty() {
                // " /10" -> complete with appropriate day (start_date day if valid, else 1)
                if let Ok(month) = month_part.parse::<u32>() {
                    if month >= 1 && month <= 12 {
                        let day = start_date.format("%d").to_string();
                        let date_str = format!("{}/{}", day, month_part);
                        let is_valid = validate_date_input(&date_str, start_date).is_ok();
                        suggestions.push((date_str, is_valid));
                    }
                }
            } else if !day_part.is_empty()
                && !month_part.is_empty()
                && (day_part.len() < 2 || month_part.len() < 2)
            {
                // Partial, show full format if matches and not full
                if let (Ok(day), Ok(month)) = (day_part.parse::<u32>(), month_part.parse::<u32>()) {
                    if day >= 1 && day <= 31 && month >= 1 && month <= 12 {
                        let date_str = format!("{:02}/{:02}", day, month);
                        let is_valid = validate_date_input(&date_str, start_date).is_ok();
                        suggestions.push((date_str, is_valid));
                    }
                }
            }
        }
    } else if input.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(num) = input.parse::<u32>() {
            if num >= 1 && num <= 31 {
                // Single or multi digit day, suggest with current month
                let month = start_date.format("%m").to_string();
                let date_str = format!("{:02}/{}", num, month);
                let is_valid = validate_date_input(&date_str, start_date).is_ok();
                suggestions.push((date_str, is_valid));
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
        let date_str = format!("Last day of month ({:02}/{:02})", day, month);
        suggestions.push((date_str, true));
    }
    if input_lower.contains("first of next") || input_lower.contains("firstofnext") {
        let day = next_month.format("%d").to_string().parse::<u32>().unwrap();
        let month = next_month.format("%m").to_string().parse::<u32>().unwrap();
        let date_str = format!("First of next month ({:02}/{:02})", day, month);
        suggestions.push((date_str, true));
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let suggestions = get_date_suggestions("tom", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("Tomorrow"));
        assert!(suggestions[0].0.contains("02/10")); // Tomorrow
        assert!(suggestions[0].1); // Valid

        let suggestions = get_date_suggestions("next", start_date);
        assert!(!suggestions.is_empty());

        let suggestions = get_date_suggestions("end", start_date);
        assert!(!suggestions.is_empty());

        // Test new relative suggestions
        let suggestions = get_date_suggestions("next monday", start_date);
        assert!(!suggestions.is_empty());
        // Next Monday from Oct 1 (Sunday) is Oct 2
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("1 day", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("next month", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("01/11")); // First of next month

        let suggestions = get_date_suggestions("same day", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("01/10"));

        // Test fuzzy matching
        let suggestions = get_date_suggestions("tomorow", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("02/10"));

        let suggestions = get_date_suggestions("endofmonth", start_date);
        assert!(!suggestions.is_empty());

        // Test partial
        let suggestions = get_date_suggestions("15/", start_date);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].0.contains("15/10"));
    }

    #[test]
    fn test_get_date_suggestions_empty_input() {
        let start_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(); // Sunday
        let suggestions = get_date_suggestions("", start_date);
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
}
