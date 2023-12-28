use std::collections::HashMap;

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

fn find_vertical_reflection_axis(matrix: &Array2<bool>) -> Option<u64> {
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

fn find_horizontal_reflection_axis(matrix: &Array2<bool>) -> Option<u64> {
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
fn find_vertical_reflection_axes(matrix: &Array2<bool>) -> Vec<u64> {
    let mut axes = vec![];
    'outer: for i in 0..matrix.shape()[1] - 1 {
        let mut j = 0;
        while i >= j && i + j + 1 < matrix.shape()[1] {
            if matrix.column(i - j) != matrix.column(i + j + 1) {
                continue 'outer;
            }
            j += 1;
        }
        axes.push(i as u64);
    }
    return axes;
}

fn find_horizontal_reflection_axes(matrix: &Array2<bool>) -> Vec<u64> {
    let mut axes = vec![];
    'outer: for i in 0..matrix.shape()[0] - 1 {
        let mut j = 0;
        while i >= j && i + j + 1 < matrix.shape()[0] {
            if matrix.row(i - j) != matrix.row(i + j + 1) {
                continue 'outer;
            }
            j += 1;
        }
        axes.push(i as u64);
    }
    return axes;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, output) = parse_input(input).unwrap();
    let mut matrices = vec![];
    for m in output.iter() {
        matrices.push(
            Array2::from_shape_vec(
                (m.len(), m[0].len()),
                m.iter().flatten().map(|v| *v).collect(),
            )
            .unwrap(),
        );
    }

    let mut old_axes = HashMap::new();
    for (idx, matrix) in matrices.iter().enumerate() {
        if let Some(axis) = find_vertical_reflection_axis(&matrix) {
            // println!("Found vertical reflection axis at {}", axis);
            old_axes.insert(idx, ('V', axis));
            continue;
        }
        if let Some(axis) = find_horizontal_reflection_axis(&matrix) {
            // println!("Found horizontal reflection axis at {}", axis);
            old_axes.insert(idx, ('H', axis));
            continue;
        }
        println!("No reflection axis found");
    }

    println!("Old axes: {:?}", old_axes);
    let mut sum = 0;
    'outer: for (matrix_idx, mut matrix) in matrices.into_iter().enumerate() {
        for i in 0..matrix.shape()[0] {
            for j in 0..matrix.shape()[1] {
                println!("FLIPPING {} {}", i, j);
                let mut matrix_new = matrix.clone();
                let el = matrix_new.get_mut((i, j)).unwrap();
                *el = !*el;
                if let Some(axis) = find_vertical_reflection_axis(&matrix_new) {
                    println!("Found vertical reflection axis at {}", axis);
                    let (old_axis_orientation, old_axis) = old_axes.get(&matrix_idx).unwrap();
                    if !(*old_axis_orientation == 'V' && *old_axis == axis) {
                        sum += axis + 1;
                        continue 'outer;
                    }
                }
                if let Some(axis) = find_horizontal_reflection_axis(&matrix_new) {
                    println!("Found horizontal reflection axis at {}", axis);
                    let (old_axis_orientation, old_axis) = old_axes.get(&matrix_idx).unwrap();
                    if !(*old_axis_orientation == 'H' && *old_axis == axis) {
                        sum += 100 * (axis + 1);
                        continue 'outer;
                    }
                }
            }
        }
        println!("No reflection axis found for matrix {}", matrix_idx);
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
        400
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
        1400
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
        5
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
        10
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: u64) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(
        "..####..##..##..#
...#..##.####.##.
.##.#.##..##..##.
...#..##.####.##.
.##..#..#....#..#
##.##.##########.
#########.##.####",
        Some(10)
    )]
    #[case(
        "###.##.##
##.####.#
##.#..#.#
####..###
....##...
##.#..#.#
...#..#..
##.####.#
##......#
##......#
..#.##.#.
...#..#..
##.####.#
....##...
...####..
....##...
##.####.#",
        Some(4)
    )]
    fn test_find_vertical_reflection_axis(#[case] input: &str, #[case] output: Option<u64>) {
        let (_, matrices) = parse_input(input).unwrap();
        let matrix = Array2::from_shape_vec(
            (matrices[0].len(), matrices[0][0].len()),
            matrices[0].iter().flatten().map(|v| *v).collect(),
        )
        .unwrap();
        let axes = find_vertical_reflection_axes(&matrix);
        println!("Axes: {:?}", axes);
        // assert_eq!(output, find_vertical_reflection_axis(&matrix));
    }
}
