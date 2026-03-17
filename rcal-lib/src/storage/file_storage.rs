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
                // Parse new format
                let mut id = None;
                let mut title = String::new();
                let mut start_date = None;
                let mut end_date = None;
                let mut start_time = None;
                let mut end_time = None;
                let mut description = String::new();
                let mut recurrence = Recurrence::None;
                for line in content.lines() {
                    if let Some(stripped) = line.strip_prefix("- **ID**: ") {
                        id = Some(stripped.trim().to_string());
                    } else if let Some(stripped) = line.strip_prefix("# Event: ") {
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
                    events.push(CalendarEvent {
                        id: id.unwrap_or_else(|| Uuid::new_v4().to_string()),
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

        // Use ID-based filename for reliable lookup by ID
        let filename = format!("{}.md", event.id);
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
            format!(
                "{} to {}",
                event.start_time.format("%H:%M"),
                end.format("%H:%M")
            )
        } else {
            event.start_time.format("%H:%M").to_string()
        };

        let rec_str = event.recurrence.to_storage_string();

        format!(
            "# Event: {}\n\n- **ID**: {}\n- **Date**: {}\n- **Time**: {}\n- **Description**: {}\n- **Recurrence**: {}\n",
            event.title, event.id, date_str, time_str, event.description, rec_str
        )
    }

    /// Finds the filepath for an event by ID.
    /// Since filenames are now ID-based, this first tries direct lookup
    /// and falls back to content search if needed.
    fn find_event_filepath(
        &self,
        calendar_dir: &Path,
        event: &CalendarEvent,
    ) -> Result<PathBuf, std::io::Error> {
        // First try direct filename lookup (optimization for ID-based filenames)
        let direct_path = calendar_dir.join(format!("{}.md", event.id));
        if direct_path.exists() {
            // Verify the content has the matching ID
            if let Ok(content) = fs::read_to_string(&direct_path) {
                for line in content.lines() {
                    if let Some(stripped) = line.strip_prefix("- **ID**: ") {
                        if stripped.trim() == event.id {
                            return Ok(direct_path);
                        }
                    }
                }
            }
        }

        // Fall back to content search
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
                    if let Some(stripped) = line.strip_prefix("- **ID**: ") {
                        let id = stripped.trim();
                        if id == event.id {
                            return Ok(path);
                        }
                    }
                }
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Event with id '{}' not found", event.id),
        ))
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

    /// Deletes an event by its ID from a specific directory.
    /// Uses find_event_filepath which has fallback logic:
    /// 1. First tries direct filename lookup ({id}.md)
    /// 2. Falls back to searching by content if not found
    ///
    /// This handles both ID-based filenames (new format) and title-based
    /// filenames (old format) that still have the ID in their content.
    pub fn delete_by_id_from_path(
        &self,
        id: &str,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // Create a minimal event with just the ID for find_event_filepath
        let event = CalendarEvent {
            id: id.to_string(),
            title: String::new(),
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

    fn delete(&self, id: &Uuid) -> Result<(), Box<dyn Error>> {
        // Load events to find the one with matching ID
        let events = self.load()?;
        if let Some(event) = events.iter().find(|e| e.id == id.to_string()) {
            self.delete_from_path(event, &self.path_provider.calendar_dir())?;
        }
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
        id: &Uuid,
        sync_provider: Option<&DynSyncProvider>,
        calendar_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // Default implementation - just delete without sync
        // Sync functionality can be handled at a higher level by the caller
        let _ = (sync_provider, calendar_dir);
        self.delete(id)
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
        assert_eq!(events[0].id, event.id); // Verify ID is preserved
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
    fn test_event_to_markdown_includes_id() {
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
        assert!(markdown.contains("- **ID**: test-uuid-1234"));
    }

    #[test]
    fn test_delete_event_by_id() {
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

        // Verify file exists (filename should be ID-based)
        let file_path = temp_dir.path().join("delete-test-id.md");
        assert!(file_path.exists());

        // Delete by ID
        repo.delete_by_id_from_path("delete-test-id", temp_dir.path())
            .unwrap();

        // Verify file is deleted
        assert!(!file_path.exists());
    }
}
