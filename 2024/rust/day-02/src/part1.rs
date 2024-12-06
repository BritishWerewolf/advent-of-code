pub fn process(input: &str) -> usize {
    let input = input.replace("\r\n", "\n");

    input
       .lines()
        .map(|line| {
            let values = line.split_whitespace().map(|num| {
                num.parse::<u32>().unwrap_or_default()
            }).collect::<Vec<u32>>();

            // Check that the values all increase or always decrease.
            let mut sorted = values.clone();
            if sorted[0] > sorted[1] {
                sorted.sort_by(|a, b| b.cmp(a));
            } else {
                sorted.sort();
            }
            if values != sorted {
                return 0;
            }

            let result = values
                .windows(2)
                .filter_map(|pair| {
                    let (a, b) = (pair[0], pair[1]);
                    let diff = a.abs_diff(b);
                    if diff >= 1 && diff <= 3 {
                        None
                    } else {
                        Some(())
                    }
                })
                .count() == 0;
            if result { 1 } else { 0 }
        })
        .sum()
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

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 326);
    }
}
