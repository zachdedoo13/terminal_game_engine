use std::time::Instant;
use terminal_game_engine_lib::*;
use crossterm::terminal;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    test()
}


fn test() {
    let mut win = TestWindow::new(1000, 1000);

    let mut pos = Vec2Int::ZERO;

    let size = 5; // size of the square
    let mut pos = Vec2Int::ZERO; // initial position
    let mut dir = Vec2Int::new(1, 1); // initial direction


    let mut joe = true;
    loop {
        let st = Instant::now();

        //if joe { for x in 0..win.size.x { for y in 0..win.size.y { win.add_point(Vec2Int::new(x, y), '#', Color::Reset);}} } else { for x in 0..win.size.x { for y in 0..win.size.y { win.add_point(Vec2Int::new(x, y), ' ', Color::Reset);}} }


        for x in pos.x..(pos.x + size) {
            for y in pos.y..(pos.y + size) {
                if x < win.size.x && y < win.size.y {
                    win.add_point(Vec2Int::new(x, y), '#', Color::Reset);
                }
            }
        }



        win.render();

        pos += dir;

        // If the square hits the edge of the screen, reverse the direction
        if pos.x <= 0 || pos.x + size >= win.size.x {
            dir.x = -dir.x;
        }
        if pos.y <= 0 || pos.y + size >= win.size.y {
            dir.y = -dir.y;
        }

        let et = Instant::now();

        win.goto(0, 0);
        let ms = (et - st).as_micros() as f64;
        print!("fps : {:?}     ", (1_000_000.0 / ms) as i32);
        wait(0);

        joe = !joe;

    }
}