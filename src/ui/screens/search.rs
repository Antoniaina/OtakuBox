use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::app::state::{AppState, Theme};

pub fn draw_search(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let (fg, border_color, accent) = match state.theme {
        Theme::Dark => (Color::White, Color::Gray, Color::Cyan),
        Theme::Light => (Color::Black, Color::Gray, Color::Blue)
    };

    let search_bar = Paragraph::new(state.search_input.clone())
        .style(Style::default().fg(accent))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title("Search")
        );
    f.render_widget(search_bar, chunks[0]);

    let results_text = if state.search_results.is_empty() {
        "No result found".to_string()
    } else{
        state.search_results.join("\n")
    };

    let results = Paragraph::new(results_text)
        .style(Style::default().fg(fg))
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(border_color))
            .title("Results"));
    f.render_widget(results, chunks[1]);
}
