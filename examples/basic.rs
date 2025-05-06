use std::io::{self, stdout};

use ratatui::crossterm;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

use ratatui_explorer::{FileExplorer, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create a new file explorer with the default theme and title.
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorer::with_theme(theme)?;

    loop {
        // Render the file explorer widget.
        terminal.draw(|f| {
            f.render_widget(&file_explorer.widget(), f.area());
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
