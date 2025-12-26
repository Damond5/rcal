use chrono::{NaiveDate, NaiveTime};
use crossterm::event::{Event, KeyCode, KeyEvent};
use rcal::app::{App, CalendarEvent, InputMode, PopupInputField, Recurrence};
use rcal::event_handling::handle_event;
use std::sync::mpsc;
use tempfile::TempDir;
use uuid;

fn setup_app() -> (App, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut app = App::new_with_calendar_dir(temp_dir.path().to_path_buf());
    app.events = rcal::persistence::load_events_from_path(&app.calendar_dir).unwrap();
    (app, temp_dir)
}

#[test]
fn test_quit_application() {
    let (mut app, _temp_dir) = setup_app();
    let key_event = KeyEvent::from(KeyCode::Char('q'));
    let should_continue = handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert!(!should_continue);
}

#[test]
fn test_navigation_left() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date - chrono::Duration::days(1));
}

#[test]
fn test_navigation_right() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Right);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date + chrono::Duration::days(1));
}

#[test]
fn test_navigation_vim_left() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('h'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date - chrono::Duration::days(1));
}

#[test]
fn test_navigation_vim_right() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('l'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date + chrono::Duration::days(1));
}

#[test]
fn test_navigation_up() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Up);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date - chrono::Duration::weeks(1));
}

#[test]
fn test_navigation_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Down);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date + chrono::Duration::weeks(1));
}

#[test]
fn test_navigation_vim_up() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('k'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date - chrono::Duration::weeks(1));
}

#[test]
fn test_navigation_vim_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('j'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.date, original_date + chrono::Duration::weeks(1));
}

#[test]
fn test_navigation_page_up() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::PageUp);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // PageUp should not change the date (removed H/L navigation)
    assert_eq!(app.date, original_date);
}

#[test]
fn test_navigation_page_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::PageDown);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // PageDown should not change the date (removed H/L navigation)
    assert_eq!(app.date, original_date);
}

#[test]
fn test_navigation_vim_page_up() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('H'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // 'H' should not change the date (removed H/L navigation)
    assert_eq!(app.date, original_date);
}

#[test]
fn test_navigation_vim_page_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('L'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // 'L' should not change the date (removed H/L navigation)
    assert_eq!(app.date, original_date);
}

#[test]
fn test_open_add_event_popup() {
    let (mut app, _temp_dir) = setup_app();
    let key_event = KeyEvent::from(KeyCode::Char('a'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert!(app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.selected_input_field, PopupInputField::Title);
    assert_eq!(app.cursor_position, 0);
    assert!(app.popup_event_title.is_empty());
    assert!(app.popup_event_time.is_empty());
}

#[test]
fn test_open_view_events_popup() {
    let (mut app, _temp_dir) = setup_app();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert!(app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
}

#[test]
fn test_close_view_events_popup() {
    let (mut app, _temp_dir) = setup_app();
    app.show_view_events_popup = true;
    app.input_mode = InputMode::ViewEventsPopup;
    app.events_to_display_in_popup = vec![]; // Add some dummy events

    let key_event = KeyEvent::from(KeyCode::Esc);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert!(!app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.events_to_display_in_popup.is_empty());
}

#[test]
fn test_type_in_title_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;

    let key_event = KeyEvent::from(KeyCode::Char('H'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('l'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('l'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_title, "Hello");
    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_type_in_time_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Time;

    let key_event = KeyEvent::from(KeyCode::Char('1'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('4'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char(':'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('3'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('0'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_time, "14:30");
    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_backspace_in_title_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Hello".to_string();
    app.cursor_position = 5;

    let key_event = KeyEvent::from(KeyCode::Backspace);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_title, "Hell");
    assert_eq!(app.cursor_position, 4);
}

#[test]
fn test_backspace_in_time_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Time;
    app.popup_event_time = "14:30".to_string();
    app.cursor_position = 5;

    let key_event = KeyEvent::from(KeyCode::Backspace);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_time, "14:3");
    assert_eq!(app.cursor_position, 4);
}

#[test]
fn test_backspace_at_start() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Hello".to_string();
    app.cursor_position = 0;

    let key_event = KeyEvent::from(KeyCode::Backspace);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_title, "Hello");
    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_cursor_left_movement() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Hello".to_string();
    app.cursor_position = 3;

    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.cursor_position, 2);
}

#[test]
fn test_cursor_left_at_start() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.cursor_position = 0;

    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.cursor_position, 0);
}

#[test]
fn test_cursor_right_movement() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Hello".to_string();
    app.cursor_position = 2;

    let key_event = KeyEvent::from(KeyCode::Right);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.cursor_position, 3);
}

#[test]
fn test_cursor_right_at_end() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Hello".to_string();
    app.cursor_position = 5;

    let key_event = KeyEvent::from(KeyCode::Right);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.cursor_position, 5);
}

#[test]
fn test_tab_switch_to_time_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.cursor_position = 7;

    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.selected_input_field, PopupInputField::Time);
    assert_eq!(app.cursor_position, 5); // Should be at end of time field
}

