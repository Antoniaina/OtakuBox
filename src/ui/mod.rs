pub mod screens;

use ratatui::Frame;

use crate::app::state::{AppState, Screen};

pub fn draw(f: &mut Frame, state: &AppState) {
    match state.current_screen {
        Screen::Home => screens::home::draw_home(f),
        Screen::Search => screens::search::draw_search(f, state),
        Screen::Player => {} //screens::player::draw_player(f),
    }
    
}