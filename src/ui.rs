use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, InputMode, PopupInputField};

const MAX_OVERLAY_HEIGHT: u16 = 5;
const MIN_OVERLAY_WIDTH: u16 = 10;

// Change ID: add-suggestions-overlay
// Renders a suggestions overlay for date suggestions in the add event popup.
// Positions the overlay to the right of the end date field, with boundary handling.
// See openspec/changes/add-suggestions-overlay/design.md for details.
fn render_suggestions_overlay(f: &mut Frame, app: &App, input_chunks: &[Rect]) {
    if !app.show_date_suggestions || app.date_suggestions.is_empty() {
        return;
    }

    let overlay_width = input_chunks[2].width.max(MIN_OVERLAY_WIDTH);
    let overlay_height = (app
        .date_suggestions
        .iter()
        .filter(|(s, _)| !s.is_empty())
        .count() as u16)
        .min(MAX_OVERLAY_HEIGHT);
    let overlay_x = input_chunks[2].x;
    let mut overlay_y = input_chunks[2].y + input_chunks[2].height;

    // Handle boundary constraints
    let frame_size = f.size();
    let space_below = frame_size.height.saturating_sub(overlay_y);
    let space_above = input_chunks[2].y;

    // Prefer below if it has space for at least 3 suggestions, otherwise try above
    if space_below < 3 && space_above >= 3 {
        overlay_y = input_chunks[2].y.saturating_sub(overlay_height);
    } else if space_below < overlay_height {
        // If below doesn't have full height, try above
        overlay_y = input_chunks[2].y.saturating_sub(overlay_height);
        if overlay_y == 0 || overlay_height > space_above {
            // If above doesn't fit, go back to below and reduce height
            overlay_y = input_chunks[2].y + input_chunks[2].height;
        }
    }

    // Ensure height fits
    let adjusted_height = frame_size.height.saturating_sub(overlay_y);
    let overlay_height = overlay_height.min(adjusted_height);
    if overlay_height < 3 {
        return; // Skip if can't show at least 3 suggestions
    }
    if overlay_height == 0 {
        return; // Skip if no height
    }

    let overlay_area = Rect::new(overlay_x, overlay_y, overlay_width, overlay_height);

    let overlay_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(Clear, overlay_area);
    f.render_widget(&overlay_block, overlay_area);

    let inner_area = overlay_block.inner(overlay_area);
    let suggestions_list: Vec<ListItem> = app
        .date_suggestions
        .iter()
        .enumerate()
        .filter(|(_, (s, _))| !s.is_empty())
        .map(|(i, (s, is_valid))| {
            let mut style = if i == app.selected_suggestion_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            if !is_valid {
                style = style.fg(Color::Gray);
            }
            ListItem::new(s.as_str()).style(style)
        })
        .collect();
    let suggestions_list_widget = List::new(suggestions_list);
    f.render_widget(suggestions_list_widget, inner_area);
}

