mod app;
mod contribution;
mod event;
mod github;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io::{self, stdout};

#[derive(Parser)]
#[command(name = "octui")]
#[command(about = "GitHub contribution graph in your terminal")]
struct Cli {
    /// GitHub username to display (defaults to authenticated user)
    #[arg(short, long)]
    user: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Run application
    let result = run(&mut terminal, cli.user);

    // Restore terminal
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, username: Option<String>) -> Result<()> {
    let mut app = App::new(username);

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::handle_events(&mut app)? {
            break;
        }

        app.check_auto_refresh();
    }

    Ok(())
}
