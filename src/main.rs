use std::{io, thread, time};

fn main() {
    print!("{esc}[2J{esc}[1:1H", esc = 27 as char);
    println!("Hello, world!");
}

fn sleep_seconds(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}
