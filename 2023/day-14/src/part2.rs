use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use ndarray::Array2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
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

fn tilt_north(mut matrix: Array2<Tile>) -> Array2<Tile> {
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

fn tilt_west(mut matrix: Array2<Tile>) -> Array2<Tile> {
    let mut irow = 0;
    while irow < matrix.shape()[0] {
        let mut icol = 0;
        let mut last_cube_rock_col = 0;
        while icol < matrix.shape()[1] {
            match matrix[(irow, icol)] {
                Tile::RoundRock => {
                    // if last_cube_rock_row != matrix.shape()[1] {
                    matrix[(irow, last_cube_rock_col)] = Tile::RoundRock;
                    if last_cube_rock_col != icol {
                        matrix[(irow, icol)] = Tile::Empty;
                    }
                    // }
                    last_cube_rock_col += 1;
                }
                Tile::CubeRock => {
                    last_cube_rock_col = icol + 1;
                }
                Tile::Empty => {}
            }
            icol += 1;
        }
        irow += 1;
    }
    matrix
}

fn tilt_south(mut matrix: Array2<Tile>) -> Array2<Tile> {
    let mut icol = 0;
    while icol < matrix.shape()[1] {
        let mut irow = (matrix.shape()[0] - 1) as isize;
        let mut last_cube_rock_row = (matrix.shape()[0] - 1) as isize;
        while irow >= 0 {
            match matrix[(irow as usize, icol as usize)] {
                Tile::RoundRock => {
                    // if last_cube_rock_row != matrix.shape()[1] {
                    matrix[(last_cube_rock_row as usize, icol as usize)] = Tile::RoundRock;
                    if last_cube_rock_row != irow {
                        matrix[(irow as usize, icol)] = Tile::Empty;
                    }
                    // }
                    last_cube_rock_row -= 1;
                }
                Tile::CubeRock => {
                    last_cube_rock_row = irow - 1;
                }
                Tile::Empty => {}
            }
            irow -= 1;
        }
        icol += 1;
    }
    matrix
}

fn tilt_east(mut matrix: Array2<Tile>) -> Array2<Tile> {
    let mut irow = 0;
    while irow < matrix.shape()[0] {
        let mut icol = (matrix.shape()[1] - 1) as isize;
        let mut last_cube_rock_col = (matrix.shape()[1] - 1) as isize;
        while icol >= 0 {
            match matrix[(irow as usize, icol as usize)] {
                Tile::RoundRock => {
                    // if last_cube_rock_row != matrix.shape()[1] {
                    matrix[(irow as usize, last_cube_rock_col as usize)] = Tile::RoundRock;
                    if last_cube_rock_col != icol {
                        matrix[(irow as usize, icol as usize)] = Tile::Empty;
                    }
                    // }
                    last_cube_rock_col -= 1;
                }
                Tile::CubeRock => {
                    last_cube_rock_col = icol - 1;
                }
                Tile::Empty => {}
            }
            icol -= 1;
        }
        irow += 1;
    }
    matrix
}

fn perform_cycle(matrix: Array2<Tile>) -> Array2<Tile> {
    let tilted = tilt_north(matrix);
    // println!("Tilted North:\n{}", PrintableArray2(&tilted));
    let tilted = tilt_west(tilted);
    // println!("Tilted West:\n{}", PrintableArray2(&tilted));
    let tilted = tilt_south(tilted);
    // println!("Tilted South:\n{}", PrintableArray2(&tilted));
    let tilted = tilt_east(tilted);
    // println!("Tilted East:\n{}", PrintableArray2(&tilted));
    tilted
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

    let mut cache = HashMap::new();

    let progress_bar = indicatif::ProgressBar::new(1000000000);
    let mut tilted = matrix.clone();
    let mut rotations = 0;
    while rotations <= 1000000000 {
        tilted = perform_cycle(tilted);
        rotations += 1;
        if let Some(value) = cache.get(&tilted) {
            println!(
                "Found a cycle after {} rotations, matches with {:?}",
                rotations, value
            );
            println!("Loads: {:?}", cache.values());
            break;
        } else {
            let load = calculate_load(&tilted);
            cache.insert(tilted.clone(), (rotations, load));
            progress_bar.set_position(rotations);
        }
    }
    let (rot_num, load) = cache
        .values()
        .find(|(rotation_num, _)| 1000000000 % (rotations - 1) == *rotation_num)
        .unwrap();
    println!(
        "The final state corresponds to rotation {} with load {}",
        rot_num, load
    );
    Ok(*load)
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
        64
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
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
    )]
    #[test_log::test]
    fn test_cycle_1(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
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
        let output_matrix = perform_cycle(input_matrix);
        assert_eq!(output_matrix, output_matrix_expected);
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
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
    )]
    #[test_log::test]
    fn test_cycle_2(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
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
        let mut output_matrix = input_matrix.clone();
        for _ in 0..2 {
            output_matrix = perform_cycle(output_matrix);
        }
        assert_eq!(output_matrix, output_matrix_expected);
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
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
    )]
    #[test_log::test]
    fn test_cycle_3(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
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
        let mut output_matrix = input_matrix.clone();
        for _ in 0..3 {
            output_matrix = perform_cycle(output_matrix);
        }
        assert_eq!(output_matrix, output_matrix_expected);
        Ok(())
    }
}
