pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let digit = match reduced_line {
                    line if line.starts_with("one") => '1',
                    line if line.starts_with("two") => '2',
                    line if line.starts_with("three") => '3',
                    line if line.starts_with("four") => '4',
                    line if line.starts_with("five") => '5',
                    line if line.starts_with("six") => '6',
                    line if line.starts_with("seven") => '7',
                    line if line.starts_with("eight") => '8',
                    line if line.starts_with("nine") => '9',
                    _ => reduced_line.chars().next().unwrap(),
                };

                digit.to_digit(10)
            });

            let first = it.next().expect("first should be a number");
            let last = match it.last() {
                Some(num) => num,
                None => first,
            };

            format!("{first}{last}")
                .parse::<u32>()
                .expect("parse should be a number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(&input);
        assert_eq!(result, 281);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 54845);
    }
}
