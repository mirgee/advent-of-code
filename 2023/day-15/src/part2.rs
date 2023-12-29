use std::collections::HashMap;

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

#[derive(Debug)]
struct Lens {
    label: String,
    strength: usize,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, parsed) = parse_input(input).unwrap();
    let mut boxes = HashMap::<usize, Vec<Lens>>::new();
    for i in 0..256 {
        boxes.insert(i, vec![]);
    }
    for word in parsed.iter() {
        let box_num = word
            .iter()
            .filter(|c| c.is_ascii_alphabetic())
            .fold(0, |acc, c| (acc + (*c as usize)) * 17 % 256);
        if word.contains(&'=') {
            let parts = word.iter().collect::<String>();
            let parts = parts.split('=').collect::<Vec<&str>>();
            let label = parts[0].to_owned();
            let strength = parts[1].parse::<usize>().unwrap();
            let abox = boxes.get_mut(&box_num).unwrap();
            if let Some(idx) = abox.iter().position(|i| i.label == label) {
                abox.remove(idx);
                abox.insert(idx, Lens { label, strength });
            } else {
                abox.push(Lens { label, strength });
            }
        } else {
            let label = word[0..word.len() - 1].iter().collect::<String>();
            let abox = boxes.get_mut(&box_num).unwrap();
            if let Some(idx) = abox.iter().position(|i| i.label == label) {
                abox.remove(idx);
            }
        }
    }
    let mut focusing_power = 0;
    for (box_num, lens) in boxes.iter() {
        for (slot_num, lens) in lens.iter().enumerate() {
            focusing_power += (box_num + 1) * (slot_num + 1) * lens.strength;
        }
    }
    Ok(focusing_power)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "145")]
    #[test_log::test]
    fn test_process(#[case] input: &str, #[case] output: usize) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }
}
