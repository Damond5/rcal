use std::io;

use chrono::{Datelike, NaiveDate, NaiveTime};
use crossterm::event::{self, Event as CrosstermEvent, KeyCode};
use dirs;
use ratatui::Terminal;
use ratatui::backend::Backend;
use std::sync::mpsc::TryRecvError;
use std::thread;

use crate::app::{App, CalendarEvent, InputMode, PopupInputField, Recurrence};
use crate::date_utils;

fn recurrence_str_to_index(s: &str) -> usize {
    match s.to_lowercase().as_str() {
        "daily" => 1,
        "weekly" => 2,
        "monthly" => 3,
        "yearly" => 4,
        _ => 0, // Default to "none" for invalid recurrence strings
    }
}
use crate::persistence;
use crate::sync::SyncProvider;
use crate::ui::ui;

/// Normalizes time input to HH:MM format.
/// Handles inputs like "14", "9", "14:30", returning "HH:MM" or the original if invalid.
/// Used for consistent time parsing in event creation.
fn normalize_time_input(input: &str) -> String {
    let trimmed = input.trim();

    // If it's already in HH:MM format, return as is
    if trimmed.contains(':') {
        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() == 2 {
            let hour = parts[0].parse::<u32>().unwrap_or(0);
            let minute = parts[1].parse::<u32>().unwrap_or(0);
            return format!("{hour:02}:{minute:02}");
        }
    }

    // If it's just hours, add :00
    if let Ok(hour) = trimmed.parse::<u32>() {
        if hour <= 23 {
            return format!("{hour:02}:00");
        }
    }

    // If it's a single digit hour, pad with zero and add :00
    if trimmed.len() == 1 {
        if let Ok(hour) = trimmed.parse::<u32>() {
            if hour <= 9 {
                return format!("0{hour}:00");
            }
        }
    }

    // Return original if we can't parse it
    trimmed.to_string()
}

pub fn find_base_event_for_instance(instance: &CalendarEvent, events: &[CalendarEvent]) -> Option<CalendarEvent> {
    if !instance.is_recurring_instance {
        return None;
    }
    if let Some(base_date) = instance.base_date {
        events.iter().find(|e| {
            e.date == base_date
                && e.title == instance.title
                && e.time == instance.time
                && e.description == instance.description
                && !e.is_recurring_instance
        }).cloned()
    } else {
        eprintln!("Warning: Recurring instance lacks base_date: {}", instance.title);
        None
    }
}



pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // Check for reload signal from async sync
        if let Some(ref receiver) = app.reload_receiver {
            match receiver.try_recv() {
                Ok(Ok(_)) => {
                     // Reload events
                     app.events = persistence::load_events_from_path(&app.calendar_dir)
                         .unwrap_or_else(|e| {
                             eprintln!("Failed to reload events after sync: {e}");
                             Vec::new()
                         });
                     // Invalidate cached instances after reloading events
                     app.invalidate_instance_cache(None);
                     // Update sync status (silently, don't interfere with sync popup)
                     app.sync_status = Some(crate::sync::SyncStatus::UpToDate);
                }
                Ok(Err(e)) => {
                    // Update sync status on error (silently, don't interfere with sync popup)
                    app.sync_status = Some(crate::sync::SyncStatus::Error(e));
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {}
            }
        }

        let event = event::read()?;
        if !handle_event(&mut app, event)? {
            break;
        }
    }
    Ok(())
}

