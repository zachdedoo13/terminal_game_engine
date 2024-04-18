



#[derive(PartialEq, Eq, Hash)]
#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Color {
    pub fn code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        }
    }
}