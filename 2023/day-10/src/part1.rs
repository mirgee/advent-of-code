use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone)]
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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let arr = Array2::from_shape_vec(
        (output.len(), output[0].len()),
        output.iter().flatten().collect(),
    )
    .unwrap();
    println!("Input: {:?}", arr);
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
