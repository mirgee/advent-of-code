use itertools::Itertools;
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
enum Item {
    Space,
    Galaxy,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Item>>> {
    separated_list1(
        line_ending,
        many1(alt((
            complete::tag(".").map(|_| Item::Space),
            complete::tag("#").map(|_| Item::Galaxy),
        ))),
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (input, output) = parse_input(input).unwrap();
    let arr = Array2::from_shape_vec(
        (output.len(), output[0].len()),
        output.iter().flatten().collect(),
    )
    .unwrap();
    let empty_rows_idx = arr
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_i, row)| row.iter().all(|&x| x == &Item::Space))
        .map(|(i, _row)| i)
        .collect::<Vec<_>>();
    let empty_col_idx = arr
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_i, col)| col.iter().all(|&x| x == &Item::Space))
        .map(|(i, _col)| i)
        .collect::<Vec<_>>();
    let galaxy_idx = arr
        .indexed_iter()
        .filter(|((_row, _col), &x)| x == &Item::Galaxy)
        .map(|((row, col), _)| (row, col))
        .collect::<Vec<_>>();
    // TODO: Map galaxy coordinates to expanded coordinates
    let galaxy_idx = galaxy_idx
        .iter()
        .map(|(row, col)| {
            (
                row + empty_rows_idx.iter().filter(|&&x| x < *row).count(),
                col + empty_col_idx.iter().filter(|&&x| x < *col).count(),
            )
        })
        .collect::<Vec<_>>();
    let distances = galaxy_idx.iter().combinations(2).map(|x| {
        let (row1, col1) = x[0].clone();
        let (row2, col2) = x[1].clone();
        let row_diff = (row1 as isize - row2 as isize).abs();
        let col_diff = (col1 as isize - col2 as isize).abs();
        row_diff as u64 + col_diff as u64
    });
    Ok(distances.sum())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        374
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
