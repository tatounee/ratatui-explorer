use termwiz::{input::InputEvent, input::KeyCode};

use super::Input;

impl From<&InputEvent> for Input {
    fn from(value: &InputEvent) -> Self {
        match value {
            InputEvent::Key(key) => match key.key {
                KeyCode::Char('j') | KeyCode::DownArrow => Input::Down,
                KeyCode::Char('k') | KeyCode::UpArrow => Input::Up,
                KeyCode::Char('h') | KeyCode::LeftArrow | KeyCode::Backspace => Input::Left,
                KeyCode::Char('l') | KeyCode::RightArrow | KeyCode::Enter => Input::Right,
                _ => Input::None,
            },
            _ => Input::None,
        }
    }
}
