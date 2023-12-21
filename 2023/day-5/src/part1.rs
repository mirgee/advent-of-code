use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct MapEntry {
    source: u64,
    destination: u64,
    range: u64,
}

#[derive(Debug, Clone)]
struct Seeds {
    seeds: Vec<u64>,
    current_start: u64,
    range: u64,
    range_remaining: u64,
}

impl Seeds {
    pub fn new(mut seeds: Vec<u64>) -> Seeds {
        let current_start = seeds.remove(0);
        let range = seeds.remove(0);
        Seeds {
            seeds,
            current_start,
            range,
            range_remaining: range,
        }
    }
}

impl Iterator for Seeds {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range_remaining > 0 {
            let res = self.current_start + self.range - self.range_remaining;
            self.range_remaining -= 1;
            Some(res)
        } else {
            if self.seeds.is_empty() {
                None
            } else {
                let current_start = self.seeds.remove(0);
                let range = self.seeds.remove(0);
                self.current_start = current_start;
                self.range = range;
                self.range_remaining = range - 1;
                Some(current_start)
            }
        }
    }
}

fn parse_map_entry(line: &str) -> Result<MapEntry, Box<dyn Error>> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid input format".into());
    }

    let destination = parts[0].parse::<u64>()?;
    let source = parts[1].parse::<u64>()?;
    let range = parts[2].parse::<u64>()?;

    Ok(MapEntry {
        source,
        destination,
        range,
    })
}

fn map_source_to_destination(map: &Vec<MapEntry>, value: u64) -> u64 {
    for entry in map.iter() {
        if value >= entry.source && value < entry.source + entry.range {
            let res = entry.destination + value - entry.source;
            return res;
        }
    }
    return value;
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;
    let sections: Vec<&str> = content.split("\n\n").collect();

    let seeds: Vec<u64> = sections[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut maps: Vec<Vec<MapEntry>> = Vec::new();
    for section in sections.iter().skip(1) {
        let mut parsed_section: Vec<MapEntry> = section
            .lines()
            .skip(1)
            .filter_map(|line| parse_map_entry(line).ok())
            .collect();
        parsed_section.sort_by_key(|entry| entry.source);
        maps.push(parsed_section);
    }

    let mut dests: Vec<u64> = Vec::new();
    for seed in &seeds {
        let mut dest: u64 = *seed;
        for map in &maps {
            dest = map_source_to_destination(&map, dest);
        }
        // println!("Mapped seed {} to final destination {}", seed, dest);
        dests.push(dest);
    }

    println!("Minimum of destinations: {}", dests.iter().min().unwrap());

    // let actual_seeds = Seeds::new(seeds);
    // // println!("Actual seeds: {:?}", actual_seeds.collect::<Vec<_>>());

    // let mut dests: Vec<u64> = Vec::new();
    // for seed in actual_seeds {
    //     // println!("Mapping actual seed {}", seed);
    //     let mut dest: u64 = seed;
    //     for map in &maps {
    //         dest = map_source_to_destination(&map, dest);
    //     }
    //     // println!("Mapped actual seed {} to final destination {}", seed, dest);
    //     dests.push(dest);
    // }
    // println!(
    //     "Actual minimum of destinations: {}",
    //     dests.iter().min().unwrap()
    // );
    Ok(())
}
