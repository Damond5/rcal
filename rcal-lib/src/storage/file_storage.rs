//! File-based storage implementation for calendar events.
//!
//! Provides FileEventRepository and DefaultPathProvider for storing events
//! as Markdown files in the file system.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{Datelike, Duration, Months, NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::models::{CalendarEvent, Recurrence};
use crate::storage::traits::{CalendarPathProvider, DynSyncProvider, EventRepository};

/// Default path provider that uses ~/calendar as the calendar directory.
pub struct DefaultPathProvider {
    calendar_dir: PathBuf,
}

impl DefaultPathProvider {
    /// Creates a new DefaultPathProvider with the default calendar directory.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        Ok(Self {
            calendar_dir: home.join("calendar"),
        })
    }

    /// Creates a new DefaultPathProvider with a custom calendar directory.
    pub fn with_path(path: PathBuf) -> Self {
        Self { calendar_dir: path }
    }
}

impl Default for DefaultPathProvider {
    fn default() -> Self {
        Self {
            calendar_dir: dirs::home_dir()
                .expect("Could not find home directory")
                .join("calendar"),
        }
    }
}

impl CalendarPathProvider for DefaultPathProvider {
    fn calendar_dir(&self) -> PathBuf {
        self.calendar_dir.clone()
    }
}

/// File-based event repository that stores events as Markdown files.
pub struct FileEventRepository {
    path_provider: Box<dyn CalendarPathProvider>,
}

impl FileEventRepository {
    /// Creates a new FileEventRepository with the given path provider.
    pub fn new(path_provider: Box<dyn CalendarPathProvider>) -> Self {
        Self { path_provider }
    }

    /// Creates a new FileEventRepository with a default path provider.
    pub fn with_default_path() -> Result<Self, Box<dyn Error>> {
        Ok(Self::new(Box::new(DefaultPathProvider::new()?)))
    }

    /// Creates a new FileEventRepository with a custom path.
    pub fn with_path(path: PathBuf) -> Self {
        Self::new(Box::new(DefaultPathProvider::with_path(path)))
    }

    /// Returns the calendar directory.
    pub fn calendar_dir(&self) -> PathBuf {
        self.path_provider.calendar_dir()
    }

    /// Loads events from a specific directory.
    pub fn load_from_path(
        &self,
        calendar_dir: &Path,
    ) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
        if !calendar_dir.exists() {
            fs::create_dir_all(calendar_dir)?;
            return Ok(Vec::new());
        }

        let mut events = Vec::new();

