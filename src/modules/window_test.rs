use std::cmp::min;
use std::io;
use std::io::Write;
use crossterm::terminal;
use crate::{Color, Vec2Int};

pub struct TestWindow<'a> {
    pub esc : ESC<'a>,
    pub size : Vec2Int,

    offset : Vec2Int,

    point_buffer : Vec<Point>,
    past_buffer : Vec<Point>,
    clean_buffer : Vec<Point>,

}
impl TestWindow<'_> {
    pub fn new(max_width:u16, max_height:u16) -> Self {
        // clear the screen
        let mut esc = ESC::new();
        esc.clear();
        esc.hide_cursor();
        esc.disable_auto_line_wrap();

        // initialize the dimensions
        let offset = Vec2Int::new(2, 2);
        let (term_width, term_height) = terminal::size().unwrap();
        let size = Vec2Int::new(min(max_width as i32, term_width as i32), min(max_height as i32, term_height as i32));

        // make clean buffer
        let mut clean_buffer = vec![];
        for _ in 0..(size.x * size.y) {
            clean_buffer.push(Point {
                char : ' ',
                color : Color::Reset,
            });
        }

        Self {
            esc,
            size,
            point_buffer: clean_buffer.clone(),
            past_buffer: clean_buffer.clone(),
            clean_buffer,
            offset,
        }
    }

    pub fn add_point(&mut self, pos: Vec2Int, char: char, color: Color) {
        self.point_buffer[((self.size.x * pos.y) + pos.x) as usize] = (Point {
            char,
            color,
        });
    }

    pub fn render(&mut self) {
        let mut i : usize = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let new_point = &self.point_buffer[i];
                let old_point = &self.past_buffer[i];
                if ! (new_point == old_point) {
                    self.esc.goto_add_char(x + self.offset.x, y + self.offset.y, new_point.char);
                    self.esc.set_color(&new_point.color);
                }
                i += 1;
            }
        }
        self.esc.flush();

        self.past_buffer = self.point_buffer.clone();
        self.point_buffer = self.clean_buffer.clone();
    }

    pub fn clear(&mut self) {
        self.esc.clear();
    }

    pub fn goto(&mut self, x: i32, y: i32) {
        self.esc.goto(x, y);
    }

    pub fn any_code(&mut self, code: &str) {
        self.esc.any_code(code);
    }
}


#[derive(Clone)]
#[derive(PartialEq)]
struct Point {
    char : char,
    color : Color,
}


struct ESC<'a> {
    handle : io::StdoutLock<'a>,
    stdout : io::Stdout,
}
impl ESC<'_> {
    fn new() -> Self {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        Self {
            stdout,
            handle,
        }
    }

    fn clear(&mut self) {
        self.handle.write(b"\x1b[2J").unwrap();
    }

    fn goto_add_char(&mut self, x: i32, y: i32, add:char) {
        self.handle.write(format!("\x1b[{y};{x}H{add}").as_bytes()).unwrap();
    }

    fn goto(&mut self, x: i32, y: i32) {
        self.handle.write(format!("\x1b[{y};{x}H").as_bytes()).unwrap();
    }

    fn goto_add_str(&mut self, x: i32, y: i32, add:&str) {
        self.handle.write(format!("\x1b[{y};{x}H{add}").as_bytes()).unwrap();
    }

    fn add_str(&mut self, add:&str) {
        self.handle.write(add.as_bytes()).unwrap();
    }

    fn add_char(&mut self, add:char) {
        self.handle.write(add.to_string().as_bytes()).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn set_color(&mut self, color: &Color) {
        self.handle.write(color.code().as_ref()).unwrap();
    }

    fn color_reset(&mut self) {
        self.handle.write(Color::Reset.code().as_ref()).unwrap();
    }

    fn hide_cursor(&mut self) {
        self.handle.write(b"\x1b[?25l").unwrap();
    }

    fn show_cursor(&mut self) {
        self.handle.write(b"\x1b[?25h").unwrap();
    }

    fn any_code(&mut self, code: &str) {
        self.handle.write(code.as_ref()).unwrap();
    }

    fn disable_auto_line_wrap(&mut self) {
        self.handle.write(b"\x1b[7l").unwrap();
    }

}