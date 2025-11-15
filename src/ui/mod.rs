pub mod screens;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame
};

use crate::app::state::{AppState, Screen};

pub fn draw(f: &mut Frame, state: &AppState) {
    match state.current_screen {
        Screen::Home => screens::home::draw_home(f),
        Screen::Search => screens::search::draw_search(f, state),
        Screen::Player => screens::player::draw_player(f, state),
    }
}

pub fn draw_navigation_footer(f: &mut Frame, area: Rect, additional_commands: Option<Vec<Line>>) {
    let mut footer_lines = vec![
        Line::from(vec![
            Span::styled("[*]  ", Style::default().fg(Color::Cyan)),
            Span::styled("Navigation", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+H", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Home  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+S", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Search  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+P", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Player  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Ctrl+Q", Style::default().fg(Color::Red)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Quit", Style::default().fg(Color::White)),
        ]),
    ];

    if let Some(additional) = additional_commands {
        footer_lines.push(Line::from(""));
        footer_lines.extend(additional);
    }

    let footer = Paragraph::new(footer_lines)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(Span::styled(
                    " Controls ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                ))
        );

    f.render_widget(footer, area);
}