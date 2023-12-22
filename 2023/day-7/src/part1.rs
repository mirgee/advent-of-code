use crate::custom_error::AocError;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Card::A, Card::A) => Some(std::cmp::Ordering::Equal),
            (Card::A, _) => Some(std::cmp::Ordering::Greater),
            (_, Card::A) => Some(std::cmp::Ordering::Less),
            (Card::K, Card::K) => Some(std::cmp::Ordering::Equal),
            (Card::K, _) => Some(std::cmp::Ordering::Greater),
            (_, Card::K) => Some(std::cmp::Ordering::Less),
            (Card::Q, Card::Q) => Some(std::cmp::Ordering::Equal),
            (Card::Q, _) => Some(std::cmp::Ordering::Greater),
            (_, Card::Q) => Some(std::cmp::Ordering::Less),
            (Card::J, Card::J) => Some(std::cmp::Ordering::Equal),
            (Card::J, _) => Some(std::cmp::Ordering::Greater),
            (_, Card::J) => Some(std::cmp::Ordering::Less),
            (Card::T, Card::T) => Some(std::cmp::Ordering::Equal),
            (Card::T, _) => Some(std::cmp::Ordering::Greater),
            (_, Card::T) => Some(std::cmp::Ordering::Less),
            (Card::N(n1), Card::N(n2)) => Some(n1.cmp(n2)),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => Card::N(c.to_digit(10).unwrap() as u8),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|c| match c {
                Card::A => 'A',
                Card::K => 'K',
                Card::Q => 'Q',
                Card::J => 'J',
                Card::T => 'T',
                Card::N(n) => std::char::from_digit(*n as u32, 10).unwrap(),
            })
            .collect_vec();
        write!(f, "{}", cards.iter().join(""))
    }
}

impl<'de> Deserialize<'de> for Hand {
    fn deserialize<D>(deserializer: D) -> Result<Hand, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let cards = s.chars().map(|s| Card::from_char(s)).collect_vec();
        Ok(Hand {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn is_five_of_a_kind(&self) -> bool {
        self.cards.iter().all(|c| c == &self.cards[0])
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards
            .iter()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .any(|(_, group)| group.count() == 4)
    }

    fn is_full_house(&self) -> bool {
        let groups = self.cards.iter().sorted().group_by(|c| *c);
        let group_counts = groups
            .into_iter()
            .map(|(_, group)| group.count())
            .collect_vec();
        let three_same = group_counts.iter().any(|count| *count == 3);
        let two_same = group_counts.iter().any(|count| *count == 2);
        three_same && two_same
    }

    fn is_three_of_a_kind(&self) -> bool {
        let groups = self.cards.iter().sorted().group_by(|c| *c);
        let group_counts = groups
            .into_iter()
            .map(|(_, group)| group.count())
            .collect_vec();
        let three_same = group_counts.iter().any(|count| *count == 3);
        let two_different = group_counts.iter().filter(|count| **count == 1).count() == 2;
        three_same && two_different
    }

    fn is_two_pair(&self) -> bool {
        let groups = self.cards.iter().sorted().group_by(|c| *c);
        let group_counts = groups
            .into_iter()
            .map(|(_, group)| group.count())
            .collect_vec();
        group_counts.iter().filter(|count| **count == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        let groups = self.cards.iter().sorted().group_by(|c| *c);
        let group_counts = groups
            .into_iter()
            .map(|(_, group)| group.count())
            .collect_vec();
        group_counts.iter().filter(|count| **count == 2).count() == 1
    }

    fn get_hand_type(&self) -> HandType {
        if self.is_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind() {
            HandType::FourOfAKind
        } else if self.is_full_house() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if self.is_two_pair() {
            HandType::TwoPair
        } else if self.is_one_pair() {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_cmp = self.get_hand_type().cmp(&other.get_hand_type());
        if type_cmp != std::cmp::Ordering::Equal {
            return Some(type_cmp);
        }
        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            let val = card1.cmp(card2);
            if val != std::cmp::Ordering::Equal {
                return Some(val);
            }
        }
        None
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    // We should group hands by type and then compare within groups
    let pairs = input.lines().map(|line| {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();
        let hand: Hand = serde_json::from_str(&format!("\"{}\"", line_parts[0])).unwrap();
        let value: u64 = line_parts[1].parse().unwrap();
        (hand, value)
    });
    // TODO: It would be more efficient to group by hand type and then sort within groups
    let pairs = pairs.sorted_by_key(|(hand, _)| *hand);
    let val = pairs
        .enumerate()
        .fold(0, |acc, (idx, (_, value))| acc + value * (idx as u64 + 1));
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
