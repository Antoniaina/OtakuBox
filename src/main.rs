mod app;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}