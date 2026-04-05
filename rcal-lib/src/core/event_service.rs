//! Event service - business logic for calendar events.
//!
//! This module provides the EventService which handles event business logic,
//! including event validation, instance generation, and caching.

use chrono::{Datelike, Duration, NaiveDate};

use crate::models::{CalendarEvent, Recurrence};

/// Buffer days for instance generation to support smooth UI navigation.
const INSTANCE_BUFFER_DAYS: i64 = 365;

/// EventService handles business logic for calendar events.
/// It provides methods for validating events, generating recurring instances,
/// and managing event state.
pub struct EventService {
    /// Base events (non-instance events).
    events: Vec<CalendarEvent>,
    /// Cached recurring event instances.
    cached_instances: Vec<CalendarEvent>,
    /// The cached date range for instance generation.
    cached_range: Option<(NaiveDate, NaiveDate)>,
}

impl EventService {
    /// Creates a new EventService with no events.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            cached_instances: Vec::new(),
            cached_range: None,
        }
    }

    /// Creates a new EventService with the given initial events.
    pub fn with_events(events: Vec<CalendarEvent>) -> Self {
        Self {
            events,
            cached_instances: Vec::new(),
            cached_range: None,
        }
    }

    /// Returns a reference to all base events.
    pub fn events(&self) -> &[CalendarEvent] {
        &self.events
    }

    /// Returns a mutable reference to all base events.
    pub fn events_mut(&mut self) -> &mut Vec<CalendarEvent> {
        &mut self.events
    }

    /// Sets the events, clearing any cached instances.
    pub fn set_events(&mut self, events: Vec<CalendarEvent>) {
        self.events = events;
        self.invalidate_instance_cache(None);
    }

    /// Adds a new event to the service.
    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
        self.invalidate_instance_cache(None);
    }

    /// Removes an event by ID.
    pub fn remove_event(&mut self, id: &str) -> Option<CalendarEvent> {
        if let Some(pos) = self.events.iter().position(|e| e.id == id) {
            let event = self.events.remove(pos);
            self.invalidate_instance_cache(Some(&event));
            Some(event)
        } else {
            None
        }
    }

    /// Updates an existing event.
    pub fn update_event(&mut self, event: CalendarEvent) -> Option<CalendarEvent> {
        if let Some(pos) = self.events.iter().position(|e| e.id == event.id) {
            let old_event = std::mem::replace(&mut self.events[pos], event);
            self.invalidate_instance_cache(Some(&old_event));
            Some(old_event)
        } else {
            None
        }
    }

    /// Retrieves all events (base events + generated instances) for the given date range.
    /// Uses session-level caching to avoid regenerating instances for the same range.
    /// Generates instances with a buffer (INSTANCE_BUFFER_DAYS) around the requested range
    /// to support smooth navigation without frequent regenerations.
    pub fn get_all_events_for_range(
        &mut self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Vec<CalendarEvent> {
        let buffer = Duration::days(INSTANCE_BUFFER_DAYS);
        let gen_start = start - buffer;
        let gen_end = end + buffer;
        if self.cached_range != Some((gen_start, gen_end)) {
            self.cached_instances =
                Self::generate_instances_for_range_internal(&self.events, gen_start, gen_end);
            self.cached_range = Some((gen_start, gen_end));
        }
        let mut all = self.events.clone();
        // Filter out non-recurring events from instances since they're already in self.events
        let recurring_instances: Vec<_> = self
            .cached_instances
            .iter()
            .filter(|e| e.is_recurring_instance)
            .cloned()
            .collect();
        all.extend(recurring_instances);
        all.sort_by(|a, b| {
            a.start_date
                .cmp(&b.start_date)
                .then(a.start_time.cmp(&b.start_time))
        });
        all
    }

    /// Invalidates the cached recurring event instances.
    /// If an event is provided, only instances related to that event are removed (selective invalidation).
    /// If no event is provided, all cached instances are cleared.
    /// Call this after events are added, deleted, or edited to ensure
    /// lazy loading refreshes the display with accurate instances.
    pub fn invalidate_instance_cache(&mut self, event: Option<&CalendarEvent>) {
        if let Some(event) = event {
            // Selective invalidation: remove only instances related to this event
            self.cached_instances.retain(|instance| {
                // Keep instances that don't match the event's title and base_date
                !(instance.title == event.title && instance.base_date == Some(event.start_date))
            });
            // Note: cached_range is kept, as other events' instances may still be valid
        } else {
            // Full invalidation
            self.cached_range = None;
            self.cached_instances.clear();
        }
    }

    /// Generates recurring event instances for the given base events within the specified date range.
    /// This function implements lazy loading by creating instances only for the requested period,
    /// with a buffer to ensure smooth UI navigation.
    pub fn generate_instances_for_range(
        &mut self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Vec<CalendarEvent> {
        Self::generate_instances_for_range_internal(&self.events, start_date, end_date)
    }

    /// Internal helper for generating instances.
    fn generate_instances_for_range_internal(
        base_events: &[CalendarEvent],
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Vec<CalendarEvent> {
        let mut instances = vec![];
        for base_event in base_events {
            if base_event.recurrence != Recurrence::None {
                instances.extend(Self::generate_recurring_instances_in_range(
                    base_event, start_date, end_date,
                ));
            } else if base_event.start_date >= start_date && base_event.start_date <= end_date {
                // Only include non-recurring events that fall within the date range
                instances.push(base_event.clone());
            }
        }
        instances
    }

    /// Helper function to advance year for yearly recurring events with Feb 29th fallback.
    fn advance_year_with_feb29_fallback(current_date: NaiveDate) -> NaiveDate {
        let next_year = current_date.year() + 1;
        if let Some(new_date) = current_date.with_year(next_year) {
            new_date
        } else if current_date.month() == 2 && current_date.day() == 29 {
            NaiveDate::from_ymd_opt(next_year, 2, 28).unwrap_or_else(|| {
                eprintln!("Warning: Could not create Feb 28 for year {next_year}");
                current_date
            })
        } else {
            eprintln!("Warning: Could not advance year from {current_date:?}");
            current_date
        }
    }

    /// Generates recurring instances for a single base event within a date range.
    fn generate_recurring_instances_in_range(
        base_event: &CalendarEvent,
        range_start: NaiveDate,
        range_end: NaiveDate,
    ) -> Vec<CalendarEvent> {
        let mut instances = vec![];
        let mut current_date = base_event.start_date;

        // Skip to the first date >= range_start
        while current_date < range_start {
            match base_event.recurrence {
                Recurrence::Daily => current_date += Duration::days(1),
                Recurrence::Weekly => current_date += Duration::weeks(1),
                Recurrence::Monthly => {
                    if let Some(new_date) = current_date.with_month(current_date.month() + 1) {
                        current_date = new_date;
                    } else {
                        return instances; // Stop if invalid
                    }
                }
                Recurrence::Yearly => {
                    current_date = Self::advance_year_with_feb29_fallback(current_date);
                }
                Recurrence::None => return instances,
            }
        }

        // Now generate from current_date to range_end
        while current_date <= range_end {
            if current_date != base_event.start_date {
                let end_date = base_event.end_date.map(|end| {
                    let duration = end - base_event.start_date;
                    current_date + duration
                });
                instances.push(CalendarEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: base_event.title.clone(),
                    description: base_event.description.clone(),
                    recurrence: Recurrence::None,
                    is_recurring_instance: true,
                    base_date: Some(base_event.start_date),
                    start_date: current_date,
                    end_date,
                    start_time: base_event.start_time,
                    end_time: base_event.end_time,
                    is_all_day: base_event.is_all_day,
                });
            }

            match base_event.recurrence {
                Recurrence::Daily => current_date += Duration::days(1),
                Recurrence::Weekly => current_date += Duration::weeks(1),
                Recurrence::Monthly => {
                    if let Some(new_date) = current_date.with_month(current_date.month() + 1) {
                        current_date = new_date;
                    } else {
                        eprintln!(
                            "Warning: Invalid date for recurring event '{}': {:?}",
                            base_event.title, current_date
                        );
                        break;
                    }
                }
                Recurrence::Yearly => {
                    current_date = Self::advance_year_with_feb29_fallback(current_date);
                }
                Recurrence::None => break,
            }
        }
        instances
    }

    /// Validates a calendar event and returns validation errors.
    pub fn validate_event(event: &CalendarEvent) -> Vec<String> {
        let mut errors = Vec::new();

        // Validate title
        if event.title.trim().is_empty() {
            errors.push("Title cannot be empty".to_string());
        }

        // Validate dates
        if event.start_date > NaiveDate::from_ymd_opt(9999, 12, 31).unwrap() {
            errors.push("Start date is too far in the future".to_string());
        }

        // Validate end date is after start date
        if let Some(end_date) = event.end_date {
            if end_date < event.start_date {
                errors.push("End date must be on or after start date".to_string());
            }
        }

        // Validate times for non-all-day events
        if !event.is_all_day {
            if let Some(end_time) = event.end_time {
                if end_time < event.start_time && event.end_date.is_none() {
                    errors
                        .push("End time must be after start time for same-day events".to_string());
                }
            }
        }

        errors
    }

    /// Returns true if the event is valid (has no validation errors).
    pub fn is_valid_event(event: &CalendarEvent) -> bool {
        Self::validate_event(event).is_empty()
    }
}

impl Default for EventService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_event_service_new() {
        let service = EventService::new();
        assert!(service.events().is_empty());
    }

    #[test]
    fn test_event_service_with_events() {
        let event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        let service = EventService::with_events(vec![event.clone()]);
        assert_eq!(service.events().len(), 1);
    }

    #[test]
    fn test_event_service_add_event() {
        let mut service = EventService::new();
        let event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        service.add_event(event);
        assert_eq!(service.events().len(), 1);
    }

    #[test]
    fn test_event_service_remove_event() {
        let mut service = EventService::new();
        let event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        let id = event.id.clone();
        service.add_event(event);
        let removed = service.remove_event(&id);
        assert!(removed.is_some());
        assert!(service.events().is_empty());
    }

    #[test]
    fn test_event_service_update_event() {
        let mut service = EventService::new();
        let mut event = CalendarEvent::new(
            "Test".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        service.add_event(event.clone());

        event.title = "Updated".to_string();
        let old = service.update_event(event);
        assert!(old.is_some());
        assert_eq!(service.events()[0].title, "Updated");
    }

    #[test]
    fn test_validate_event_empty_title() {
        let event = CalendarEvent {
            id: "test".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let errors = EventService::validate_event(&event);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("Title")));
    }

    #[test]
    fn test_validate_event_invalid_date_range() {
        let mut event = CalendarEvent {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()), // Before start
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let errors = EventService::validate_event(&event);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("End date")));
    }

    #[test]
    fn test_validate_event_valid() {
        let event = CalendarEvent {
            id: "test".to_string(),
            title: "Valid Event".to_string(),
            description: "A valid event".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2024, 1, 16).unwrap()),
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: Some(NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
            is_all_day: false,
        };

        let errors = EventService::validate_event(&event);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_generate_instances_for_range_daily() {
        let base_event = CalendarEvent {
            id: "test".to_string(),
            title: "Daily Event".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::Daily,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let mut service = EventService::with_events(vec![base_event]);
        let instances = service.generate_instances_for_range(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
        );

        // Should generate 6 instances (Jan 2-7)
        assert_eq!(instances.len(), 6);
    }

    #[test]
    fn test_generate_instances_for_range_weekly() {
        let base_event = CalendarEvent {
            id: "test".to_string(),
            title: "Weekly Event".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::Weekly,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let mut service = EventService::with_events(vec![base_event]);
        let instances = service.generate_instances_for_range(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        // Should generate 4 instances (Jan 8, 15, 22, 29)
        assert_eq!(instances.len(), 4);
    }

    #[test]
    fn test_invalidate_instance_cache_selective() {
        let base_event = CalendarEvent {
            id: "test".to_string(),
            title: "Daily Event".to_string(),
            description: "".to_string(),
            recurrence: Recurrence::Daily,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let mut service = EventService::with_events(vec![base_event]);

        // Generate instances to populate cache
        let _ = service.generate_instances_for_range(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );

        // Invalidate with an event - should only clear if it matches
        service.invalidate_instance_cache(None);
        assert!(service.cached_instances.is_empty());
    }
}