#[test]
fn test_tab_switch_to_description_field() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Time;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.cursor_position = 5;

    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.selected_input_field, PopupInputField::EndDate);
    assert_eq!(app.cursor_position, 0); // Should be at start of description field (empty)
}

#[test]
fn test_tab_switch_to_title_field_from_description() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Description;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.popup_event_description = "Description".to_string();
    app.cursor_position = 11;

    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.selected_input_field, PopupInputField::Recurrence);
    assert_eq!(app.cursor_position, 0); // Should be at end of recurrence field
}

#[test]
fn test_tab_switch_to_title_field_from_recurrence() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Recurrence;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_recurrence = "daily".to_string();
    app.cursor_position = 5;

    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.selected_input_field, PopupInputField::Title);
    assert_eq!(app.cursor_position, 7); // Should be at end of title field
}

#[test]
fn test_unicode_character_input() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "café".to_string();
    app.cursor_position = 4; // After "café"

    // Insert a Unicode character (é)
    let key_event = KeyEvent::from(KeyCode::Char('é'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_title, "caféé");
    assert_eq!(app.cursor_position, 5); // Should be after the inserted character
}

#[test]
fn test_create_event_with_hours_only() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Time;
    app.popup_event_time = "14".to_string();
    app.popup_event_title = "Test Event".to_string();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_app_has_reload_receiver() {
    let (mut app, _temp_dir) = setup_app();
    let (tx, rx) = mpsc::channel::<Result<(), String>>();
    app.reload_receiver = Some(rx);
    // Simulate sending reload signal
    tx.send(Ok(())).unwrap();
    // In real run_app, it would reload, but here just check receiver works
    if let Some(ref receiver) = app.reload_receiver {
        match receiver.try_recv() {
            Ok(Ok(_)) => {
                // Would reload here
            }
            _ => panic!("Should receive signal"),
        }
    }
}

#[test]
fn test_create_event_with_single_digit_hour() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Time;
    app.popup_event_time = "9".to_string();
    app.popup_event_title = "Morning Event".to_string();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Morning Event");
    assert_eq!(
        app.events[0].time,
        NaiveTime::from_hms_opt(9, 0, 0).unwrap()
    );
}

#[test]
fn test_delete_event_from_view_popup() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Event to Delete".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    });
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: today,
        time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        title: "Event to Keep".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_view_events_popup);
    assert_eq!(app.events_to_display_in_popup.len(), 2);
    assert_eq!(app.selected_event_index, 0);

    // Press 'd' to initiate delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.input_mode, InputMode::DeleteConfirmation);

    // Confirm deletion with 'y'
    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Event to Keep");
    assert_eq!(app.events_to_display_in_popup.len(), 1);
    assert_eq!(app.events_to_display_in_popup[0].title, "Event to Keep");
}

#[test]
fn test_cancel_delete_event_confirmation() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: "test_id".to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Daily Event".to_string(),
        description: String::new(),
        recurrence: Recurrence::Daily,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'd' to initiate delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.input_mode, InputMode::DeleteConfirmation);

    // Cancel deletion with 'n'
    let key_event = KeyEvent::from(KeyCode::Char('n'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1); // Event should still exist
    assert_eq!(app.events[0].title, "Daily Event");
}

#[test]
fn test_add_event_from_view_popup() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;

    // Open view events popup (even with no events)
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);

    // Press 'a' to add event from the view popup
    let key_event = KeyEvent::from(KeyCode::Char('a'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.current_date_for_new_event, today);
    assert!(app.popup_event_title.is_empty());
    assert!(app.popup_event_time.is_empty());
    assert!(app.popup_event_description.is_empty());

    // Fill in event details
    app.popup_event_title = "New Event".to_string();
    app.popup_event_time = "15:30".to_string();

    // Submit the event
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should return to view events popup with the new event
    assert!(!app.show_add_event_popup);
    assert!(app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events_to_display_in_popup.len(), 1);
    assert_eq!(app.events_to_display_in_popup[0].title, "New Event");
}

#[test]
fn test_navigate_events_in_view_popup() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "First Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: today,
        time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        title: "Second Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.selected_event_index, 0);

    // Navigate down
    let key_event = KeyEvent::from(KeyCode::Down);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_event_index, 1);

    // Navigate up
    let key_event = KeyEvent::from(KeyCode::Up);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_event_index, 0);

    // Try to go up from first item (should cycle to last)
    let key_event = KeyEvent::from(KeyCode::Up);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_event_index, 1);
}

