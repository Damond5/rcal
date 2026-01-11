use crate::sync::{SyncProvider, SyncStatus};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use std::sync::mpsc::Receiver;

const INSTANCE_BUFFER_DAYS: i64 = 365;

#[derive(Clone, PartialEq, Debug)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CalendarEvent {
    pub id: String,
    pub date: NaiveDate, // Deprecated: use start_date
    pub time: NaiveTime, // Deprecated: use start_time
    pub title: String,
    pub description: String,
    pub recurrence: Recurrence,
    pub is_recurring_instance: bool,
    pub base_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
    pub is_all_day: bool,
}

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    EditingEventPopup,
    SelectingRecurrence,
    ViewEventsPopup,
    DeleteConfirmation,
    Sync,
}

#[derive(PartialEq, Debug)]
pub enum PopupInputField {
    Title,
    StartTime,
    EndDate,
    EndTime,
    Description,
    Recurrence,
}

pub struct App {
    pub date: NaiveDate, // Now represents the selected date
    pub view_start_month: u32,
    pub view_start_year: i32,
    pub events: Vec<CalendarEvent>, // Base events only
    pub cached_instances: Vec<CalendarEvent>,
    pub cached_range: Option<(NaiveDate, NaiveDate)>,
    pub input: String,
    pub input_mode: InputMode,
    pub popup_event_title: String,
    pub popup_event_start_time: String,
    pub popup_event_end_date: String,
    pub popup_event_end_time: String,
    pub popup_event_description: String,
    pub popup_event_recurrence: String,
    pub selected_input_field: PopupInputField,
    pub selected_recurrence_index: usize,
    pub show_add_event_popup: bool,
    pub show_view_events_popup: bool,
    pub events_to_display_in_popup: Vec<CalendarEvent>,
    pub selected_event_index: usize,
    pub event_to_delete_index: Option<usize>,
    pub current_date_for_new_event: NaiveDate,
    pub cursor_position: usize, // Character index for Unicode support
    pub is_editing: bool,
    pub event_being_edited: Option<CalendarEvent>,
    pub sync_provider: Option<Box<dyn SyncProvider>>,
    pub sync_status: Option<SyncStatus>,
    pub sync_message: String,
    pub calendar_dir: std::path::PathBuf,
    pub error_message: String,
    pub reload_receiver: Option<Receiver<Result<(), String>>>,
    pub date_input_error: Option<String>,
    pub date_suggestions: Vec<(String, bool)>,
    pub show_date_suggestions: bool,
    pub selected_suggestion_index: usize,
    pub time_input_error: Option<String>,
    pub end_time_input_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        let date = Local::now().date_naive();
        App {
            date,
            view_start_month: date.month(),
            view_start_year: date.year(),
            events: Vec::new(),
            cached_instances: Vec::new(),
            cached_range: None,
            input: String::new(),
            input_mode: InputMode::Normal,
            popup_event_title: String::new(),
            popup_event_start_time: String::new(),
            popup_event_end_date: String::new(),
            popup_event_end_time: String::new(),
            popup_event_description: String::new(),
            popup_event_recurrence: String::new(),
            selected_input_field: PopupInputField::Title,
            selected_recurrence_index: 0,
            show_add_event_popup: false,
            show_view_events_popup: false,
            events_to_display_in_popup: Vec::new(),
            selected_event_index: 0,
            event_to_delete_index: None,
            current_date_for_new_event: Local::now().date_naive(),
            cursor_position: 0,
            is_editing: false,
            event_being_edited: None,
            sync_provider: None,
            sync_status: None,
            sync_message: String::new(),
            calendar_dir: dirs::home_dir()
                .expect("Could not find home directory")
                .join("calendar"),
            error_message: String::new(),
            reload_receiver: None,
            date_input_error: None,
            date_suggestions: Vec::new(),
            show_date_suggestions: false,
            selected_suggestion_index: 0,
            time_input_error: None,
            end_time_input_error: None,
        }
    }

    pub fn new_with_calendar_dir(calendar_dir: std::path::PathBuf) -> App {
        let date = Local::now().date_naive();
        App {
            date,
            view_start_month: date.month(),
            view_start_year: date.year(),
            events: Vec::new(),
            cached_instances: Vec::new(),
            cached_range: None,
            input: String::new(),
            input_mode: InputMode::Normal,
            popup_event_title: String::new(),
            popup_event_start_time: String::new(),
            popup_event_end_date: String::new(),
            popup_event_end_time: String::new(),
            popup_event_description: String::new(),
            popup_event_recurrence: String::new(),
            selected_input_field: PopupInputField::Title,
            selected_recurrence_index: 0,
            show_add_event_popup: false,
            show_view_events_popup: false,
            events_to_display_in_popup: Vec::new(),
            selected_event_index: 0,
            event_to_delete_index: None,
            current_date_for_new_event: Local::now().date_naive(),
            cursor_position: 0,
            is_editing: false,
            event_being_edited: None,
            sync_provider: None,
            sync_status: None,
            sync_message: String::new(),
            calendar_dir,
            error_message: String::new(),
            reload_receiver: None,
            date_input_error: None,
            date_suggestions: Vec::new(),
            show_date_suggestions: false,
            selected_suggestion_index: 0,
            time_input_error: None,
            end_time_input_error: None,
        }
    }

    // Helper functions for Unicode-safe cursor handling
    pub fn char_to_byte_index(s: &str, char_index: usize) -> usize {
        s.char_indices()
            .nth(char_index)
            .map(|(i, _)| i)
            .unwrap_or(s.len())
    }

    pub fn get_current_field(&self) -> &str {
        match self.selected_input_field {
            PopupInputField::Title => &self.popup_event_title,
            PopupInputField::StartTime => &self.popup_event_start_time,
            PopupInputField::EndDate => &self.popup_event_end_date,
            PopupInputField::EndTime => &self.popup_event_end_time,
            PopupInputField::Description => &self.popup_event_description,
            PopupInputField::Recurrence => &self.popup_event_recurrence,
        }
    }

    pub fn get_current_field_mut(&mut self) -> &mut String {
        match self.selected_input_field {
            PopupInputField::Title => &mut self.popup_event_title,
            PopupInputField::StartTime => &mut self.popup_event_start_time,
            PopupInputField::EndDate => &mut self.popup_event_end_date,
            PopupInputField::EndTime => &mut self.popup_event_end_time,
            PopupInputField::Description => &mut self.popup_event_description,
            PopupInputField::Recurrence => &mut self.popup_event_recurrence,
        }
    }

    pub fn get_current_field_char_count(&self) -> usize {
        self.get_current_field().chars().count()
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
                crate::persistence::generate_instances_for_range(&self.events, gen_start, gen_end);
            self.cached_range = Some((gen_start, gen_end));
        }
        let mut all = self.events.clone();
        all.extend(self.cached_instances.iter().cloned());
        all.sort_by(|a, b| {
            a.start_date
                .cmp(&b.start_date)
                .then(a.start_time.cmp(&b.start_time))
        });
        all
    }

    pub fn adjust_view_boundaries(&mut self) {
        let cursor_number = (self.date.year() as i64 * 12) + self.date.month() as i64;
        let view_start_number = (self.view_start_year as i64 * 12) + self.view_start_month as i64;
        let view_end_number = view_start_number + 2;

        if cursor_number < view_start_number {
            // Shift backward
            self.view_start_month = self.date.month();
            self.view_start_year = self.date.year();
        } else if cursor_number > view_end_number {
            // Shift forward
            let new_start = cursor_number - 2;
            self.view_start_year = (new_start / 12) as i32;
            let month = (new_start % 12) as u32;
            self.view_start_month = if month == 0 { 12 } else { month };
            if month == 0 {
                self.view_start_year -= 1;
            }
        }
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
}
