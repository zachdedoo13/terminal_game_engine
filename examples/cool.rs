use terminal_game_engine_lib::*;

fn main() {
    let mut win = Window::new(50, 50);
    win.border();
    win.set_color(Color::RED);

    loop {
        win.fill('/', 1);
        win.fill('|', 1);
        win.fill('<', 1);
        win.fill('>', 1);
        win.fill('?', 1);
        win.fill(':', 1);
        win.fill('"', 1);
        win.fill('{', 1);
        win.fill('}', 1);

        win.fill(' ', 1);

        win.fill('!', 1);
        win.fill('@', 1);
        win.fill('#', 1);
        win.fill('$', 1);
        win.fill('%', 1);
        win.fill('^', 1);
        win.fill('&', 1);
        win.fill('*', 1);
        win.fill('(', 1);
        win.fill(')', 1);

        win.fill(' ', 1);

        win.fill('1', 1);
        win.fill('2', 1);
        win.fill('3', 1);
        win.fill('4', 1);
        win.fill('5', 1);
        win.fill('6', 1);
        win.fill('7', 1);
        win.fill('8', 1);
        win.fill('9', 1);
        win.fill('0', 1);

        win.fill(' ', 1);

        win.fill('q', 1);
        win.fill('w', 1);
        win.fill('e', 1);
        win.fill('r', 1);
        win.fill('t', 1);
        win.fill('y', 1);
        win.fill('u', 1);
        win.fill('i', 1);
        win.fill('o', 1);
        win.fill('p', 1);

        win.fill(' ', 1);
    }
}