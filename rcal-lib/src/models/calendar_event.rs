//! CalendarEvent and Recurrence types.
//!
//! These are the core domain models for calendar events, including support
//! for recurring events.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents the recurrence pattern for a calendar event.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Recurrence {
    /// No recurrence - single event
    #[default]
    None,
    /// Event repeats daily
    Daily,
    /// Event repeats weekly
    Weekly,
    /// Event repeats monthly
    Monthly,
    /// Event repeats yearly
    Yearly,
}

impl Recurrence {
    /// Returns true if the recurrence is not None.
    pub fn is_recurring(&self) -> bool {
        !matches!(self, Recurrence::None)
    }

    /// Converts the recurrence to a string representation for storage.
    pub fn to_storage_string(&self) -> &'static str {
        match self {
            Recurrence::None => "none",
            Recurrence::Daily => "daily",
            Recurrence::Weekly => "weekly",
            Recurrence::Monthly => "monthly",
            Recurrence::Yearly => "yearly",
        }
    }

    /// Parses a recurrence from a storage string.
    pub fn from_storage_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "daily" => Recurrence::Daily,
            "weekly" => Recurrence::Weekly,
            "monthly" => Recurrence::Monthly,
            "yearly" => Recurrence::Yearly,
            _ => Recurrence::None,
        }
    }
}

/// A calendar event representing either a base event or a recurring instance.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// Unique identifier for the event.
    pub id: String,
    /// Title of the event.
    pub title: String,
    /// Optional description of the event.
    pub description: String,
    /// Recurrence pattern for the event.
    pub recurrence: Recurrence,
    /// Whether this event is a generated recurring instance (vs a base event).
    pub is_recurring_instance: bool,
    /// For recurring instances, the date of the base event this instance derives from.
    pub base_date: Option<NaiveDate>,
    /// Start date of the event.
    pub start_date: NaiveDate,
    /// Optional end date (for multi-day events).
    pub end_date: Option<NaiveDate>,
    /// Start time of the event.
    pub start_time: chrono::NaiveTime,
    /// Optional end time.
    pub end_time: Option<chrono::NaiveTime>,
    /// Whether this is an all-day event.
    pub is_all_day: bool,
}

impl CalendarEvent {
    /// Creates a new calendar event with the given parameters.
    pub fn new(
        title: String,
        description: String,
        start_date: NaiveDate,
        start_time: chrono::NaiveTime,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date,
            end_date: None,
            start_time,
            end_time: None,
            is_all_day: false,
        }
    }

    /// Returns the effective end date, defaulting to start_date if not set.
    pub fn effective_end_date(&self) -> NaiveDate {
        self.end_date.unwrap_or(self.start_date)
    }

    /// Returns true if the event occurs on the given date.
    pub fn occurs_on(&self, date: NaiveDate) -> bool {
        if self.is_recurring_instance {
            self.start_date == date
        } else {
            self.start_date <= date && date <= self.effective_end_date()
        }
    }

    /// Returns true if this is a multi-day event.
    pub fn is_multi_day(&self) -> bool {
        self.end_date
            .map(|end| end > self.start_date)
            .unwrap_or(false)
    }

    /// Creates a recurring instance from this base event for a specific date.
    pub fn create_instance(&self, instance_date: NaiveDate) -> Self {
        let duration = self
            .end_date
            .map(|end| end - self.start_date)
            .or_else(|| Some(chrono::Duration::days(0)));

        CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            title: self.title.clone(),
            description: self.description.clone(),
            recurrence: Recurrence::None,
            is_recurring_instance: true,
            base_date: Some(self.start_date),
            start_date: instance_date,
            end_date: duration.map(|d| instance_date + d),
            start_time: self.start_time,
            end_time: self.end_time,
            is_all_day: self.is_all_day,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_recurrence_is_recurring() {
        assert!(!Recurrence::None.is_recurring());
        assert!(Recurrence::Daily.is_recurring());
        assert!(Recurrence::Weekly.is_recurring());
        assert!(Recurrence::Monthly.is_recurring());
        assert!(Recurrence::Yearly.is_recurring());
    }

    #[test]
    fn test_recurrence_to_storage_string() {
        assert_eq!(Recurrence::None.to_storage_string(), "none");
        assert_eq!(Recurrence::Daily.to_storage_string(), "daily");
        assert_eq!(Recurrence::Weekly.to_storage_string(), "weekly");
        assert_eq!(Recurrence::Monthly.to_storage_string(), "monthly");
        assert_eq!(Recurrence::Yearly.to_storage_string(), "yearly");
    }

    #[test]
    fn test_recurrence_from_storage_string() {
        assert_eq!(Recurrence::from_storage_string("daily"), Recurrence::Daily);
        assert_eq!(Recurrence::from_storage_string("DAILY"), Recurrence::Daily);
        assert_eq!(Recurrence::from_storage_string("unknown"), Recurrence::None);
    }

    #[test]
    fn test_calendar_event_occurs_on() {
        let event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );

        assert!(event.occurs_on(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(!event.occurs_on(NaiveDate::from_ymd_opt(2024, 1, 16).unwrap()));
    }

    #[test]
    fn test_calendar_event_multi_day() {
        let mut event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );

        assert!(!event.is_multi_day());

        event.end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 17).unwrap());
        assert!(event.is_multi_day());
    }

    #[test]
    fn test_calendar_event_create_instance() {
        let base_event = CalendarEvent {
            id: "base-id".to_string(),
            title: "Weekly Meeting".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::Weekly,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let instance = base_event.create_instance(NaiveDate::from_ymd_opt(2024, 1, 22).unwrap());

        assert!(instance.is_recurring_instance);
        assert_eq!(
            instance.base_date,
            Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
        );
        assert_eq!(
            instance.start_date,
            NaiveDate::from_ymd_opt(2024, 1, 22).unwrap()
        );
    }
}
