use std::fmt::Display;

use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    RoundRock,
    CubeRock,
    #[default]
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::RoundRock => write!(f, "O"),
            Tile::CubeRock => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

struct PrintableArray2<'a, T>(&'a Array2<T>);

impl<'a> Display for PrintableArray2<'a, Tile> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.outer_iter() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        newline,
        many1(one_of(".#O")).map(|line| {
            line.into_iter()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::CubeRock,
                    'O' => Tile::RoundRock,
                    _ => unreachable!(),
                })
                .collect()
        }),
    )(input)
}

fn tilt_matrix(mut matrix: Array2<Tile>) -> Array2<Tile> {
    // Iterate over columns, in each column, count RoundRocks between each pair of CubeRocks and move them after
    // the most recent CubeRock (or the start if no CubeRock preceded)
    let mut icol = 0;
    while icol < matrix.shape()[1] {
        let mut irow = 0;
        let mut last_cube_rock_row = 0;
        while irow < matrix.shape()[0] {
            match matrix[(irow, icol)] {
                Tile::RoundRock => {
                    // if last_cube_rock_row != matrix.shape()[1] {
                    matrix[(last_cube_rock_row, icol)] = Tile::RoundRock;
                    if last_cube_rock_row != irow {
                        matrix[(irow, icol)] = Tile::Empty;
                    }
                    // }
                    last_cube_rock_row += 1;
                }
                Tile::CubeRock => {
                    last_cube_rock_row = irow + 1;
                }
                Tile::Empty => {}
            }
            irow += 1;
        }
        icol += 1;
    }
    matrix
}

fn calculate_load(matrix: &Array2<Tile>) -> usize {
    let mut load = 0;
    let dim = matrix.dim();
    for (idx, tile) in matrix.indexed_iter() {
        if *tile == Tile::RoundRock {
            load += dim.1 - idx.0;
        }
    }
    load
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, tiles) = parse_input(input).unwrap();
    let matrix = Array2::from_shape_vec(
        (tiles.len(), tiles[0].len()),
        tiles.into_iter().flatten().collect(),
    )
    .unwrap();
    let tilted = tilt_matrix(matrix);
    Ok(calculate_load(&tilted))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        136
    )]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: usize) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
    )]
    #[test_log::test]
    fn test_tilt(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
        let (_, tiles) = parse_input(input).unwrap();
        let input_matrix = Array2::from_shape_vec(
            (tiles.len(), tiles[0].len()),
            tiles.into_iter().flatten().collect(),
        )
        .unwrap();
        let (_, tiles) = parse_input(input).unwrap();
        let output_matrix_expected = Array2::from_shape_vec(
            (tiles.len(), tiles[0].len()),
            parse_input(output)
                .unwrap()
                .1
                .into_iter()
                .flatten()
                .collect(),
        )
        .unwrap();
        let output_matrix = tilt_matrix(input_matrix);
        assert_eq!(output_matrix, output_matrix_expected);
        Ok(())
    }
}
