use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list1},
    IResult,
};

use crate::custom_error::AocError;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(tag(","), many1(none_of(",\n")))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, parsed) = parse_input(input).unwrap();
    let sum = parsed
        .into_iter()
        .map(|word| {
            word.iter()
                .fold(0, |acc, c| (acc + (*c as usize)) * 17 % 256)
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "1320")]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: usize) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
