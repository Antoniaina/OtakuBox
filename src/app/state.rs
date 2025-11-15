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
    pub search_results: Vec<String>,
    pub selected_result_index: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        Self { 
            running: true,
            current_screen: Screen::Home,
            search_active: false,
            search_input: String::new(),
            search_results: vec![],
            selected_result_index: None,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn update_search_results(&mut self) {
        if self.search_input.is_empty() {
            self.search_results = vec![];
            return;
        }

        let mock_results = vec![
            format!("{} - Episode 1", self.search_input),
            format!("{} - Episode 2", self.search_input),
            format!("{} - Episode 3", self.search_input),
            format!("{} - Episode 4", self.search_input),
            format!("{} - Episode 5", self.search_input),
            format!("{} - Episode 6", self.search_input),
            format!("{} - Episode 7", self.search_input),
            format!("{} - Episode 8", self.search_input),
            format!("{} - Episode 9", self.search_input),
            format!("{} - Episode 10", self.search_input),
        ];

        self.search_results = mock_results;
    }
}
