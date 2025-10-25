use std::path::Path;

use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use dirs;

use crate::app::CalendarEvent;
use crate::sync::SyncProvider;

fn sanitize_title_for_filename(title: &str) -> String {
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
    if collapsed.len() > 100 {
        collapsed.truncate(100);
    }
    if collapsed.is_empty() {
        "untitled".to_string() // Fallback for empty or entirely invalid titles
    } else {
        collapsed
    }
}

pub fn load_events() -> Vec<CalendarEvent> {
    let home = dirs::home_dir().expect("Could not find home directory");
    load_events_from_path(&home.join("calendar"))
}

pub fn load_events_from_path(calendar_dir: &Path) -> Vec<CalendarEvent> {
    if !calendar_dir.exists() {
        std::fs::create_dir_all(calendar_dir).expect("Could not create calendar directory");
        return Vec::new();
    }

    let mut events = Vec::new();

    for entry in std::fs::read_dir(calendar_dir).expect("Could not read calendar directory") {
        let entry = entry.expect("Error reading entry");
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.ends_with(".md"))
            .unwrap_or(false)
        {
            let content = std::fs::read_to_string(&path).expect("Could not read file");
            // new format
            let mut id = String::new();
            let mut title = String::new();
            let mut start_date = None;
            let mut end_date = None;
            let mut start_time = None;
            let mut end_time = None;
            let mut description = String::new();
            let mut recurrence = crate::app::Recurrence::None;
            for line in content.lines() {
                if let Some(stripped) = line.strip_prefix("# Event: ") {
                    title = stripped.trim().to_string();
                } else if let Some(stripped) = line.strip_prefix("- **Date**: ") {
                    let date_str = stripped.trim();
                    if date_str.contains(" to ") {
                        let parts: Vec<&str> = date_str.split(" to ").collect();
                        start_date = Some(NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").unwrap());
                        end_date = Some(NaiveDate::parse_from_str(parts[1], "%Y-%m-%d").unwrap());
                    } else {
                        start_date = Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap());
                    }
                } else if let Some(stripped) = line.strip_prefix("- **Time**: ") {
                    let time_str = stripped.trim();
                    if time_str.contains(" to ") {
                        let parts: Vec<&str> = time_str.split(" to ").collect();
                        start_time = Some(NaiveTime::parse_from_str(parts[0], "%H:%M").unwrap());
                        end_time = Some(NaiveTime::parse_from_str(parts[1], "%H:%M").unwrap());
                    } else {
                        start_time = Some(NaiveTime::parse_from_str(time_str, "%H:%M").unwrap());
                    }
                } else if let Some(stripped) = line.strip_prefix("- **Description**: ") {
                    description = stripped.trim().to_string();
                } else if let Some(stripped) = line.strip_prefix("- **ID**: ") {
                    id = stripped.trim().to_string();
                } else if let Some(stripped) = line.strip_prefix("- **Recurrence**: ") {
                    let rec_str = stripped.trim();
                    recurrence = match rec_str {
                        "daily" => crate::app::Recurrence::Daily,
                        "weekly" => crate::app::Recurrence::Weekly,
                        "monthly" => crate::app::Recurrence::Monthly,
                        _ => crate::app::Recurrence::None,
                    };
                }
            }
            if let (Some(sd), Some(st)) = (start_date, start_time) {
                println!("Pushing event");
                events.push(CalendarEvent {
                    date: sd,
                    time: st,
                    title,
                    description,
                    recurrence,
                    is_recurring_instance: false,
                    base_date: None,
                    start_date: sd,
                    end_date,
                    start_time: st,
                    end_time,
                    id,
                });
            }
        }
    }

    events.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));

    // Generate recurring instances
    let mut all_events = events.clone();
    for event in &events {
        if event.recurrence != crate::app::Recurrence::None {
            let until = chrono::Local::now().date_naive() + Duration::days(365);
            all_events.extend(generate_recurring_instances(event, until));
        }
    }
    all_events.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
    all_events
}

