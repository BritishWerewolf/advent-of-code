pub fn process(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "";
        let result = process(&input);
        assert_eq!(result, 0);
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
