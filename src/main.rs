mod ui;

use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
fn main () -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    
    loop {
        terminal.draw(|f| {
            ui::screens::home::draw_home(f);
        })?;

        if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(event) = crossterm::event::read() {
                if matches!(event, crossterm::event::Event::Key(crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char('q'), ..
                })) {
                    break; 
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}