        let entries = fs::read_dir(calendar_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
            {
                let content = fs::read_to_string(&path)?;
                // Parse new format (title-based, no ID in content)
                let mut title = String::new();
                let mut start_date = None;
                let mut end_date = None;
                let mut start_time = None;
                let mut end_time = None;
                let mut description = String::new();
                let mut recurrence = Recurrence::None;
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
                        recurrence = Recurrence::from_storage_string(rec_str);
                    }
                }
                if let Some(sd) = start_date {
                    let is_all_day = start_time.is_none();
                    let st = start_time.unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                    // Generate a new UUID for each loaded event
                    events.push(CalendarEvent {
                        id: Uuid::new_v4().to_string(),
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

        events.sort_by(|a, b| {
            a.start_date
                .cmp(&b.start_date)
                .then(a.start_time.cmp(&b.start_time))
        });

        Ok(events)
    }

    /// Saves an event to a specific directory.
    pub fn save_to_path(
        &self,
        event: &CalendarEvent,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(calendar_dir)?;

        // Check if an event with the same title AND start_date already exists
        // If so, delete it first to avoid duplicates
        if let Ok(existing_path) =
            self.find_event_filepath_by_key(calendar_dir, &event.title, event.start_date)
        {
            fs::remove_file(existing_path)?;
        }

        // Use title-based filename with collision handling
        let base_name = sanitize_title_for_filename(&event.title);
        let mut filename = format!("{base_name}.md");
        let mut counter = 1;
        while calendar_dir.join(&filename).exists() {
            filename = format!("{base_name}_{counter}.md");
            counter += 1;
        }
        let filepath = calendar_dir.join(&filename);

        let content = Self::event_to_markdown(event);

        fs::write(filepath, content)?;

        Ok(())
    }

    /// Converts an event to markdown format.
    pub fn event_to_markdown(event: &CalendarEvent) -> String {
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
            // Check if end_time equals start_time (no duration)
            if end == event.start_time {
                event.start_time.format("%H:%M").to_string()
            } else {
                format!(
                    "{} to {}",
                    event.start_time.format("%H:%M"),
                    end.format("%H:%M")
                )
            }
        } else {
            event.start_time.format("%H:%M").to_string()
        };

        let rec_str = event.recurrence.to_storage_string();

        format!(
            "# Event: {}\n\n- **Date**: {}\n- **Time**: {}\n- **Description**: {}\n- **Recurrence**: {}\n",
            event.title, date_str, time_str, event.description, rec_str
        )
    }

    /// Finds the filepath for an event by title.
    /// Searches through all .md files in the directory and looks for
    /// a matching title in the content.
    fn find_event_filepath(
        &self,
        calendar_dir: &Path,
        event: &CalendarEvent,
    ) -> Result<PathBuf, std::io::Error> {
        let entries = fs::read_dir(calendar_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
            {
                let content = fs::read_to_string(&path)?;
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

    /// Finds the filepath for an event by title and start_date key.
    /// This is used for delete operations since UUIDs are not persisted in files.
    fn find_event_filepath_by_key(
        &self,
        calendar_dir: &Path,
        title: &str,
        start_date: NaiveDate,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let entries = fs::read_dir(calendar_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
            {
                let content = fs::read_to_string(&path)?;
                let mut found_title: Option<String> = None;
                let mut found_date: Option<NaiveDate> = None;

                for line in content.lines() {
                    if let Some(stripped) = line.strip_prefix("# Event: ") {
                        found_title = Some(stripped.trim().to_string());
                    } else if let Some(stripped) = line.strip_prefix("- **Date**: ") {
                        let date_str = stripped.trim();
                        // Extract just the start date (before " to " if present)
                        let start_str = if date_str.contains(" to ") {
                            date_str.split(" to ").next().unwrap_or(date_str)
                        } else {
                            date_str
                        };
                        found_date = NaiveDate::parse_from_str(start_str, "%Y-%m-%d").ok();
                    }
                }

                if let Some(ref t) = found_title {
                    if t == title && found_date == Some(start_date) {
                        return Ok(path);
                    }
                }
            }
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Event '{}' on {} not found", title, start_date),
        )))
    }

    /// Deletes an event from a specific directory.
    pub fn delete_from_path(
        &self,
        event: &CalendarEvent,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let filepath = self.find_event_filepath(calendar_dir, event)?;
        fs::remove_file(filepath)?;
        Ok(())
    }

    /// Deletes an event by its title from a specific directory.
    /// Searches for the event file by title in the content.
    pub fn delete_by_title_from_path(
        &self,
        title: &str,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // Create a minimal event with just the title for find_event_filepath
        let event = CalendarEvent {
            id: String::new(),
            title: title.to_string(),
            description: String::new(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let filepath = self.find_event_filepath(calendar_dir, &event)?;
        fs::remove_file(filepath)?;
        Ok(())
    }

    /// Generates recurring event instances for the given base events within the specified date range.
    pub fn generate_instances_for_range(
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
                        return instances;
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
                    id: Uuid::new_v4().to_string(),
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

    /// Generates recurring instances for a base event until a specific date.
    pub fn generate_recurring_instances(
        base_event: &CalendarEvent,
        until: NaiveDate,
    ) -> Vec<CalendarEvent> {
        Self::generate_recurring_instances_in_range(base_event, base_event.start_date, until)
    }
}

impl EventRepository for FileEventRepository {
    fn load(&self) -> Result<Vec<CalendarEvent>, Box<dyn Error>> {
        self.load_from_path(&self.path_provider.calendar_dir())
    }

    fn save(&self, event: &CalendarEvent) -> Result<(), Box<dyn Error>> {
        self.save_to_path(event, &self.path_provider.calendar_dir())
    }

    fn delete(&self, title: &str, start_date: NaiveDate) -> Result<(), Box<dyn Error>> {
        // Find and delete event by title and start_date
        let filepath =
            self.find_event_filepath_by_key(&self.path_provider.calendar_dir(), title, start_date)?;
        fs::remove_file(filepath)?;
        Ok(())
    }

    fn save_with_sync(
        &self,
        event: &CalendarEvent,
        sync_provider: Option<&DynSyncProvider>,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // Default implementation - just save without sync
        // Sync functionality can be handled at a higher level by the caller
        let _ = (sync_provider, calendar_dir);
        self.save_to_path(event, &self.path_provider.calendar_dir())
    }

    fn delete_with_sync(
        &self,
        title: &str,
        start_date: NaiveDate,
        sync_provider: Option<&DynSyncProvider>,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // Default implementation - just delete without sync
        // Sync functionality can be handled at a higher level by the caller
        let _ = (sync_provider, calendar_dir);
        self.delete(title, start_date)
    }
}

/// Sanitizes a title for use as a filename.
/// Note: This function is kept for potential future use but is not currently used
/// since we now use ID-based filenames.
#[allow(dead_code)]
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
        "untitled".to_string()
    } else {
        collapsed
    }
}

/// Checks if an event finished before a cutoff date.
pub fn is_finished_before(event: &CalendarEvent, cutoff: NaiveDate) -> bool {
    // Don't auto-delete recurring events to preserve ongoing schedules
    if event.recurrence != Recurrence::None {
        return false;
    }
    let end_date = event.end_date.unwrap_or(event.start_date);
    end_date < cutoff
}

/// Cleans up old events from the calendar directory.
pub fn cleanup_old_events(
    calendar_dir: &Path,
    sync_provider: Option<&DynSyncProvider>,
) -> Result<usize, Box<dyn Error>> {
    cleanup_old_events_with_cutoff(
        calendar_dir,
        sync_provider,
        chrono::Local::now().date_naive() - Months::new(2),
    )
}

/// Cleans up old events older than a specific cutoff date.
pub fn cleanup_old_events_with_cutoff(
    calendar_dir: &Path,
    _sync_provider: Option<&DynSyncProvider>,
    cutoff: NaiveDate,
) -> Result<usize, Box<dyn Error>> {
    let repo = FileEventRepository::with_path(calendar_dir.to_path_buf());
    let events = repo.load()?;

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
        if let Err(e) = repo.delete_from_path(&event, calendar_dir) {
            eprintln!("Failed to delete old event '{}': {}", event.title, e);
        } else {
            deleted_count += 1;
        }
    }

    // Sync can be handled at a higher level by the caller
    println!("Cleaned up {deleted_count} old events.");

    Ok(deleted_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_events_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileEventRepository::with_path(temp_dir.path().to_path_buf());
        let events = repo.load().unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn test_save_and_load_event() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileEventRepository::with_path(temp_dir.path().to_path_buf());

        let event = CalendarEvent {
            id: "test-id".to_string(),
            title: "Test Event".to_string(),
            description: String::new(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        repo.save(&event).unwrap();
        let events = repo.load().unwrap();

        assert_eq!(events.len(), 1);
        // ID is NOT preserved - a new UUID is generated when loading
        assert_ne!(events[0].id, event.id);
        // But title and other fields should match
        assert_eq!(events[0].start_date, event.start_date);
        assert_eq!(events[0].start_time, event.start_time);
        assert_eq!(events[0].title, event.title);
    }

    #[test]
    fn test_sanitize_title_for_filename() {
        assert_eq!(sanitize_title_for_filename("Team Meeting"), "Team_Meeting");
        assert_eq!(sanitize_title_for_filename("Hello World!"), "Hello_World");
        assert_eq!(sanitize_title_for_filename(""), "untitled");
        assert_eq!(sanitize_title_for_filename("!@#"), "untitled");
        assert_eq!(sanitize_title_for_filename("   "), "untitled");
    }

    #[test]
    fn test_generate_recurring_instances_daily() {
        let base_event = CalendarEvent {
            id: "test-id".to_string(),
            title: "Daily Event".to_string(),
            description: String::new(),
            recurrence: Recurrence::Daily,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let until = NaiveDate::from_ymd_opt(2023, 10, 5).unwrap();
        let instances = FileEventRepository::generate_recurring_instances(&base_event, until);

        assert_eq!(instances.len(), 4); // 2,3,4,5
    }

    #[test]
    fn test_default_path_provider() {
        let provider = DefaultPathProvider::new().unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(provider.calendar_dir(), home.join("calendar"));
    }

    #[test]
    fn test_event_to_markdown_excludes_id() {
        let event = CalendarEvent {
            id: "test-uuid-1234".to_string(),
            title: "Test Event".to_string(),
            description: "Test description".to_string(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        let markdown = FileEventRepository::event_to_markdown(&event);
        // ID should NOT be in the markdown content
        assert!(!markdown.contains("- **ID**:"));
        // But title should be there
        assert!(markdown.contains("# Event: Test Event"));
    }

    #[test]
    fn test_delete_event_by_title() {
        let temp_dir = TempDir::new().unwrap();
        let repo = FileEventRepository::with_path(temp_dir.path().to_path_buf());

        let event = CalendarEvent {
            id: "delete-test-id".to_string(),
            title: "Event to Delete".to_string(),
            description: String::new(),
            recurrence: Recurrence::None,
            is_recurring_instance: false,
            base_date: None,
            start_date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end_date: None,
            start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
            end_time: None,
            is_all_day: false,
        };

        // Save the event
        repo.save(&event).unwrap();

        // Verify file exists (filename should be title-based)
        let file_path = temp_dir.path().join("Event_to_Delete.md");
        assert!(file_path.exists());

        // Delete by title
        repo.delete_by_title_from_path("Event to Delete", temp_dir.path())
            .unwrap();

        // Verify file is deleted
        assert!(!file_path.exists());
    }
}
