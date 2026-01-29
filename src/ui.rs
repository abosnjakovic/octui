use crate::app::{App, AppState, Cursor};
use crate::contribution::{ContributionCalendar, ContributionDay, Week};
use chrono::{Datelike, NaiveDate};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

const DAY_LABELS: [&str; 7] = ["    ", "Mon ", "    ", "Wed ", "    ", "Fri ", "    "];
const DAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

pub fn render(frame: &mut Frame, app: &App) {
    match &app.state {
        AppState::Loaded(response) => {
            let user = &response.data.user;
            let calendar = &user.contributions_collection.contribution_calendar;
            render_graph(frame, &user.login, calendar, app.year, &app.cursor);
        }
        AppState::Error(msg) => render_error(frame, msg),
    }

    if app.show_help {
        render_help(frame);
    }
}

fn render_error(frame: &mut Frame, message: &str) {
    let paragraph = Paragraph::new(vec![
        Line::from("Error fetching contributions:").red(),
        Line::from(""),
        Line::from(message.to_string()),
        Line::from(""),
        Line::from("Press q to quit").dark_gray(),
    ])
    .centered()
    .block(Block::default().borders(Borders::ALL).title("Error"));
    frame.render_widget(paragraph, frame.area());
}

fn render_graph(
    frame: &mut Frame,
    username: &str,
    calendar: &ContributionCalendar,
    year: i32,
    cursor: &Cursor,
) {
    let [title_area, month_area, grid_area, status_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(7),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    render_title(
        frame,
        title_area,
        username,
        calendar.total_contributions,
        year,
    );
    render_month_labels(frame, month_area, &calendar.weeks);
    render_contribution_grid(frame, grid_area, &calendar.weeks, cursor);
    render_status(frame, status_area, &calendar.weeks, cursor);
}

fn render_title(frame: &mut Frame, area: Rect, username: &str, total: u32, year: i32) {
    let title = format!("{username} - {total} contributions in {year}");
    let paragraph = Paragraph::new(title)
        .centered()
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn render_month_labels(frame: &mut Frame, area: Rect, weeks: &[Week]) {
    let month_positions = calculate_month_positions(weeks);

    let mut spans = vec![Span::raw("     ")]; // Space for day labels (4 chars + 1)
    let mut current_col = 0;

    for (week_idx, month_name) in &month_positions {
        let padding = week_idx.saturating_sub(current_col);
        if padding > 0 {
            spans.push(Span::raw(" ".repeat(padding * 2)));
        }
        spans.push(Span::styled(
            format!("{:<4}", month_name),
            Style::default().fg(Color::Gray),
        ));
        current_col = week_idx + 2;
    }

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);
}

fn calculate_month_positions(weeks: &[Week]) -> Vec<(usize, &'static str)> {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut positions = Vec::new();
    let mut last_month: Option<u32> = None;

    for (week_idx, week) in weeks.iter().enumerate() {
        if let Some(first_day) = week.contribution_days.first()
            && let Ok(date) = NaiveDate::parse_from_str(&first_day.date, "%Y-%m-%d")
        {
            let month = date.month();
            if last_month != Some(month) {
                last_month = Some(month);
                positions.push((week_idx, months[(month - 1) as usize]));
            }
        }
    }

    positions
}

fn render_contribution_grid(frame: &mut Frame, area: Rect, weeks: &[Week], cursor: &Cursor) {
    let rows: Vec<Row> = (0..7)
        .map(|day_idx| {
            let label_cell =
                Cell::from(DAY_LABELS[day_idx]).style(Style::default().fg(Color::Gray));

            let day_cells: Vec<Cell> = weeks
                .iter()
                .enumerate()
                .map(|(week_idx, week)| {
                    let is_selected = week_idx == cursor.week && day_idx == cursor.day;

                    week.contribution_days
                        .get(day_idx)
                        .map(|day| {
                            let mut style = Style::default().bg(day.contribution_level.to_color());
                            if is_selected {
                                style = style.add_modifier(Modifier::REVERSED);
                            }
                            Cell::from("  ").style(style)
                        })
                        .unwrap_or_else(|| Cell::from("  "))
                })
                .collect();

            let mut cells = vec![label_cell];
            cells.extend(day_cells);
            Row::new(cells)
        })
        .collect();

    let mut widths = vec![Constraint::Length(4)];
    widths.extend(std::iter::repeat_n(Constraint::Length(2), weeks.len()));

    let table = Table::new(rows, widths).column_spacing(0);
    frame.render_widget(table, area);
}

fn render_status(frame: &mut Frame, area: Rect, weeks: &[Week], cursor: &Cursor) {
    let status = get_selected_day(weeks, cursor)
        .map(format_day_info)
        .unwrap_or_default();

    let paragraph = Paragraph::new(status).style(Style::default().fg(Color::Gray));
    frame.render_widget(paragraph, area);
}

fn get_selected_day<'a>(weeks: &'a [Week], cursor: &Cursor) -> Option<&'a ContributionDay> {
    weeks.get(cursor.week)?.contribution_days.get(cursor.day)
}

fn format_day_info(day: &ContributionDay) -> String {
    let date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").ok();
    let day_name = date
        .map(|d| DAY_NAMES[d.weekday().num_days_from_sunday() as usize])
        .unwrap_or("Unknown");

    let formatted_date = date
        .map(|d| d.format("%B %d, %Y").to_string())
        .unwrap_or_else(|| day.date.clone());

    let contribution_text = match day.contribution_count {
        0 => "No contributions".to_string(),
        1 => "1 contribution".to_string(),
        n => format!("{n} contributions"),
    };

    format!("{day_name}, {formatted_date} - {contribution_text}")
}

fn render_help(frame: &mut Frame) {
    let help_text = vec![
        Line::from("Keybindings".bold()),
        Line::from(""),
        Line::from(vec![
            Span::styled("  h/j/k/l ", Style::default().fg(Color::Cyan)),
            Span::raw("Navigate days"),
        ]),
        Line::from(vec![
            Span::styled("  p / n   ", Style::default().fg(Color::Cyan)),
            Span::raw("Previous / next year"),
        ]),
        Line::from(vec![
            Span::styled("  ?       ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle this help"),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
        Line::from(""),
        Line::from("Press any key to close".dark_gray()),
    ];

    let help_width = 34;
    let help_height = help_text.len() as u16 + 2;

    let [area] = Layout::horizontal([Constraint::Length(help_width)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Constraint::Length(help_height)])
        .flex(Flex::Center)
        .areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" Help ")),
        area,
    );
}
