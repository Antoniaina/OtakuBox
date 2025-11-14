use std::vec;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Borders, Paragraph, Wrap}, 
    Frame
};

use crate::ui;

pub fn draw_home(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(12), 
            Constraint::Min(8),   
            Constraint::Length(6), 
        ])
        .split(f.area());

    let logo: &'static str = r#"
██████╗ ████████╗ █████╗ ██╗  ██╗██╗   ██╗██████╗  ██████╗ ██╗  ██╗
██╔═══██╗╚══██╔══╝██╔══██╗██║ ██╔╝██║   ██║██╔══██╗██╔═══██╗╚██╗██╔╝
██║   ██║   ██║   ███████║█████╔╝ ██║   ██║██████╔╝██║   ██║ ╚███╔╝ 
██║   ██║   ██║   ██╔══██║██╔═██╗ ██║   ██║██╔══██╗██║   ██║ ██╔██╗ 
╚██████╔╝   ██║   ██║  ██║██║  ██╗╚██████╔╝██████╔╝╚██████╔╝██╔╝ ██╗
 ╚═════╝    ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
"#;                                                                  

    let logo_widget = Paragraph::new(logo)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Span::styled(
                    " STREAMEKO ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                ))
        );
    f.render_widget(logo_widget, chunks[0]);
    

    let welcome_lines = vec![
        Line::from(vec![
            Span::styled("[*] ", Style::default().fg(Color::Yellow)),
            Span::styled("Welcome to Streameko", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" [*]", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[*] ", Style::default().fg(Color::Magenta)),
            Span::styled("Your terminal gateway to anime streaming ", Style::default().fg(Color::White)),
            Span::styled("[*] ", Style::default().fg(Color::Magenta)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", 
                Style::default().fg(Color::DarkGray))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[*] ", Style::default().fg(Color::Green)),
            Span::styled("Quick Start:", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [*] ", Style::default().fg(Color::Cyan)),
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled("Ctrl+S", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to search for anime", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [*] ", Style::default().fg(Color::Cyan)),
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled("Ctrl+P", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to open player", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [*] ", Style::default().fg(Color::Cyan)),
            Span::styled("Press ", Style::default().fg(Color::White)),
            Span::styled("Ctrl+Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(" to quit", Style::default().fg(Color::White)),
        ]),
    ];

    let content_height = welcome_lines.len() as u16;
    let available_height = chunks[1].height;
    let padding = (available_height.saturating_sub(content_height)) / 2;

    let mut padded_lines = vec![];
    for _ in 0..padding {
        padded_lines.push(Line::raw(""));
    }

    padded_lines.extend(welcome_lines);    

    let menu = Paragraph::new(padded_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    " Dashboard ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                ))
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(menu, chunks[1]);

    ui::draw_navigation_footer(f, chunks[2], None);
}
