use std::collections::{HashSet, BTreeMap};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, digit1, multispace0},
    IResult, sequence::tuple,
};

#[derive(Debug)]
struct Game {
    id: u32,
    winning_card: ScratchCard,
    chosen_card: ScratchCard,
}

impl Game {
    fn winning_matches(&self) -> HashSet<&u32> {
        self.winning_card.numbers.intersection(&self.chosen_card.numbers).collect()
    }
}

#[derive(Debug)]
struct ScratchCard {
    numbers: HashSet<u32>,
}

fn parse_numbers(input: &str) -> IResult<&str, ScratchCard> {
    let (input, numbers) = nom::multi::separated_list1(multispace1, digit1)(input)?;
    let numbers: HashSet<u32> = numbers.into_iter().map(|number| number.parse::<u32>().unwrap_or(0)).collect();
    Ok((input, ScratchCard { numbers }))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = tuple((tag("Card"), multispace1))(input)?;
    let (input, id) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, _) = multispace1(input)?;
    let (input, winning_card) = parse_numbers(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, chosen_card) = parse_numbers(input)?;
    let game = Game { id: id.parse::<u32>().unwrap(), winning_card, chosen_card };

    Ok((input, game))
}

pub fn process(input: &str) -> u32 {
    let mut num_cards: BTreeMap<u32, u32> = BTreeMap::new();

    input.lines()
    .filter_map(|line| parse_line(line).ok())
    .for_each(|(_, game)| {
        let win_matches = game.winning_matches().len() as u32;

        // There is always at least one version of the current card.
        *num_cards.entry(game.id).or_insert(0) += 1;

        // Now increment the scratch card for the matches.
        // Here we are taking the current number of cards we have, and adding
        // that to all the winning tickets below it.
        for next_id in (game.id + 1)..=(game.id + win_matches) {
            *num_cards.entry(next_id).or_insert(0) += *num_cards.get(&game.id).unwrap_or(&1);
        }

    });

    num_cards.values().sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(&input);
        assert_eq!(result, 30);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 5667240);
    }
}