fn build_calendar_table(
    year: i32,
    month: u32,
    events: &[crate::app::CalendarEvent],
    selected_date: NaiveDate,
) -> (Table<'static>, usize) {
    let today = Local::now().date_naive();
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let weekday_of_first = first_day_of_month.weekday().num_days_from_monday();

    let mut calendar_days = vec![];
    for _ in 0..weekday_of_first {
        calendar_days.push("".to_string());
    }

    let days_in_month = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .signed_duration_since(first_day_of_month)
        .num_days();

    for day in 1..=days_in_month {
        calendar_days.push(day.to_string());
    }

    let mut rows = vec![];
    let mut week_cells = vec![];

    for (i, day_str) in calendar_days.iter().enumerate() {
        let mut cell = Cell::from(day_str.clone());
        let mut current_date_for_week_num = first_day_of_month;
        let mut final_style = Style::default();

        if !day_str.is_empty() {
            let day = day_str.parse::<u32>().unwrap();
            let current_day_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            current_date_for_week_num = current_day_date;

            let mut day_display_str = day_str.clone();
            let has_event = events.iter().any(|event| {
                event.start_date <= current_day_date
                    && event.end_date.is_none_or(|end| end >= current_day_date)
            });
            let symbol = if has_event { "*" } else { "" };
            day_display_str.push_str(symbol);

            // Apply Saturday/Sunday colors
            if current_day_date.weekday().num_days_from_monday() == 5 {
                final_style = final_style.fg(Color::LightYellow);
            } else if current_day_date.weekday().num_days_from_monday() == 6 {
                final_style = final_style.fg(Color::Red);
            }

            // Apply current day background if it's today
            if current_day_date == today {
                final_style = final_style.bg(Color::DarkGray);
            }

            // Apply selected day foreground and bold if it's the selected date
            if current_day_date == selected_date {
                final_style = final_style.fg(Color::Black).bg(Color::LightBlue);
            }
            cell = Cell::from(day_display_str).style(final_style);
        }
        week_cells.push(cell);

        if (i + 1) % 7 == 0 || i == calendar_days.len() - 1 {
            let week_num = current_date_for_week_num.iso_week().week();
            let row_cells =
                vec![Cell::from(week_num.to_string()).style(Style::default().fg(Color::Magenta))];
            let mut full_row_cells = row_cells;
            full_row_cells.append(&mut week_cells);
            rows.push(Row::new(full_row_cells));
            week_cells.clear();
        }
    }

    let mut all_rows = vec![Row::new(vec![
        Cell::from(""),
        Cell::from("Mo"),
        Cell::from("Tu"),
        Cell::from("We"),
        Cell::from("Th"),
        Cell::from("Fr"),
        Cell::from("Sa").style(Style::default().fg(Color::LightYellow)),
        Cell::from("Su").style(Style::default().fg(Color::Red)),
    ])];
    all_rows.append(&mut rows);

    let height = all_rows.len();

    let calendar = Table::new(
        all_rows,
        &[
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ],
    )
    .style(Style::default().fg(Color::White))
    .column_spacing(1);

    (calendar, height)
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let calendar_chunk = chunks[0];
    let hints_chunk = chunks[1];

    let calendar_block = Block::default()
        .title("RCal")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    let calendar_area = calendar_block.inner(calendar_chunk);
    f.render_widget(calendar_block, calendar_chunk);

    // Calculate visible date range
    let mut overall_start = NaiveDate::MAX;
    let mut overall_end = NaiveDate::MIN;
    for i in 0..3 {
        let month_offset = i as u32;
        let (year, month) = if app.view_start_month + month_offset > 12 {
            (
                app.view_start_year + 1,
                app.view_start_month + month_offset - 12,
            )
        } else {
            (app.view_start_year, app.view_start_month + month_offset)
        };
        let start_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let end_of_month = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap(); // last day of month
        if start_of_month < overall_start {
            overall_start = start_of_month;
        }
        if end_of_month > overall_end {
            overall_end = end_of_month;
        }
    }
    let all_events = app.get_all_events_for_range(overall_start, overall_end);

    let mut calendars = vec![];
    let mut constraints = vec![];

    for i in 0..3 {
        let month_offset = i as u32;
        let (year, month) = if app.view_start_month + month_offset > 12 {
            (
                app.view_start_year + 1,
                app.view_start_month + month_offset - 12,
            )
        } else {
            (app.view_start_year, app.view_start_month + month_offset)
        };
        let month_name = match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        };
        constraints.push(Constraint::Length(1)); // title
        let (calendar, height) = build_calendar_table(year, month, &all_events, app.date);
        constraints.push(Constraint::Length(height as u16)); // table
        if i < 2 {
            constraints.push(Constraint::Length(1)); // spacing
        }
        calendars.push((month_name, year, calendar));
    }

    let month_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(calendar_area);

    for (i, (month_name, year, calendar)) in calendars.into_iter().enumerate() {
        let title =
            Paragraph::new(format!("{month_name} {year}")).style(Style::default().fg(Color::Cyan));
        f.render_widget(title, month_chunks[i * 3]);
        f.render_widget(calendar, month_chunks[i * 3 + 1]);
    }

    // Render main hints
    let main_hints = Paragraph::new("q: quit, a: add, o: view, s: sync, h/j/k/l: navigate")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(main_hints, hints_chunk);

    if app.show_view_events_popup {
        let popup_block = Block::default()
            .title(format!("Events on {}", app.date.format("%Y-%m-%d")))
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightCyan));

        let popup_events: Vec<ListItem> = app
            .events_to_display_in_popup
            .iter()
            .enumerate()
            .map(|(index, event)| {
                let recurring_indicator = if event.recurrence != crate::app::Recurrence::None
                    || event.is_recurring_instance
                {
                    " (R)"
                } else {
                    ""
                };
                let time_str = if event.is_all_day {
                    "All day".to_string()
                } else {
                    event.time.format("%H:%M").to_string()
                };
                let content = if event.description.is_empty() {
                    format!("{} - {}{}", time_str, event.title, recurring_indicator)
                } else {
                    format!(
                        "{} - {}: {}{}",
                        time_str, event.title, event.description, recurring_indicator
                    )
                };

                if index == app.selected_event_index {
                    ListItem::new(content)
                        .style(Style::default().fg(Color::Black).bg(Color::LightBlue))
                } else {
                    ListItem::new(content)
                }
            })
            .collect();

        let popup_list = List::new(popup_events)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightBlue));

        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(f.size())[1];

        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area)[1];

        let inner_area = popup_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(inner_area);

        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);
        f.render_widget(popup_list, chunks[0]);

        // Render hints
        let hints = Paragraph::new("j/k: navigate, e: edit, a: add, d: delete, Esc: close")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, chunks[1]);
    }

    if app.input_mode == InputMode::DeleteConfirmation {
        if let Some(index) = app.event_to_delete_index {
            if index < app.events_to_display_in_popup.len() {
                let event = &app.events_to_display_in_popup[index];

                let confirm_block = Block::default()
                    .title("Confirm Delete")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightRed));

                let time_str = if event.is_all_day {
                    "All day".to_string()
                } else {
                    event.time.format("%H:%M").to_string()
                };
                let confirmation_text = format!(
                    "Delete event:\n\n  {}\n  {}\n\nPress 'y' to confirm, 'n' to cancel",
                    event.title, time_str
                );

                let area = {
                    let size = f.size();
                    let popup_width = 50;
                    let popup_height = 8;
                    Rect::new(
                        (size.width - popup_width) / 2,
                        (size.height - popup_height) / 2,
                        popup_width,
                        popup_height,
                    )
                };

                let inner_area = confirm_block.inner(area);
                f.render_widget(Clear, area);
                f.render_widget(confirm_block, area);

                let text = ratatui::widgets::Paragraph::new(confirmation_text)
                    .style(Style::default().fg(Color::White))
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(text, inner_area);
            }
        }
    }

    if app.show_add_event_popup {
        let title = if app.is_editing {
            format!(
                "Edit Event for {}",
                app.current_date_for_new_event.format("%Y-%m-%d")
            )
        } else {
            format!(
                "Add Event for {}",
                app.current_date_for_new_event.format("%Y-%m-%d")
            )
        };
        let popup_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightCyan));

        let area = {
            let size = f.size();
            let popup_width = 70.min(size.width.saturating_sub(2));
            let popup_height = 25.min(size.height.saturating_sub(2));
            Rect::new(
                (size.width - popup_width) / 2,
                (size.height - popup_height) / 2,
                popup_width,
                popup_height,
            )
        };

        let inner_area = popup_block.inner(area);
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let (input_area, error_area, hints_area) = if app.error_message.is_empty() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
                .split(inner_area);
            (chunks[0], None, chunks[1])
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(inner_area);
            (chunks[0], Some(chunks[1]), chunks[2])
        };

        let input_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(7),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(input_area);

        let title_style = if app.selected_input_field == PopupInputField::Title {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };
        let time_style = if app.selected_input_field == PopupInputField::Time {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };
        let end_date_style = if app.selected_input_field == PopupInputField::EndDate {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };
        let end_time_style = if app.selected_input_field == PopupInputField::EndTime {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };
        let recurrence_style = if app.selected_input_field == PopupInputField::Recurrence {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };
        let description_style = if app.selected_input_field == PopupInputField::Description {
            Style::default().fg(Color::Black).bg(Color::LightBlue)
        } else {
            Style::default()
        };

        let title_input = ratatui::widgets::Paragraph::new(app.popup_event_title.as_str())
            .style(title_style)
            .block(Block::default().borders(Borders::ALL).title("Title"));
        f.render_widget(title_input, input_chunks[0]);

        let time_input = ratatui::widgets::Paragraph::new(app.popup_event_time.as_str())
            .style(time_style)
            .block(Block::default().borders(Borders::ALL).title("Time"));
        f.render_widget(time_input, input_chunks[1]);

        let time_block = if app.time_input_error.is_some() {
            Block::default()
                .borders(Borders::ALL)
                .title("Time")
                .border_style(Style::default().fg(Color::Red))
        } else {
            Block::default().borders(Borders::ALL).title("Time")
        };
        let time_text = if let Some(ref error) = app.time_input_error {
            format!("{}\n{}", app.popup_event_time, error)
        } else {
            app.popup_event_time.clone()
        };
        let time_input = ratatui::widgets::Paragraph::new(time_text)
            .style(time_style)
            .block(time_block);
        f.render_widget(time_input, input_chunks[1]);

        let end_date_block = if app.date_input_error.is_some() {
            Block::default()
                .borders(Borders::ALL)
                .title("End Date")
                .border_style(Style::default().fg(Color::Red))
        } else {
            Block::default().borders(Borders::ALL).title("End Date")
        };
        let end_date_text = if let Some(ref error) = app.date_input_error {
            format!("{}\n{}", app.popup_event_end_date, error)
        } else {
            app.popup_event_end_date.clone()
        };
        let end_date_input = ratatui::widgets::Paragraph::new(end_date_text)
            .style(end_date_style)
            .block(end_date_block);
        f.render_widget(end_date_input, input_chunks[2]);

        let end_time_input = ratatui::widgets::Paragraph::new(app.popup_event_end_time.as_str())
            .style(end_time_style)
            .block(Block::default().borders(Borders::ALL).title("End Time"));
        f.render_widget(end_time_input, input_chunks[3]);

        let end_time_block = if app.end_time_input_error.is_some() {
            Block::default()
                .borders(Borders::ALL)
                .title("End Time")
                .border_style(Style::default().fg(Color::Red))
        } else {
            Block::default().borders(Borders::ALL).title("End Time")
        };
        let end_time_text = if let Some(ref error) = app.end_time_input_error {
            format!("{}\n{}", app.popup_event_end_time, error)
        } else {
            app.popup_event_end_time.clone()
        };
        let end_time_input = ratatui::widgets::Paragraph::new(end_time_text)
            .style(end_time_style)
            .block(end_time_block);
        f.render_widget(end_time_input, input_chunks[3]);

        let description_input =
            ratatui::widgets::Paragraph::new(app.popup_event_description.as_str())
                .style(description_style)
                .block(Block::default().borders(Borders::ALL).title("Description"));
        f.render_widget(description_input, input_chunks[4]);

        let recurrence_input =
            ratatui::widgets::Paragraph::new(app.popup_event_recurrence.as_str())
                .style(recurrence_style)
                .block(Block::default().borders(Borders::ALL).title("Recurrence"));
        f.render_widget(recurrence_input, input_chunks[5]);

        if app.input_mode == InputMode::EditingEventPopup {
            match app.selected_input_field {
                PopupInputField::Title => {
                    f.set_cursor(
                        input_chunks[0].x + app.cursor_position as u16 + 1,
                        input_chunks[0].y + 1,
                    );
                }
                PopupInputField::Time => {
                    f.set_cursor(
                        input_chunks[1].x + app.cursor_position as u16 + 1,
                        input_chunks[1].y + 1,
                    );
                }
                PopupInputField::EndDate => {
                    f.set_cursor(
                        input_chunks[2].x + app.cursor_position as u16 + 1,
                        input_chunks[2].y + 1,
                    );
                }
                PopupInputField::EndTime => {
                    f.set_cursor(
                        input_chunks[3].x + app.cursor_position as u16 + 1,
                        input_chunks[3].y + 1,
                    );
                }
                PopupInputField::Description => {
                    f.set_cursor(
                        input_chunks[4].x + app.cursor_position as u16 + 1,
                        input_chunks[4].y + 1,
                    );
                }
                PopupInputField::Recurrence => {
                    // No cursor for recurrence field as input is disabled
                }
            }
        }

        // Render error message if present
        if let Some(error_area) = error_area {
            let error =
                Paragraph::new(app.error_message.as_str()).style(Style::default().fg(Color::Red));
            f.render_widget(error, error_area);
        }

        // Render hints
        let hints_text = if app.input_mode == InputMode::SelectingRecurrence {
            "j/k: navigate, Enter: select, Esc: cancel"
        } else {
            "Tab/Shift+Tab: switch field, Enter: save, Esc: cancel"
        };
        let hints = Paragraph::new(hints_text).style(Style::default().fg(Color::Gray));
        f.render_widget(hints, hints_area);

        // Render suggestions overlay if active
        render_suggestions_overlay(f, app, &input_chunks);
    }

    if app.input_mode == InputMode::SelectingRecurrence {
        let popup_block = Block::default()
            .title("Select Recurrence")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightCyan));

        let area = {
            let size = f.size();
            let popup_width = 20;
            let popup_height = 7;
            Rect::new(
                (size.width - popup_width) / 2,
                (size.height - popup_height) / 2,
                popup_width,
                popup_height,
            )
        };

        let inner_area = popup_block.inner(area);
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let recurrence_options: Vec<ListItem> = ["none", "daily", "weekly", "monthly", "yearly"]
            .iter()
            .enumerate()
            .map(|(i, &opt)| {
                if i == app.selected_recurrence_index {
                    ListItem::new(opt).style(Style::default().fg(Color::Black).bg(Color::LightBlue))
                } else {
                    ListItem::new(opt)
                }
            })
            .collect();

        let recurrence_list = List::new(recurrence_options);
        f.render_widget(recurrence_list, inner_area);
    }

    if app.input_mode == InputMode::Sync {
        let popup_block = Block::default()
            .title("Sync")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightGreen));

        let area = {
            let size = f.size();
            let popup_width = 50.min(size.width.saturating_sub(2));
            let popup_height = 10.min(size.height.saturating_sub(2));
            Rect::new(
                (size.width - popup_width) / 2,
                (size.height - popup_height) / 2,
                popup_width,
                popup_height,
            )
        };

        let inner_area = popup_block.inner(area);
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(inner_area);

        let instructions = List::new(vec![
            ListItem::new("f: Pull from remote"),
            ListItem::new("p: Push to remote"),
        ]);
        f.render_widget(instructions, chunks[0]);

        if !app.sync_message.is_empty() {
            let message = List::new(vec![ListItem::new(app.sync_message.as_str())]);
            f.render_widget(message, chunks[1]);
        }

        let status_text = match app.sync_status {
            Some(crate::sync::SyncStatus::UpToDate) => "Status: Up to date",
            Some(crate::sync::SyncStatus::Ahead) => "Status: Ahead",
            Some(crate::sync::SyncStatus::Behind) => "Status: Behind",
            Some(crate::sync::SyncStatus::Conflicts) => "Status: Conflicts",
            Some(crate::sync::SyncStatus::Error(_)) => "Status: Error",
            None => "Status: Unknown",
        };
        let status = List::new(vec![ListItem::new(status_text)]);
        f.render_widget(status, chunks[2]);
    }
}
