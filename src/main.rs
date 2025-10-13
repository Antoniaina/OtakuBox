use ratatui::{prelude::*, widgets::{Block, Borders, List, ListItem, Paragraph}};
use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    // loop {
        terminal.draw(|f| {
            let size = f.area();

            let header = Block::default()
                .title("header")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow));

            f.render_widget(header, Rect::new(0, 0, size.width, 3));

            let body = Rect::new(0, 3, size.width, size.height - 6);

            let left_col = Rect::new(0, 3, size.width / 3, body.height);
            let right_col = Rect::new(size.width/3, 3, size.width - size.width/3, body.height);

            let items = vec![
                ListItem::new("Opt1"),
                ListItem::new("Opt2"),
                ListItem::new("Opt3"),
                ListItem::new("Opt4"),
            ];
            let lists = List::new(items)
                .block(Block::default().title("Opts").borders(Borders::ALL));
            f.render_widget(lists, left_col);

            let content = Paragraph::new(format!("Contnets here..., {}, {}", body.height, size.height))
                .block(Block::default().title("Title").borders(Borders::ALL));
            f.render_widget(content, right_col);

            let footer = Block::default()
                .title("Footer")
                .borders(Borders::ALL);
            f.render_widget(footer, Rect::new(0, size.height -3, size.width, 3));
        })?;
    // }

    terminal.show_cursor()?;
    // terminal.clear()?;

    Ok(())
}

