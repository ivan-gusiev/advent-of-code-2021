#![feature(array_windows)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

use util::BResult;

fn main() {
    let section = std::env::args()
        .last()
        .unwrap_or_else(|| "unknown".to_string());

    match section.as_str() {
        "1" => process_result(day1::run()),
        "2" => process_result(day2::run()),
        "3" => process_result(day3::run()),
        "4" => process_result(day4::run()),
        "5" => process_result(day5::run()),
        _ => println!("huh?"),
    }
}

fn process_result<T>(res: BResult<T>) {
    match res {
        Ok(_) => (),
        Err(e) => println!("error:\n{}", e),
    }
}
