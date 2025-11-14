use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame
};

use crate::app::state::AppState;
use crate::ui;

pub fn draw_search(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(6),  
        ])
        .split(f.area());

    let search_prompt = if state.search_input.is_empty() {
        vec![
            Span::styled(">> ", Style::default().fg(Color::Yellow)),
            Span::styled("Type to search for anime...", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
        ]
    } else {
        vec![
            Span::styled(">> ", Style::default().fg(Color::Yellow)),
            Span::styled(&state.search_input, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("█", Style::default().fg(Color::Green).add_modifier(Modifier::SLOW_BLINK)),
        ]
    };

    let search_bar = Paragraph::new(Line::from(search_prompt))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if state.search_active { Color::Green } else { Color::Blue }))
                .title(Span::styled(
                    " Search Anime ",
                    Style::default()
                        .fg(if state.search_active { Color::Green } else { Color::Cyan })
                        .add_modifier(Modifier::BOLD)
                ))
        )
        .alignment(Alignment::Left);
    f.render_widget(search_bar, chunks[0]);

    let results: Vec<ListItem> = if state.search_results.is_empty() {
        vec![
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("  [*] ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        "No results yet. Start typing to search...",
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)
                    ),
                ])
            ])
        ]
    } else {
        state.search_results
            .iter()
            .enumerate()
            .map(|(idx, result)| {
                let icon = if idx % 2 == 0 { "[*] " } else { "-> " };
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(icon, Style::default().fg(Color::Cyan)),
                        Span::styled(
                            result,
                            Style::default().fg(Color::White)
                        ),
                    ])
                ])
            })
            .collect()
    };

    let results_list = List::new(results)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    format!(" Results ({}) ", state.search_results.len()),
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                ))
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(results_list, chunks[1]);

    ui::draw_navigation_footer(f, chunks[2], None);
}
