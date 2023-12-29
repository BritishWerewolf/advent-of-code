use nom::{IResult, character::complete::{multispace1, digit1, newline}, multi::separated_list1, Parser, sequence::separated_pair};
use nom_supreme::{ParserExt, tag::complete::tag};

#[derive(Debug)]
struct Data {
    time: u64,
    distance: u64,
}

impl Data {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn get_wins(&self) -> u64 {
        (0..=div_half_floor(self.time))
            .rev()
            .take_while(|&second| (self.time - second) * second > self.distance)
            .count() as u64
            * 2
            // We need to add one for even number of seconds due to the way we
            // half the numbers.
            - match self.time % 2 == 0 { true => 1, false => 0 }
    }
}

fn div_half_floor(lhs: u64) -> u64 {
    match lhs % 2 == 0 {
        true => lhs / 2,
        false => (lhs - 1) / 2,
    }
}

fn parse_times(input: &str) -> IResult<&str, u64> {
    tag("Time:")
        .precedes(multispace1)
        .precedes(
            separated_list1(multispace1, digit1)
            .map(|list| list.join("").parse::<u64>().expect("a valid number"))
        )
        .parse(input)
}

fn parse_distances(input: &str) -> IResult<&str, u64> {
    tag("Distance:")
        .precedes(multispace1)
        .precedes(
            separated_list1(multispace1, digit1)
            .map(|list| list.join("").parse::<u64>().expect("a valid number"))
        )
        .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_times, newline, parse_distances)
        .parse(input)
}

pub fn process(input: &str) -> u64 {
    let input = input.replace("\r\n", "\n");
    let (_, (time, distance)) = parse_input(&input).expect("to succeed");

    Data::new(time, distance).get_wins()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn half_even_number() {
        assert_eq!(super::div_half_floor(10), 5);
    }

    #[test]
    fn half_odd_number() {
        assert_eq!(super::div_half_floor(7), 3);
    }

    #[test]
    fn example_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 34934171);
    }
}