pub fn handle_event(app: &mut App, event: CrosstermEvent) -> io::Result<bool> {
    if let CrosstermEvent::Key(key) = event {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('a') => {
                    app.show_add_event_popup = true;
                    app.input_mode = InputMode::EditingEventPopup;
                    app.current_date_for_new_event = app.date;
                    app.popup_event_title.clear();
                    app.popup_event_time.clear();
                    app.popup_event_end_date.clear();
                    app.popup_event_end_time.clear();
                    app.popup_event_recurrence.clear();
                    app.popup_event_description.clear();
                    app.input.clear();
                    app.selected_input_field = PopupInputField::Title;
                    app.cursor_position = 0;
                    app.is_editing = false;
                    app.event_being_edited = None;
                    app.date_input_error = None;
                    app.date_suggestions.clear();
                    app.show_date_suggestions = false;
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    app.date -= chrono::Duration::days(1);
                    app.adjust_view_boundaries();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    app.date += chrono::Duration::days(1);
                    app.adjust_view_boundaries();
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.date -= chrono::Duration::weeks(1);
                    app.adjust_view_boundaries();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.date += chrono::Duration::weeks(1);
                    app.adjust_view_boundaries();
                }

                KeyCode::Char('o') => {
                    app.show_view_events_popup = true;
                    let all_events = app.get_all_events_for_range(app.date, app.date);
                     app.events_to_display_in_popup = all_events
                         .iter()
                         .filter(|event| {
                             if let Some(end) = event.end_date {
                                 event.start_date <= app.date && end >= app.date
                             } else {
                                 event.start_date == app.date
                             }
                         })
                         .cloned()
                         .collect();
                    app.events_to_display_in_popup
                        .sort_by_key(|event| event.time);
                    app.selected_event_index = 0;
                    app.input_mode = InputMode::ViewEventsPopup;
                }
                KeyCode::Char('s') => {
                    if app.sync_provider.is_some() {
                        app.input_mode = InputMode::Sync;
                        app.sync_message.clear();
                        app.sync_status = None;
                        // Automatically check sync status on entry
                        if let Some(provider) = &app.sync_provider {
                            match provider.status(&app.calendar_dir) {
                                Ok(status) => {
                                    app.sync_message = match &status {
                                        crate::sync::SyncStatus::UpToDate => "".to_string(),
                                        crate::sync::SyncStatus::Ahead => {
                                            "Ahead of remote".to_string()
                                        }
                                        crate::sync::SyncStatus::Behind => {
                                            "Behind remote".to_string()
                                        }
                                        crate::sync::SyncStatus::Conflicts => {
                                            "Conflicts detected".to_string()
                                        }
                                        crate::sync::SyncStatus::Error(e) => {
                                            format!("Status error: {e}")
                         }
                     };
                     if app.selected_input_field == PopupInputField::Recurrence {
                         app.input_mode = InputMode::SelectingRecurrence;
                         app.selected_recurrence_index = recurrence_str_to_index(&app.popup_event_recurrence);
                     }
                 }
                                Err(e) => {
                                    app.sync_message = format!("Status failed: {e}");
                                    app.sync_status =
                                        Some(crate::sync::SyncStatus::Error(e.to_string()));
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            InputMode::EditingEventPopup => match key.code {
                KeyCode::Enter => {
                     if app.popup_event_title.trim().is_empty() {
                         app.error_message = "Title cannot be empty".to_string();
                         return Ok(true);
                     }
                     if let Some(ref error) = app.date_input_error {
                         app.error_message = error.clone();
                         return Ok(true);
                     }
                     // Additional validation for end date on submit
                     if !app.popup_event_end_date.trim().is_empty() {
                         if let Err(e) = date_utils::validate_date_input(&app.popup_event_end_date, app.current_date_for_new_event) {
                             app.error_message = e;
                             return Ok(true);
                         }
                     }
                     let time_str = app.popup_event_time.clone();
                     let normalized_time_str = normalize_time_input(&time_str);
                     let is_all_day = normalized_time_str.trim().is_empty();
                     if !is_all_day && NaiveTime::parse_from_str(&normalized_time_str, "%H:%M").is_err() {
                         app.error_message = "Invalid time format. Use HH:MM".to_string();
                         return Ok(true);
                     }
                     let time = if is_all_day {
                         NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                     } else {
                         NaiveTime::parse_from_str(&normalized_time_str, "%H:%M").unwrap()
                     };
                    let end_date_str = app.popup_event_end_date.drain(..).collect::<String>();
                    let end_date = if end_date_str.trim().is_empty() {
                        Some(app.current_date_for_new_event)
                    } else {
                        let parts: Vec<&str> = end_date_str.trim().split('/').collect();
                        if parts.len() == 2 {
                            if let (Ok(day), Ok(month)) =
                                (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                            {
                                let start_date = app.current_date_for_new_event;
                                let mut year = start_date.year();
                                if month < start_date.month()
                                    || (month == start_date.month() && day < start_date.day())
                                {
                                    year += 1;
                                }
                                NaiveDate::from_ymd_opt(year, month, day)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };
                    let end_time_str = app.popup_event_end_time.drain(..).collect::<String>();
                    let normalized_end_time_str = normalize_time_input(&end_time_str);
                    let end_time = if normalized_end_time_str.trim().is_empty() {
                        Some(time)
                    } else {
                        NaiveTime::parse_from_str(&normalized_end_time_str, "%H:%M").ok()
                    };
                    let title = app.popup_event_title.drain(..).collect();
                    let recurrence_str = app.popup_event_recurrence.drain(..).collect::<String>();
                    let recurrence = match recurrence_str.trim().to_lowercase().as_str() {
                        "daily" => crate::app::Recurrence::Daily,
                        "weekly" => crate::app::Recurrence::Weekly,
                        "monthly" => crate::app::Recurrence::Monthly,
                        "yearly" => crate::app::Recurrence::Yearly,
                        _ => crate::app::Recurrence::None,
                    };
                    let description = app.popup_event_description.drain(..).collect();
                    let mut event = CalendarEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        date: app.current_date_for_new_event,
                        time,
                        title,
                        description,
                        recurrence,
                        is_recurring_instance: false,
                        base_date: None,
                        start_date: app.current_date_for_new_event,
                        end_date,
                        start_time: time,
                        end_time,
                        is_all_day,
                    };

                    if app.is_editing {
                        if let Some(old_event) = &app.event_being_edited {
                            // Remove old event from main events list
                            app.events.retain(|e| e != old_event);
                            // Remove from persistence
                            let _ = persistence::delete_event_from_path_without_sync(
                                old_event,
                                &app.calendar_dir,
                            );

                            // Spawn async sync for delete
                            if let Some(provider) = &app.sync_provider {
                                if let Some(git_provider) = provider
                                    .as_any()
                                    .downcast_ref::<crate::sync::GitSyncProvider>(
                                ) {
                                    let remote_url = git_provider.remote_url.clone();
                                    let calendar_dir = app.calendar_dir.clone();
                                    thread::spawn(move || {
                                        let provider =
                                            crate::sync::GitSyncProvider::new(remote_url);
                                        let _ = provider.push(&calendar_dir);
                                    });
                                }
                            }
                        }
                    }

                     app.events.push(event.clone());
                     let _ =
                         persistence::save_event_to_path_without_sync(&mut event, &app.calendar_dir);

                     // Reset editing state
                     app.is_editing = false;
                     app.event_being_edited = None;

                     // If we came from the view events popup, refresh it and stay in that mode
                     if app.show_view_events_popup {
                         let all_events = app.get_all_events_for_range(app.date, app.date);
                          app.events_to_display_in_popup = all_events
                              .iter()
                              .filter(|event| {
                                  if let Some(end) = event.end_date {
                                      event.start_date <= app.date && end >= app.date
                                  } else {
                                      event.start_date == app.date
                                  }
                              })
                              .cloned()
                              .collect();
                         app.events_to_display_in_popup
                             .sort_by_key(|event| event.time);
                         app.selected_event_index = 0;
                         app.input_mode = InputMode::ViewEventsPopup;
                     } else {
                         app.input_mode = InputMode::Normal;
                     }
                     app.show_add_event_popup = false;

                     // Invalidate cached instances after event modification and UI refresh
                     app.invalidate_instance_cache(None);
                }
                  KeyCode::Char(c) => {
                      if app.selected_input_field == PopupInputField::Recurrence {
                          return Ok(true);
                      }
                      let cursor_pos = app.cursor_position;
                      let field = app.get_current_field_mut();
                      let byte_index = App::char_to_byte_index(field, cursor_pos);
                      field.insert(byte_index, c);
                      app.cursor_position += 1;

                      // Real-time validation for end date
                      if app.selected_input_field == PopupInputField::EndDate {
                          let start_date = app.current_date_for_new_event;
                          match date_utils::validate_date_input(&app.popup_event_end_date, start_date) {
                              Ok(_) => {
                                  app.date_input_error = None;
                                  app.date_suggestions = date_utils::get_date_suggestions(&app.popup_event_end_date, start_date);
                                  app.show_date_suggestions = !app.date_suggestions.is_empty();
                              }
                              Err(e) => {
                                  app.date_input_error = Some(e);
                                  app.date_suggestions.clear();
                                  app.show_date_suggestions = false;
                              }
                          }
                      }
                  }
                 KeyCode::Backspace => {
                     if app.cursor_position > 0 {
                         let cursor_pos = app.cursor_position - 1;
                         let field = app.get_current_field_mut();
                         let byte_index = App::char_to_byte_index(field, cursor_pos);
                         field.remove(byte_index);
                         app.cursor_position -= 1;

                         // Real-time validation for end date
                         if app.selected_input_field == PopupInputField::EndDate {
                             let start_date = app.current_date_for_new_event;
                             match date_utils::validate_date_input(&app.popup_event_end_date, start_date) {
                                 Ok(_) => {
                                     app.date_input_error = None;
                                     app.date_suggestions = date_utils::get_date_suggestions(&app.popup_event_end_date, start_date);
                                     app.show_date_suggestions = !app.date_suggestions.is_empty();
                                 }
                                 Err(e) => {
                                     app.date_input_error = Some(e);
                                     app.date_suggestions.clear();
                                     app.show_date_suggestions = false;
                                 }
                             }
                         }
                     }
                 }
                KeyCode::Esc => {
                    app.show_add_event_popup = false;
                    app.popup_event_title.clear();
                    app.popup_event_time.clear();
                    app.popup_event_end_date.clear();
                    app.popup_event_end_time.clear();
                    app.popup_event_description.clear();
                    app.popup_event_recurrence.clear();
                    app.input.clear();
                    app.is_editing = false;
                    app.event_being_edited = None;

                    // Return to view events popup if that's where we came from
                    if app.show_view_events_popup {
                        app.input_mode = InputMode::ViewEventsPopup;
                    } else {
                        app.input_mode = InputMode::Normal;
                    }
                }
                KeyCode::Left => {
                    if app.cursor_position > 0 {
                        app.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.cursor_position < app.get_current_field_char_count() {
                        app.cursor_position += 1;
                    }
                }
                 KeyCode::BackTab => {
                     app.selected_input_field = match app.selected_input_field {
                         PopupInputField::Title => {
                             app.cursor_position = app.popup_event_recurrence.chars().count();
                             PopupInputField::Recurrence
                         }
                         PopupInputField::Time => {
                             app.cursor_position = app.popup_event_title.chars().count();
                             PopupInputField::Title
                         }
                         PopupInputField::EndDate => {
                             app.cursor_position = app.popup_event_time.chars().count();
                             PopupInputField::Time
                         }
                          PopupInputField::EndTime => {
                              app.cursor_position = app.popup_event_end_date.chars().count();
                              // Clear suggestions when entering EndDate
                              app.date_input_error = None;
                              app.date_suggestions.clear();
                              app.show_date_suggestions = false;
                              PopupInputField::EndDate
                          }
                         PopupInputField::Description => {
                             app.cursor_position = app.popup_event_end_time.chars().count();
                             PopupInputField::EndTime
                         }
                         PopupInputField::Recurrence => {
                             app.cursor_position = app.popup_event_description.chars().count();
                             PopupInputField::Description
                          }
                     };
                     if app.selected_input_field == PopupInputField::Recurrence {
                         app.input_mode = InputMode::SelectingRecurrence;
                         app.selected_recurrence_index = recurrence_str_to_index(&app.popup_event_recurrence);
                     }
                 }

                 KeyCode::Tab => {
                     app.selected_input_field = match app.selected_input_field {
                         PopupInputField::Title => {
                             app.cursor_position = app.popup_event_time.chars().count();
                             PopupInputField::Time
                         }
                          PopupInputField::Time => {
                              app.cursor_position = app.popup_event_end_date.chars().count();
                              // Clear suggestions when entering EndDate
                              app.date_input_error = None;
                              app.date_suggestions.clear();
                              app.show_date_suggestions = false;
                              PopupInputField::EndDate
                          }
                          PopupInputField::EndDate => {
                              if app.show_date_suggestions && !app.date_suggestions.is_empty() {
                                  // Cycle through suggestions
                                  if app.date_suggestions.len() == 1 {
                                      app.popup_event_end_date = app.date_suggestions[0].clone();
                                      app.date_input_error = None;
                                      app.show_date_suggestions = false;
                                  } else {
                                      // For multiple suggestions, cycle through them
                                      // For simplicity, just pick the first one for now
                                      app.popup_event_end_date = app.date_suggestions[0].clone();
                                      app.date_input_error = None;
                                      app.show_date_suggestions = false;
                                  }
                                  app.cursor_position = app.popup_event_end_date.chars().count();
                                  PopupInputField::EndDate
                              } else {
                                  app.cursor_position = app.popup_event_end_time.chars().count();
                                  PopupInputField::EndTime
                              }
                          }
                         PopupInputField::EndTime => {
                             app.cursor_position = app.popup_event_description.chars().count();
                             PopupInputField::Description
                         }
                         PopupInputField::Description => {
                             app.cursor_position = app.popup_event_recurrence.chars().count();
                             PopupInputField::Recurrence
                         }
                         PopupInputField::Recurrence => {
                             app.cursor_position = app.popup_event_title.chars().count();
                             PopupInputField::Title
                         }
                     };
                     if app.selected_input_field == PopupInputField::Recurrence {
                         app.input_mode = InputMode::SelectingRecurrence;
                         app.selected_recurrence_index = recurrence_str_to_index(&app.popup_event_recurrence);
                     }
                 }
                 _ => {}
             },
             InputMode::SelectingRecurrence => match key.code {
                 KeyCode::Up | KeyCode::Char('k') => {
                     if app.selected_recurrence_index > 0 {
                         app.selected_recurrence_index -= 1;
                     }
                 }
                 KeyCode::Down | KeyCode::Char('j') => {
                     if app.selected_recurrence_index < 4 {
                         app.selected_recurrence_index += 1;
                     }
                 }
                 KeyCode::Enter => {
                     let recurrence_str = match app.selected_recurrence_index {
                         0 => "none",
                         1 => "daily",
                         2 => "weekly",
                         3 => "monthly",
                         4 => "yearly",
                         _ => "none",
                     };
                     app.popup_event_recurrence = recurrence_str.to_string();
                     app.input_mode = InputMode::EditingEventPopup;
                     app.selected_input_field = PopupInputField::Recurrence;
                 }
                 KeyCode::Esc => {
                     app.input_mode = InputMode::EditingEventPopup;
                     app.selected_input_field = PopupInputField::Recurrence;
                 }
                 _ => {}
             },
             InputMode::ViewEventsPopup => match key.code {
                KeyCode::Esc => {
                    app.show_view_events_popup = false;
                    app.events_to_display_in_popup.clear();
                    app.selected_event_index = 0;
                    app.event_to_delete_index = None;
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.events_to_display_in_popup.is_empty() {
                        return Ok(true);
                    }
                    if app.selected_event_index == 0 {
                        app.selected_event_index = app.events_to_display_in_popup.len() - 1;
                    } else {
                        app.selected_event_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.events_to_display_in_popup.is_empty() {
                        return Ok(true);
                    }
                    if app.selected_event_index == app.events_to_display_in_popup.len() - 1 {
                        app.selected_event_index = 0;
                    } else {
                        app.selected_event_index += 1;
                    }
                }
                KeyCode::Char('e') => {
                    if !app.events_to_display_in_popup.is_empty() {
                        let selected_event =
                            &app.events_to_display_in_popup[app.selected_event_index];
                        let base_event = if let Some(base) = find_base_event_for_instance(selected_event, &app.events) {
                            base
                        } else {
                            selected_event.clone()
                        };
                        app.popup_event_title = base_event.title.clone();
                        app.popup_event_time = base_event.time.format("%H:%M").to_string();
                        app.popup_event_end_date = base_event
                            .end_date
                            .map_or(String::new(), |d| d.format("%d/%m").to_string());
                        app.popup_event_end_time = base_event
                            .end_time
                            .map_or(String::new(), |t| t.format("%H:%M").to_string());
                        app.popup_event_recurrence = match base_event.recurrence {
                            crate::app::Recurrence::None => "none".to_string(),
                            crate::app::Recurrence::Daily => "daily".to_string(),
                            crate::app::Recurrence::Weekly => "weekly".to_string(),
                            crate::app::Recurrence::Monthly => "monthly".to_string(),
                            crate::app::Recurrence::Yearly => "yearly".to_string(),
                        };
                        app.popup_event_description = base_event.description.clone();
                        app.current_date_for_new_event = base_event.date;
                        app.is_editing = true;
                        app.event_being_edited = Some(base_event.clone());
                        app.show_add_event_popup = true;
                        app.input_mode = InputMode::EditingEventPopup;
                        app.selected_input_field = PopupInputField::Title;
                        app.cursor_position = app.popup_event_title.chars().count();
                    }
                }
                KeyCode::Char('a') => {
                    app.show_add_event_popup = true;
                    app.input_mode = InputMode::EditingEventPopup;
                    app.current_date_for_new_event = app.date;
                    app.popup_event_title.clear();
                    app.popup_event_time.clear();
                    app.popup_event_end_date.clear();
                    app.popup_event_end_time.clear();
                    app.popup_event_recurrence.clear();
                    app.popup_event_description.clear();
                    app.input.clear();
                    app.selected_input_field = PopupInputField::Title;
                    app.cursor_position = 0;
                    app.is_editing = false;
                    app.event_being_edited = None;
                    app.date_input_error = None;
                    app.date_suggestions.clear();
                    app.show_date_suggestions = false;
                }

                KeyCode::Char('d') | KeyCode::Delete => {
                    if !app.events_to_display_in_popup.is_empty() {
                        app.event_to_delete_index = Some(app.selected_event_index);
                        app.input_mode = InputMode::DeleteConfirmation;
                    }
                }
                _ => {}
            },
            InputMode::DeleteConfirmation => match key.code {
                KeyCode::Char('y') => {
                    if let Some(index) = app.event_to_delete_index {
                        if index < app.events_to_display_in_popup.len() {
                              let event_to_delete = app.events_to_display_in_popup[index].clone();
                              // Determine if we need to delete a recurring series
                              let base_to_delete = if event_to_delete.is_recurring_instance {
                                  find_base_event_for_instance(&event_to_delete, &app.events)
                              } else if event_to_delete.recurrence != Recurrence::None {
                                  Some(event_to_delete.clone())
                              } else {
                                  None
                              };

                              let deleted_title = if let Some(ref base) = base_to_delete {
                                  // Invalidate cached instances before deletion
                                  app.invalidate_instance_cache(Some(base));
                                  // Delete the entire recurring series: remove all events with matching title (base + instances) from memory
                                  // and delete only the base event file from persistence (instances are in-memory only)
                                  app.events.retain(|event| !(event.title == base.title && (event.is_recurring_instance || event == base)));
                                  // Remove base event from persistence
                                  let _ = persistence::delete_event_from_path_without_sync(
                                      base,
                                      &app.calendar_dir,
                                  );
                                  Some(base.title.clone())
                              } else {
                                  // Invalidate cached instances before deletion
                                  app.invalidate_instance_cache(Some(&event_to_delete.clone()));
                                  // Delete single non-recurring event
                                  app.events.retain(|event| event != &event_to_delete);
                                  // Remove from persistence only if not a recurring instance
                                  if !event_to_delete.is_recurring_instance {
                                      let _ = persistence::delete_event_from_path_without_sync(
                                          &event_to_delete,
                                          &app.calendar_dir,
                                      );
                                  }
                                  None
                              };

                            // Spawn async sync for delete
                            if let Some(provider) = &app.sync_provider {
                                if let Some(git_provider) = provider
                                    .as_any()
                                    .downcast_ref::<crate::sync::GitSyncProvider>(
                                ) {
                                    let remote_url = git_provider.remote_url.clone();
                                    let calendar_dir = app.calendar_dir.clone();
                                    thread::spawn(move || {
                                        let provider =
                                            crate::sync::GitSyncProvider::new(remote_url);
                                        let _ = provider.push(&calendar_dir);
                                    });
                                }
                            }
                            // Update display list - remove all matching events from popup
                            if let Some(title) = deleted_title {
                                app.events_to_display_in_popup.retain(|event| event.title != title);
                            } else {
                                app.events_to_display_in_popup.remove(index);
                            }
                            // Adjust selection if necessary
                            if app.selected_event_index >= app.events_to_display_in_popup.len() {
                                if app.events_to_display_in_popup.is_empty() {
                                    app.selected_event_index = 0;
                                } else {
                                    app.selected_event_index = app.events_to_display_in_popup.len() - 1;
                                }
                            }
                        }
                    }
                    app.event_to_delete_index = None;
                    app.input_mode = InputMode::ViewEventsPopup;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    app.event_to_delete_index = None;
                    app.input_mode = InputMode::ViewEventsPopup;
                }
                _ => {}
            },
            InputMode::Sync => match key.code {
                KeyCode::Char('f') => {
                    if let Some(provider) = &app.sync_provider {
                        let home = dirs::home_dir().expect("Could not find home directory");
                        let calendar_dir = home.join("calendar");
                        match provider.pull(&calendar_dir) {
                            Ok(status) => {
                                app.sync_message = "Pull successful".to_string();
                                app.sync_status = Some(status);
                                 // Reload events
                                 app.events = persistence::load_events().unwrap_or_else(|e| {
                                     eprintln!("Failed to reload events after pull: {e}");
                                     Vec::new()
                                 });
                                 // Invalidate cached instances after reloading events
                                 app.invalidate_instance_cache(None);
                            }
                            Err(e) => {
                                app.sync_message = format!("Pull failed: {e}");
                                app.sync_status =
                                    Some(crate::sync::SyncStatus::Error(e.to_string()));
                            }
                        }
                    }
                }
                KeyCode::Char('p') => {
                    if let Some(provider) = &app.sync_provider {
                        let home = dirs::home_dir().expect("Could not find home directory");
                        let calendar_dir = home.join("calendar");
                        match provider.push(&calendar_dir) {
                            Ok(status) => {
                                app.sync_message = "Push successful".to_string();
                                app.sync_status = Some(status);
                            }
                            Err(e) => {
                                app.sync_message = format!("Push failed: {e}");
                                app.sync_status =
                                    Some(crate::sync::SyncStatus::Error(e.to_string()));
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                    app.sync_message.clear();
                    app.sync_status = None;
                }
                _ => {}
            },
        }
    }
    Ok(true)
}
