mod part1;
mod part2;

use std::{env, fs};

fn main() {
    // First split the string on path separators, reverse the array, skip the
    // first item (that would be the file name), then reverse again and join it
    // all together again.
    let input_path = file!().replace('\\', "/").split('/').collect::<Vec<&str>>().into_iter().rev().skip(1).rev().collect::<Vec<&str>>().join("/");
    // Get the current directory and join it with the path we just made.
    // Then join the input file.
    let input_path = env::current_dir().expect("to get current directory")
    .join(input_path).join("input.txt");

    let input = fs::read_to_string(input_path).expect("input to exist.");

    println!("Answers");
    println!("Part one: {}", part1::process(&input));
    println!("Part two: {}", part2::process(&input));
}
