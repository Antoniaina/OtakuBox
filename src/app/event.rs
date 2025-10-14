use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crate::app::state::{AppState, Screen};

pub fn handle_event(state: &mut AppState) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Home;
                    state.search_active = false;
                },
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Search;
                    state.search_active = true;
                },
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Player;
                    state.search_active = false;
                },
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.toggle_theme();
                }
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => state.quit(),
                
                KeyCode::Char(c) if key.modifiers.is_empty() => {
                    if state.current_screen == Screen::Search {
                        state.search_input.push(c);
                    }
                },
                KeyCode::Backspace => {
                    state.search_input.pop();
                }
                _ => {},
            }
        }
    }
    Ok(())
}