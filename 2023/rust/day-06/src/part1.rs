use nom::{IResult, character::complete::{multispace1, digit1, newline}, multi::separated_list1, Parser, sequence::separated_pair};
use nom_supreme::{ParserExt, tag::complete::tag};

#[derive(Debug)]
struct Data {
    time: u32,
    distance: u32,
}

impl Data {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    fn get_wins(&self) -> u32 {
        (0..=div_half_floor(self.time))
            .rev()
            .take_while(|&second| (self.time - second) * second > self.distance)
            .count() as u32
            * 2
            // We need to add one for even number of seconds due to the way we
            // half the numbers.
            - match self.time % 2 == 0 { true => 1, false => 0 }
    }
}

fn div_half_floor(lhs: u32) -> u32 {
    match lhs % 2 == 0 {
        true => lhs / 2,
        false => (lhs - 1) / 2,
    }
}

fn parse_times(input: &str) -> IResult<&str, Vec<&str>> {
    tag("Time:")
        .precedes(multispace1)
        .precedes(separated_list1(multispace1, digit1))
        .parse(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<&str>> {
    tag("Distance:")
        .precedes(multispace1)
        .precedes(separated_list1(multispace1, digit1))
        .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(parse_times, newline, parse_distances)
        .parse(input)
}

fn vec_str_to_int(input: Vec<&str>) -> Vec<u32> {
    // TODO It would be nice if this was incorporated into the parser.
    input.iter().map(|s| s.parse::<u32>().unwrap_or(0)).collect()
}

pub fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let (_, (times, distances)) = parse_input(&input).expect("to succeed");

    let times = vec_str_to_int(times);
    let distances = vec_str_to_int(distances);

    times.into_iter().zip(distances)
        .map(|(time, distance)| Data::new(time, distance).get_wins())
        .product::<u32>()
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
        assert_eq!(result, 288);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 2065338);
    }
}
