use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style}, 
    text::Span, 
    widgets::{Block, Borders, Paragraph, Wrap}, 
    Frame
};

pub fn draw_home(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(5),   
            Constraint::Length(5), 
        ])
        .split(f.area());

    let title = Paragraph::new("Streameko")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("WELCOME"));
    f.render_widget(title, chunks[0]);

    let menu = Paragraph::new("Menu contents")
        .block(Block::default().borders(Borders::ALL).title("MAIN MENU"))
        .style(Style::default().fg(Color::White));

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
