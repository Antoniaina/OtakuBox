use std::vec;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Borders, Paragraph, Wrap}, 
    Frame
};

pub fn draw_home(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(10), 
            Constraint::Min(6),   
            Constraint::Length(5), 
        ])
        .split(f.area());

    let logo: &'static str = r#"
███████╗████████╗██████╗ ███████╗ █████╗ ███╗   ███╗███████╗██╗  ██╗ ██████╗ 
██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔══██╗████╗ ████║██╔════╝██║ ██╔╝██╔═══██╗
███████╗   ██║   ██████╔╝█████╗  ███████║██╔████╔██║█████╗  █████╔╝ ██║   ██║
╚════██║   ██║   ██╔══██╗██╔══╝  ██╔══██║██║╚██╔╝██║██╔══╝  ██╔═██╗ ██║   ██║
███████║   ██║   ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████╗██║  ██╗╚██████╔╝
╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝ ╚═════╝ 
"#;

    let logo_widget = Paragraph::new(logo)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(logo_widget, chunks[0]);
    

    let lines = vec![
        Line::from(Span::raw("[*] Welcome to Streameko [*]")),
        Line::from(""),
        Line::from("Your terminal gateway to anime streaming."),
        Line::from(""),
        Line::from("Use the available commands to begin your anime journey."),
    ];

    let content_height = lines.len() as u16;
    let available_height = chunks[1].height;
    let padding = (available_height.saturating_sub(content_height)) / 2;

    let mut padded_lines = vec![];
    for _ in 0..padding {
        padded_lines.push(Line::raw(""));
    }

    padded_lines.extend(lines);    

    let menu = Paragraph::new(padded_lines)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(menu, chunks[1]);

    let footer_text = "[↑/↓] Navigate  •  [Enter] Select  •  [S] Search  •  [F] Favorites  •  [Q] Quit";
    let footer = Paragraph::new(footer_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Controls", 
                Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD))                  
            )
        );

    f.render_widget(footer, chunks[2]);
}
