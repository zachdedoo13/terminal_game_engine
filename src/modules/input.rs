use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use crate::modules::other::wait;


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Input {
    device_state : DeviceState,
    input_listener: InputListener,
    just_pressed : Vec<Keycode>,
    exit_on_q : bool,
}
#[allow(dead_code)]
impl Input {
    pub fn new(exit_on_q: bool) -> Self {
        Self {
            device_state : DeviceState::new(),
            input_listener: InputListener::new(),
            just_pressed : vec![],
            exit_on_q,
        }
    }

    pub fn is_pressed(&self, key:Keycode) -> bool{
        let keys: Vec<Keycode> = self.device_state.get_keys();
        keys.contains(&key)
    }

    pub fn get_keys(&self) -> Vec<Keycode> {
        self.device_state.get_keys()
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        let mouse: MouseState = self.device_state.get_mouse();
        mouse.coords
    }

    pub fn mouse_pressed(&self, button:usize) -> bool {
        // left = 1;
        // right = 2;
        // middle = 3;
        // side = 5;

        let mouse: MouseState = self.device_state.get_mouse();
        mouse.button_pressed[button]
    }

    pub fn update_just_pressed(&mut self) {
        self.just_pressed = self.input_listener.get_just_pressed();
        if self.exit_on_q && self.is_just_pressed(Keycode::Q) {
            std::process::exit(0);
        }
    }

    pub fn is_just_pressed(&self, key : Keycode) -> bool {
        return if self.just_pressed.contains(&key) {
            true
        } else {
            false
        }

    }

    pub fn ijp(&self, key : Keycode) -> bool {
        return if self.just_pressed.contains(&key) {
            true
        } else {
            false
        }

    }
}



// this can hold back the rest of the program if it takes to long
struct InputListener {
    kill_tx : Sender<()>,
    data_rx : Receiver<Vec<Keycode>>,
}
#[allow(dead_code)]
impl InputListener {
    fn new() -> Self {
        let (kill_tx, kill_rx) = mpsc::channel();
        let (data_tx, data_rx) = mpsc::channel();
        let input = DeviceState::new();

        // the wait time between input checks
        let wait_time = 0;

        let _input_detector = thread::spawn(move || {
            let mut local_just_pressed: Vec<Keycode> = vec![];
            loop {
                let keys = input.get_keys();
                for key in keys {
                    if ! local_just_pressed.contains(&key) {
                        local_just_pressed.push(key)
                    }
                }

                if kill_rx.try_recv().is_ok() {
                    data_tx.send(local_just_pressed.clone()).unwrap();
                    local_just_pressed = vec![];
                }

                wait(wait_time)
            }
        });

        Self {
            kill_tx,
            data_rx,
        }
    }


    fn get_just_pressed(&mut self) -> Vec<Keycode> {
        self.kill_tx.send(()).unwrap();
        let data = self.data_rx.recv().unwrap();
        data
    }
}