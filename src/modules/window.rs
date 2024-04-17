use std::io;
use std::io::Write;
use crate::modules::vectors::Vec2Int;


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Window<'a> {
    stdout: io::Stdout,
    pub handle: io::StdoutLock<'a>,
    rows: i32,
    cols : i32,
    pub size : Vec2Int,
    buffer: Vec<char>,
    o_buff: Vec<char>,
}
#[allow(dead_code)]
impl Window<'_> {
    pub fn new(width: i32, height: i32) -> Self {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        let rows = height;
        let cols = width;

        let buffer = vec![' '; (rows * cols) as usize];

        // hide cursor
        handle.write(b"\x1b[?25l").unwrap();

        // clear terminal
        handle.write(b"\x1b[2J").unwrap();

        // update terminal
        handle.flush().unwrap();

        Self {
            stdout,
            handle,
            size : Vec2Int::new(width, height),
            cols,
            rows,
            buffer: buffer.clone(),
            o_buff : buffer.clone(),
        }
    }

    pub fn border(&mut self) {
        let f = '-';
        let w = '|';

        let c = '+';

        // roof
        for x in 0..self.cols {
            self.print_at_true(x + 2, 1, f);
        }

        // floor
        for x in 0..self.cols {
            self.print_at_true(x + 2, self.rows + 2, f);
        }

        // left wall
        for y in 0..self.rows {
            self.print_at_true(0, y + 2, w);
        }

        // right wall
        for y in 0..self.rows {
            self.print_at_true(self.cols + 2, y + 2, w);
        }

        // corners top
        self.print_at_true(0, 0, c);
        self.print_at_true(self.cols + 2, 0, c);

        // cornets bottom
        self.print_at_true(0, self.rows + 2, c);
        self.print_at_true(self.cols + 2, self.rows + 2, c);

        // flush buffer
        self.flush()
    }

    pub fn print(&mut self, x:i32, y:i32, char:char) {
        // -1 because (xy > 0) not work
        if (x < self.cols) && (y < self.rows) && (x > -1) && (y > -1) {
            self.buffer[((self.rows * x ) + y) as usize] = char;
        } else {
            println!("print index out of range on fn print x : {} y : {}", x, y)
        }
    }
    // prints to buffer

    pub fn print_vec(&mut self, pos:&Vec2Int, char:char) {
        self.print(pos.x, pos.y, char)
    }
    // prints to buffer from Vec2Int

    pub fn print_at(&mut self, x:i32, y:i32, add:char) {
        let offset_x = 2;
        let offset_y = 2;
        self.print_at_true(x + offset_x, y + offset_y, add)
    }
    // adds an offset to the print, so it stays withing the border bounds

    pub fn print_at_true(&mut self, x:i32, y:i32, add:char) {
        let sequence = format!("[{y};{x}H{add}");
        self.handle.write(sequence.as_bytes()).unwrap();
    }
    // has no offset, directly prints

    pub fn clear(&mut self) {
        self.handle.write(b"\x1b[2J").unwrap();
        self.handle.flush().unwrap();
    }
    // clears the terminal

    pub fn flush(&mut self) {
        let mut i = 0;
        for x in 0..self.cols {
            for y in 0..self.rows {
                self.print_at(x, y, self.buffer[i as usize]);
                i += 1;
            }
        }

        self.buffer = self.o_buff.clone();
        self.stdout.flush().unwrap();
    }
    // update clear buffer

    pub fn flush_but_theres_a_hole(&mut self) {
        let mut i = 0;
        for x in 0..self.cols {
            for y in 0..self.rows {
                self.print_at(x, y, self.buffer[i as usize]);
                i += 1;
            }
        }

        self.stdout.flush().unwrap();
    }
    // update dont clear buffer

    pub fn fill(&mut self, fill_with : char, do_it_slowly_i_dont_fucken_know_why_but_why_not: u64) {
        for x in 0..self.cols{
            for y in 0..self.rows {
                self.print(x, y, fill_with);

                if do_it_slowly_i_dont_fucken_know_why_but_why_not != 0 {
                    std::thread::sleep(std::time::Duration::from_millis(do_it_slowly_i_dont_fucken_know_why_but_why_not));
                    self.flush_but_theres_a_hole()
                }
            }
        }
    }

    pub fn debug_str(&mut self, x:i32, y:i32, to_print : String) {
        self.print_at_true(x, y, ' ');
        self.handle.write(to_print.as_ref()).unwrap();
    }

    pub fn set_color(&mut self, color: &str) {
        self.handle.write(color.as_ref()).unwrap();
    }

    pub fn flush_escape_code(&mut self) {
        self.stdout.flush().unwrap();
    }
}
