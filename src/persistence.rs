use std::path::Path;

use chrono::{Datelike, Duration, Local, Months, NaiveDate, NaiveTime};
use dirs;

use crate::app::CalendarEvent;
use crate::sync::SyncProvider;

pub fn is_finished_before(event: &CalendarEvent, cutoff: NaiveDate) -> bool {
    // Don't auto-delete recurring events to preserve ongoing schedules
    if event.recurrence != crate::app::Recurrence::None {
        return false;
    }
    let end_date = event.end_date.unwrap_or(event.start_date);
    end_date < cutoff
}

pub fn cleanup_old_events(
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<usize, Box<dyn std::error::Error>> {
    cleanup_old_events_with_cutoff(calendar_dir, sync_provider, Local::now().date_naive() - Months::new(2))
}

pub fn cleanup_old_events_with_cutoff(
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
    cutoff: NaiveDate,
) -> Result<usize, Box<dyn std::error::Error>> {
    let events = load_events_from_path(calendar_dir)?;

    let mut to_delete = Vec::new();
    for event in events {
        // Only consider base events for deletion, not generated recurring instances
        if event.is_recurring_instance {
            continue;
        }
        if is_finished_before(&event, cutoff) {
            to_delete.push(event);
        }
    }

    let mut deleted_count = 0;
    for event in to_delete {
        if let Err(e) = delete_event_from_path_without_sync(&event, calendar_dir) {
            eprintln!("Failed to delete old event '{}': {}", event.title, e);
        } else {
            deleted_count += 1;
        }
    }

    // Batch sync once after all deletions
    if deleted_count > 0 {
        if let Some(provider) = sync_provider {
            if let Err(e) = provider.push(calendar_dir) {
                eprintln!("Sync push failed after cleanup: {e}");
            }
        }
        println!("Cleaned up {deleted_count} old events.");
    }

    Ok(deleted_count)
}

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

fn find_event_filepath(calendar_dir: &Path, event: &CalendarEvent) -> Result<std::path::PathBuf, std::io::Error> {
    let entries = std::fs::read_dir(calendar_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.ends_with(".md"))
            .unwrap_or(false)
        {
            let content = std::fs::read_to_string(&path)?;
            for line in content.lines() {
                if let Some(stripped) = line.strip_prefix("# Event: ") {
                    let title = stripped.trim();
                    if title == event.title {
                        return Ok(path);
                    }
                }
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Event '{}' not found", event.title),
    ))
}

pub fn load_events() -> Result<Vec<CalendarEvent>, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    load_events_from_path(&home.join("calendar"))
}

