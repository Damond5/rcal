use chrono::{Local, NaiveDate, NaiveTime};

#[derive(Clone, PartialEq)]
pub struct CalendarEvent {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub title: String,
    pub description: String,
}

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    EditingEventPopup,
    ViewEventsPopup,
    DeleteConfirmation,
}

#[derive(PartialEq, Debug)]
pub enum PopupInputField {
    Title,
    Time,
    Description,
}

pub struct App {
    pub date: NaiveDate, // Now represents the selected date
    pub events: Vec<CalendarEvent>,
    pub input: String,
    pub input_mode: InputMode,
    pub popup_event_title: String,
    pub popup_event_time: String,
    pub popup_event_description: String,
    pub selected_input_field: PopupInputField,
    pub show_add_event_popup: bool,
    pub show_view_events_popup: bool,
    pub events_to_display_in_popup: Vec<CalendarEvent>,
    pub selected_event_index: usize,
    pub event_to_delete_index: Option<usize>,
    pub current_date_for_new_event: NaiveDate,
    pub cursor_position: usize, // Character index for Unicode support
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
            popup_event_description: String::new(),
            selected_input_field: PopupInputField::Title,
            show_add_event_popup: false,
            show_view_events_popup: false,
            events_to_display_in_popup: Vec::new(),
            selected_event_index: 0,
            event_to_delete_index: None,
            current_date_for_new_event: Local::now().date_naive(),
            cursor_position: 0,
        }
    }

    // Helper functions for Unicode-safe cursor handling
    pub fn char_to_byte_index(s: &str, char_index: usize) -> usize {
        s.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(s.len())
    }

    pub fn get_current_field(&self) -> &str {
        match self.selected_input_field {
            PopupInputField::Title => &self.popup_event_title,
            PopupInputField::Time => &self.popup_event_time,
            PopupInputField::Description => &self.popup_event_description,
        }
    }

    pub fn get_current_field_mut(&mut self) -> &mut String {
        match self.selected_input_field {
            PopupInputField::Title => &mut self.popup_event_title,
            PopupInputField::Time => &mut self.popup_event_time,
            PopupInputField::Description => &mut self.popup_event_description,
        }
    }

    pub fn get_current_field_char_count(&self) -> usize {
        self.get_current_field().chars().count()
    }
}
