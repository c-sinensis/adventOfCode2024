// Advent of Code Day 2
use std::fs;
use std::path::Path;

pub fn part1() {
    // getting input
    let dataset: Vec<String> = get_data();
    // output value
    let mut safe_sets = 0;

    // iterating through each line
    for i in &dataset{

        // converting line to vector of int values
        let mut line_vector: Vec<i8> = Vec::new();
        let mut iter = i.split_whitespace();
        let mut currentval = iter.next().unwrap_or_default();
        if currentval == "" {
            panic!("File Data Error -- Invalid")
        }

        // parse line, push values to line vector
        while currentval != "" {
            let datapt = currentval.parse::<i8>().unwrap_or_default();
            if datapt == 0 {
                panic!("File Data Error -- Invalid");
            }
            line_vector.push(datapt);
            currentval = iter.next().unwrap_or_default();
        }

        // innocent until proven guilty
        let mut safe_line: bool = true;

        // interval test
        if interval_test(&line_vector) == false {
            safe_line = false;
        }

        // skipping second test if first fails, allows simpler logic in second test
        if safe_line == false {
            continue;
        }

        // sign flip test
        if sign_test(&line_vector) == false {
            safe_line = false;
        }

        if safe_line == true {
            safe_sets = safe_sets + 1;
        }
    }

    println!("Number of safe reports: {}", safe_sets);
}

pub fn part2() {
    // getting input
    let dataset: Vec<String> = get_data();
    // output value
    let mut safe_sets = 0;

    // iterating through each line
    for i in &dataset{

        // converting line to vector of int values
        let mut line_vector: Vec<i8> = Vec::new();
        let mut iter = i.split_whitespace();
        let mut currentval = iter.next().unwrap_or_default();
        if currentval == "" {
            panic!("File Data Error -- Invalid")
        }

        // parse line, push values to line vector
        while currentval != "" {
            let datapt = currentval.parse::<i8>().unwrap_or_default();
            if datapt == 0 {
                panic!("File Data Error -- Invalid");
            }
            line_vector.push(datapt);
            currentval = iter.next().unwrap_or_default();
        }

        // innocent until proven guilty
        let mut safe_line: bool = true;

        // interval test
        if interval_test(&line_vector) == false {
            safe_line = false;
        }

        // sign flip test
        if sign_test(&line_vector) == false {
            safe_line = false;
        }

        
        // now to test vectors with removed elements
        if safe_line == false {
            let mut rm_idx = 0;
            while rm_idx < line_vector.len() {
                // remove nth element 
                let mut rm_lv = line_vector.clone();
                rm_lv.remove(rm_idx);

                if (interval_test(&rm_lv) == true) && (sign_test(&rm_lv) == true) {
                    safe_line = true;
                    break;
                }
                rm_idx = rm_idx + 1;
            }
        }
        

        if safe_line == true {
            safe_sets = safe_sets + 1;
        }
    }

    println!("Number of safe reports: {}", safe_sets);
}


fn get_data() -> Vec<String> {
    let mut out_vector: Vec<String> = Vec::new(); 
    // read from file
    let filepath = Path::new("./src/day02/data.txt");
    let fileinput = fs::read_to_string(filepath).unwrap();
    let mut lines = fileinput.lines();
    let mut currentline = lines.next().unwrap_or_default();
    if currentline == "" {
        panic!("File Data Error -- Empty");
    }

    while currentline != "" {
        // exit loop at end of input

        // push line vector to output vector
        out_vector.push(currentline.to_string());

        // get next line
        currentline = lines.next().unwrap_or_default();
    }

    // return output vector
    out_vector
}

fn interval_test(line_vector: &Vec<i8>) -> bool {
    let mut index = 0;
    while index < (line_vector.len() - 1) {
        if ((line_vector[index] - line_vector[index+1]).abs() > 3) || ((line_vector[index] - line_vector[index+1]).abs() < 1) {
            return false;
        }
        index = index + 1;
    }
    return true;
}

fn sign_test(line_vector: &Vec<i8>) -> bool {
    let inc: bool = if line_vector[0] < line_vector[1] { true } else { false };
    let mut index = 1;
    while  index < (line_vector.len() - 1) {
        // find sign of interval
        let interval_inc: bool = if line_vector[index] < line_vector[index + 1] { true } else { false };

        // compare with initial
        if interval_inc != inc {
            return false
        }
        index = index + 1;
    }
    return true;
}