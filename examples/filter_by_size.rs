use std::{
    io::{self, stdout},
    sync::{Arc, Mutex},
};

use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{crossterm, widgets::FrameExt};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType},
};

use ratatui_explorer::{File, FileExplorerBuilder, Theme};

const MIN_SIZE: u64 = 10 << 8; // 10kiB

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let show_all_file = Arc::new(Mutex::new(true));
    let status = Arc::clone(&show_all_file);

    // Create a new file explorer with the default theme and titles showing the current status of the filter.
    let mut file_explorer = FileExplorerBuilder::default()
        .theme(get_theme(status))
        .filter_map(|file| filter_hight_volume(file, 0)) // Add our filter to the file explorer
        .build()?;

    loop {
        // Render the file explorer widget.
        terminal.draw(|f| {
            f.render_widget_ref(file_explorer.widget(), f.area());
        })?;

        // Read the next event from the terminal.
        let event = read()?;

        // If the user presses `q`, quit the application.
        // If the user presses `t`, toggle the filter.
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('t') => {
                    let mut show_all_file = show_all_file.lock().unwrap();
                    *show_all_file = !*show_all_file;
                    if *show_all_file {
                        file_explorer
                            .set_filter_map(|file| filter_hight_volume(file, 0))
                            .unwrap();
                    } else {
                        file_explorer
                            .set_filter_map(|file| filter_hight_volume(file, MIN_SIZE))
                            .unwrap();
                    }
                }
                _ => {}
            }
        }

        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn filter_hight_volume(mut file: File, min_size: u64) -> Option<File> {
    if file.is_dir {
        return Some(file);
    }

    let metadata = file.path.metadata().ok()?;
    let size = metadata.len();

    if size < min_size {
        None
    } else {
        let size = format!("{size}B");
        file.name = format!("{:>6}  {}", size, file.name);
        Some(file)
    }
}

fn get_theme(status: Arc<Mutex<bool>>) -> Theme {
    let green_style: Style = Style::default().fg(Color::Green);

    Theme::default()
        .with_block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(Color::LightGreen)),
        )
        .with_dir_style(green_style.add_modifier(Modifier::BOLD))
        .with_highlight_dir_style(green_style.add_modifier(Modifier::BOLD).bg(Color::DarkGray))
        .with_item_style(green_style)
        .with_highlight_item_style(green_style.bg(Color::DarkGray))
        .add_default_title()
        .with_title_bottom(|_| " q Quit | t Toggle filter ".into())
        .with_title_top(move |_| {
            if *status.lock().unwrap() {
                Line::from(format!(" filter: OFF ({}kiB) ", MIN_SIZE >> 8))
                    .style(Style::default().fg(Color::Red))
                    .right_aligned()
            } else {
                Line::from(format!(" filter: ON  ({}kiB) ", MIN_SIZE >> 8))
                    .style(green_style)
                    .right_aligned()
            }
        })
}
