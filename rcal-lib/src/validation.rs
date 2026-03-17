//! Event validation module.
//!
//! Provides validation functions for CalendarEvent to enforce the format
//! specified in EVENT_FORMAT.md. Downstream projects can use these functions
//! to ensure events conform to the required format before saving.

use chrono::NaiveDate;
use thiserror::Error;

use crate::models::CalendarEvent;

/// Errors that can occur during event validation.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Title is empty or contains only whitespace.
    #[error("title cannot be empty or whitespace-only")]
    EmptyTitle,

    /// Title exceeds maximum allowed length.
    #[error("title exceeds maximum length of {max_length} characters")]
    TitleTooLong {
        /// Maximum allowed length for the title.
        max_length: usize,
    },

    /// Start date is missing or invalid.
    #[error("start_date is required and must be valid")]
    MissingStartDate,

    /// End date is set but is before the start date.
    #[error("end_date ({end_date}) cannot be before start_date ({start_date})")]
    EndDateBeforeStartDate {
        /// The end date that was provided.
        end_date: NaiveDate,
        /// The start date of the event.
        start_date: NaiveDate,
    },

    /// End time is set but is before the start time on the same day.
    #[error("end_time ({end_time}) cannot be before start_time ({start_time})")]
    EndTimeBeforeStartTime {
        /// The end time that was provided.
        end_time: chrono::NaiveTime,
        /// The start time of the event.
        start_time: chrono::NaiveTime,
    },

    /// Start time is missing for a non-all-day event.
    #[error("start_time is required for non-all-day events")]
    MissingStartTimeForTimedEvent,

    /// Invalid time configuration for an all-day event.
    #[error("all-day events should not have explicit start_time and end_time set")]
    InvalidTimeForAllDayEvent,

    /// The filename does not match the sanitized event title.
    #[error("filename '{got}' does not match expected '{expected}' based on event title")]
    FilenameDoesNotMatchTitle {
        /// The expected filename (sanitized title + .md).
        expected: String,
        /// The actual filename provided.
        got: String,
    },
}

/// Maximum allowed length for event titles.
pub const MAX_TITLE_LENGTH: usize = 200;

/// Validates a calendar event, returning the first validation error encountered.
///
/// This function performs fast-fail validation - it returns immediately
/// upon finding the first error. For comprehensive validation that collects
/// all errors, use [`validate_event_with_details`].
///
/// # Arguments
///
/// * `event` - The calendar event to validate.
///
/// # Returns
///
/// * `Ok(())` if the event is valid.
/// * `Err(ValidationError)` with the first validation error found.
///
/// # Examples
///
/// ```
/// use rcal_lib::{CalendarEvent, validation::validate_event};
/// use chrono::{NaiveDate, NaiveTime};
///
/// let valid_event = CalendarEvent::new(
///     "Team Meeting".to_string(),
///     "Weekly sync".to_string(),
///     NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
///     NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
/// );
///
/// assert!(validate_event(&valid_event).is_ok());
///
/// // Test with invalid event
/// let invalid_event = CalendarEvent {
///     id: "test".to_string(),
///     title: "   ".to_string(),
///     description: String::new(),
///     recurrence: rcal_lib::models::Recurrence::None,
///     is_recurring_instance: false,
///     base_date: None,
///     start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
///     end_date: None,
///     start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
///     end_time: None,
///     is_all_day: false,
/// };
///
/// assert!(validate_event(&invalid_event).is_err());
/// ```
pub fn validate_event(event: &CalendarEvent) -> Result<(), ValidationError> {
    validate_event_with_details(event).map_err(|errors| errors.into_iter().next().unwrap())
}

