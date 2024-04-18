use std::time::{Duration, Instant};
use terminal_game_engine_lib::*;
use crossterm::terminal;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    test()
}


fn test() {
    let mut win = TestWindow::new(1000, 1000);
    //win.border();


    loop {
        let st = Instant::now();

        win.add_point(Vec2Int::new(0, 0), Color::Red, '#');
        win.add_point(Vec2Int::new(10, 10), Color::Red, '$');
        win.add_point(Vec2Int::new(10, 0), Color::Red, '%');
        win.add_point(Vec2Int::new(0, 10), Color::Red, 'F');




        win.render();


        win.goto(0, 0);
        let et = Instant::now();
        wait(20);
        let t = et - st;
        //print!("fps : {:?}     ", t);

    }
}