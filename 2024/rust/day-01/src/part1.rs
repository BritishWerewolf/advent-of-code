pub fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");

    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    input
        .lines()
        .for_each(|line| {
            let mut splits = line.split_whitespace();
            left.push(splits.next().unwrap_or_default().parse::<u32>().unwrap_or_default());
            right.push(splits.next().unwrap_or_default().parse::<u32>().unwrap_or_default());
        });

    left.sort();
    right.sort();

    left
        .into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = process(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 1151792);
    }
}
