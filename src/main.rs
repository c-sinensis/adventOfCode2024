// main file to route control to individual puzzle functions
mod day01;

use std::env;

fn main() {
    let mut cmdargs= env::args().skip(1);
    let puzzle: String = cmdargs.next().unwrap_or_default();
    match puzzle.as_str() {
        "1.1" => day01::part1(),
        "1.2" => day01::part2(),

        _ => panic!("Puzzle Not Found"), // throw error for invalid puzzle
    };
}