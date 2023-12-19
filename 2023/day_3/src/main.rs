use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn is_symbol(c: char) -> bool {
    if !c.is_digit(10) && c != '.' {
        true
    } else {
        false
    }
}

fn is_symbol_around(pos_row: usize, pos_col: usize, arr: &Vec<Vec<char>>) -> bool {
    let dirs = [
        [-1, 0],
        [0, -1],
        [0, 1],
        [1, 0],
        [-1, -1],
        [1, 1],
        [1, -1],
        [-1, 1],
    ];
    for dir in dirs {
        let y: i32 = pos_row as i32 + dir[0];
        let x: i32 = pos_col as i32 + dir[1];
        if x >= 0 && y >= 0 && (y as usize) < arr.len() && (x as usize) < arr[0].len() {
            let x = x as usize;
            let y = y as usize;
            if is_symbol(arr[y][x]) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut current_number = String::new();
        let mut is_part = false;
        let mut is_number = false;

        for (col, c) in line.iter().enumerate() {
            is_number = c.is_digit(10);

            if !is_number && is_part && !current_number.is_empty() {
                println!("Current number: {}", current_number);
                sum += current_number.parse::<usize>().unwrap();
                is_part = false;
            }

            if is_number {
                current_number += c.to_string().as_str();
                is_part = is_part || is_symbol_around(row, col, &lines);
            } else {
                current_number = String::new();
            }
        }

        if is_number && is_part {
            sum += current_number.parse::<usize>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}