pub fn load_events_from_path(
    calendar_dir: &Path,
) -> Result<Vec<CalendarEvent>, Box<dyn std::error::Error>> {
    if !calendar_dir.exists() {
        std::fs::create_dir_all(calendar_dir)?;
        return Ok(Vec::new());
    }

    let mut events = Vec::new();

    let entries = std::fs::read_dir(calendar_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.ends_with(".md"))
            .unwrap_or(false)
        {
            let content = std::fs::read_to_string(&path)?;
            // new format
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
                        start_date = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").ok();
                        end_date = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d").ok();
                    } else {
                        start_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
                    }
                } else if let Some(stripped) = line.strip_prefix("- **Time**: ") {
                    let time_str = stripped.trim();
                    if time_str.contains(" to ") {
                        let parts: Vec<&str> = time_str.split(" to ").collect();
                        start_time = NaiveTime::parse_from_str(parts[0], "%H:%M").ok();
                        end_time = NaiveTime::parse_from_str(parts[1], "%H:%M").ok();
                    } else {
                        start_time = NaiveTime::parse_from_str(time_str, "%H:%M").ok();
                    }
                } else if let Some(stripped) = line.strip_prefix("- **Description**: ") {
                    description = stripped.trim().to_string();
                } else if let Some(stripped) = line.strip_prefix("- **Recurrence**: ") {
                    let rec_str = stripped.trim();
                     recurrence = match rec_str {
                         "daily" => crate::app::Recurrence::Daily,
                         "weekly" => crate::app::Recurrence::Weekly,
                         "monthly" => crate::app::Recurrence::Monthly,
                         "yearly" => crate::app::Recurrence::Yearly,
                         _ => crate::app::Recurrence::None,
                     };
                }
            }
            if let Some(sd) = start_date {
                let is_all_day = start_time.is_none();
                let st = start_time.unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                events.push(CalendarEvent {
                    date: sd,
                    time: st,
                    title,
                    description,
                    recurrence,
                    is_recurring_instance: false,
                    base_date: None,
                    start_date: sd,
                    end_date: end_date.or(Some(sd)),
                    start_time: st,
                    end_time,
                    is_all_day,
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
    Ok(all_events)
}

pub fn generate_recurring_instances(
    base_event: &CalendarEvent,
    until: NaiveDate,
) -> Vec<CalendarEvent> {
    let mut instances = vec![];
    let mut current_date = base_event.date;

    while current_date <= until {
        if current_date != base_event.date {
            let end_date = base_event.end_date.map(|end| {
                let duration = end - base_event.start_date;
                current_date + duration
            });
            instances.push(CalendarEvent {
                date: current_date,
                time: base_event.time,
                title: base_event.title.clone(),
                description: base_event.description.clone(),
                recurrence: crate::app::Recurrence::None,
                is_recurring_instance: true,
                base_date: Some(base_event.date),
                start_date: current_date,
                end_date,
                start_time: base_event.start_time,
                end_time: base_event.end_time,
                is_all_day: base_event.is_all_day,
            });
        }

        match base_event.recurrence {
            crate::app::Recurrence::Daily => current_date += Duration::days(1),
            crate::app::Recurrence::Weekly => current_date += Duration::weeks(1),
            crate::app::Recurrence::Monthly => {
                // Handle invalid dates (e.g., Feb 31) by stopping generation to avoid errors
                if let Some(new_date) = current_date.with_month(current_date.month() + 1) {
                    current_date = new_date;
                } else {
                    break;
                }
            }
            crate::app::Recurrence::Yearly => {
                // Handle invalid dates (e.g., Feb 29 on non-leap years) by stopping generation
                if let Some(new_date) = current_date.with_year(current_date.year() + 1) {
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

pub fn save_event_to_path_without_sync(
    event: &mut CalendarEvent,
    calendar_dir: &Path,
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

    let date_str = if let Some(end) = event.end_date {
        if end != event.start_date {
            format!(
                "{} to {}",
                event.start_date.format("%Y-%m-%d"),
                end.format("%Y-%m-%d")
            )
        } else {
            event.start_date.format("%Y-%m-%d").to_string()
        }
    } else {
        event.start_date.format("%Y-%m-%d").to_string()
    };

    let time_str = if event.is_all_day {
        "all-day".to_string()
    } else if let Some(end) = event.end_time {
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
        crate::app::Recurrence::Yearly => "yearly",
    };

    let content = format!(
        "# Event: {}\n\n- **Date**: {}\n- **Time**: {}\n- **Description**: {}\n- **Recurrence**: {}\n",
        event.title, date_str, time_str, event.description, rec_str
    );

    std::fs::write(filepath, content)?;

    Ok(())
}

pub fn save_event_to_path(
    event: &mut CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    save_event_to_path_without_sync(event, calendar_dir)?;

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

pub fn delete_event_from_path_without_sync(
    event: &CalendarEvent,
    calendar_dir: &Path,
) -> Result<(), std::io::Error> {
    let filepath = find_event_filepath(calendar_dir, event)?;
    std::fs::remove_file(filepath)?;

    Ok(())
}

pub fn delete_event_from_path(
    event: &CalendarEvent,
    calendar_dir: &Path,
    sync_provider: Option<&dyn SyncProvider>,
) -> Result<(), std::io::Error> {
    delete_event_from_path_without_sync(event, calendar_dir)?;

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
        let events = load_events_from_path(temp_dir.path()).unwrap();
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
            is_all_day: false,
        };

        save_event_to_path(&mut event, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path()).unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].date, event.date);
        assert_eq!(events[0].time, event.time);
        assert_eq!(events[0].title, event.title);
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
            is_all_day: false,
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
            is_all_day: false,
        };

        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path()).unwrap();

        assert_eq!(events.len(), 2);
        // Should be sorted by date then time
        assert_eq!(events[0].title, "Day 1 Event");
        assert_eq!(events[1].title, "Day 2 Event");
    }

    #[test]
    fn test_save_and_load_all_day_event() {
        let temp_dir = TempDir::new().unwrap();
        let mut event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            title: "All Day Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            end_time: None,
            is_all_day: true,
        };

        save_event_to_path(&mut event, temp_dir.path(), None).unwrap();
        let events = load_events_from_path(temp_dir.path()).unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].date, event.date);
        assert_eq!(events[0].time, event.time);
        assert_eq!(events[0].title, event.title);
        assert!(events[0].is_all_day);
    }

    #[test]
    fn test_save_and_load_event_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let mut event = CalendarEvent {
            is_all_day: false,
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
        let events = load_events_from_path(temp_dir.path()).unwrap();

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
            is_all_day: false,
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
            is_all_day: false,
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

        let events = load_events_from_path(temp_dir.path()).unwrap();
        assert_eq!(events.len(), 2);

        // Delete first event
        let _ = delete_event_from_path(&event1, temp_dir.path(), None);
        let events_after_delete = load_events_from_path(temp_dir.path()).unwrap();
        assert_eq!(events_after_delete.len(), 1);
        assert_eq!(events_after_delete[0].title, "Event 2");

        // Delete remaining event
        let _ = delete_event_from_path(&event2, temp_dir.path(), None);
        let events_after_second_delete = load_events_from_path(temp_dir.path()).unwrap();
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
            is_all_day: false,
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

        // Load and verify events
        let events = load_events_from_path(temp_dir.path()).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].title, "Test Event");
        assert_eq!(events[1].title, "Test Event");
    }

    #[test]
    fn test_delete_event_with_duplicate_titles() {
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
            is_all_day: false,
        };
        let mut event2 = event1.clone();
        event2.time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();
        event2.start_time = NaiveTime::from_hms_opt(11, 0, 0).unwrap();

        save_event_to_path(&mut event1, temp_dir.path(), None).unwrap();
        save_event_to_path(&mut event2, temp_dir.path(), None).unwrap();

        // Load events
        let mut events = load_events_from_path(temp_dir.path()).unwrap();
        assert_eq!(events.len(), 2);

        // Delete the first event (both have same title, should delete one)
        let event_to_delete = events.remove(0);
        delete_event_from_path(&event_to_delete, temp_dir.path(), None).unwrap();

        // Load again, should have one left
        let events_after_delete = load_events_from_path(temp_dir.path()).unwrap();
        assert_eq!(events_after_delete.len(), 1);
        assert_eq!(events_after_delete[0].title, "Test Event");
    }

    #[test]
    fn test_generate_recurring_instances_daily() {
        let base_event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Daily Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::Daily,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };
        let until = NaiveDate::from_ymd_opt(2023, 10, 5).unwrap();
        let instances = generate_recurring_instances(&base_event, until);
        assert_eq!(instances.len(), 4); // 2,3,4,5
        assert_eq!(instances[0].start_date, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap());
        assert_eq!(instances[1].start_date, NaiveDate::from_ymd_opt(2023, 10, 3).unwrap());
        assert!(instances.iter().all(|i| i.is_recurring_instance));
        assert!(instances.iter().all(|i| i.base_date == Some(base_event.date)));
    }

    #[test]
    fn test_generate_recurring_instances_weekly() {
        let base_event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Weekly Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::Weekly,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };
        let until = NaiveDate::from_ymd_opt(2023, 10, 22).unwrap();
        let instances = generate_recurring_instances(&base_event, until);
        assert_eq!(instances.len(), 3); // 8,15,22
        assert_eq!(instances[0].start_date, NaiveDate::from_ymd_opt(2023, 10, 8).unwrap());
    }

    #[test]
    fn test_generate_recurring_instances_yearly() {
        let base_event = CalendarEvent {
            date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            title: "Yearly Event".to_string(),
            description: String::new(),
            recurrence: crate::app::Recurrence::Yearly,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };
        let until = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
        let instances = generate_recurring_instances(&base_event, until);
        assert_eq!(instances.len(), 3); // 2024, 2025, 2026
        assert_eq!(instances[0].start_date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
        assert_eq!(instances[1].start_date, NaiveDate::from_ymd_opt(2025, 10, 1).unwrap());
        assert_eq!(instances[2].start_date, NaiveDate::from_ymd_opt(2026, 10, 1).unwrap());
    }
}
