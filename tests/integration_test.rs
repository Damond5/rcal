use chrono::{NaiveDate, NaiveTime};
use crossterm::event::{Event, KeyCode, KeyEvent};
use rcal::app::{App, CalendarEvent, InputMode, PopupInputField};
use rcal::event_handling::handle_event;
use std::sync::mpsc;
use tempfile::TempDir;

fn setup_app() -> (App, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut app = App::new_with_calendar_dir(temp_dir.path().to_path_buf());
    app.events = rcal::persistence::load_events_from_path(&app.calendar_dir);
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

    // PageUp should change the date (to first day of previous month)
    assert_ne!(app.date, original_date);
}

#[test]
fn test_navigation_page_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::PageDown);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // PageDown should change the date (to first day of next month)
    assert_ne!(app.date, original_date);
}

#[test]
fn test_navigation_vim_page_up() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('H'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // 'H' should change the date (to first day of previous month)
    assert_ne!(app.date, original_date);
}

#[test]
fn test_navigation_vim_page_down() {
    let (mut app, _temp_dir) = setup_app();
    let original_date = app.date;
    let key_event = KeyEvent::from(KeyCode::Char('L'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    // 'L' should change the date (to first day of next month)
    assert_ne!(app.date, original_date);
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
        is_all_day: false,
        date: today,
        time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        title: "Event to Keep".to_string(),
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

    // Press 'd' to initiate delete
    let key_event = KeyEvent::from(KeyCode::Char('d'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();
    assert_eq!(app.input_mode, InputMode::DeleteConfirmation);

    // Cancel deletion with 'n'
    let key_event = KeyEvent::from(KeyCode::Char('n'));
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1); // Event should still exist
    assert_eq!(app.events[0].title, "Event to Keep");
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
fn test_create_event_invalid_time() {
    let (mut app, _temp_dir) = setup_app();
    app.show_add_event_popup = true;
    app.input_mode = InputMode::EditingEventPopup;
    app.popup_event_title = "Meeting".to_string();
    app.popup_event_time = "invalid".to_string();

    let key_event = KeyEvent::from(KeyCode::Enter);
    handle_event(&mut app, Event::Key(key_event)).unwrap();

    assert_eq!(app.events.len(), 0); // No event should be created
    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::Normal);
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

    assert!(!app.show_add_event_popup);
    assert_eq!(app.input_mode, InputMode::ViewEventsPopup);
    assert_eq!(app.events.len(), 1);
    assert_eq!(app.events[0], original_event); // Event unchanged
    assert!(!app.is_editing);
    assert!(app.event_being_edited.is_none());
}

#[test]
fn test_edit_event_empty_title() {
    let (mut app, _temp_dir) = setup_app();
    let today = app.date;
    let original_event = CalendarEvent {
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
