# Advent of Code
This repository contains my solutions for the [Advent of Code](https://adventofcode.com) programming challenges.

My solutions are organised by year, then language, then each day within that.  
This just helps me keep track of my progress for each language.

## Usage
To run the code, you will need to have [Rust](https://www.rust-lang.org) installed.  
I have also created a `justfile` too; below is an example of some commands that can be run from within each language (for instance `2023/rust`).

```shell
$ just
just --list
Available recipes:
    create day
    default
$ just create day-04
cargo generate --path ./daily-template --name day-06
 Destination: advent-of-code\2023\rust\day-06
 project-name: day-06
 Generating template
 Moving generated files into: `advent-of-code\2023\rust\day-06`s
 Initializing a fresh Git repository
 Done! New project created advent-of-code\2023\rust\day-06
```
