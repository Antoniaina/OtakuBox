pub mod event;
pub mod state;

use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::ui;
use event::handle_event;
use state::AppState;

pub fn run () -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    
    let mut state = AppState::new();

    while state.running {
        terminal.draw(|f| ui::draw(f))?;
        handle_event(&mut state)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}