#[test]
fn test_create_event_success() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Meeting");
    assert_eq!(
        app.events[0].time,
        NaiveTime::from_hms_opt(14, 30, 0).unwrap()
    );
    assert_eq!(
        app.events[0].date,
        NaiveDate::from_ymd_opt(2025, 10, 19).unwrap()
    );
    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_suggestions_overlay_appears_when_typing_end_date() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Meeting".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    // Tab to time field
    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_input_field, PopupInputField::Time);

    // Tab to end date field
    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_input_field, PopupInputField::EndDate);
    app.cursor_position = 0; // Initialize cursor for end date field

    // Type 't'
    let key_event = KeyEvent::from(KeyCode::Char('t'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.popup_event_end_date, "t");

    // Type 'o'
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.popup_event_end_date, "to");

    // Type 'm'
    let key_event = KeyEvent::from(KeyCode::Char('m'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.popup_event_end_date, "tom");

    // Check that suggestions are generated
    let manual_suggestions = rcal::date_utils::get_date_suggestions(&app.popup_event_end_date, app.current_date_for_new_event);
    assert!(!manual_suggestions.is_empty());
    // Note: The integration with event handling is tested separately; here we verify suggestions generation
    assert!(manual_suggestions.iter().any(|(s, _)| s.contains("20/10")));
}

#[test]
fn test_suggestions_fuzzy_matching() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::Title;
    app.popup_event_title = "Meeting".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    // Tab to end date field
    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_input_field, PopupInputField::EndDate);

    // Type 'tomorow' (typo)
    let key_event = KeyEvent::from(KeyCode::Char('t'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('m'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('r'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('w'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.popup_event_end_date, "tomorow");

    // Check suggestions include fuzzy match
    let suggestions = rcal::date_utils::get_date_suggestions(&app.popup_event_end_date, app.current_date_for_new_event);
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|(s, _)| s.contains("Tomorrow")));
}

#[test]
fn test_suggestions_arrow_navigation() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::EndDate;
    app.popup_event_end_date = "next".to_string(); // Should have multiple suggestions
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    // Simulate suggestions being set
    app.date_suggestions = rcal::date_utils::get_date_suggestions(&app.popup_event_end_date, app.current_date_for_new_event);
    app.show_date_suggestions = true;
    app.selected_suggestion_index = 0;

    // Check we have multiple suggestions
    assert!(app.date_suggestions.len() > 1);

    // Down arrow to select next
    let key_event = KeyEvent::from(KeyCode::Down);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_suggestion_index, 1);

    // Up arrow to select previous
    let key_event = KeyEvent::from(KeyCode::Up);
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.selected_suggestion_index, 0);
}

#[test]
fn test_suggestions_tab_accepts_selected() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::EndDate;
    app.popup_event_end_date = "tom".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    // Simulate suggestions
    app.date_suggestions = rcal::date_utils::get_date_suggestions(&app.popup_event_end_date, app.current_date_for_new_event);
    app.show_date_suggestions = true;
    app.selected_suggestion_index = 0;

    // Tab to accept
    let key_event = KeyEvent::from(KeyCode::Tab);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should accept the selected suggestion
    assert_eq!(app.popup_event_end_date, "20/10"); // Extracted date
    assert!(!app.show_date_suggestions);
}

#[test]
fn test_create_event_invalid_time_shows_error() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "invalid".to_string();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.error_message, "Invalid time format. Use HH:MM");
}

#[test]
fn test_create_event_empty_title() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "".to_string();
    app.popup_event_time = "14:30".to_string();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert_eq!(app.error_message, "Title cannot be empty");
    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
}

#[test]
fn test_create_event_invalid_end_date_shows_error() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.popup_event_end_date = "99/99".to_string(); // Invalid date
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();
    // Simulate real-time validation setting the error
    app.date_input_error = Some("Invalid date".to_string());

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.error_message, "Invalid date");
}

#[test]
fn test_create_event_invalid_time_24_hour() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "24:00".to_string(); // Invalid hour

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.error_message, "Invalid time format. Use HH:MM");
}

#[test]
fn test_create_event_malformed_end_date() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();
    app.popup_event_end_date = "abc".to_string(); // Malformed
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.error_message, "Invalid format. Use DD/MM");
}

#[test]
fn test_create_event_empty_end_date_sets_to_start_date() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Single Day Event".to_string();
    app.popup_event_time = "14:30".to_string();
    app.popup_event_end_date = "".to_string(); // Empty end date
    app.popup_event_end_time = "15:30".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Single Day Event");
    assert_eq!(
        app.events[0].start_date,
        NaiveDate::from_ymd_opt(2025, 10, 19).unwrap()
    );
    assert_eq!(
        app.events[0].end_date,
        Some(NaiveDate::from_ymd_opt(2025, 10, 19).unwrap())
    );
    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_create_event_empty_end_time_sets_to_start_time() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Point Event".to_string();
    app.popup_event_time = "14:30".to_string();
    app.popup_event_end_date = "20/10".to_string();
    app.popup_event_end_time = "".to_string(); // Empty end time
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Point Event");
    assert_eq!(
        app.events[0].start_time,
        NaiveTime::from_hms_opt(14, 30, 0).unwrap()
    );
    assert_eq!(
        app.events[0].end_time,
        Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap())
    );
    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_cancel_add_event_popup() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "14:30".to_string();

    let key_event = KeyEvent::from(KeyCode::Esc);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.popup_event_title.is_empty());
    assert!(app.popup_event_time.is_empty());
}

