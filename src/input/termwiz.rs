use ratatui::termwiz;
use ratatui::termwiz::{input::InputEvent, input::KeyCode, input::Modifiers};

use super::Input;

impl From<&InputEvent> for Input {
    /// Convert termwiz [`InputEvent`](https://docs.rs/termwiz/latest/termwiz/input/enum.InputEvent.html) to [`Input`].
    ///
    /// **Note:** This implementation is only available when the `termwiz` feature is enabled.
    fn from(value: &InputEvent) -> Self {
        match value {
            InputEvent::Key(key) => match key.key {
                KeyCode::Char('j') | KeyCode::DownArrow => Input::Down,
                KeyCode::Char('k') | KeyCode::UpArrow => Input::Up,
                KeyCode::LeftArrow | KeyCode::Backspace => Input::Left,
                KeyCode::Char('h') => {
                    if key.modifiers.contains(Modifiers::CTRL) {
                        Input::ToggleShowHidden
                    } else {
                        Input::Left
                    }
                }

                KeyCode::Char('l') | KeyCode::RightArrow | KeyCode::Enter => Input::Right,
                KeyCode::Home => Input::Home,
                KeyCode::End => Input::End,
                KeyCode::PageUp => Input::PageUp,
                KeyCode::PageDown => Input::PageDown,
                _ => Input::None,
            },
            _ => Input::None,
        }
    }
}
