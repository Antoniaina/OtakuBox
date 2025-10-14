use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crate::app::state::{AppState, Screen};

pub fn handle_event(state: &mut AppState) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Home;
                },
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Search;
                    state.search_active = true;
                },
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Player;
                },
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => state.quit(),
                _ => {},
            }
        }
    }
    Ok(())
}