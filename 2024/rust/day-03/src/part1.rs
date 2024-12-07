use nom::{bytes::complete::tag, character::complete::{anychar, digit1}, combinator::map_res, multi::{many0, many_till}, sequence::{delimited, separated_pair}, IResult, Parser};

#[derive(Clone, Copy, Debug, Default)]
struct Pair {
    left: u32,
    right: u32,
}

impl Pair {
    fn mul(&self) -> u32 {
        self.left * self.right
    }
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    delimited(
        tag("mul("),
        map_res(
            separated_pair(digit1, tag(","), digit1),
            |(left, right): (&str, &str)| -> Result<Pair, std::num::ParseIntError> {
                Ok(Pair {
                    left: left.parse::<u32>()?,
                    right: right.parse::<u32>()?
                })
            }
        ),
        tag(")")
    )(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    many0(
        many_till(
            anychar,
            parse_pair
        ).map(|(_discard, pair)| pair)
    )(input)
}

pub fn process(input: &str) -> u32 {
    let _input = input.replace("\r\n", "\n");
    let (_, pairs) = parse_pairs(input).unwrap_or_default();

    pairs.into_iter().map(|pair| pair.mul()).sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = process(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 153469856);
    }
}
