use nom::{
    character::complete::{i64, line_ending, space1},
    multi::separated_list1,
    IResult,
};

use crate::custom_error::AocError;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(space1, i64)(input)
    }

    separated_list1(line_ending, parse_line)(input)
}

fn predict_next_value(mut input: Vec<i64>) -> i64 {
    let mut last_values = Vec::<i64>::new();
    while !input.iter().all(|v| v == &0) {
        last_values.push(input.last().unwrap().clone());
        input = input.iter().map_windows(|&[a, b]| b - a).collect();
    }
    last_values.iter().sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i64, AocError> {
    let (_, values) = parse_input(input).unwrap();
    Ok(values.into_iter().map(|v| predict_next_value(v)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
