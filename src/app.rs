use crate::contribution::GraphQLResponse;
use crate::github;
use chrono::{Datelike, NaiveDate, Utc};
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub enum AppState {
    Loaded(GraphQLResponse),
    Error(String),
}

pub struct Cursor {
    pub week: usize,
    pub day: usize,
}

pub struct App {
    pub state: AppState,
    pub should_quit: bool,
    pub show_help: bool,
    pub year: i32,
    pub cursor: Cursor,
    username: Option<String>,
    last_fetch: Instant,
}

impl App {
    pub fn new(username: Option<String>) -> Self {
        let year = Utc::now().year();
        let state = match github::fetch_contributions(username.as_deref(), year) {
            Ok(response) => AppState::Loaded(response),
            Err(e) => AppState::Error(e.to_string()),
        };

        let cursor = Self::cursor_for_today(&state, year);

        Self {
            state,
            should_quit: false,
            show_help: false,
            year,
            cursor,
            username,
            last_fetch: Instant::now(),
        }
    }

    fn cursor_for_today(state: &AppState, year: i32) -> Cursor {
        let target_date = Self::target_date_for_year(year);

        match state {
            AppState::Loaded(response) => {
                let weeks = &response.data.user.contributions_collection.contribution_calendar.weeks;

                // Search for the target date in the grid
                for (week_idx, week) in weeks.iter().enumerate() {
                    for (day_idx, day) in week.contribution_days.iter().enumerate() {
                        if day.date == target_date {
                            return Cursor { week: week_idx, day: day_idx };
                        }
                    }
                }

                // Fallback: last day with data
                let week = weeks.len().saturating_sub(1);
                let day = weeks.get(week).map(|w| w.contribution_days.len().saturating_sub(1)).unwrap_or(0);
                Cursor { week, day }
            }
            AppState::Error(_) => Cursor { week: 0, day: 0 },
        }
    }

    fn target_date_for_year(year: i32) -> String {
        let today = Utc::now().date_naive();
        let current_year = today.year();

        if year == current_year {
            // Current year: go to today
            today.format("%Y-%m-%d").to_string()
        } else {
            // Past year: go to same month/day, or Dec 31 if that date doesn't exist
            let target = NaiveDate::from_ymd_opt(year, today.month(), today.day())
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 12, 31).unwrap());
            target.format("%Y-%m-%d").to_string()
        }
    }

    pub fn previous_year(&mut self) {
        self.year -= 1;
        self.refresh();
    }

    pub fn next_year(&mut self) {
        let current_year = Utc::now().year();
        if self.year < current_year {
            self.year += 1;
            self.refresh();
        }
    }

    fn refresh(&mut self) {
        self.state = match github::fetch_contributions(self.username.as_deref(), self.year) {
            Ok(response) => AppState::Loaded(response),
            Err(e) => AppState::Error(e.to_string()),
        };
        self.cursor = Self::cursor_for_today(&self.state, self.year);
        self.last_fetch = Instant::now();
    }

    pub fn check_auto_refresh(&mut self) {
        if self.last_fetch.elapsed() >= REFRESH_INTERVAL {
            if self.year == Utc::now().year() {
                self.refresh();
            } else {
                self.last_fetch = Instant::now();
            }
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    fn weeks_count(&self) -> usize {
        match &self.state {
            AppState::Loaded(r) => r.data.user.contributions_collection.contribution_calendar.weeks.len(),
            AppState::Error(_) => 0,
        }
    }

    fn days_in_week(&self, week: usize) -> usize {
        match &self.state {
            AppState::Loaded(r) => {
                r.data.user.contributions_collection.contribution_calendar.weeks
                    .get(week)
                    .map(|w| w.contribution_days.len())
                    .unwrap_or(0)
            }
            AppState::Error(_) => 0,
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor.week > 0 {
            self.cursor.week -= 1;
            let max_day = self.days_in_week(self.cursor.week).saturating_sub(1);
            self.cursor.day = self.cursor.day.min(max_day);
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.week + 1 < self.weeks_count() {
            self.cursor.week += 1;
            let max_day = self.days_in_week(self.cursor.week).saturating_sub(1);
            self.cursor.day = self.cursor.day.min(max_day);
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.day > 0 {
            self.cursor.day -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let max_day = self.days_in_week(self.cursor.week).saturating_sub(1);
        if self.cursor.day < max_day {
            self.cursor.day += 1;
        }
    }
}
