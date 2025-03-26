#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "termwiz")]
mod termwiz;

/// Input enum to represent the fours different actions available inside a [`FileExplorer`](crate::FileExplorer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Input {
    /// Move the selection up.
    Up,
    /// Move the selection down.
    Down,
    /// Select the first entry.
    Home,
    /// Select the last entry.
    End,
    /// Scroll several entries up.
    PageUp,
    /// Scroll several entries down.
    PageDown,
    /// Go to the parent directory.
    Left,
    /// Go to the child directory (if the selected item is a directory).
    Right,
    /// Do nothing (used for converting events from other libraries, like
    /// [crossterm](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html),
    /// [termion](https://docs.rs/termion/latest/termion/event/enum.Event.html) and
    /// [termwiz](https://docs.rs/termwiz/latest/termwiz/input/enum.InputEvent.html) to [`Input`]).
    None,
}
