use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub fn wait(mills:u64) {
    sleep(Duration::from_millis(mills))
}