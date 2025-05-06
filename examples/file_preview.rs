use std::{
    borrow::Cow,
    fs::read_to_string,
    io::{self, stdout},
};

use ratatui::crossterm;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use ratatui_explorer::{File, FileExplorer, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let layout = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);

    // Create a new file explorer with the default theme and title.
    let theme = get_theme();
    let mut file_explorer = FileExplorer::with_theme(theme)?;

    loop {
        // Get the content of the current selected file (if it's indeed a file).
        let file_content = get_file_content(file_explorer.current());

        let file_content = match file_content {
            Ok(file_content) => file_content,
            _ => "Couldn't load file.".into(),
        };

        // Render the file explorer widget and the file content.
        terminal.draw(|f| {
            let chunks = layout.split(f.area());

            f.render_widget(&file_explorer.widget(), chunks[0]);
            f.render_widget(Clear, chunks[1]);
            f.render_widget(
                Paragraph::new(file_content).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                ),
                chunks[1],
            );
        })?;

        // Read the next event from the terminal.
        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn get_file_content(file: &File) -> io::Result<Cow<'_, str>> {
    // If the path is a file, read its content.
    if file.is_file() {
        read_to_string(file.path()).map(Into::into)
    } else if file.is_dir() {
        Ok("".into())
    } else {
        Ok("<not a regular file>".into())
    }
}

fn get_theme() -> Theme {
    Theme::default()
        .with_block(Block::default().borders(Borders::ALL))
        .with_dir_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .with_highlight_dir_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .with_scroll_padding(1)
}
