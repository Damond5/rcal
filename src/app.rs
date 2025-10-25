use crate::sync::{SyncProvider, SyncStatus};
use chrono::{Local, NaiveDate, NaiveTime};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CalendarEvent {
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
    pub id: Uuid,
}

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    EditingEventPopup,
    ViewEventsPopup,
    DeleteConfirmation,
    Sync,
}

#[derive(PartialEq, Debug)]
pub enum PopupInputField {
    Title,
    Time,
    EndDate,
    EndTime,
    Description,
    Recurrence,
}

pub struct App {
    pub date: NaiveDate, // Now represents the selected date
    pub events: Vec<CalendarEvent>,
    pub input: String,
    pub input_mode: InputMode,
    pub popup_event_title: String,
    pub popup_event_time: String,
    pub popup_event_end_date: String,
    pub popup_event_end_time: String,
    pub popup_event_description: String,
    pub popup_event_recurrence: String,
    pub selected_input_field: PopupInputField,
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        App {
            date: Local::now().date_naive(),
            events: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            popup_event_title: String::new(),
            popup_event_time: String::new(),
            popup_event_end_date: String::new(),
            popup_event_end_time: String::new(),
            popup_event_description: String::new(),
            popup_event_recurrence: String::new(),
            selected_input_field: PopupInputField::Title,
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
            PopupInputField::Time => &self.popup_event_time,
            PopupInputField::EndDate => &self.popup_event_end_date,
            PopupInputField::EndTime => &self.popup_event_end_time,
            PopupInputField::Description => &self.popup_event_description,
            PopupInputField::Recurrence => &self.popup_event_recurrence,
        }
    }

    pub fn get_current_field_mut(&mut self) -> &mut String {
        match self.selected_input_field {
            PopupInputField::Title => &mut self.popup_event_title,
            PopupInputField::Time => &mut self.popup_event_time,
            PopupInputField::EndDate => &mut self.popup_event_end_date,
            PopupInputField::EndTime => &mut self.popup_event_end_time,
            PopupInputField::Description => &mut self.popup_event_description,
            PopupInputField::Recurrence => &mut self.popup_event_recurrence,
        }
    }

    pub fn get_current_field_char_count(&self) -> usize {
        self.get_current_field().chars().count()
    }
}
