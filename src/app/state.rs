#[derive(PartialEq)]
pub enum Screen {
    Home,
    Search,
    Player
}
pub struct AppState {
    pub running: bool,
    pub current_screen: Screen,
    pub search_active: bool,
    pub search_input: String,
    pub search_results: Vec<String>
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            running: true,
            current_screen: Screen::Home,
            search_active: false,
            search_input: String::new(),
            search_results: vec![],        
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
