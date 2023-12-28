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

fn find_vertical_reflection_axis(matrix: &Array2<&bool>) -> Option<u64> {
    'outer: for i in 0..matrix.shape()[1] - 1 {
        let mut j = 0;
        while i >= j && i + j + 1 < matrix.shape()[1] {
            if matrix.column(i - j) != matrix.column(i + j + 1) {
                continue 'outer;
            }
            j += 1;
        }
        return Some(i as u64);
    }
    return None;
}

fn find_horizontal_reflection_axis(matrix: &Array2<&bool>) -> Option<u64> {
    'outer: for i in 0..matrix.shape()[0] - 1 {
        let mut j = 0;
        while i >= j && i + j + 1 < matrix.shape()[0] {
            if matrix.row(i - j) != matrix.row(i + j + 1) {
                continue 'outer;
            }
            j += 1;
        }
        return Some(i as u64);
    }
    return None;
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
    let mut sum = 0;
    for matrix in matrices {
        if let Some(axis) = find_vertical_reflection_axis(&matrix) {
            // println!("Found vertical reflection axis at {}", axis);
            sum += axis + 1;
            continue;
        }
        if let Some(axis) = find_horizontal_reflection_axis(&matrix) {
            // println!("Found horizontal reflection axis at {}", axis);
            sum += 100 * (axis + 1);
            continue;
        }
        println!("No reflection axis found");
    }
    return Ok(sum);
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
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.",
        709
    )]
    #[case(
        "###.##.##
##.####.#
##.#..#.#
####..###
....##...
##.#..#.#
...#..#..
##..###.#
##......#
##......#
..#.##.#.
...#..#..
##.####.#
....##...
...####..
....##...
##.####.#",
        1
    )]
    #[case(
        ".##.##...##...##.
#####..##..##..##
.....##..##..##..
.##.#.#.####.#.#.
.##...#.#..#.#...
....#..........#.
#..#..#......#..#
....###.....####.
.##...#.#..#.#...
.....#..####..#..
#..#...##..##...#
....#...#..#...#.
#..#.##########.#
#..##...####...##
#####.##.##.##.##",
        2
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
