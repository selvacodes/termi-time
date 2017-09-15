extern crate chrono;

use std::{thread};
use std::time::{Duration , Instant };
use chrono::prelude::*;

fn clear() -> () {
    print!("{}[2J", 27 as char);
}

fn main() {
    let now = Instant::now();
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", Local::now().format("%H:%M:%S"));
        clear();
    }
}