#[test]
fn test_view_events_popup_with_events() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Morning Meeting".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        title: "Afternoon Call".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        end_time: None,
    });

    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events_to_display_in_popup.len(), 2);
    // Events should be sorted by time
    assert_eq!(app.events_to_display_in_popup[0].title, "Morning Meeting");
    assert_eq!(app.events_to_display_in_popup[1].title, "Afternoon Call");
}

#[test]
fn test_view_events_popup_no_events() {
    let (mut app, _temp_dir) = setup_app();

    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_view_events_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert!(app.events_to_display_in_popup.is_empty());
}

#[test]
fn test_view_events_popup_filters_by_date() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    let tomorrow = today + chrono::Duration::days(1);

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Today Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: tomorrow,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Tomorrow Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: tomorrow,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events_to_display_in_popup.len(), 1);
    assert_eq!(app.events_to_display_in_popup[0].title, "Today Event");
}

#[test]
fn test_popup_state() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;

    // Just verify that the popup state is set correctly
    assert!(app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
}

#[test]
fn test_open_edit_event_popup() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Event to Edit".to_string(),
        description: "Description".to_string(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert!(app.is_editing);
    assert_eq!(app.popup_event_title, "Event to Edit");
    assert_eq!(app.popup_event_time, "10:00");
    assert_eq!(app.popup_event_description, "Description");
    assert_eq!(app.cursor_position, "Event to Edit".chars().count());
}

#[test]
fn test_edit_event_success() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Original Title".to_string(),
        description: "Original Description".to_string(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Modify title
    app.popup_event_title = "Edited Title".to_string();
    app.popup_event_time = "11:30".to_string();
    app.popup_event_description = "Edited Description".to_string();

    // Save
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0].title, "Edited Title");
    assert_eq!(
        app.events[0].time,
        NaiveTime::from_hms_opt(11, 30, 0).unwrap()
    );
    assert_eq!(app.events[0].description, "Edited Description");
    assert!(!app.is_editing);
    assert!(app.event_being_edited.is_none());
}

#[test]
fn test_cancel_edit_event() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    let original_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Original Title".to_string(),
        description: "Original Description".to_string(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    app.events.push(original_event.clone());

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Modify fields
    app.popup_event_title = "Modified Title".to_string();

    // Cancel
    let key_event = KeyEvent::from(KeyCode::Esc);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0], original_event); // Event unchanged
    assert!(!app.is_editing);
    assert!(app.event_being_edited.is_none());
}

#[test]
fn test_edit_event_invalid_time() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    let original_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Original Title".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    app.events.push(original_event.clone());

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Set invalid time
    app.popup_event_time = "invalid".to_string();

    // Try to save
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.error_message, "Invalid time format. Use HH:MM");
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0], original_event); // Event unchanged
    assert!(app.is_editing);
    assert!(app.event_being_edited.is_some());
}

#[test]
fn test_edit_event_empty_title() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    let original_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Original Title".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    app.events.push(original_event.clone());

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Set empty title
    app.popup_event_title = "".to_string();

    // Try to save
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert!(app.show_add_event_popup); // Popup should remain open
    assert_eq!(app.input_mode, InputMode::EditingEventPopup);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0], original_event); // Event unchanged
    assert_eq!(app.error_message, "Title cannot be empty");
    assert!(app.is_editing);
    assert!(app.event_being_edited.is_some());
}

#[test]
fn test_edit_event_change_time_sorting() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        title: "Noon Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        end_time: None,
    });
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Morning Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Select first event (Morning Event)
    assert_eq!(app.events_to_display_in_popup[0].title, "Morning Event");

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Change time to 14:00
    app.popup_event_time = "14:00".to_string();

    // Save
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Check sorting: Morning should still be first, then Noon, then edited to 14:00
    assert_eq!(app.events_to_display_in_popup.len(), 2);
    assert_eq!(app.events_to_display_in_popup[0].title, "Noon Event"); // 12:00
    assert_eq!(app.events_to_display_in_popup[1].title, "Morning Event"); // 14:00
}

#[test]
fn test_edit_event_persistence() {
    // This test would require mocking persistence, but since persistence uses real files,
    // we'll assume the save/delete functions work as tested separately.
    // In a real scenario, we'd use a temp dir for the app.
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Old Title".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Press 'e' to edit
    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Change title
    app.popup_event_title = "New Title".to_string();

    // Save
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Verify in memory
    assert_eq!(app.events[0].title, "New Title");
}

