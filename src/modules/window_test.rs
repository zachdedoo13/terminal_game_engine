use std::io;
use std::io::Write;
use crate::Color;

pub struct TestWindow<'a> {
    pub esc : ESC<'a>,
}
impl TestWindow<'_> {
    pub fn new() -> Self {
        let mut esc = ESC::new();
        esc.clear();

        Self {
            esc,
        }
    }
}


pub struct ESC<'a> {
    handle : io::StdoutLock<'a>,
    stdout : io::Stdout,
}
impl ESC<'_> {
    pub fn new() -> Self {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        Self {
            stdout,
            handle,
        }
    }

    pub fn clear(&mut self) {
        self.handle.write(b"\x1b[2J").unwrap();
    }

    pub fn goto_add_char(&mut self, x: i32, y: i32, add:char) {
        self.handle.write(format!("\x1b[{x};{y}H{add}").as_bytes()).unwrap();
    }

    pub fn goto(&mut self, x: i32, y: i32) {
        self.handle.write(format!("\x1b[{x};{y}H").as_bytes()).unwrap();
    }

    pub fn goto_add_str(&mut self, x: i32, y: i32, add:&str) {
        self.handle.write(format!("\x1b[{x};{y}H{add}").as_bytes()).unwrap();
    }

    pub fn add_str(&mut self, add:&str) {
        self.handle.write(add.as_bytes()).unwrap();
    }

    pub fn add_char(&mut self, add:char) {
        self.handle.write(add.to_string().as_bytes()).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn set_color(&mut self, color: &str) {
        self.handle.write(color.as_ref()).unwrap();
    }

    pub fn color_reset(&mut self) {
        self.handle.write(Color::RESET.as_ref()).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        self.handle.write(b"\x1b[?25l").unwrap();
    }

    pub fn show_cursor(&mut self) {
        self.handle.write(b"\x1b[?25h").unwrap();
    }

}