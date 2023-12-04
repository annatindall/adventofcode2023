use std::collections::HashMap;

pub fn day04(input_lines: &str) -> (String, String) {
    let cards: Vec<Card> = input_lines.lines().map(Card::create).collect();
    let answer1 = part1(&cards);
    let answer2 = part2(&cards);
    (format!("{}", answer1), format!("{}", answer2))
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|card| card.points()).sum()
}

fn part2(cards: &Vec<Card>) -> u32 {
    let mut cards_set = HashMap::new();
    for card in cards {
        cards_set.insert(card.id, card.clone());
    }

    let mut cards_won: HashMap<u32, u32> = HashMap::new();

    for card in cards {
        // Record original card
        let copies_of_this_card: u32 = *cards_won.get(&card.id).unwrap_or(&0) + 1;
        cards_won.insert(card.id, copies_of_this_card);
        for won_card_id in card.cards_won() {
            cards_won.insert(
                won_card_id,
                cards_won.get(&won_card_id).unwrap_or(&0) + copies_of_this_card,
            );
        }
    }

    cards_won.values().sum()
}

#[derive(Clone)]
struct Card {
    id: u32,
    choices: Vec<u32>,
    winners: Vec<u32>,
}

impl Card {
    fn create(input_line: &str) -> Self {
        let mut initial_split = input_line.split(':');
        let id = initial_split.next().unwrap().trim()[5..]
            .trim()
            .parse()
            .unwrap();
        let mut number_lists = initial_split.next().unwrap().split('|');
        let choices = Self::numbers_from_list(number_lists.next().unwrap());
        let winners = Self::numbers_from_list(number_lists.next().unwrap());
        Card {
            id,
            choices,
            winners,
        }
    }

    fn numbers_from_list(input: &str) -> Vec<u32> {
        input
            .trim()
            .split_ascii_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect()
    }

    fn count_matches(&self) -> u32 {
        self.choices
            .iter()
            .filter(|choice| self.winners.contains(choice))
            .count()
            .try_into()
            .unwrap()
    }

    fn points(&self) -> u32 {
        let matches = self.count_matches();
        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }

    fn cards_won(&self) -> Vec<u32> {
        (0..self.count_matches()).map(|i| self.id + i + 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: Vec<Card> = input.lines().map(Card::create).collect();
        assert_eq!(part1(&cards), 13);
    }

    #[test]
    fn check_day04_part2_case1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: Vec<Card> = input.lines().map(Card::create).collect();
        assert_eq!(part2(&cards), 30)
    }

    #[test]
    fn check_won_cards() {
        let card1 = Card::create("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card1.cards_won(), vec![2, 3, 4, 5]);
    }
}
