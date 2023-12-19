use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

const RED_LIMIT: usize = 12;
const GREEN_LIMIT: usize = 13;
const BLUE_LIMIT: usize = 14;

fn main() {
    let mut sum_idx = 0;
    let mut sum_pow = 0;
    for (i, line) in read_lines("./src/input.txt")
        .unwrap()
        .into_iter()
        .enumerate()
    {
        if let Ok(line) = line {
            let pow = compute_power(&line);
            println!("The power of game {} is {}", i + 1, pow);
            sum_pow += pow;
            if is_possible(&line) {
                sum_idx += i + 1;
            } else {
                println!("Game {} is impossible", i + 1);
            }
        } else {
            panic!("Shouldn't happen");
        }
    }
    println!("Sum of indexes of possible games {:?}", sum_idx);
    println!("Sum of powers of all games {:?}", sum_pow);
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_possible(line: &String) -> bool {
    let reg = Regex::new(r"^Game \d+: ").unwrap();
    let stripped = reg.replace_all(line, "");
    // println!("stripped: {}", stripped);
    for round in stripped.split("; ") {
        // println!("game: {}", game);
        for draw in round.split(", ") {
            // println!("draw: {}", draw);
            let mut split = draw.split(" ");
            // println!("split: {}", draw);
            let number = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" if number > RED_LIMIT => return false,
                "green" if number > GREEN_LIMIT => return false,
                "blue" if number > BLUE_LIMIT => return false,
                _ => {}
            }
        }
    }
    return true;
}

fn compute_power(line: &String) -> usize {
    let reg = Regex::new(r"^Game \d+: ").unwrap();
    let stripped = reg.replace_all(line, "");
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for round in stripped.split("; ") {
        for draw in round.split(", ") {
            let mut split = draw.split(" ");
            let number = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => {
                    max_red = std::cmp::max(max_red, number);
                }
                "green" => {
                    max_green = std::cmp::max(max_green, number);
                }
                "blue" => {
                    max_blue = std::cmp::max(max_blue, number);
                }
                _ => {
                    panic!("shouldn't happen");
                }
            }
        }
    }
    max_red * max_green * max_blue
}
