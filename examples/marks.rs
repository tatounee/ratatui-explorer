use std::io::{self, stdout};

use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, KeyEvent, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::crossterm;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, FrameExt, Paragraph},
};
use std::fs::FileType;

use ratatui_explorer::{FileExplorerBuilder, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let layout = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)]);

    let mut special_marked_styles = false;
    let mut file_explorer = FileExplorerBuilder::default()
        .theme(get_theme(special_marked_styles))
        .show_hidden(true)
        // Allowing marking any files apart from "..".
        .mark_condition(|f| f.name != "..")
        .build()?;

    loop {
        let marks: Vec<_> = file_explorer
            .marked()
            .iter()
            .map(|f| {
                Line::from_iter(vec![
                    render_file_type_span(f.file_type),
                    Span::raw("  "),
                    Span::raw(f.path.to_str().unwrap_or_default()),
                ])
            })
            .collect();

        terminal.draw(|f| {
            let frame_rect = f.area();
            // Add margins
            let app_rect = frame_rect.centered(
                Constraint::Length(frame_rect.width - 2),
                Constraint::Length(frame_rect.height - 2),
            );
            let chunks = layout.split(app_rect);
            let explorer_panel = chunks[0];
            let marked_files_panel = chunks[1];

            // File explorer widget
            f.render_widget_ref(file_explorer.widget(), explorer_panel);

            // Marked files listing
            f.render_widget(
                Paragraph::new(marks).block(
                    Block::default()
                        .title(
                            Span::raw(format!(
                                "{} marked file{}",
                                file_explorer.marked_count(),
                                if file_explorer.marked_count() != 1 {
                                    "s"
                                } else {
                                    ""
                                }
                            ))
                            .bold(),
                        )
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                ),
                marked_files_panel,
            );
        })?;

        // Read the next event from the terminal.
        let event = read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('u'),
                ..
            }) => {
                file_explorer.unmark_all();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            }) => {
                special_marked_styles = !special_marked_styles;
                file_explorer.set_theme(get_theme(special_marked_styles));
            }
            _ => {}
        }
        // Handle the event in the file explorer.
        file_explorer.handle(&event)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn get_theme(special_marked_styles: bool) -> Theme {
    let theme = Theme::default()
        .with_title_top(|_f| {
            Line::from_iter(vec![
                Span::raw("M").bold(),
                ": mark/unmark ".into(),
                Span::raw("U").bold(),
                ": unmark all".into(),
            ])
        })
        .with_title_bottom(|_f| Line::from_iter(vec![Span::raw("S").bold(), ": mark style".into()]))
        .with_scroll_padding(1);

    if special_marked_styles {
        theme
            .with_marked_item_style(Style::new().light_magenta().bold())
            .with_marked_dir_style(Style::new().light_red().bold())
    } else {
        theme
    }
}

fn render_file_type_span(file_type: Option<FileType>) -> Span<'static> {
    file_type
        .map(|t| {
            if t.is_file() {
                Span::raw("F").white()
            } else if t.is_dir() {
                Span::raw("D").light_blue()
            } else {
                Span::raw("?").red()
            }
        })
        .unwrap_or(Span::raw("?"))
        .bold()
}
