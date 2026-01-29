use crate::app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
    {
        return Ok(handle_key(app, key));
    }
    Ok(app.should_quit)
}

fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    if app.show_help {
        app.show_help = false;
        return false;
    }

    match key.code {
        KeyCode::Char('q') => true,
        KeyCode::Esc => true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
        KeyCode::Char('?') => {
            app.toggle_help();
            false
        }
        KeyCode::Char('p') => {
            app.previous_year();
            false
        }
        KeyCode::Char('n') => {
            app.next_year();
            false
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.move_left();
            false
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.move_right();
            false
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_up();
            false
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_down();
            false
        }
        _ => app.should_quit,
    }
}
