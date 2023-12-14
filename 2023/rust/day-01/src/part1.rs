pub fn process(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut iterator = line.chars().filter_map(|character| {
            character.to_digit(10)
        });

        let first = iterator.next().expect("should be a number.");
        let last = match iterator.last() {
            Some(num) => num,
            None      => first,
        };

        format!("{first}{last}").parse::<u32>().expect("should be a number.")
    })
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(&input);
        assert_eq!(result, 142);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 55090);
    }
}
