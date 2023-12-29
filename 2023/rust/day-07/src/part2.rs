use std::{collections::HashMap, cmp::Ordering};
use nom::Parser;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit1;
use nom::{sequence::separated_pair, character::complete::space1, IResult};

const MAX_CARDS: u32 = 5;

#[derive(PartialEq, Eq, Hash, Debug)]
enum CardType {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl CardType {
    fn new(card: char) -> CardType {
        match card {
            'A' => CardType::Ace,
            'K' => CardType::King,
            'Q' => CardType::Queen,
            'T' => CardType::Ten,
            '9' => CardType::Nine,
            '8' => CardType::Eight,
            '7' => CardType::Seven,
            '6' => CardType::Six,
            '5' => CardType::Five,
            '4' => CardType::Four,
            '3' => CardType::Three,
            '2' => CardType::Two,
            'J' => CardType::Joker,
            _ => panic!("Invalid card."),
        }
    }

    fn index(&self) -> u32 {
        match self {
            CardType::Ace => 14,
            CardType::King => 13,
            CardType::Queen => 12,
            CardType::Ten => 10,
            CardType::Nine => 9,
            CardType::Eight => 8,
            CardType::Seven => 7,
            CardType::Six => 6,
            CardType::Five => 5,
            CardType::Four => 4,
            CardType::Three => 3,
            CardType::Two => 2,
            CardType::Joker => 1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum HandType {
    FiveOfAKind,  // AAAAA -> 5
    FourOfAKind,  // AA8AA -> 4,1
    FullHouse,    // 23332 -> 3,2
    ThreeOfAKind, // TTT98 -> 3,1,1
    TwoPair,      // 23432 -> 2,2,1
    OnePair,      // A23A4 -> 2,1,1,1
    HighCard,     // 23456 -> 1,1,1,1,1
}

impl HandType {
    fn index(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Hand {
    cards: Vec<CardType>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<CardType>, bid: u32) -> Hand {
        Hand { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut cards: HashMap<&CardType, u32> = HashMap::new();

        // First count each card type.
        self.cards.iter()
        .for_each(|card| *cards.entry(card).or_insert(0) += 1);

        // Ensure there are Joker's but that the entire hand isn't Joker's.
        if cards.contains_key(&CardType::Joker) && cards.get(&CardType::Joker).unwrap_or(&MAX_CARDS) < &MAX_CARDS {
            // Now replace Jokers with the card with the highest value.
            let highest_card = cards.iter()
            // Remove Joker's as we don't want to replace Joker with itself.
            .filter(|card| *card.0 != &CardType::Joker)
            .max_by(|a, b| {
                // Where there are multiple cards with the same count, we need
                // to sort by the CardType index.
                if a.1 == b.1 {
                    return a.0.index().cmp(&b.0.index());
                }

                a.1.cmp(b.1)
            })
            .expect("a high card to exist.");

            // Replace the Joker with the highest card count.
            *cards.entry(*highest_card.0).or_insert(0) += cards[&CardType::Joker];
            cards.remove(&CardType::Joker);
        }

        let mut cards_values = cards.values().collect::<Vec<&u32>>();
        cards_values.sort();

        match cards_values.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => {
                dbg!(cards);
                panic!("Invalid hand.")
            },
        }
    }

    fn compare_against(&self, other: &Hand) -> Ordering {
        // First we want to compare the HandType.
        if self.hand_type() != other.hand_type() {
            return self.hand_type().index().cmp(&other.hand_type().index());
        }

        // However, if the HandType are the same, then compare card by card.
        for i in 0..self.cards.len() {
            let card_a = &self.cards[i];
            let card_b = &other.cards[i];

            if card_a != card_b {
                return card_a.index().cmp(&card_b.index());
            }
        }

        Ordering::Equal
    }
}

fn is_valid_card(card: char) -> bool {
    matches!(card, 'A' | 'K' | 'Q' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J')
}

fn parse_cards(line: &str) -> IResult<&str, Vec<CardType>> {
    take_while1(is_valid_card)
    .parse(line)
    .map(|(line, cards)| {
        // Convert each card char into a valid CardType.
        (
            line,
            cards
            .chars()
            .map(CardType::new)
            .collect::<Vec<CardType>>()
        )
    })
}

fn parse_bid(line: &str) -> IResult<&str, u32> {
    digit1
    .parse(line)
    .map(|(input, numbers)| (input, numbers.parse::<u32>().expect("valid number")))
}

fn parse_hand(line: &str) -> Hand {
    separated_pair(parse_cards, space1, parse_bid)(line)
    .map(|(_, (cards, bid))| Hand::new(cards, bid))
    .expect("Parsed a valid hand")
}

pub fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");

    let mut hands: Vec<Hand> = input
        .lines()
        .map(parse_hand)
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_against(b));

    hands.iter()
    .enumerate()
    .map(|(i, hand)| hand.bid * (i as u32 + 1))
    .sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(input);
        assert_eq!(result, 5905);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 253362743);
    }
}