#[test]
fn test_view_boundary_adjustment_forward_shift() {
    let (mut app, _temp_dir) = setup_app();
    // Set date to last day of the third month in the view
    app.date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(); // Dec 31
    app.view_start_month = 10;
    app.view_start_year = 2025;

    // Navigate to next day (Jan 1, 2026)
    let key_event = KeyEvent::from(KeyCode::Right);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should shift view forward: view should now start at Nov 2025
    assert_eq!(app.view_start_month, 11);
    assert_eq!(app.view_start_year, 2025);
    assert_eq!(app.date, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
}

#[test]
fn test_view_boundary_adjustment_backward_shift() {
    let (mut app, _temp_dir) = setup_app();
    // Set date to first day of a month in the view
    app.date = NaiveDate::from_ymd_opt(2025, 11, 1).unwrap(); // Nov 1
    app.view_start_month = 11;
    app.view_start_year = 2025;

    // Navigate to previous day (Oct 31)
    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should shift view backward: view should now start at Oct 2025
    assert_eq!(app.view_start_month, 10);
    assert_eq!(app.view_start_year, 2025);
    assert_eq!(app.date, NaiveDate::from_ymd_opt(2025, 10, 31).unwrap());
}

#[test]
fn test_view_boundary_adjustment_year_boundary_forward() {
    let (mut app, _temp_dir) = setup_app();
    // Set date to Feb 28, 2025 (last day of Feb in view)
    app.date = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();
    app.view_start_month = 12;
    app.view_start_year = 2024;

    // Navigate to next day (Mar 1, 2025)
    let key_event = KeyEvent::from(KeyCode::Right);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should shift view forward: view should now start at Jan 2025
    assert_eq!(app.view_start_month, 1);
    assert_eq!(app.view_start_year, 2025);
    assert_eq!(app.date, NaiveDate::from_ymd_opt(2025, 3, 1).unwrap());
}

#[test]
fn test_view_boundary_adjustment_year_boundary_backward() {
    let (mut app, _temp_dir) = setup_app();
    // Set date to Jan 1, 2025
    app.date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    app.view_start_month = 1;
    app.view_start_year = 2025;

    // Navigate to previous day (Dec 31, 2024)
    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should shift view backward: view should now start at Dec 2024
    assert_eq!(app.view_start_month, 12);
    assert_eq!(app.view_start_year, 2024);
    assert_eq!(app.date, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
}

#[test]
fn test_view_boundary_no_shift_within_view() {
    let (mut app, _temp_dir) = setup_app();
    // Set date to middle of view
    app.date = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap();
    app.view_start_month = 10;
    app.view_start_year = 2025;

    // Navigate within the view (Oct 14)
    let key_event = KeyEvent::from(KeyCode::Left);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // View should not shift
    assert_eq!(app.view_start_month, 10);
    assert_eq!(app.view_start_year, 2025);
    assert_eq!(app.date, NaiveDate::from_ymd_opt(2025, 10, 14).unwrap());
}

#[test]
fn test_cleanup_old_events() {
    let temp_dir = TempDir::new().unwrap();
    let cutoff = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();

    // Create old event (finished before cutoff)
    let mut old_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Old Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    };
    rcal::persistence::save_event_to_path(&mut old_event, temp_dir.path(), None).unwrap();

    // Create recent event (finished after cutoff)
    let mut recent_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Recent Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap()),
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    };
    rcal::persistence::save_event_to_path(&mut recent_event, temp_dir.path(), None).unwrap();

    // Create multi-day old event
    let mut multi_day_old = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Multi Day Old".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2023, 7, 5).unwrap()),
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    };
    rcal::persistence::save_event_to_path(&mut multi_day_old, temp_dir.path(), None).unwrap();

    // Verify 3 events loaded
    let events_before = rcal::persistence::load_events_from_path(temp_dir.path()).unwrap();
    assert_eq!(events_before.len(), 3);

    // Run cleanup
    let deleted = rcal::persistence::cleanup_old_events_with_cutoff(temp_dir.path(), None, cutoff).unwrap();
    assert_eq!(deleted, 2); // Old and multi-day old should be deleted

    // Verify only recent event remains
    let events_after = rcal::persistence::load_events_from_path(temp_dir.path()).unwrap();
    assert_eq!(events_after.len(), 1);
    assert_eq!(events_after[0].title, "Recent Event");
}

#[test]
fn test_cleanup_old_events_preserves_recurring() {
    let temp_dir = TempDir::new().unwrap();
    let cutoff = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();

    // Create old recurring event (should not be deleted even if base date is old)
    let mut recurring_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Old Recurring".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Weekly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    };
    rcal::persistence::save_event_to_path(&mut recurring_event, temp_dir.path(), None).unwrap();

    // Create old non-recurring event (should be deleted)
    let mut old_non_recurring = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Old Non-Recurring".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
        is_all_day: false,
    };
    rcal::persistence::save_event_to_path(&mut old_non_recurring, temp_dir.path(), None).unwrap();

    // Check number of event files before cleanup
    let files_before: Vec<_> = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    assert_eq!(files_before.len(), 2);

    // Run cleanup
    let deleted = rcal::persistence::cleanup_old_events_with_cutoff(temp_dir.path(), None, cutoff).unwrap();
    assert_eq!(deleted, 1); // Only non-recurring should be deleted

    // Check number of event files after cleanup
    let files_after: Vec<_> = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    assert_eq!(files_after.len(), 1);

    // Verify the remaining file is the recurring event
    let events_after = rcal::persistence::load_events_from_path(temp_dir.path()).unwrap();
    assert!(events_after.iter().any(|e| e.title == "Old Recurring" && e.recurrence == rcal::app::Recurrence::Weekly));
}

