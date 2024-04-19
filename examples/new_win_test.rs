use crossterm::event::KeyCode;
use terminal_game_engine_lib::*;
use device_query::Keycode;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    test()
}


fn test() {
    let mut win = TestWindow::new(1000, 1000);
    let mut input = Input::new(true);
    win.border(Color::Red);
    win.render();

    const SPEED: f32 = 0.05;
    const DECEL: f32 = 0.99;


    let mut pos = Vec2Float::middle_of(Vec2Float::new(0.0, 0.0), win.size.as_float());
    let mut velocity = Vec2Float::ZERO;
    loop {
        input.update_just_pressed();
        let input = Vec2Float::input_vec(input.is_pressed(Keycode::A), input.is_pressed(Keycode::D), input.is_pressed(Keycode::S), input.is_pressed(Keycode::W), true);


        velocity += input * Vec2Float::sv(SPEED);
        velocity *= Vec2Float::sv(DECEL);
        pos += velocity;
        pos.limit(Vec2Float::ZERO, win.size.as_float() - Vec2Float::sv(1.0));


        win.add_point(pos.as_int(), Color::Cyan, '#');
        win.render();

        wait(1000 / 60);
    }
}