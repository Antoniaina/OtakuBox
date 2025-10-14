use std::clone;

#[derive(PartialEq)]
pub enum Screen {
    Home,
    Search,
    Player
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}
pub struct AppState {
    pub running: bool,
    pub current_screen: Screen,
    pub search_active: bool,
    pub search_input: String,
    pub search_results: Vec<String>,
    pub theme: Theme
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            running: true,
            current_screen: Screen::Home,
            search_active: false,
            search_input: String::new(),
            search_results: vec![],
            theme: Theme::Dark        
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
    }
}