#[test]
fn test_recurring_event_instances_generated() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Weekly Meeting".to_string();
    app.popup_event_time = "10:00".to_string();
    app.popup_event_recurrence = "weekly".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have base event only (instances generated lazily)
    assert_eq!(app.events.len(), 1);
    // Check that base event is saved
    assert!(app.events.iter().any(|e| e.title == "Weekly Meeting" && !e.is_recurring_instance));
    // Check that base event has recurrence
    assert!(app.events.iter().any(|e| e.title == "Weekly Meeting" && e.recurrence == Recurrence::Weekly));
}

#[test]
fn test_delete_recurring_instance() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    // Add a recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Daily Standup".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Daily,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    // Generate instances
    let until = today + chrono::Duration::days(5);
    let instances = rcal::persistence::generate_recurring_instances(&app.events[0], until);
    app.events.extend(instances);

    let initial_count = app.events.len();

    // Navigate to tomorrow where an instance is
    app.date = today + chrono::Duration::days(1);
    app.adjust_view_boundaries();

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Select the instance
    app.selected_event_index = 0; // Should be the instance on tomorrow

    // Press 'd' to delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Confirm
    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have removed all events with that title (entire series)
    assert!(!app.events.iter().any(|e| e.title == "Daily Standup"));
    assert!(app.events.len() < initial_count);
}

#[test]
fn test_delete_recurring_base_event() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    // Add a recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Daily Standup".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Daily,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    // Generate instances
    let until = today + chrono::Duration::days(5);
    let instances = rcal::persistence::generate_recurring_instances(&app.events[0], until);
    app.events.extend(instances);

    let initial_count = app.events.len();

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Select the base event (first one)
    app.selected_event_index = 0;

    // Press 'd' to delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Confirm
    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have removed all events with that title
    assert!(!app.events.iter().any(|e| e.title == "Daily Standup"));
    assert!(app.events.len() < initial_count);
}

#[test]
fn test_find_base_event_for_instance() {
    let today = chrono::Local::now().date_naive();
    let base_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Base Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Daily,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    let instance = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today + chrono::Duration::days(1),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Base Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: true,
        base_date: Some(today),
        start_date: today + chrono::Duration::days(1),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    let events = vec![base_event.clone(), instance.clone()];

    let found = rcal::event_handling::find_base_event_for_instance(&instance, &events);
    assert_eq!(found, Some(base_event));
}

#[test]
fn test_find_base_event_for_non_instance() {
    let today = chrono::Local::now().date_naive();
    let event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Event".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    let events = vec![event.clone()];

    let found = rcal::event_handling::find_base_event_for_instance(&event, &events);
    assert_eq!(found, None);
}

#[test]
fn test_find_base_event_no_match() {
    let today = chrono::Local::now().date_naive();
    let instance = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today + chrono::Duration::days(1),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Instance".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::None,
        is_recurring_instance: true,
        base_date: Some(today),
        start_date: today + chrono::Duration::days(1),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    let events = vec![]; // No base event

    let found = rcal::event_handling::find_base_event_for_instance(&instance, &events);
    assert_eq!(found, None);
}

#[test]
fn test_delete_recurring_instance_deletes_series() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    // Add a recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Weekly Meeting".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Weekly,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });
    // Generate instances
    let until = today + chrono::Duration::weeks(4);
    let instances = rcal::persistence::generate_recurring_instances(&app.events[0], until);
    app.events.extend(instances);

    let initial_count = app.events.len();
    assert!(initial_count > 1); // Should have base + instances

    // Navigate to a future week where an instance is
    app.date = today + chrono::Duration::weeks(1);
    app.adjust_view_boundaries();

    // Open view events popup
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Select the instance
    app.selected_event_index = 0; // Should be the instance

    // Press 'd' to delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Confirm
    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have removed all events with that title (entire series)
    assert!(!app.events.iter().any(|e| e.title == "Weekly Meeting"));
    assert!(app.events.is_empty() || app.events.len() < initial_count);
}

#[test]
fn test_delete_recurring_series_persistence() {
    let (mut app, temp_dir) = setup_app();
    let today = app.date;
    // Create and save a recurring event
    let mut recurring_event = CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Persistent Recurring".to_string(),
        description: String::new(),
        recurrence: rcal::app::Recurrence::Weekly,
        is_recurring_instance: false,
        base_date: None,
        start_date: today,
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    };
    rcal::persistence::save_event_to_path(&mut recurring_event, temp_dir.path(), None).unwrap();

    // Reload events to simulate app restart
    app.events = rcal::persistence::load_events_from_path(temp_dir.path()).unwrap();
    // Generate instances
    let until = today + chrono::Duration::weeks(4);
    let instances = rcal::persistence::generate_recurring_instances(&app.events[0], until);
    app.events.extend(instances);

    assert!(app.events.iter().any(|e| e.title == "Persistent Recurring"));

    // Delete an instance
    app.date = today + chrono::Duration::weeks(1);
    app.adjust_view_boundaries();

    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    app.selected_event_index = 0;
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Series should be deleted from memory
    assert!(!app.events.iter().any(|e| e.title == "Persistent Recurring"));

    // Reload events to simulate restart - should not come back
    app.events = rcal::persistence::load_events_from_path(temp_dir.path()).unwrap();
    assert!(!app.events.iter().any(|e| e.title == "Persistent Recurring"));
}

