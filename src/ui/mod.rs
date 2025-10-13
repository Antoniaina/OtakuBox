pub mod screens;

use ratatui::Frame;

pub fn draw(f: &mut Frame) {
    self::screens::home::draw_home(f);
}