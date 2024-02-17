use crossterm::event::{Event, KeyCode};

use super::Input;

impl From<&Event> for Input {
    fn from(value: &Event) -> Self {
        if let Event::Key(key) = value {
            if matches!(
                key.kind,
                crossterm::event::KeyEventKind::Press | crossterm::event::KeyEventKind::Repeat
            ) {
                let input = match key.code {
                    KeyCode::Char('j') | KeyCode::Down => Input::Down,
                    KeyCode::Char('k') | KeyCode::Up => Input::Up,
                    KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => Input::Left,
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => Input::Right,
                    _ => Input::None,
                };

                return input;
            }
        }

        Input::None
    }
}
