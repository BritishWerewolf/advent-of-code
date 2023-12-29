use std::collections::BTreeMap;
use nom::{IResult, branch::alt, bytes::complete::{tag, take_while_m_n}, Parser, character::complete::{char, newline}, multi::{many1, separated_list1}, sequence::{tuple, delimited}};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(direction: char) -> Direction {
        match direction {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                dbg!(direction);
                panic!("Invalid direction.");
            },
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Element(String);

impl Element {
    fn new(source: &str) -> Element {
        Element(source.to_owned())
    }
}

// Create our own parser, because the nom parser is for byte strings, not chars.
fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(
        alt((
            char('L'),
            char('R'),
        ))
    )
    .parse(input)
    .map(|(input, directions)| {
        (input, directions.iter().map(|d| Direction::new(*d)).collect())
    })
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    take_while_m_n(3, 3, is_alphabetic)
    .parse(input)
    .map(|(input, letters)| (input, Element::new(letters)))
}

// Parse all Element rows such as `AAA = (BBB, CCC)`.
fn parse_element_row(input: &str) -> IResult<&str, (Element, Vec<Element>)> {
    let (input, key) = parse_element(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, value) = delimited(
        tag("("),
        separated_list1(tag(", "), parse_element),
        tag(")"))
        .parse(input)?;

    Ok((input, (key, value)))
}

// All the parsers acting together in a single function.
fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<Element, Vec<Element>>)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = tuple((newline, newline))(input)?;

    let elements: BTreeMap<Element, Vec<Element>> = input.lines()
    // `parse_element_row` returns a tuple, we don't care for the leftovers.
   .map(|line| parse_element_row(line).expect("a valid element row.").1)
   .collect();

    Ok((input, (directions, elements)))
}

pub fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");

    // In theory there should be no more `input`.
    let (_input, (directions, elements)) = parse_input(&input).expect("valid input");

    let mut current_element = &Element::new("AAA");

    // Use find_map to keep iterating over the Elements.
    // We will return None to iterate through the directions again.
    // When we return Some, it will cause this infinite loop to end.
    directions.iter().cycle().enumerate().find_map(|(index, direction)| {
        let next_element = match direction {
            Direction::Left  => &elements.get(current_element).expect("has destinations.")[0],
            Direction::Right => &elements.get(current_element).expect("has destinations.")[1],
        };

        if *next_element == Element::new("ZZZ") {
            Some(index as u32 + 1)
        } else {
            current_element = next_element;
            None
        }
    })
    .expect("Element(ZZZ) to exist.")
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_input_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = process(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 18157);
    }
}
