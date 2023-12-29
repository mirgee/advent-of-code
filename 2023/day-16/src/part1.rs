use std::fmt::{Display, Formatter};

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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Vertical => write!(f, "|"),
            Tile::Horizontal => write!(f, "-"),
            Tile::MirrorFw => write!(f, "/"),
            Tile::MirrorBw => write!(f, "\\"),
        }
    }
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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, parsed) = parse_input(input).unwrap();
    let matrix = Array2::from_shape_vec(
        (parsed.len(), parsed[0].len()),
        parsed.into_iter().flatten().collect(),
    )
    .unwrap();
    println!("{:?}", matrix);
    todo!()
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
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
