use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn insert_or_update(
    map: &mut HashMap<usize, usize>,
    key: usize,
    update_by: usize,
    card_num: usize,
) {
    // println!(
    //     "Updating {} from card number {} by {}",
    //     key, card_num, update_by
    // );
    match map.get_mut(&key) {
        Some(val) => {
            *val += update_by;
        }
        None => {
            map.insert(key, update_by);
        }
    }
}

fn main() {
    let mut sum = 0;
    let mut cards_map: HashMap<usize, usize> = HashMap::new();
    let lines: Vec<String> = read_lines("./input.txt")
        .unwrap()
        .into_iter()
        .map(|l| l.unwrap())
        .collect();
    for (i, line) in lines.iter().enumerate() {
        let card_num = i + 1;
        insert_or_update(&mut cards_map, card_num, 1, card_num);
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
        let mut matches: usize = 0;
        for number in &my_numbers {
            if winning_numbers.contains(number) {
                // println!("Matched number {} on card {}", number, card_num);
                matches += 1;
            }
        }
        let copies_of_this_card = cards_map.get(&card_num).unwrap().clone();
        for card_copy_num in 1..matches + 1 {
            if card_num + card_copy_num <= lines.len() {
                insert_or_update(
                    &mut cards_map,
                    card_num + card_copy_num,
                    copies_of_this_card,
                    card_num,
                );
            }
        }
    }
    // println!("Cards map: {:?}", cards_map.iter());
    println!("Sum: {}", cards_map.values().sum::<usize>());
}
