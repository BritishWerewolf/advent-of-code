use std::collections::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, digit1, multispace0},
    IResult, sequence::tuple,
};

#[derive(Debug)]
struct ScratchCard {
    numbers: HashSet<u32>,
}

fn parse_numbers(input: &str) -> IResult<&str, ScratchCard> {
    let (input, numbers) = nom::multi::separated_list1(multispace1, digit1)(input)?;
    let numbers: HashSet<u32> = numbers.into_iter().map(|number| number.parse::<u32>().unwrap_or(0)).collect();
    Ok((input, ScratchCard { numbers }))
}

fn parse_line(input: &str) -> IResult<&str, (ScratchCard, ScratchCard)> {
    let (input, _) = tuple((tag("Card"), multispace1))(input)?;
    let (input, _) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, _) = multispace1(input)?;
    let (input, card1) = parse_numbers(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, card2) = parse_numbers(input)?;
    Ok((input, (card1, card2)))
}

pub fn process(input: &str) -> u32 {
    input.lines()
    .filter_map(|line| parse_line(line).ok())
    .map(|(_, (winning_card, my_card))| {
        let win_count = winning_card.numbers.intersection(&my_card.numbers).count() as u32;

        match win_count.checked_sub(1) {
            Some(num) => (2 as u32).pow(num),
            None => 0,
        }
    })
    .sum()
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
        assert_eq!(result, 13);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 25183);
    }
}
