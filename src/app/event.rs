use crossterm::event::{self, Event, KeyCode};
use crate::app::state::AppState;

pub fn handle_event(state: &mut AppState) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => state.quit(),
                _ => {},
            }
        }
    }
    Ok(())
}