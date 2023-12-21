use crate::custom_error::AocError;

fn count_solutions(time_limit: u64, distance: u64) -> u64 {
    (0..time_limit)
        .filter_map(|time_pressing| {
            if time_pressing * (time_limit - time_pressing) > distance {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap_or("");
    let distance_line = lines.next().unwrap_or("");

    let times: Vec<u64> = time_line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<u64> = distance_line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);

    let res: u64 = count_solutions(*times.get(0).unwrap(), *distances.get(0).unwrap());

    Ok(res)
}

// t_t * t_p - t_t - t_p + 1 >= d - t_l + 1
// (t_t - 1)*(t_p - 1) >= d - t_l + 1 >= 0
// cont solutions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
