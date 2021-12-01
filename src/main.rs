#![feature(array_windows)]

mod day1;
mod util;

fn main() {
    match day1::run() {
        Ok(()) => (),
        Err(e) => println!("error:\n{}", e),
    }
}