fn generate_recurring_instances(
    base_event: &CalendarEvent,
    until: NaiveDate,
) -> Vec<CalendarEvent> {
    let mut instances = vec![];
    let mut current_date = base_event.date;

    while current_date <= until {
        if current_date != base_event.date {
            instances.push(CalendarEvent {
                date: current_date,
                time: base_event.time,
                title: base_event.title.clone(),
                description: base_event.description.clone(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: true,
                base_date: Some(base_event.date),
                start_date: current_date,
                end_date: base_event.end_date,
                start_time: base_event.start_time,
                end_time: base_event.end_time,
                id: String::new(),
            });
        }

        match base_event.recurrence {
            crate::app::Recurrence::Daily => current_date += Duration::days(1),
            crate::app::Recurrence::Weekly => current_date += Duration::weeks(1),
            crate::app::Recurrence::Monthly => {
                if let Some(new_date) = current_date.with_month(current_date.month() + 1) {
                    current_date = new_date;
                } else {
                    break;
                }
            }
            crate::app::Recurrence::None => break,
        }
    }
    instances
}

pub fn save_event(event: &mut CalendarEvent) -> Result<(), std::io::Error> {
    let home = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    save_event_to_path(event, &home.join("calendar"), None)
}

pub fn save_event_with_sync(
    event: &mut CalendarEvent,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    let home = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    save_event_to_path(event, &home.join("calendar"), sync_provider)
}

pub fn save_event_to_path(
    event: &mut CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(calendar_dir)?;

    let base_name = sanitize_title_for_filename(&event.title);
    let mut filename = format!("{base_name}.md");
    let mut counter = 1;
    while calendar_dir.join(&filename).exists() {
        filename = format!("{base_name}_{counter}.md");
        counter += 1;
    }
    let filepath = calendar_dir.join(&filename);
    event.id = filename.trim_end_matches(".md").to_string();

    let date_str = if let Some(end) = event.end_date {
        format!(
            "{} to {}",
            event.start_date.format("%Y-%m-%d"),
            end.format("%Y-%m-%d")
        )
    } else {
        event.start_date.format("%Y-%m-%d").to_string()
    };

    let time_str = if let Some(end) = event.end_time {
        format!(
            "{} to {}",
            event.start_time.format("%H:%M"),
            end.format("%H:%M")
        )
    } else {
        event.start_time.format("%H:%M").to_string()
    };

    let rec_str = match event.recurrence {
        crate::app::Recurrence::None => "none",
        crate::app::Recurrence::Daily => "daily",
        crate::app::Recurrence::Weekly => "weekly",
        crate::app::Recurrence::Monthly => "monthly",
    };

    let content = format!(
        "# Event: {}\n\n- **ID**: {}\n- **Date**: {}\n- **Time**: {}\n- **Description**: {}\n- **Recurrence**: {}\n",
        event.title, event.id, date_str, time_str, event.description, rec_str
    );

    std::fs::write(filepath, content)?;

    // Sync after save
    if let Some(provider) = sync_provider {
        if let Err(e) = provider.push(calendar_dir) {
            eprintln!("Sync push failed: {e}");
        }
    }
    Ok(())
}

pub fn delete_event(event: &CalendarEvent) -> Result<(), std::io::Error> {
    let home = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    delete_event_from_path(event, &home.join("calendar"), None)
}

pub fn delete_event_with_sync(
    event: &CalendarEvent,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    let home = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    delete_event_from_path(event, &home.join("calendar"), sync_provider)
}

pub fn delete_event_from_path(
    event: &CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    let filename = format!("{}.md", event.id);
    let filepath = calendar_dir.join(filename);
    std::fs::remove_file(filepath)?;

    // Sync after delete
    if let Some(provider) = sync_provider {
        if let Err(e) = provider.push(calendar_dir) {
            eprintln!("Sync push failed: {e}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_events_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let events = load_events_from_path(temp_dir.path());
        assert!(events.is_empty());
    }

    #[test]
    fn test_save_and_load_event() {
        let temp_dir = TempDir::new().unwrap();
        let mut event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };

        save_event_to_path(&mut event, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].date, event.date);
        assert_eq!(events[0].time, event.time);
        assert_eq!(events[0].title, event.title);
    }

    #[test]
    fn test_save_multiple_events_same_day() {
        let temp_dir = TempDir::new().unwrap();
        let mut event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            title: "Later Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };
        let mut event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Earlier Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };

        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 2);
        // Should be sorted by time
        assert_eq!(events[0].title, "Earlier Event");
        assert_eq!(events[1].title, "Later Event");
    }

    #[test]
    fn test_save_events_different_days() {
        let temp_dir = TempDir::new().unwrap();
        let mut event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Day 1 Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };
        let mut event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Day 2 Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };

        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 2);
        // Should be sorted by date then time
        assert_eq!(events[0].title, "Day 1 Event");
        assert_eq!(events[1].title, "Day 2 Event");
    }

    #[test]
    fn test_save_and_load_event_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let mut event = CalendarEvent {
            id: String::new(),
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Test Event".to_string(),
            description: "This is a test description".to_string(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
        };

        save_event_to_path(&mut event, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].date, event.date);
        assert_eq!(events[0].time, event.time);
        assert_eq!(events[0].title, event.title);
        assert_eq!(events[0].description, event.description);
    }

    #[test]
    fn test_delete_event() {
        let temp_dir = TempDir::new().unwrap();
        let mut event1 = CalendarEvent {
            id: String::new(),
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Event 1".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
        };
        let mut event2 = CalendarEvent {
            id: String::new(),
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            title: "Event 2".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            end_time: None,
        };

        // Save both events
        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();

        let events = load_events_from_path(temp_dir.path());
        assert_eq!(events.len(), 2);

        // Delete first event
        let _ = delete_event_from_path(&event1, temp_dir.path(), None);
        let events_after_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_delete.len(), 1);
        assert_eq!(events_after_delete[0].title, "Event 2");

        // Delete remaining event
        let _ = delete_event_from_path(&event2, temp_dir.path(), None);
        let events_after_second_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_second_delete.len(), 0);
    }

    #[test]
    fn test_sanitize_title_for_filename() {
        assert_eq!(sanitize_title_for_filename("Team Meeting"), "Team_Meeting");
        assert_eq!(sanitize_title_for_filename("Hello World!"), "Hello_World");
        assert_eq!(sanitize_title_for_filename(""), "untitled");
        assert_eq!(sanitize_title_for_filename("!@#"), "untitled");
        assert_eq!(sanitize_title_for_filename("   "), "untitled");
        assert_eq!(sanitize_title_for_filename("a_b"), "a_b");
        assert_eq!(sanitize_title_for_filename("123"), "123");
        assert_eq!(sanitize_title_for_filename("test-event"), "testevent");
        assert_eq!(sanitize_title_for_filename("Café"), "Café");
        assert_eq!(sanitize_title_for_filename("___hello___"), "hello");
        assert_eq!(sanitize_title_for_filename("hello___world"), "hello_world");
        let long_title = "a".repeat(150);
        let sanitized = sanitize_title_for_filename(&long_title);
        assert_eq!(sanitized.len(), 100);
        assert!(sanitized.starts_with("a"));
    }

    #[test]
    fn test_save_events_with_duplicate_titles() {
        let temp_dir = TempDir::new().unwrap();
        let mut event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            id: String::new(),
        };
        let mut event2 = event1.clone();
        event2.time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        event2.start_time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();

        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();

        // Check filenames exist
        let files: Vec<String> = std::fs::read_dir(temp_dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"Test_Event.md".to_string()));
        assert!(files.contains(&"Test_Event_1.md".to_string()));

        // Check IDs set correctly
        assert_eq!(event1.id, "Test_Event");
        assert_eq!(event2.id, "Test_Event_1");

        // Load and verify events
        let events = load_events_from_path(temp_dir.path());
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].title, "Test Event");
        assert_eq!(events[1].title, "Test Event");
        assert_eq!(events[0].id, "Test_Event");
        assert_eq!(events[1].id, "Test_Event_1");
    }
}
