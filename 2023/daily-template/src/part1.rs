use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("", "")]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
        todo!("haven't built test yet");
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
