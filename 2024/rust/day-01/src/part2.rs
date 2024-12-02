pub fn process(input: &str) -> u32 {
    let _input = input.replace("\r\n", "\n");

    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    input
        .lines()
        .for_each(|line| {
            let mut splits = line.split_whitespace();
            left.push(splits.next().unwrap_or_default().parse::<u32>().unwrap_or_default());
            right.push(splits.next().unwrap_or_default().parse::<u32>().unwrap_or_default());
        });

    left
        .into_iter()
        .map(|l| l * u32::try_from(right.iter().filter(|&&r| l == r).count()).unwrap_or_default())
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
        assert_eq!(result, 31);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 21790168);
    }
}
