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
    let mut sum = 0;
    for (i, line) in read_lines("./src/input.txt")
        .unwrap()
        .into_iter()
        .enumerate()
    {
        if let Ok(line) = line {
            if is_possible(line) {
                println!("Game {} is possible", i + 1);
                sum += i + 1;
            } else {
                println!("Game {} is impossible", i + 1);
            }
        } else {
            panic!("Shouldn't happen");
        }
    }
    print!("{:?}", sum);
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_possible(line: String) -> bool {
    let reg = Regex::new(r"^Game \d+: ").unwrap();
    let stripped = reg.replace_all(&line, "");
    // println!("stripped: {}", stripped);
    for game in stripped.split("; ") {
        // println!("game: {}", game);
        for draw in game.split(", ") {
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
