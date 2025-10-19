use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Row, Table},
};

use crate::app::{App, InputMode, PopupInputField};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let calendar_chunk = chunks[0];

    let today = Local::now().date_naive();
    let year = app.date.year();
    let month = app.date.month();

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
        let mut current_date_for_week_num = app.date;
        let mut final_style = Style::default(); // Start with a default style

        if !day_str.is_empty() {
            let day = day_str.parse::<u32>().unwrap();
            let current_day_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            current_date_for_week_num = current_day_date;

            let mut day_display_str = day_str.clone();
            let has_event = app
                .events
                .iter()
                .any(|event| event.date == current_day_date);
            let symbol = if has_event { "*" } else { "" };
            day_display_str.push_str(symbol);

            // Apply Saturday/Sunday colors
            if current_day_date.weekday().num_days_from_monday() == 5 {
                // Saturday
                final_style = final_style.fg(Color::LightYellow);
            } else if current_day_date.weekday().num_days_from_monday() == 6 {
                // Sunday
                final_style = final_style.fg(Color::Red);
            }

            // Apply current day background if it's today
            if current_day_date == today {
                final_style = final_style.bg(Color::DarkGray);
            }

            // Apply selected day foreground and bold if it's the selected date
            if current_day_date == app.date {
                final_style = final_style.fg(Color::Black).bg(Color::LightBlue);
            }
            cell = Cell::from(day_display_str).style(final_style);
        }
        week_cells.push(cell);

        if (i + 1) % 7 == 0 || i == calendar_days.len() - 1 {
            let week_num = current_date_for_week_num.iso_week().week();
            let mut row_cells =
                vec![Cell::from(week_num.to_string()).style(Style::default().fg(Color::Magenta))];
            row_cells.append(&mut week_cells);
            rows.push(Row::new(row_cells));
            week_cells.clear();
        }
    }

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

    let calendar = Table::new(
        rows,
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
    .header(Row::new(vec![
        Cell::from(""),
        Cell::from("Mo"),
        Cell::from("Tu"),
        Cell::from("We"),
        Cell::from("Th"),
        Cell::from("Fr"),
        Cell::from("Sa").style(Style::default().fg(Color::LightYellow)),
        Cell::from("Su").style(Style::default().fg(Color::Red)),
    ]))
    .block(
        Block::default()
            .title(format!("{month_name} {year}"))
            .borders(Borders::ALL),
    )
    .style(Style::default().fg(Color::White))
    .column_spacing(1);

    f.render_widget(calendar, calendar_chunk);

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
                let content = if event.description.is_empty() {
                    format!(
                        "{} - {}{}",
                        event.time.format("%H:%M"),
                        event.title,
                        recurring_indicator
                    )
                } else {
                    format!(
                        "{} - {}: {}{}",
                        event.time.format("%H:%M"),
                        event.title,
                        event.description,
                        recurring_indicator
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

        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);
        f.render_widget(popup_list, inner_area);
    }

    if app.input_mode == InputMode::DeleteConfirmation {
        if let Some(index) = app.event_to_delete_index {
            if index < app.events_to_display_in_popup.len() {
                let event = &app.events_to_display_in_popup[index];

                let confirm_block = Block::default()
                    .title("Confirm Delete")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::LightRed));

                let confirmation_text = format!(
                    "Delete event:\n\n  {}\n  {}\n\nPress 'y' to confirm, 'n' to cancel",
                    event.title,
                    event.time.format("%H:%M")
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
            let popup_height = 17.min(size.height.saturating_sub(2));
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

        let input_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ]
                .as_ref(),
            )
            .split(inner_area);

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

        let description_input =
            ratatui::widgets::Paragraph::new(app.popup_event_description.as_str())
                .style(description_style)
                .block(Block::default().borders(Borders::ALL).title("Description"));
        f.render_widget(description_input, input_chunks[2]);

        let recurrence_input =
            ratatui::widgets::Paragraph::new(app.popup_event_recurrence.as_str())
                .style(recurrence_style)
                .block(Block::default().borders(Borders::ALL).title("Recurrence"));
        f.render_widget(recurrence_input, input_chunks[3]);

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
            PopupInputField::Description => {
                f.set_cursor(
                    input_chunks[2].x + app.cursor_position as u16 + 1,
                    input_chunks[2].y + 1,
                );
            }
            PopupInputField::Recurrence => {
                f.set_cursor(
                    input_chunks[3].x + app.cursor_position as u16 + 1,
                    input_chunks[3].y + 1,
                );
            }
        }
    }
}
