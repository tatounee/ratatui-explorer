use std::io::{self, stdout};

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

use ratatui_explorer::{FileExplorer, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create a new file explorer with the light theme.
    let mut dark_theme = false;
    let mut file_explorer = FileExplorer::with_theme(get_light_theme())?;

    loop {
        // Render the file explorer widget.
        terminal.draw(|f| {
            f.render_widget(&file_explorer.widget(), f.area());
        })?;

        // Read the next event from the terminal.
        let event = read()?;
        // If the user presses `Ctrl + s`, switch the theme.
        // If the user presses `Ctrl + q`, quit the application.
        if let Event::Key(key) = event {
            if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('s') => {
                        dark_theme = !dark_theme;
                        if dark_theme {
                            file_explorer.set_theme(get_dark_theme());
                        } else {
                            file_explorer.set_theme(get_light_theme());
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn get_light_theme() -> Theme {
    Theme::new()
        .with_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::Black).bg(Color::White)),
        )
        .with_item_style(Style::default().fg(Color::Yellow))
        .with_dir_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .with_highlight_symbol("> ")
        .add_default_title()
        .with_title_top(|_| Line::from(" ☀ Theme ").right_aligned())
        .with_title_bottom(|_| " ^q Quit | ^s Switch theme ".into())
}

fn get_dark_theme() -> Theme {
    Theme::new()
        .with_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::White).bg(Color::Black)),
        )
        .with_item_style(Style::default().fg(Color::Yellow))
        .with_dir_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .with_highlight_symbol("> ")
        .add_default_title()
        .with_title_top(|_| Line::from(" ☾ Theme ").right_aligned())
        .with_title_bottom(|_| " ^q Quit | ^s Switch theme ".into())
}