#[test]
fn test_yearly_recurring_event_creation_and_display() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Yearly Anniversary".to_string();
    app.popup_event_time = "12:00".to_string();
    app.popup_event_recurrence = "yearly".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have base event only (instances generated lazily)
    assert_eq!(app.events.len(), 1);
    // Check that base event is saved
    assert!(app.events.iter().any(|e| e.title == "Yearly Anniversary" && !e.is_recurring_instance));
    // Check that base event has recurrence
    assert!(app.events.iter().any(|e| e.title == "Yearly Anniversary" && e.recurrence == Recurrence::Yearly));
}

#[test]
fn test_cache_invalidation_on_event_add() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();

    // Initially, cache should be empty
    assert!(app.cached_range.is_none());
    assert!(app.cached_instances.is_empty());

    // Get events for range (should populate cache)
    let events = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());
    assert_eq!(app.cached_range, Some((start - chrono::Duration::days(365), end + chrono::Duration::days(365))));

    // Add a recurring event
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Weekly Meeting".to_string();
    app.popup_event_time = "10:00".to_string();
    app.popup_event_recurrence = "weekly".to_string();
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Cache should be invalidated
    assert!(app.cached_range.is_none());
    assert!(app.cached_instances.is_empty());

    // Get events again (should regenerate cache)
    let events_after = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());
    // Should have the base event + instances
    assert!(events_after.len() > events.len());
    assert!(events_after.iter().any(|e| e.title == "Weekly Meeting"));
}

#[test]
fn test_cache_invalidation_on_event_delete() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();

    // Add a recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Weekly Meeting".to_string(),
        description: String::new(),
        recurrence: Recurrence::Weekly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Populate cache
    let _ = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());

    // Delete the event
    app.date = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    let key_event = KeyEvent::from(KeyCode::Char('y'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Cache should be selectively invalidated (range kept, but instances for deleted event removed)
    assert!(app.cached_range.is_some()); // Range is kept since other events may be cached
    // Check that no instances of the deleted event remain
    assert!(!app.cached_instances.iter().any(|e| e.title == "Weekly Meeting"));
}

#[test]
fn test_cache_invalidation_on_event_edit() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();

    // Add an event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Meeting".to_string(),
        description: String::new(),
        recurrence: Recurrence::None,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Populate cache
    let _ = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());

    // Edit the event
    app.date = NaiveDate::from_ymd_opt(2025, 10, 15).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('o'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    let key_event = KeyEvent::from(KeyCode::Char('e'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    app.popup_event_title = "Updated Meeting".to_string();
    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Cache should be invalidated
    assert!(app.cached_range.is_none());
    assert!(app.cached_instances.is_empty());
}

#[test]
fn test_get_all_events_for_range_cache_hit() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();

    // Add a recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Weekly Meeting".to_string(),
        description: String::new(),
        recurrence: Recurrence::Weekly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // First call should generate cache
    let events1 = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());
    let initial_cache_size = app.cached_instances.len();

    // Second call with same range should hit cache
    let events2 = app.get_all_events_for_range(start, end);
    assert_eq!(events1, events2);
    assert_eq!(app.cached_instances.len(), initial_cache_size);
}

#[test]
fn test_get_all_events_for_range_large_range() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();

    // Add a yearly recurring event
    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Yearly Event".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2025, 10, 15).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    // Should handle large range without issues
    let events = app.get_all_events_for_range(start, end);
    assert!(events.len() > 1); // Base + instances
    assert!(app.cached_range.is_some());
}

#[test]
fn test_get_all_events_for_range_invalid_dates() {
    let (mut app, _temp_dir) = setup_app();
    // Invalid range: end before start
    let start = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();

    // Should handle gracefully (though chrono might prevent this, but test robustness)
    let events = app.get_all_events_for_range(start, end);
    // Should return base events at least
    assert!(events.len() >= app.events.len());
}

