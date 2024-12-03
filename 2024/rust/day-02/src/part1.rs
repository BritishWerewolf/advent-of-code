pub fn process(input: &str) -> u32 {
    let _input = input.replace("\r\n", "\n");

    0
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = process(&input);
        assert_eq!(result, 2);
    }

    #[ignore]
    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 0);
    }
}