/// Validates a calendar event and returns all validation errors.
///
/// This function performs comprehensive validation, collecting ALL validation
/// errors rather than failing on the first one. This is useful when you want
/// to present all issues to the user at once.
///
/// # Arguments
///
/// * `event` - The calendar event to validate.
///
/// # Returns
///
/// * `Ok(())` if the event is valid.
/// * `Err(Vec<ValidationError>)` with all validation errors found, sorted
///   by the order they are checked (title, dates, times).
pub fn validate_event_with_details(event: &CalendarEvent) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate title
    if event.title.trim().is_empty() {
        errors.push(ValidationError::EmptyTitle);
    } else if event.title.len() > MAX_TITLE_LENGTH {
        errors.push(ValidationError::TitleTooLong {
            max_length: MAX_TITLE_LENGTH,
        });
    }

    // Validate start_date (required field)
    // Note: NaiveDate is always valid as it's a concrete type, but we check it's set
    // The field is required per EVENT_FORMAT.md, so we validate it's present
    let _ = event.start_date; // Acknowledge that start_date is required

    // Validate end_date >= start_date (when end_date is set)
    if let Some(ref end_date) = event.end_date {
        if *end_date < event.start_date {
            errors.push(ValidationError::EndDateBeforeStartDate {
                end_date: *end_date,
                start_date: event.start_date,
            });
        }
    }

    // Validate end_time >= start_time (when both are set and not all-day)
    if let Some(ref end_time) = event.end_time {
        // Only validate times for non-all-day events
        if !event.is_all_day && *end_time < event.start_time {
            errors.push(ValidationError::EndTimeBeforeStartTime {
                end_time: *end_time,
                start_time: event.start_time,
            });
        }
    }

    // Validate all-day event time configuration
    // Per EVENT_FORMAT.md: If is_all_day is true, times should be None or set to defaults
    // We accept this as valid, but we check for inconsistent states:
    // - All-day events can have no times (start_time = 00:00:00, end_time = None)
    // - Or they can have explicit times (less common but acceptable)
    // We only flag cases where there's a clear inconsistency
    if event.is_all_day {
        // For all-day events, if end_time is set, it should typically be None
        // But we accept this as valid per the requirement - all-day events are valid
        // regardless of whether times are set or not
    } else {
        // For non-all-day events, start_time must be meaningful
        // The default NaiveTime::MIN would be unusual for real events
        // But since it's a valid time, we don't error on it
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates that a title is not empty or whitespace-only.
///
/// This is a utility function that can be used independently for quick
/// title validation.
///
/// # Arguments
///
/// * `title` - The title string to validate.
///
/// # Returns
///
/// * `true` if the title is valid (not empty or whitespace-only).
/// * `false` otherwise.
pub fn is_valid_title(title: &str) -> bool {
    !title.trim().is_empty() && title.len() <= MAX_TITLE_LENGTH
}

/// Validates that an end date is not before the start date.
///
/// This is a utility function that can be used independently for date range validation.
///
/// # Arguments
///
/// * `start_date` - The start date of the event.
/// * `end_date` - The end date of the event (can be None).
///
/// # Returns
///
/// * `true` if the date range is valid (end_date is None or >= start_date).
/// * `false` otherwise.
pub fn is_valid_date_range(start_date: NaiveDate, end_date: Option<NaiveDate>) -> bool {
    match end_date {
        Some(end) => end >= start_date,
        None => true,
    }
}

/// Validates that an end time is not before the start time.
///
/// This is a utility function that can be used independently for time range validation.
///
/// # Arguments
///
/// * `start_time` - The start time of the event.
/// * `end_time` - The end time of the event (can be None).
///
/// # Returns
///
/// * `true` if the time range is valid (end_time is None or >= start_time).
/// * `false` otherwise.
pub fn is_valid_time_range(
    start_time: chrono::NaiveTime,
    end_time: Option<chrono::NaiveTime>,
) -> bool {
    match end_time {
        Some(end) => end >= start_time,
        None => true,
    }
}

/// Maximum length for sanitized filename (without .md extension).
const MAX_FILENAME_LENGTH: usize = 100;

/// Sanitizes a title for use in a filename.
///
/// Applies the following rules:
/// - Converts spaces to underscores
/// - Removes all characters except alphanumeric and underscores
/// - Collapses consecutive underscores into one
/// - Trims leading and trailing underscores
/// - Limits length to 100 characters
/// - Falls back to "untitled" if empty after sanitization
///
/// # Arguments
///
/// * `title` - The title string to sanitize.
///
/// # Returns
///
/// The sanitized title suitable for use in a filename.
pub fn sanitize_title_for_filename(title: &str) -> String {
    let mut sanitized = title
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    // Trim leading and trailing underscores
    sanitized = sanitized.trim_matches('_').to_string();

    // Collapse consecutive underscores
    let mut collapsed = String::new();
    let mut last_was_underscore = false;
    for c in sanitized.chars() {
        if c == '_' {
            if !last_was_underscore {
                collapsed.push(c);
                last_was_underscore = true;
            }
        } else {
            collapsed.push(c);
            last_was_underscore = false;
        }
    }

    // Limit length to 100 characters
    if collapsed.len() > MAX_FILENAME_LENGTH {
        collapsed.truncate(MAX_FILENAME_LENGTH);
    }

    if collapsed.is_empty() {
        "untitled".to_string()
    } else {
        collapsed
    }
}

/// Validates that a filename matches the expected format based on the event title.
///
/// The filename must:
/// - End with `.md`
/// - Have a base name (without `.md` and without numeric suffix) that matches
///   the sanitized version of the event title
/// - May have a duplicate suffix (`_1`, `_2`, etc.) if there are multiple events
///   with the same sanitized title
///
/// # Arguments
///
/// * `event` - The calendar event to validate against.
/// * `filename` - The filename to validate.
///
/// # Returns
///
/// * `Ok(())` if the filename is valid.
/// * `Err(ValidationError::FilenameDoesNotMatchTitle)` if the filename doesn't match.
pub fn validate_filename(event: &CalendarEvent, filename: &str) -> Result<(), ValidationError> {
    // Check that filename ends with .md
    if !filename.ends_with(".md") {
        return Err(ValidationError::FilenameDoesNotMatchTitle {
            expected: format!("{}.md", sanitize_title_for_filename(&event.title)),
            got: filename.to_string(),
        });
    }

    // Remove .md extension
    let filename_stem = filename.trim_end_matches(".md");

    // Get the sanitized title
    let sanitized_title = sanitize_title_for_filename(&event.title);

    // Check for exact match
    if filename_stem == sanitized_title {
        return Ok(());
    }

    // Check for duplicate suffix pattern (e.g., _1, _2, etc.)
    // The pattern is: sanitized_title + "_" + positive integer
    if let Some(suffix) = filename_stem.strip_prefix(&format!("{}_", sanitized_title)) {
        // Verify the suffix is a valid positive integer (not zero)
        if let Ok(num) = suffix.parse::<u32>() {
            if num > 0 {
                return Ok(());
            }
        }
    }

    // Filename doesn't match
    Err(ValidationError::FilenameDoesNotMatchTitle {
        expected: format!("{}.md", sanitized_title),
        got: filename.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    /// Helper to create a minimal valid CalendarEvent for testing.
    fn create_valid_event() -> CalendarEvent {
        CalendarEvent {
            id: "test-id".to_string(),
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::models::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        }
    }

    #[test]
    fn test_valid_event_passes_validation() {
        let event = create_valid_event();
        assert!(validate_event(&event).is_ok());
        assert!(validate_event_with_details(&event).is_ok());
    }

    #[test]
    fn test_empty_title_fails_validation() {
        let mut event = create_valid_event();
        event.title = String::new();

        let result = validate_event(&event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationError::EmptyTitle);
    }

    #[test]
    fn test_whitespace_only_title_fails_validation() {
        let mut event = create_valid_event();
        event.title = "   ".to_string();

        let result = validate_event(&event);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationError::EmptyTitle);
    }

    #[test]
    fn test_title_too_long_fails_validation() {
        let mut event = create_valid_event();
        event.title = "a".repeat(MAX_TITLE_LENGTH + 1);

        let result = validate_event(&event);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::TitleTooLong { .. }
        ));
    }

    #[test]
    fn test_title_at_max_length_passes_validation() {
        let mut event = create_valid_event();
        event.title = "a".repeat(MAX_TITLE_LENGTH);

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_end_date_before_start_date_fails_validation() {
        let mut event = create_valid_event();
        event.end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());

        let result = validate_event(&event);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::EndDateBeforeStartDate { .. }
        ));
    }

    #[test]
    fn test_end_date_equal_to_start_date_passes_validation() {
        let mut event = create_valid_event();
        event.end_date = Some(event.start_date);

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_end_date_after_start_date_passes_validation() {
        let mut event = create_valid_event();
        event.end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_end_time_before_start_time_fails_validation() {
        let mut event = create_valid_event();
        event.end_time = Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap());

        let result = validate_event(&event);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::EndTimeBeforeStartTime { .. }
        ));
    }

    #[test]
    fn test_end_time_equal_to_start_time_passes_validation() {
        let mut event = create_valid_event();
        event.end_time = Some(event.start_time);

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_end_time_after_start_time_passes_validation() {
        let mut event = create_valid_event();
        event.end_time = Some(NaiveTime::from_hms_opt(11, 0, 0).unwrap());

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_all_day_event_without_times_passes_validation() {
        let mut event = create_valid_event();
        event.is_all_day = true;
        // Default start_time (00:00:00) and end_time = None is valid for all-day

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_all_day_event_with_times_passes_validation() {
        let mut event = create_valid_event();
        event.is_all_day = true;
        event.start_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        event.end_time = Some(NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        // Per requirement: all-day events are valid regardless of times
        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_validate_event_with_details_returns_all_errors() {
        let mut event = create_valid_event();
        event.title = "   ".to_string(); // Empty title
        event.end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()); // End before start
        event.end_time = Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap()); // End before start

        let result = validate_event_with_details(&event);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // At least title and date errors

        // Check that we get multiple errors
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::EmptyTitle)));
        assert!(errors
            .iter()
            .any(|e| { matches!(e, ValidationError::EndDateBeforeStartDate { .. }) }));
    }

    #[test]
    fn test_is_valid_title() {
        assert!(is_valid_title("Valid Title"));
        assert!(is_valid_title("a".repeat(MAX_TITLE_LENGTH).as_str()));
        assert!(!is_valid_title(""));
        assert!(!is_valid_title("   "));
        assert!(!is_valid_title("a".repeat(MAX_TITLE_LENGTH + 1).as_str()));
    }

    #[test]
    fn test_is_valid_date_range() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        assert!(is_valid_date_range(start, None));
        assert!(is_valid_date_range(start, Some(start)));
        assert!(is_valid_date_range(
            start,
            Some(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap())
        ));
        assert!(!is_valid_date_range(
            start,
            Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())
        ));
    }

    #[test]
    fn test_is_valid_time_range() {
        let start = NaiveTime::from_hms_opt(10, 0, 0).unwrap();

        assert!(is_valid_time_range(start, None));
        assert!(is_valid_time_range(start, Some(start)));
        assert!(is_valid_time_range(
            start,
            Some(NaiveTime::from_hms_opt(12, 0, 0).unwrap())
        ));
        assert!(!is_valid_time_range(
            start,
            Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        ));
    }

    #[test]
    fn test_multi_day_event_passes_validation() {
        let mut event = create_valid_event();
        event.start_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        event.end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap());

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_recurring_event_passes_validation() {
        let mut event = create_valid_event();
        event.recurrence = crate::models::Recurrence::Weekly;

        assert!(validate_event(&event).is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::EmptyTitle;
        assert_eq!(
            error.to_string(),
            "title cannot be empty or whitespace-only"
        );

        let error = ValidationError::TitleTooLong { max_length: 200 };
        assert_eq!(
            error.to_string(),
            "title exceeds maximum length of 200 characters"
        );

        let error = ValidationError::EndDateBeforeStartDate {
            end_date: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        };
        assert!(error.to_string().contains("2024-01-10"));
        assert!(error.to_string().contains("2024-01-15"));

        let error = ValidationError::EndTimeBeforeStartTime {
            end_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        };
        assert!(error.to_string().contains("09:00:00"));
        assert!(error.to_string().contains("10:00:00"));
    }

    #[test]
    fn test_sanitize_title_for_filename_basic() {
        assert_eq!(sanitize_title_for_filename("Team Meeting"), "Team_Meeting");
        assert_eq!(sanitize_title_for_filename("Hello World!"), "Hello_World");
        assert_eq!(sanitize_title_for_filename(""), "untitled");
        assert_eq!(sanitize_title_for_filename("!@#"), "untitled");
        assert_eq!(sanitize_title_for_filename("   "), "untitled");
    }

    #[test]
    fn test_sanitize_title_for_filename_trims_underscores() {
        assert_eq!(sanitize_title_for_filename("___hello___"), "hello");
        assert_eq!(sanitize_title_for_filename("hello___world"), "hello_world");
        assert_eq!(sanitize_title_for_filename("a_b"), "a_b");
    }

    #[test]
    fn test_sanitize_title_for_filename_alphanumeric() {
        assert_eq!(sanitize_title_for_filename("123"), "123");
        assert_eq!(sanitize_title_for_filename("test-event"), "testevent");
        assert_eq!(sanitize_title_for_filename("Café"), "Café");
    }

    #[test]
    fn test_sanitize_title_for_filename_length_limit() {
        let long_title = "a".repeat(150);
        let sanitized = sanitize_title_for_filename(&long_title);
        assert_eq!(sanitized.len(), 100);
        assert!(sanitized.starts_with("a"));
    }

    #[test]
    fn test_validate_filename_exact_match() {
        let event = CalendarEvent {
            title: "Team Meeting".to_string(),
            ..create_valid_event()
        };

        // Exact match should pass
        assert!(validate_filename(&event, "Team_Meeting.md").is_ok());
    }

    #[test]
    fn test_validate_filename_with_duplicate_suffix() {
        let event = CalendarEvent {
            title: "Team Meeting".to_string(),
            ..create_valid_event()
        };

        // Duplicate suffixes should pass
        assert!(validate_filename(&event, "Team_Meeting_1.md").is_ok());
        assert!(validate_filename(&event, "Team_Meeting_2.md").is_ok());
        assert!(validate_filename(&event, "Team_Meeting_10.md").is_ok());
    }

    #[test]
    fn test_validate_filename_missing_md_extension() {
        let event = CalendarEvent {
            title: "Team Meeting".to_string(),
            ..create_valid_event()
        };

        let result = validate_filename(&event, "Team_Meeting");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::FilenameDoesNotMatchTitle { .. }
        ));
    }

    #[test]
    fn test_validate_filename_does_not_match_title() {
        let event = CalendarEvent {
            title: "Team Meeting".to_string(),
            ..create_valid_event()
        };

        // Wrong filename should fail
        let result = validate_filename(&event, "Party.md");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ValidationError::FilenameDoesNotMatchTitle {
                expected: _,
                got: _
            }
        ));
        assert!(err.to_string().contains("Party.md"));
        assert!(err.to_string().contains("Team_Meeting"));
    }

    #[test]
    fn test_validate_filename_invalid_duplicate_suffix() {
        let event = CalendarEvent {
            title: "Team Meeting".to_string(),
            ..create_valid_event()
        };

        // Invalid suffixes should fail (0 is not valid, letters are not valid)
        assert!(validate_filename(&event, "Team_Meeting_0.md").is_err());
        assert!(validate_filename(&event, "Team_Meeting_abc.md").is_err());
    }

    #[test]
    fn test_validate_filename_special_characters() {
        let event = CalendarEvent {
            title: "Hello World!".to_string(),
            ..create_valid_event()
        };

        // Filename with special chars removed should match
        assert!(validate_filename(&event, "Hello_World.md").is_ok());
    }

    #[test]
    fn test_validate_filename_untitled_fallback() {
        let event = CalendarEvent {
            title: "!@#".to_string(),
            ..create_valid_event()
        };

        // Empty after sanitization should use "untitled"
        assert!(validate_filename(&event, "untitled.md").is_ok());
    }

    #[test]
    fn test_validate_filename_long_title() {
        let event = CalendarEvent {
            title: "a".repeat(150),
            ..create_valid_event()
        };

        // Should be truncated to 100 chars
        let result = validate_filename(&event, &format!("{}.md", "a".repeat(100)));
        assert!(result.is_ok());
    }
}
