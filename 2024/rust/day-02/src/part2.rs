use itertools::Itertools;

pub fn process(input: &str) -> usize {
    let input = input.replace("\r\n", "\n");
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace().map(|num| {
                num.parse::<u32>().unwrap_or_default()
            })
            .tuple_windows::<(u32, u32)>()
            .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();

    let mut safe_lines: usize = 0;
    for line in lines {
        let mut is_safe = true;
        let mut ascending: Option<bool> = None;
        let mut errors: u8 = 0;
        for (a, b) in &line {
            let diff = a.abs_diff(*b);
            // First we will compare the ascending direction.
            match a.cmp(&b) {
                std::cmp::Ordering::Greater => {
                    if ascending.is_some() && ascending == Some(false) {
                        errors += 1;
                        if errors > 1 {
                            is_safe = false;
                            break;
                        }
                    }
                    ascending = Some(true);
                },
                std::cmp::Ordering::Equal => {
                    if ascending.is_none() {
                        errors += 1;
                        if errors > 1 {
                            is_safe = false;
                            break;
                        }
                    }
                },
                std::cmp::Ordering::Less => {
                    if ascending.is_some() && ascending == Some(true) {
                        errors += 1;
                        if errors > 1 {
                            is_safe = false;
                            break;
                        }
                    }
                    ascending = Some(false);
                },
            };

            // Now compare the difference.
            if diff == 0 || diff > 3 {
                errors += 1;
                if errors > 1 {
                    is_safe = false;
                }
            }
        }

        if is_safe {
            dbg!(line.iter().map(|(a, _)| format!("{a} ")).collect::<String>().trim());
            safe_lines += 1;
        }
    }

    safe_lines
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
        assert_eq!(result, 4);
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
