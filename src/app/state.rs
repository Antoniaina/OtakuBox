#[derive(Default)]
pub struct AppState {
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self { running: true}
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
