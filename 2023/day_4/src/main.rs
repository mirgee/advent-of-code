use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum = 0;
    for (i, line) in read_lines("./input.txt").unwrap().into_iter().enumerate() {
        let line = line.unwrap();
        let card_parts: Vec<&str> = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" | ")
            .collect();
        let winning_numbers: HashSet<&str> = card_parts
            .get(0)
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        let my_numbers: Vec<&str> = card_parts
            .get(1)
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        let mut matches: u32 = 0;
        for number in &my_numbers {
            if winning_numbers.contains(number) {
                // println!("Matched number {}", number);
                matches += 1;
            }
        }
        // println!("Card {} matches: {}", i + 1, matches);
        if matches > 0 {
            let inc = 2_u32.pow(matches - 1);
            // println!("Points: {}", inc);
            sum += inc;
        }
    }
    println!("Sum: {}", sum);
}
