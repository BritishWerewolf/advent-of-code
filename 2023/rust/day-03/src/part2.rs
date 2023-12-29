use std::{fmt::Debug, collections::HashSet};

#[derive(Debug, PartialEq)]
enum Variant {
    Dot,
    Gear,
    Number,
    Symbol,
}

#[derive(Debug, Default)]
struct Grid {
    data: Vec<Vec<Cell>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let mut grid = Self::default();

        let mut current_line = 0;
        input.lines().for_each(|line| {
            let mut current_column = 0;
            let row = line.chars().map(|c| {
                let cell = Cell {
                    x: current_line,
                    y: current_column,
                    value: c,
                };
                current_column += 1;
                cell
            })
            .collect::<Vec<Cell>>();

            grid.data.push(row);
            current_line += 1;
        });

        grid
    }

    fn get_width(&self) -> u32 {
        if self.data.is_empty() {
            return 0;
        }
        self.data[0].len() as u32
    }

    fn get_height(&self) -> u32 {
        self.data.len() as u32
    }

    fn get_variants(&self, variant: Variant) -> Vec<&Cell> {
        self.data.iter().flat_map(|row| {
            row.iter().filter(|cell| {
               cell.variant() == variant
            }).collect::<Vec<&Cell>>()
        }).collect::<Vec<&Cell>>()
    }

    fn get_adjoining(&self, cell: &Cell, variant: Variant) -> Option<HashSet<MergeCell>> {
        let mut found_cells: HashSet<MergeCell> = HashSet::new();

        // Iterate over the box around the current cell.
        for i in (cell.x.saturating_sub(1))..=(cell.x + 1) {
            for j in (cell.y.saturating_sub(1))..=(cell.y + 1) {
                // We don't want to include the current cell, or out of bounds.
                //dbg!(format!("height: {}, width: {}, i: {i}, j: {j}", self.get_height(), self.get_width()));
                if (i == cell.x && j == cell.y)
                    || i >= self.get_height()
                    || j > self.get_width()
                {
                    continue;
                }

                //dbg!(format!("cell: ({0}, {1}) | {i} {j}", cell.x, cell.y));
                let current_cell = &self.data[i as usize][j as usize];

                // Now gather all the cells that match.
                if current_cell.variant() == variant {
                    let mut merge_cell: Vec<&Cell> = Vec::new();

                    // Save that exact cell.
                    merge_cell.push(current_cell);

                    // Now expand the cell match.
                    // Go left.
                    for tmp_j in (0..j).rev() {
                        let tmp_cell = &self.data[i as usize][tmp_j as usize];
                        if tmp_cell.variant() != variant {
                            break;
                        }
                        merge_cell.push(tmp_cell);
                    }
                    // Go right.
                    for tmp_j in ((j + 1) as usize)..=(self.data[i as usize].len() - 1) {
                        let tmp_cell = &self.data[i as usize][tmp_j];
                        if tmp_cell.variant() != variant {
                            break;
                        }
                        merge_cell.push(tmp_cell);
                    }

                    if !merge_cell.is_empty() {
                        found_cells.insert(MergeCell::from(merge_cell));
                    }
                }
            }
        }

        Some(found_cells)
    }
}

//#[derive(Debug)]
struct Cell {
    x: u32,
    y: u32,
    value: char,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell {{ x: {}, y: {}, value: {} }}", self.x, self.y, self.value)
    }
}

impl Cell {
    fn variant(&self) -> Variant {
        match self.value {
            '.' => Variant::Dot,
            '*' => Variant::Gear,
            c if c.is_numeric() => Variant::Number,
            _ => Variant::Symbol,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct MergeCell {
    x_min: u32,
    x_max: u32,
    y_min: u32,
    y_max: u32,
    value: String,
}

impl MergeCell {
    fn from(cells: Vec<&Cell>) -> Self {
        let mut cells = cells;
        cells.sort_by(|a, b| {
            a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y))
        });

        Self {
            x_min: cells.iter().map(|cell| cell.x).min().unwrap(),
            x_max: cells.iter().map(|cell| cell.x).max().unwrap(),
            y_min: cells.iter().map(|cell| cell.y).min().unwrap(),
            y_max: cells.iter().map(|cell| cell.y).max().unwrap(),
            value: cells.iter().map(|cell| cell.value.to_string()).reduce(|acc, value| {
                acc + &value
            }).unwrap()
        }
    }
}

pub fn process(input: &str) -> u32 {
    let grid = Grid::from(input);
    let symbols = grid.get_variants(Variant::Gear);

    symbols.into_iter().filter_map(|symbol_cell| {
        grid.get_adjoining(symbol_cell, Variant::Number)
    }).filter_map(|gear_match_cells| {
        if gear_match_cells.len() != 2 {
            return None;
        }

        Some(
            gear_match_cells
            .into_iter()
            .map(|cell| cell.value.parse::<u32>().unwrap_or(1))
            .product::<u32>()
        )
    })
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn partial_real_input() {
        let input = "............830..743.......59..955.......
.......284.....*............*.....$...+..
....%.........976..679.461.7..........350
...992.....#......=...../........701.....
.........868........................*....
....................*200............311..
.......266.......209......589.....=......
..836..........................949....607
........367.....328.&......%.............
........*.........*..119.253.............";
        let result = process(input);
        assert_eq!(result, 1070304);
    }

    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 69527306);
    }
}
