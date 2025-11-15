use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame
};

use crate::app::state::AppState;
use crate::ui;

pub fn draw_player(f: &mut Frame, _state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  
            Constraint::Length(1),  
            Constraint::Min(5),     
            Constraint::Length(6),  
            Constraint::Length(6), 
        ])
        .split(f.area());

   
    let title = "One Piece - Episode 1000";
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("▶️  ", Style::default().fg(Color::Red)),
            Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ])
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(Span::styled(
                " Now Playing ",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            ))
    )
    .alignment(Alignment::Left);
    f.render_widget(header, chunks[0]);


    let progress = 0.45;
    let current_time = "22:30";
    let total_time = "50:00";
    
    let progress_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::NONE)
        )
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::DarkGray)
        )
        .percent((progress * 100.0) as u16)
        .label(Span::styled(
            format!("{} / {}", current_time, total_time),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        ));
    f.render_widget(progress_gauge, chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),  
            Constraint::Percentage(30),  
        ])
        .split(chunks[2]);

    let video_area = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ╔═══════════════════════════════════════════════════════════╗", 
                Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ║", Style::default().fg(Color::DarkGray)),
            Span::styled("                                                           ", 
                Style::default().fg(Color::DarkGray)),
            Span::styled("║", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ║", Style::default().fg(Color::DarkGray)),
            Span::styled("                                                           ", 
                Style::default().fg(Color::DarkGray)),
            Span::styled("║", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ║", Style::default().fg(Color::DarkGray)),
            Span::styled("                           ", Style::default().fg(Color::DarkGray)),
            Span::styled(" LOG ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("                           ", Style::default().fg(Color::DarkGray)),
            Span::styled("║", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ║", Style::default().fg(Color::DarkGray)),
            Span::styled("                                                           ", 
                Style::default().fg(Color::DarkGray)),
            Span::styled("║", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ║", Style::default().fg(Color::DarkGray)),
            Span::styled("                                                           ", 
                Style::default().fg(Color::DarkGray)),
            Span::styled("║", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ╚═══════════════════════════════════════════════════════════╝", 
                Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  📝 ", Style::default().fg(Color::Yellow)),
            Span::styled("Description:", Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "Luffy and his crew continue their adventure in the New World...",
                Style::default().fg(Color::White)
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(Span::styled(
                " Video ",
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            ))
    )
    .wrap(Wrap { trim: true });
    f.render_widget(video_area, main_chunks[0]);

    let info_lines = vec![
        Line::from(vec![
            Span::styled("ℹ️  ", Style::default().fg(Color::Cyan)),
            Span::styled("Episode Info", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📺 ", Style::default().fg(Color::Magenta)),
            Span::styled("Episode: ", Style::default().fg(Color::White)),
            Span::styled("1000", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("⭐ ", Style::default().fg(Color::Yellow)),
            Span::styled("Rating: ", Style::default().fg(Color::White)),
            Span::styled("9.5/10", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("📅 ", Style::default().fg(Color::Blue)),
            Span::styled("Aired: ", Style::default().fg(Color::White)),
            Span::styled("2021", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("━━━━━━━━━━━━━━━━", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🎬 ", Style::default().fg(Color::Green)),
            Span::styled("Quality:", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("1080p", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🔊 ", Style::default().fg(Color::Magenta)),
            Span::styled("Audio:", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("Japanese", Style::default().fg(Color::Cyan)),
        ]),
    ];

    let info_panel = Paragraph::new(info_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .title(Span::styled(
                    " Info ",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD)
                ))
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info_panel, main_chunks[1]);

    let controls_lines = vec![
        Line::from(vec![
            Span::styled("[*]  ", Style::default().fg(Color::Cyan)),
            Span::styled("Player Controls", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Space", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Play/Pause  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("←", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Rewind  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("→", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Forward", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("↑", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Volume Up  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("↓", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Volume Down  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("M", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Mute", Style::default().fg(Color::White)),
        ]),
    ];

    let controls = Paragraph::new(controls_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(Span::styled(
                    " Player Controls ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                ))
        )
        .wrap(Wrap { trim: true });
    f.render_widget(controls, chunks[3]);

    let additional_commands = vec![
        Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Space", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Play/Pause  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("←", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Rewind  ", Style::default().fg(Color::White)),
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("→", Style::default().fg(Color::Yellow)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled("Forward", Style::default().fg(Color::White)),
        ]),
    ];
    ui::draw_navigation_footer(f, chunks[4], Some(additional_commands));
}

