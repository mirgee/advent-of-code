use std::{
    collections::{HashMap, HashSet},
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

fn add_stars_around(
    pos_row: usize,
    pos_col: usize,
    arr: &Vec<Vec<char>>,
    stars_set: &mut HashSet<[usize; 2]>,
) {
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
            if arr[y][x] == '*' {
                stars_set.insert([y, x]);
            }
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut gear_map = HashMap::<[usize; 2], Vec<usize>>::new();
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut current_number = String::new();
        let mut is_part = false;
        let mut stars_around: HashSet<[usize; 2]> = HashSet::new();
        let mut is_number = false;

        for (col, c) in line.iter().enumerate() {
            is_number = c.is_digit(10);

            if !is_number && is_part && !current_number.is_empty() {
                // println!("Current number: {}", current_number);
                let new_number = current_number.parse::<usize>().unwrap();
                sum += new_number;
                is_part = false;
                for star in &stars_around {
                    if let Some(val) = gear_map.get_mut(star) {
                        val.push(new_number);
                    } else {
                        gear_map.insert(*star, vec![new_number]);
                    }
                }
                stars_around.clear();
            }

            if is_number {
                current_number += c.to_string().as_str();
                is_part = is_part || is_symbol_around(row, col, &lines);
                add_stars_around(row, col, &lines, &mut stars_around);
            } else {
                current_number = String::new();
            }
        }

        if is_number && is_part {
            let new_number = current_number.parse::<usize>().unwrap();
            sum += new_number;
            for star in &stars_around {
                if let Some(val) = gear_map.get_mut(star) {
                    val.push(new_number);
                } else {
                    gear_map.insert(*star, vec![new_number]);
                }
            }
        }
    }
    println!("Sum number: {}", sum);

    let mut sum_stars = 0;
    for (_key, val) in gear_map {
        if val.len() == 2 {
            sum_stars += val[0] * val[1]
        }
    }
    println!("Sum stars: {}", sum_stars);
}
