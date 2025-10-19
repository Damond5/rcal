use std::path::Path;

use chrono::{NaiveDate, NaiveTime};
use dirs;

use crate::app::CalendarEvent;

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
        if path.extension() == Some(std::ffi::OsStr::new("md")) {
            if let Some(date_str) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    let content = std::fs::read_to_string(&path).expect("Could not read file");
                    for line in content.lines().skip(1) {
                        // skip header
                        if line.trim_start().starts_with("- ") {
                            let line = line.trim_start();
                            let parts: Vec<&str> = line[2..].split(" - ").collect();
                            if parts.len() == 2 {
                                if let Ok(time) = NaiveTime::parse_from_str(parts[0], "%H:%M") {
                                    let title_and_desc: Vec<&str> = parts[1].split(": ").collect();
                                    let title = title_and_desc[0].to_string();
                                    let description = if title_and_desc.len() > 1 {
                                        title_and_desc[1..].join(": ")
                                    } else {
                                        String::new()
                                    };
                                    events.push(CalendarEvent { date, time, title, description });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    events.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
    events
}

pub fn save_event(event: &CalendarEvent) {
    let home = dirs::home_dir().expect("Could not find home directory");
    save_event_to_path(event, &home.join("calendar"));
}

pub fn save_event_to_path(event: &CalendarEvent, calendar_dir: &Path) {
    std::fs::create_dir_all(calendar_dir).expect("Could not create calendar directory");

    let filename = format!("{}.md", event.date.format("%Y-%m-%d"));
    let filepath = calendar_dir.join(filename);

    let mut events_for_day = Vec::new();

    if filepath.exists() {
        let content = std::fs::read_to_string(&filepath).expect("Could not read file");
        for line in content.lines().skip(1) {
            if line.trim_start().starts_with("- ") {
                let line = line.trim_start();
                let parts: Vec<&str> = line[2..].split(" - ").collect();
                if parts.len() == 2 {
                    if let Ok(time) = NaiveTime::parse_from_str(parts[0], "%H:%M") {
                        let title_and_desc: Vec<&str> = parts[1].split(": ").collect();
                        let title = title_and_desc[0].to_string();
                        let description = if title_and_desc.len() > 1 {
                            title_and_desc[1..].join(": ")
                        } else {
                            String::new()
                        };
                        events_for_day.push(CalendarEvent {
                            date: event.date,
                            time,
                            title,
                            description,
                        });
                    }
                }
            }
        }
    }

    events_for_day.push(event.clone());
    events_for_day.sort_by(|a, b| a.time.cmp(&b.time));

    let mut content = format!("# Events for {}\n\n", event.date.format("%Y-%m-%d"));
    for e in events_for_day {
        if e.description.is_empty() {
            content.push_str(&format!("- {} - {}\n", e.time.format("%H:%M"), e.title));
        } else {
            content.push_str(&format!("- {} - {}: {}\n", e.time.format("%H:%M"), e.title, e.description));
        }
    }

    std::fs::write(filepath, content).expect("Could not write file");
}

pub fn delete_event(event: &CalendarEvent) {
    let home = dirs::home_dir().expect("Could not find home directory");
    delete_event_from_path(event, &home.join("calendar"));
}

pub fn delete_event_from_path(event: &CalendarEvent, calendar_dir: &Path) {
    let filename = format!("{}.md", event.date.format("%Y-%m-%d"));
    let filepath = calendar_dir.join(filename);

    if !filepath.exists() {
        return; // Event file doesn't exist, nothing to delete
    }

    let content = std::fs::read_to_string(&filepath).expect("Could not read file");
    let mut events_for_day = Vec::new();

    // Parse all events from the file
    for line in content.lines().skip(1) {
        if line.trim_start().starts_with("- ") {
            let line = line.trim_start();
            let parts: Vec<&str> = line[2..].split(" - ").collect();
            if parts.len() == 2 {
                if let Ok(time) = NaiveTime::parse_from_str(parts[0], "%H:%M") {
                    let title_and_desc: Vec<&str> = parts[1].split(": ").collect();
                    let title = title_and_desc[0].to_string();
                    let description = if title_and_desc.len() > 1 {
                        title_and_desc[1..].join(": ")
                    } else {
                        String::new()
                    };
                    let parsed_event = CalendarEvent {
                        date: event.date,
                        time,
                        title,
                        description,
                    };
                    // Only keep events that don't match the one to delete
                    if parsed_event != *event {
                        events_for_day.push(parsed_event);
                    }
                }
            }
        }
    }

    // Rewrite the file with remaining events
    if events_for_day.is_empty() {
        // If no events left, remove the file
        let _ = std::fs::remove_file(filepath);
    } else {
        let mut content = format!("# Events for {}\n\n", event.date.format("%Y-%m-%d"));
        for e in events_for_day {
            if e.description.is_empty() {
                content.push_str(&format!("- {} - {}\n", e.time.format("%H:%M"), e.title));
            } else {
                content.push_str(&format!("- {} - {}: {}\n", e.time.format("%H:%M"), e.title, e.description));
            }
        }
        std::fs::write(filepath, content).expect("Could not write file");
    }
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
        let event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Test Event".to_string(),
            description: String::new(),
        };

        save_event_to_path(&event, temp_dir.path());
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].date, event.date);
        assert_eq!(events[0].time, event.time);
        assert_eq!(events[0].title, event.title);
    }

    #[test]
    fn test_save_multiple_events_same_day() {
        let temp_dir = TempDir::new().unwrap();
        let event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            title: "Later Event".to_string(),
            description: String::new(),
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Earlier Event".to_string(),
            description: String::new(),
        };

        save_event_to_path(&event1, temp_dir.path());
        save_event_to_path(&event2, temp_dir.path());
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 2);
        // Should be sorted by time
        assert_eq!(events[0].title, "Earlier Event");
        assert_eq!(events[1].title, "Later Event");
    }

    #[test]
    fn test_save_events_different_days() {
        let temp_dir = TempDir::new().unwrap();
        let event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Day 1 Event".to_string(),
            description: String::new(),
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Day 2 Event".to_string(),
            description: String::new(),
        };

        save_event_to_path(&event1, temp_dir.path());
        save_event_to_path(&event2, temp_dir.path());
        let events = load_events_from_path(temp_dir.path());

        assert_eq!(events.len(), 2);
        // Should be sorted by date then time
        assert_eq!(events[0].title, "Day 1 Event");
        assert_eq!(events[1].title, "Day 2 Event");
    }

    #[test]
    fn test_save_and_load_event_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Test Event".to_string(),
            description: "This is a test description".to_string(),
        };

        save_event_to_path(&event, temp_dir.path());
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
        let event1 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Event 1".to_string(),
            description: String::new(),
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            title: "Event 2".to_string(),
            description: String::new(),
        };

        // Save both events
        save_event_to_path(&event1, temp_dir.path());
        save_event_to_path(&event2, temp_dir.path());

        let events = load_events_from_path(temp_dir.path());
        assert_eq!(events.len(), 2);

        // Delete first event
        delete_event_from_path(&event1, temp_dir.path());
        let events_after_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_delete.len(), 1);
        assert_eq!(events_after_delete[0].title, "Event 2");

        // Delete remaining event
        delete_event_from_path(&event2, temp_dir.path());
        let events_after_second_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_second_delete.len(), 0);
    }
}
