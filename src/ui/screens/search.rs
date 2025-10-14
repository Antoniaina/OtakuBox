use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::app::state::AppState;

pub fn draw_search(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    let search_bar = Paragraph::new(state.search_input.clone())
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search")
        );
    f.render_widget(search_bar, chunks[0]);

    let results_text = state.search_results.join("\n");
    let results = Paragraph::new(results_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Results"));
    f.render_widget(results, chunks[1]);
}
