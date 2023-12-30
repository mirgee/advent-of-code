use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    MirrorFw,
    MirrorBw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(&self, (row, col): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    direction: Direction,
    position: (usize, usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Vertical => write!(f, "|"),
            Tile::Horizontal => write!(f, "-"),
            Tile::MirrorFw => write!(f, "/"),
            Tile::MirrorBw => write!(f, "\\"),
        }
    }
}

impl Display for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.direction, self.position) {
            (Direction::Up, x) => write!(f, "^{:?}", x),
            (Direction::Down, x) => write!(f, "v{:?}", x),
            (Direction::Left, x) => write!(f, "<{:?}", x),
            (Direction::Right, x) => write!(f, ">{:?}", x),
        }
    }
}

fn print_matrix(matrix: &Array2<Tile>, energed: &HashSet<(usize, usize)>) {
    for (row_idx, row) in matrix.outer_iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if energed.contains(&(row_idx, col_idx)) {
                print!("#");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
    println!();
}

fn print_beams(beams: &Vec<Beam>) {
    for beam in beams {
        print!("{}, ", beam);
    }
    println!()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        newline,
        many1(one_of(r#".|-/\"#).map(|c| match c {
            '.' => Tile::Empty,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '/' => Tile::MirrorFw,
            '\\' => Tile::MirrorBw,
            _ => unreachable!(),
        })),
    )(input)
}

fn step_beam(beam: Beam, matrix: &Array2<Tile>) -> (Option<Beam>, Option<Beam>) {
    if beam.direction == Direction::Up && beam.position.0 == 0
        || beam.direction == Direction::Left && beam.position.1 == 0
        || beam.direction == Direction::Down && beam.position.0 == matrix.shape()[0] - 1
        || beam.direction == Direction::Right && beam.position.1 == matrix.shape()[1] - 1
    {
        return (None, None);
    }

    let next_position = beam.direction.step(beam.position);

    match (matrix.get(next_position).unwrap(), beam.direction) {
        (Tile::Empty, _)
        | (Tile::Vertical, Direction::Up)
        | (Tile::Vertical, Direction::Down)
        | (Tile::Horizontal, Direction::Left)
        | (Tile::Horizontal, Direction::Right) => (
            Some(Beam {
                direction: beam.direction,
                position: next_position,
            }),
            None,
        ),
        (Tile::Vertical, _) => (
            Some(Beam {
                direction: Direction::Up,
                position: next_position,
            }),
            Some(Beam {
                direction: Direction::Down,
                position: next_position,
            }),
        ),
        (Tile::Horizontal, _) => (
            Some(Beam {
                direction: Direction::Left,
                position: next_position,
            }),
            Some(Beam {
                direction: Direction::Right,
                position: next_position,
            }),
        ),
        (Tile::MirrorFw, Direction::Up) => (
            Some(Beam {
                direction: Direction::Right,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorFw, Direction::Down) => (
            Some(Beam {
                direction: Direction::Left,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorFw, Direction::Left) => (
            Some(Beam {
                direction: Direction::Down,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorFw, Direction::Right) => (
            Some(Beam {
                direction: Direction::Up,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorBw, Direction::Up) => (
            Some(Beam {
                direction: Direction::Left,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorBw, Direction::Down) => (
            Some(Beam {
                direction: Direction::Right,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorBw, Direction::Left) => (
            Some(Beam {
                direction: Direction::Up,
                position: next_position,
            }),
            None,
        ),
        (Tile::MirrorBw, Direction::Right) => (
            Some(Beam {
                direction: Direction::Down,
                position: next_position,
            }),
            None,
        ),
    }
}

fn get_initial_beam(matrix: &Array2<Tile>) -> Beam {
    let position = (0, 0);
    let tile = matrix.get((0, 0)).unwrap();
    let direction = match tile {
        Tile::Empty => Direction::Right,
        Tile::Vertical => Direction::Down,
        Tile::Horizontal => Direction::Right,
        Tile::MirrorFw => Direction::Up,
        Tile::MirrorBw => Direction::Down,
    };
    Beam {
        direction,
        position,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, parsed) = parse_input(input).unwrap();
    let matrix = Array2::from_shape_vec(
        (parsed.len(), parsed[0].len()),
        parsed.into_iter().flatten().collect(),
    )
    .unwrap();
    let mut beams = vec![get_initial_beam(&matrix)];
    let mut seen_beams = HashSet::new();
    let mut energized = HashSet::new();
    energized.insert((0, 0));
    seen_beams.insert(get_initial_beam(&matrix));
    while !beams.is_empty() {
        let current_bream = beams.pop().unwrap();
        let (new_beam1, new_beam2) = step_beam(current_bream, &matrix);
        if let Some(new_beam1) = new_beam1 {
            energized.insert(new_beam1.position);
            if !seen_beams.contains(&new_beam1) {
                beams.push(new_beam1);
                seen_beams.insert(new_beam1);
            }
        }
        if let Some(new_beam2) = new_beam2 {
            energized.insert(new_beam2.position);
            if !seen_beams.contains(&new_beam2) {
                beams.push(new_beam2);
                seen_beams.insert(new_beam2);
            }
        }
        // print_matrix(&matrix, &energized);
        // print_beams(&beams);
    }
    Ok(energized.len() as u64)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
        46
    )]
    #[case(
        r#"..|..
.....
..-.."#,
        9
    )]
    #[case(
        r#"...\...
.......
-......
.......
\../..."#,
        18
    )]
    #[case(
        r#"|....-
......
......
-....|"#,
        16
    )]
    #[case(
        r#"......|...\..\...
..../........|...
....\.-.../......
......|....../...
................."#,
        41
    )]
    #[case(
        r#"\........-.........\................................|...
......-/.............|-.../.....|...........././..\.....
-.........................|.....\...................|.\.
.......-........../.......\.........|..../........-.-|.."#,
        89
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
