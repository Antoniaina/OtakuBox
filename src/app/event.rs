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
                    state.selected_result_index = None;
                },
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    state.current_screen = Screen::Player;
                    state.search_active = false;
                },
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => state.quit(),
                
                KeyCode::Char(c) if key.modifiers.is_empty() => {
                    if state.current_screen == Screen::Search {
                        state.search_input.push(c);
                        state.selected_result_index = None;
                        state.update_search_results();
                    }
                },
                KeyCode::Backspace => {
                    if state.current_screen == Screen::Search {
                        state.search_input.pop();
                        state.selected_result_index = None;
                        state.update_search_results();
                    }
                },
                KeyCode::Up => {
                    if state.current_screen == Screen::Search && !state.search_results.is_empty() {
                        match state.selected_result_index {
                            Some(idx) if idx > 0 => {
                                state.selected_result_index = Some(idx - 1);
                            },
                            Some(_) => {
                                state.selected_result_index = Some(state.search_results.len() - 1);
                            },
                            None => {
                                state.selected_result_index = Some(state.search_results.len() - 1);
                            }
                        }
                    }
                },
                KeyCode::Down => {
                    if state.current_screen == Screen::Search && !state.search_results.is_empty() {
                        match state.selected_result_index {
                            Some(idx) if idx < state.search_results.len() - 1 => {
                                state.selected_result_index = Some(idx + 1);
                            },
                            Some(_) => {
                                state.selected_result_index = Some(0);
                            },
                            None => {
                                state.selected_result_index = Some(0);
                            }
                        }
                    }
                },
                KeyCode::Enter => {
                    if state.current_screen == Screen::Search {
                        if let Some(idx) = state.selected_result_index {
                            if idx < state.search_results.len() {
                                let _selected = &state.search_results[idx];
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }
    Ok(())
}