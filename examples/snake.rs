use std::time;
use std::time::Duration;
use device_query::Keycode::{A, D, S, W};
use terminal_game_engine_lib::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    snake();
}

// snake WIP
#[allow(dead_code)]
fn snake() {
    // uses 0.2.1

    let mut win = Window::new(80, 80);
    let mut input = Input::new(true);
    win.border();



    let mut head_pos = Vec2::middle_of(Vec2::ZERO, win.size);
    let mut dir = Vec2::ZERO;
    let mut tail_len = 3;

    let mut history: Vec<Vec2> = vec![Vec2::ZERO; tail_len];

    let mut points = 0;

    let mut food = Vec2::random_range(Vec2::ZERO, win.size);
    while food == head_pos {
        food = Vec2::random_range(Vec2::ZERO, win.size);
    }


    let mut latest = std::time::Instant::now();

    loop {
        // handle input
        input.update_just_pressed();
        let input = Vec2::input_vec(input.ijp(A), input.ijp(D), input.ijp(W), input.ijp(S), false);
        if input != Vec2::ZERO {
            dir = input;
        }
        head_pos += dir;

        // logic
        if head_pos.x >= win.size.x { head_pos.x = 0; }
        else if head_pos.x < 0 { head_pos.x = win.size.x - 1; }
        if head_pos.y >= win.size.y { head_pos.y = 0; }
        else if head_pos.y < 0 { head_pos.y = win.size.y - 1; }




        if history.len() > tail_len {
            for point in history.iter().skip(history.len() - tail_len) {
                if point == &food || head_pos == food{
                    points += 1;
                    tail_len += 1;
                    food = spawn_food(&history, tail_len, win.size);
                }
                if point == &head_pos {
                    kill()
                }
                win.print_vec(point, '#')
            }
        }

        win.print_vec(&head_pos, '#');

        win.print_vec(&food, '*');

        let time = time::Instant::now() - latest;
        latest = time::Instant::now();
        println!("{:?}", time);

        if dir != Vec2::ZERO {history.push(head_pos);}


        // flush updates and wait
        win.flush();
        wait(100);
    }
}


fn spawn_food(history: &Vec<Vec2>, tail_len:usize, size : Vec2) -> Vec2 {
    let mut food = Vec2::random_range(Vec2::ZERO, size);
    if history.len() > tail_len {
        for point in history.iter().skip(history.len() - tail_len) {
            if point == &food {
                food = spawn_food(history, tail_len, size);
            }
        }
    }
    food
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
