use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        line_ending,
        many1(alt((
            complete::tag("-").map(|_| Tile::Horizontal),
            complete::tag("|").map(|_| Tile::Vertical),
            complete::tag("L").map(|_| Tile::NorthEast),
            complete::tag("J").map(|_| Tile::NorthWest),
            complete::tag("7").map(|_| Tile::SouthWest),
            complete::tag("F").map(|_| Tile::SouthEast),
            complete::tag("S").map(|_| Tile::Start),
            complete::tag(".").map(|_| Tile::Ground),
        ))),
    )(input)
}

fn find_next_pos_from_start(
    arr: &Array2<&Tile>,
    (curr_row, curr_col): (usize, usize),
) -> (usize, usize) {
    for (dir_row, dir_col) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (trying_row, trying_col) = (curr_row as isize + dir_row, curr_col as isize + dir_col);
        if trying_row < 0 || trying_col < 0 {
            continue;
        }
        let (trying_row, trying_col) = (trying_row as usize, trying_col as usize);
        if let Some(tile) = arr.get((trying_row, trying_col)) {
            // TODO: This assumes we can move from start to any pipe surrounding it
            // which is not necessarily true
            if *tile != &Tile::Ground {
                return (trying_row, trying_col);
            }
        }
    }
    panic!("No pipe after starting position found")
}

fn find_next_pos(
    arr: &Array2<&Tile>,
    (curr_row, curr_col): (usize, usize),
    (prev_row, prev_col): (usize, usize),
) -> (usize, usize) {
    // println!("Current position: {:?}", (curr_row, curr_col));
    // println!("Previous position: {:?}", (prev_row, prev_col));
    // println!("Current tile: {:?}", arr[(curr_row, curr_col)]);
    match arr[(curr_row, curr_col)] {
        Tile::Vertical => {
            if curr_row > prev_row {
                return (curr_row + 1, curr_col);
            } else if curr_row < prev_row {
                return (curr_row - 1, curr_col);
            } else {
                panic!("Invalid position: Vertical")
            }
        }
        Tile::Horizontal => {
            if curr_col > prev_col {
                return (curr_row, curr_col + 1);
            } else if curr_col < prev_col {
                return (curr_row, curr_col - 1);
            } else {
                panic!("Invalid position: Horizontal")
            }
        }
        Tile::NorthEast => {
            if curr_row > prev_row {
                // We came from the north
                return (curr_row, curr_col + 1);
            } else if curr_col < prev_col {
                // We came from the east
                return (curr_row - 1, curr_col);
            } else {
                panic!("Invalid position: NE")
            }
        }
        Tile::NorthWest => {
            if curr_row > prev_row {
                // We came from the north
                return (curr_row, curr_col - 1);
            } else if curr_col > prev_col {
                // We came from the west
                return (curr_row - 1, curr_col);
            } else {
                panic!("Invalid position: NW")
            }
        }
        Tile::SouthWest => {
            if curr_row < prev_row {
                // We came from the south
                return (curr_row, curr_col - 1);
            } else if curr_col > prev_col {
                // We came from the west
                return (curr_row + 1, curr_col);
            } else {
                panic!("Invalid position: SW")
            }
        }
        Tile::SouthEast => {
            if curr_row < prev_row {
                // We came from the south
                return (curr_row, curr_col + 1);
            } else if curr_col < prev_col {
                // We came from the east
                return (curr_row + 1, curr_col);
            } else {
                panic!("Invalid position SE")
            }
        }
        Tile::Ground => {
            panic!("Invalid position: Ground")
        }
        Tile::Start => {
            panic!("Invalid position: Start")
        }
    }
}

fn count_interior_points(path: &[(isize, isize)]) -> isize {
    // Shoelace formula
    let mut area: isize = 0;
    for i in 0..path.len() {
        let (y1, x1) = path[i];
        let (y2, x2) = path[(i + 1) % path.len()];
        area += (x1 * y2) - (x2 * y1);
    }
    area = area.abs() / 2;

    // Pick's theorem
    area - (path.len() as isize / 2) + 1
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<isize, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let arr = Array2::from_shape_vec(
        (output.len(), output[0].len()),
        output.iter().flatten().collect(),
    )
    .unwrap();
    let (start_row, start_col) = arr
        .indexed_iter()
        .find(|((_, _), &tile)| *tile == Tile::Start)
        .unwrap()
        .0;
    // println!("Input: {:?}", arr);
    // println!("Starting position: {:?}", (start_row, start_col));
    let (mut curr_row, mut curr_col) = find_next_pos_from_start(&arr, (start_row, start_col));
    let (mut prev_row, mut prev_col) = (start_row, start_col);
    let mut path = vec![(start_row as isize, start_col as isize)];
    while (curr_row, curr_col) != (start_row, start_col) {
        path.push((curr_row as isize, curr_col as isize));
        let (new_row, new_col) = find_next_pos(&arr, (curr_row, curr_col), (prev_row, prev_col));
        (prev_row, prev_col) = (curr_row, curr_col);
        (curr_row, curr_col) = (new_row, new_col);
    }
    Ok(count_interior_points(&path))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: isize) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
