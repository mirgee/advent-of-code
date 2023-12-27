use ndarray::Array2;
use nom::{
    character::complete::{multispace1, newline, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Vec<bool>>>> {
    separated_list1(
        multispace1,
        separated_list1(
            newline,
            many1(one_of(".#")).map(|line| {
                line.into_iter()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            }),
        ),
    )(input)
}

fn is_reflected_across_column(matrix: &Array2<&bool>, column: usize) -> bool {
    let mut i = 1;
    while column >= i && column + i < matrix.shape()[1] {
        if matrix.column(column - i) != matrix.column(column + i) {
            return false;
        }
        i += 1;
    }
    return true;
}

fn is_reflected_across_row(matrix: &Array2<&bool>, row: usize) -> bool {
    let mut i = 1;
    while row >= i && row + i < matrix.shape()[0] {
        if matrix.row(row - i) != matrix.row(row + i) {
            return false;
        }
        i += 1;
    }
    return true;
}

fn find_reflection_axis_column(matrix: &Array2<&bool>) -> Option<usize> {
    for i in 1..matrix.shape()[1] {
        if is_reflected_across_column(matrix, i) {
            return Some(i);
        }
    }
    None
}

fn find_reflection_axis_row(matrix: &Array2<&bool>) -> Option<usize> {
    for i in 1..matrix.shape()[0] {
        if is_reflected_across_row(matrix, i) {
            return Some(i);
        }
    }
    None
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let mut matrices = vec![];
    for m in output.iter() {
        matrices.push(
            Array2::from_shape_vec((m.len(), m[0].len()), m.iter().flatten().collect()).unwrap(),
        );
    }
    println!("{:?}", matrices[0]);
    println!("{:?}", find_reflection_axis_column(&matrices[0]));
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        405
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
