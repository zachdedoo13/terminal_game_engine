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

    let mut win = Window::new(20, 20);
    let mut input = Input::new(true);
    win.set_color(Color::RED);
    win.border();



    let mut head_pos = Vec2::middle_of(Vec2::ZERO, win.size);
    let mut dir = Vec2::ZERO;
    let mut tail_len = 6;

    let mut history: Vec<Vec2> = vec![head_pos.clone(); tail_len];



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



        history.push(head_pos);

        if history.len() > tail_len {
            for point in history.iter().skip(history.len() - tail_len) {
                win.print_vec(point, '#')
            }
        }

        println!("{}", head_pos);


        // flush updates and wait
        win.flush();
        wait(10);
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
