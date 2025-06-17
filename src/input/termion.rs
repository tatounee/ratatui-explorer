use ratatui::termion;
use ratatui::termion::event::{Event, Key};

use super::Input;

impl From<&Event> for Input {
    /// Convert termion [`Event`](https://docs.rs/termion/latest/termion/event/enum.Event.html) to [`Input`].
    ///
    /// **Note:** This implementation is only available when the `termion` feature is enabled.
    fn from(value: &Event) -> Self {
        match value {
            Event::Key(key) => match key {
                Key::Char('j') | Key::Down => Input::Down,
                Key::Char('k') | Key::Up => Input::Up,
                Key::Char('h') | Key::Left | Key::Backspace => Input::Left,
                Key::Char('l') | Key::Right | Key::Char('\n') => Input::Right,
                Key::Home => Input::Home,
                Key::End => Input::End,
                Key::PageUp => Input::PageUp,
                Key::PageDown => Input::PageDown,
                Key::Ctrl('h') => Input::ToggleShowHidden,
                _ => Input::None,
            },
            _ => Input::None,
        }
    }
}
