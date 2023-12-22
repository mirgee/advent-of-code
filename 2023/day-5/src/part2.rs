use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::error::Error;
use std::fs;
use std::iter::Iterator;
use std::ops::Range;

#[derive(Debug, Clone)]
struct MapEntry {
    source_range: Range<u64>,
    destination_range: Range<u64>,
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
        source_range: source..source + range,
        destination_range: destination..destination + range,
    })
}

fn map_source_to_destination(map: &Vec<MapEntry>, value: u64) -> u64 {
    for entry in map.iter() {
        if value >= entry.source_range.start && value < entry.source_range.end {
            let res = entry.destination_range.start + value - entry.source_range.start;
            return res;
        }
    }
    return value;
}

fn seeds_to_ranges(seeds: Vec<u64>) -> Vec<Range<u64>> {
    let mut res = Vec::new();
    for (start, range) in seeds.into_iter().tuples() {
        res.push(start..start + range);
    }
    res
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;
    let sections: Vec<&str> = content.split("\n\n").collect();

    let seeds: Vec<Range<u64>> = seeds_to_ranges(
        sections[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect(),
    );

    let mut maps: Vec<Vec<MapEntry>> = Vec::new();
    for section in sections.iter().skip(1) {
        let mut parsed_section: Vec<MapEntry> = section
            .lines()
            .skip(1)
            .filter_map(|line| parse_map_entry(line).ok())
            .collect();
        parsed_section.sort_by_key(|entry| entry.source_range.start);
        maps.push(parsed_section);
    }

    let count = seeds.iter().map(|r| r.end - r.start).sum::<u64>();

    let min_location = seeds
        .into_par_iter()
        .flatten()
        .progress_count(count)
        .map(|seed: u64| -> u64 {
            maps.iter()
                .fold(seed, |dest, map| map_source_to_destination(map, dest))
        })
        .min()
        .unwrap();
    println!("Minimum of destinations: {}", min_location);
    Ok(())
}
