use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub fn wait(mills:u64) {
    sleep(Duration::from_millis(mills))
}

#[allow(dead_code)]
pub fn kill() {
    println!("killed proses");
    std::process::exit(0)
}

#[allow(dead_code)]
pub fn time_func<F>(func: F, repeats:usize) -> Duration
    where
        F: FnOnce() + Copy,
{
    let mut total = Duration::new(0, 0);
    for _ in 0..repeats {
        let start = std::time::Instant::now();
        func();
        let end = std::time::Instant::now();
        total += end - start
    }
    total / repeats as u32
}