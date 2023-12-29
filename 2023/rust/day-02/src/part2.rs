#[derive(Debug, Default)]
struct RoundResult {
    red: u32,
    green: u32,
    blue: u32,
}

impl RoundResult {
    fn product(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn process(input: &str) -> u32 {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let _id = parts[0].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap_or(0);

        let max_result = parts[1].split("; ").filter_map(|round| {
            let mut result = RoundResult::default();
            round.split(", ").for_each(|ball| {
                let ball_total: u32 = ball.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap_or(0);

                match ball.to_lowercase() {
                    x if x.contains("red")   => result.red = ball_total,
                    x if x.contains("green") => result.green = ball_total,
                    x if x.contains("blue")  => result.blue = ball_total,
                    _ => ()
                };
            });

            Some(result)
        })
        .reduce(|carry, round| {
            RoundResult {
                red: carry.red.max(round.red),
                green: carry.green.max(round.green),
                blue: carry.blue.max(round.blue),
            }
        });

        if let Some(result) = max_result {
            result.product()
        } else {
            0
        }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 67335);
    }
}
