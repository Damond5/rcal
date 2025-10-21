use std::path::Path;

use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use dirs;

use crate::app::CalendarEvent;
use crate::sync::SyncProvider;

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
                                    let title_part = title_and_desc[0];
                                    let (title, recurrence) = if let Some(pos) =
                                        title_part.rfind(" (")
                                    {
                                        if title_part.ends_with(')') {
                                            let rec_str =
                                                &title_part[pos + 2..title_part.len() - 1];
                                            let rec = match rec_str {
                                                "daily" => crate::app::Recurrence::Daily,
                                                "weekly" => crate::app::Recurrence::Weekly,
                                                "monthly" => crate::app::Recurrence::Monthly,
                                                _ => crate::app::Recurrence::None,
                                            };
                                            (title_part[..pos].to_string(), rec)
                                        } else {
                                            (title_part.to_string(), crate::app::Recurrence::None)
                                        }
                                    } else {
                                        (title_part.to_string(), crate::app::Recurrence::None)
                                    };
                                    let description = if title_and_desc.len() > 1 {
                                        title_and_desc[1..].join(": ")
                                    } else {
                                        String::new()
                                    };
                                    events.push(CalendarEvent {
                                        date,
                                        time,
                                        title,
                                        description,
                                        recurrence,
                                        is_recurring_instance: false,
                                        base_date: None,
                                    });
                                }
                            }
                        }
                    }
                }
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

pub fn save_event(event: &CalendarEvent) {
    let home = dirs::home_dir().expect("Could not find home directory");
    save_event_to_path(event, &home.join("calendar"), None);
}

pub fn save_event_with_sync(event: &CalendarEvent, sync_provider: Option<&dyn SyncProvider>) {
    let home = dirs::home_dir().expect("Could not find home directory");
    save_event_to_path(event, &home.join("calendar"), sync_provider);
}

pub fn save_event_to_path(
    event: &CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) {
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
                        let title_part = title_and_desc[0];
                        let (title, recurrence) = if let Some(pos) = title_part.rfind(" (") {
                            if title_part.ends_with(')') {
                                let rec_str = &title_part[pos + 2..title_part.len() - 1];
                                let rec = match rec_str {
                                    "daily" => crate::app::Recurrence::Daily,
                                    "weekly" => crate::app::Recurrence::Weekly,
                                    "monthly" => crate::app::Recurrence::Monthly,
                                    _ => crate::app::Recurrence::None,
                                };
                                (title_part[..pos].to_string(), rec)
                            } else {
                                (title_part.to_string(), crate::app::Recurrence::None)
                            }
                        } else {
                            (title_part.to_string(), crate::app::Recurrence::None)
                        };
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
                            recurrence,
                            is_recurring_instance: false,
                            base_date: None,
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
        let rec_str = match e.recurrence {
            crate::app::Recurrence::None => "",
            crate::app::Recurrence::Daily => " (daily)",
            crate::app::Recurrence::Weekly => " (weekly)",
            crate::app::Recurrence::Monthly => " (monthly)",
        };
        if e.description.is_empty() {
            content.push_str(&format!(
                "- {} - {}{}\n",
                e.time.format("%H:%M"),
                e.title,
                rec_str
            ));
        } else {
            content.push_str(&format!(
                "- {} - {}: {}{}\n",
                e.time.format("%H:%M"),
                e.title,
                e.description,
                rec_str
            ));
        }
    }

    std::fs::write(filepath, content).expect("Could not write file");

    // Sync after save
    if let Some(provider) = sync_provider {
        if let Err(e) = provider.push(calendar_dir) {
            eprintln!("Sync push failed: {e}");
        }
    }
}

pub fn delete_event(event: &CalendarEvent) {
    let home = dirs::home_dir().expect("Could not find home directory");
    delete_event_from_path(event, &home.join("calendar"), None);
}

pub fn delete_event_with_sync(event: &CalendarEvent, sync_provider: Option<&dyn SyncProvider>) {
    let home = dirs::home_dir().expect("Could not find home directory");
    delete_event_from_path(event, &home.join("calendar"), sync_provider);
}

pub fn delete_event_from_path(
    event: &CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) {
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
                    let title_part = title_and_desc[0];
                    let (title, recurrence) = if let Some(pos) = title_part.rfind(" (") {
                        if title_part.ends_with(')') {
                            let rec_str = &title_part[pos + 2..title_part.len() - 1];
                            let rec = match rec_str {
                                "daily" => crate::app::Recurrence::Daily,
                                "weekly" => crate::app::Recurrence::Weekly,
                                "monthly" => crate::app::Recurrence::Monthly,
                                _ => crate::app::Recurrence::None,
                            };
                            (title_part[..pos].to_string(), rec)
                        } else {
                            (title_part.to_string(), crate::app::Recurrence::None)
                        }
                    } else {
                        (title_part.to_string(), crate::app::Recurrence::None)
                    };
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
                        recurrence,
                        is_recurring_instance: false,
                        base_date: None,
                    };
                    // Only keep events that don't match the one to delete
                    let should_delete = if event.is_recurring_instance {
                        parsed_event.date == event.base_date.unwrap()
                            && parsed_event.title == event.title
                            && parsed_event.time == event.time
                            && parsed_event.description == event.description
                    } else {
                        parsed_event == *event
                    };
                    if !should_delete {
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
                content.push_str(&format!(
                    "- {} - {}: {}\n",
                    e.time.format("%H:%M"),
                    e.title,
                    e.description
                ));
            }
        }
        std::fs::write(filepath, content).expect("Could not write file");
    }

    // Sync after delete
    if let Some(provider) = sync_provider {
        if let Err(e) = provider.push(calendar_dir) {
            eprintln!("Sync push failed: {e}");
        }
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
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };

        save_event_to_path(&event, temp_dir.path(), None);
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
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            title: "Earlier Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };

        save_event_to_path(&event1, temp_dir.path(), None);
        save_event_to_path(&event2, temp_dir.path(), None);
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
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Day 2 Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };

        save_event_to_path(&event1, temp_dir.path(), None);
        save_event_to_path(&event2, temp_dir.path(), None);
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
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };

        save_event_to_path(&event, temp_dir.path(), None);
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
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };
        let event2 = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            title: "Event 2".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
        };

        // Save both events
        save_event_to_path(&event1, temp_dir.path(), None);
        save_event_to_path(&event2, temp_dir.path(), None);

        let events = load_events_from_path(temp_dir.path());
        assert_eq!(events.len(), 2);

        // Delete first event
        delete_event_from_path(&event1, temp_dir.path(), None);
        let events_after_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_delete.len(), 1);
        assert_eq!(events_after_delete[0].title, "Event 2");

        // Delete remaining event
        delete_event_from_path(&event2, temp_dir.path(), None);
        let events_after_second_delete = load_events_from_path(temp_dir.path());
        assert_eq!(events_after_second_delete.len(), 0);
    }
}
