
#[cfg(feature = "crossterm")]
mod crossterm;


#[cfg(feature = "termion")]
mod termion;


#[cfg(feature = "termwiz")]
mod termwiz;


pub enum Input {
    Up,
    Down,
    Left,
    Right,
    None,
}