#[test]
fn test_end_date_input_validation_and_suggestions() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.selected_input_field = PopupInputField::EndDate;
    app.current_date_for_new_event = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();

    // Type invalid date
    let key_event = KeyEvent::from(KeyCode::Char('a'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('b'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('c'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should have validation error
    assert!(app.date_input_error.is_some());
    assert!(app.date_suggestions.is_empty());

    // Clear and type valid date
    app.popup_event_end_date.clear();
    app.date_input_error = None;
    let key_event = KeyEvent::from(KeyCode::Char('2'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('0'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('/'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('1'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    let key_event = KeyEvent::from(KeyCode::Char('0'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // Should be valid and no suggestions
    assert!(app.date_input_error.is_none());
    assert!(app.date_suggestions.is_empty());
}

#[test]
fn test_performance_frequent_invalidations() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap();

    // Add 10 recurring events to simulate load
    for i in 0..10 {
        app.events.push(CalendarEvent {
            id: uuid::Uuid::new_v4().to_string(),
            is_all_day: false,
            date: NaiveDate::from_ymd_opt(2025, 10, (i % 28) + 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: format!("Meeting {}", i),
            description: String::new(),
            recurrence: Recurrence::Weekly,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2025, 10, (i % 28) + 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
        });
    }

    // Measure time for multiple invalidations and regenerations
    use std::time::Instant;
    let mut total_time = std::time::Duration::new(0, 0);

    for _ in 0..5 {
        let timer = Instant::now();
        app.invalidate_instance_cache(None);
        let _ = app.get_all_events_for_range(start, end);
        total_time += timer.elapsed();
    }

    // Should complete in reasonable time (less than 1 second total for 5 iterations)
    assert!(total_time < std::time::Duration::from_secs(1));
}

#[test]
fn test_feb29_yearly_fallback_to_feb28() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2027, 12, 31).unwrap();

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Leap Day Birthday".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    let events = app.get_all_events_for_range(start, end);

    assert!(events.iter().any(|e| e.title == "Leap Day Birthday" && e.start_date == NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()));
    assert!(events.iter().any(|e| e.title == "Leap Day Birthday" && e.start_date == NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()));
    assert!(events.iter().any(|e| e.title == "Leap Day Birthday" && e.start_date == NaiveDate::from_ymd_opt(2026, 2, 28).unwrap()));
    assert!(events.iter().any(|e| e.title == "Leap Day Birthday" && e.start_date == NaiveDate::from_ymd_opt(2027, 2, 28).unwrap()));
}

#[test]
fn test_feb29_century_year_transitions() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(1899, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(1901, 12, 31).unwrap();

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(1896, 2, 29).unwrap(),
        time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        title: "Century Test Event".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(1896, 2, 29).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        end_time: None,
    });

    let events = app.get_all_events_for_range(start, end);

    assert!(events.iter().any(|e| e.title == "Century Test Event" && e.start_date == NaiveDate::from_ymd_opt(1899, 2, 28).unwrap()));
    assert!(events.iter().any(|e| e.title == "Century Test Event" && e.start_date == NaiveDate::from_ymd_opt(1900, 2, 28).unwrap()));
    assert!(events.iter().any(|e| e.title == "Century Test Event" && e.start_date == NaiveDate::from_ymd_opt(1901, 2, 28).unwrap()));
}

#[test]
fn test_feb29_multiday_event_duration_preservation() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        title: "Multi-Day Conference".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap()),
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
    });

    let events = app.get_all_events_for_range(start, end);

    let feb29_2024 = events.iter().find(|e| e.title == "Multi-Day Conference" && e.start_date == NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    let feb28_2025 = events.iter().find(|e| e.title == "Multi-Day Conference" && e.start_date == NaiveDate::from_ymd_opt(2025, 2, 28).unwrap());

    assert!(feb29_2024.is_some());
    assert_eq!(feb29_2024.unwrap().end_date, Some(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap()));

    assert!(feb28_2025.is_some());
    assert_eq!(feb28_2025.unwrap().end_date, Some(NaiveDate::from_ymd_opt(2025, 3, 2).unwrap()));
}

#[test]
fn test_feb28_yearly_no_fallback() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(),
        time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        title: "Feb 28 Event".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2024, 2, 28).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        end_time: None,
    });

    let events = app.get_all_events_for_range(start, end);

    assert!(events.iter().any(|e| e.title == "Feb 28 Event" && e.start_date == NaiveDate::from_ymd_opt(2024, 2, 28).unwrap()));
    assert!(events.iter().any(|e| e.title == "Feb 28 Event" && e.start_date == NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()));
}

#[test]
fn test_feb29_event_cache_invalidation() {
    let (mut app, _temp_dir) = setup_app();
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();

    app.events.push(CalendarEvent {
        id: uuid::Uuid::new_v4().to_string(),
        is_all_day: false,
        date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Cached Leap Event".to_string(),
        description: String::new(),
        recurrence: Recurrence::Yearly,
        is_recurring_instance: false,
        base_date: None,
        start_date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        end_date: None,
        start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        end_time: None,
    });

    let events1 = app.get_all_events_for_range(start, end);
    assert!(app.cached_range.is_some());
    let initial_cache_size = app.cached_instances.len();
    assert!(initial_cache_size > 0);

    app.invalidate_instance_cache(None);
    assert!(app.cached_instances.is_empty());

    let events2 = app.get_all_events_for_range(start, end);
    assert_eq!(events1.len(), events2.len());
    assert!(events1.iter().all(|e1| events2.iter().any(|e2| e1.title == e2.title && e1.start_date == e2.start_date)));
    assert!(app.cached_instances.len() > 0);
}
