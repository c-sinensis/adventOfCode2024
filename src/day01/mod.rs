// Advent of Code Day 1, Part 1

use std::fs;
use std::path::Path;

pub fn part1() {
    // vectors to be sorted
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // read from file
    let filepath = Path::new("./src/day01/numbers.txt");
    let fileinput = fs::read_to_string(filepath).unwrap();
    let mut lines = fileinput.lines();
    let mut currentline = lines.next().unwrap_or_default();
    if currentline == "" {
        panic!("File Data Error");
    }

    while currentline != "" {
        // exit loop at end of input
        
        // break loop into int values
        let mut iter = currentline.split_whitespace();
        let str1 = iter.next().unwrap().parse::<i32>().unwrap();
        let str2 = iter.next().unwrap().parse::<i32>().unwrap();

        // push values to vectors
        v1.push(str1);
        v2.push(str2);

        currentline = lines.next().unwrap_or_default();
    }

    // thank you rust for having built-in sorting
    // i am lazy and did not want to implement that
    v1.sort();
    v2.sort();

    // looping to find and sum distances
    let mut sum = 0;
    let mut index = 0;
    for i in v1 {
        let diff = (i - v2[index]).abs();
        sum = sum + diff;
        index = index + 1;
    }

    println!("Total list distance: {}", sum);

}