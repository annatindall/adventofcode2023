use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

pub fn day07(input_lines: &str) -> (String, String) {
    let answer1 = part1(input_lines);
    let answer2 = part2(input_lines);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(input_lines: &str) -> i32 {
    let hands_bids: HashMap<CamelHand, i32> = input_lines
        .lines()
        .map(|l| {
            let mut i = l.split_ascii_whitespace();
            (
                CamelHand::create(i.next().unwrap()),
                i.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    winnings(&hands_bids)
}

fn part2(input_lines: &str) -> i32 {
    let hands_bids: HashMap<CamelHand, i32> = input_lines
        .lines()
        .map(|l| {
            let mut i = l.split_ascii_whitespace();
            (
                CamelHand::create_with_jokers(i.next().unwrap()),
                i.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    winnings(&hands_bids)
}

fn winnings(hands_bids: &HashMap<CamelHand, i32>) -> i32 {
    let mut sorted_hands = hands_bids.keys().collect_vec();
    sorted_hands.sort_unstable_by(|h1, h2| h1.partial_cmp(h2).unwrap());

    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank: i32 = (i + 1).try_into().unwrap();
            rank * hands_bids.get(hand).unwrap()
        })
        .sum()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct CamelHand(Vec<CamelCard>);

impl CamelHand {
    fn get_type(&self) -> CamelType {
        let mut frequencies = self.0.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
        let (mode, mut mode_frequency): (CamelCard, i32) = frequencies
            .iter()
            .filter(|(k, _)| k != &&&CamelCard::Joker) // Avoid returning Joker if multiple modes
            .map(|(k, v)| (**k, *v))
            .max_by_key(|(_k, v)| *v)
            .unwrap_or_else(|| {
                // Joker was the only mode
                (
                    CamelCard::Joker,
                    *frequencies.get(&CamelCard::Joker).unwrap(),
                )
            });
        if mode != CamelCard::Joker {
            // Can add jokers to frequency of mode to calculate highest score with same logic (I think)
            let jokers: i32 = *frequencies.get(&CamelCard::Joker).unwrap_or(&0);
            *frequencies.entry(&mode).or_insert(0) += jokers;
            mode_frequency += jokers;
            frequencies.insert(&CamelCard::Joker, 0);
        }

        if mode_frequency >= 5 {
            CamelType::Five
        } else if mode_frequency == 4 {
            CamelType::Four
        } else if mode_frequency == 3 {
            if frequencies.values().contains(&2) {
                CamelType::Full
            } else {
                CamelType::Three
            }
        } else if mode_frequency == 2 {
            if frequencies
                .iter()
                .filter(|(k, _)| **k != &mode)
                .any(|(_, v)| v == &2)
            {
                CamelType::TwoPair
            } else {
                CamelType::Pair
            }
        } else {
            CamelType::High
        }
    }

    fn create(input: &str) -> Self {
        Self(input.chars().map(CamelCard::from_char).collect())
    }

    fn create_with_jokers(input: &str) -> Self {
        Self(
            input
                .chars()
                .map(CamelCard::from_char_with_jokers)
                .collect(),
        )
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_compare = self.get_type().cmp(&other.get_type());

        if type_compare == Ordering::Equal {
            // Fall back to comparing by cards, in order of hand
            let (self_card, other_card) = self
                .0
                .iter()
                .zip(other.0.iter())
                .find(|(c1, c2)| c1 != c2)
                .unwrap();
            Some(self_card.cmp(other_card))
        } else {
            Some(type_compare)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CamelType {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum CamelCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CamelCard {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid input"),
        }
    }

    fn from_char_with_jokers(c: char) -> Self {
        if c == 'J' {
            Self::Joker
        } else {
            Self::from_char(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_types() {
        let hand1 = CamelHand::create("32T3K");
        assert_eq!(hand1.get_type(), CamelType::Pair);
        let hand2 = CamelHand::create("QQQJA");
        assert_eq!(hand2.get_type(), CamelType::Three);
        assert!(hand2 > hand1);
    }

    #[test]
    fn check_day07_part1() {
        let input_string = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(part1(input_string), 6440)
    }

    #[test]
    fn check_day07_part2() {
        let input_string = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(part2(input_string), 5905)
    }
}
