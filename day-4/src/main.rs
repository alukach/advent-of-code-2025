use std::{fs::File, io::Read};

// https://adventofcode.com/2025/day/4
fn main() {
    let mut input = File::open("input.txt").expect("open failed");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).expect("failed to read");
    println!("Accessible: {}", get_accessible_rolls_count(buffer))
}

struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn build(lines: impl AsRef<str>) -> Self {
        let mut grid = Vec::<Vec<bool>>::new();
        for line in lines.as_ref().trim().lines().map(|l| l.trim()) {
            grid.push(line.chars().map(|c| c == '@').collect());
        }
        Self(grid)
    }

    fn get_neighbors(&self, row_idx: usize, col_idx: usize) -> u32 {
        let mut cell_neighbors = 0;
        let y_min = row_idx.saturating_sub(1);
        let y_max = (row_idx + 1).min(self.0.len() - 1);
        let x_min = col_idx.saturating_sub(1);
        let x_max = (col_idx + 1).min(self.0[row_idx].len() - 1);
        println!(
            "y_min={} y_max={}, x_min={} x_max={}",
            y_min, y_max, x_min, x_max,
        );

        for y_i in y_min..=y_max {
            for x_i in x_min..=x_max {
                if (y_i, x_i) == (row_idx, col_idx) {
                    println!("Skipping self (row {}, col {})", y_i, x_i);
                    continue;
                }
                print!("Checking row {}, col {}: ", y_i, x_i);
                if self.0[y_i][x_i] {
                    cell_neighbors += 1;
                    println!("hit")
                } else {
                    println!("miss")
                }
            }
        }
        cell_neighbors
    }
}

fn get_accessible_rolls_count(input: impl AsRef<str>) -> u32 {
    let grid = Grid::build(input);
    let mut accessible_count = 0;
    for (row_idx, row) in grid.0.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            if grid.get_neighbors(row_idx, col_idx) >= 4 {
                continue;
            }
            accessible_count += 1
        }
    }
    accessible_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt_1_ex() {
        assert_eq!(
            get_accessible_rolls_count(
                "
                ..@@.@@@@.
                @@@.@.@.@@
                @@@@@.@.@@
                @.@@@@..@.
                @@.@@@@.@@
                .@@@@@@@.@
                .@.@.@.@@@
                @.@@@.@@@@
                .@@@@@@@@.
                @.@.@@@.@.
                "
            ),
            13
        );
    }

    #[test]
    fn test_build_grid() {
        assert_eq!(
            Grid::build(
                "
                ..@@.
                @.@..
                "
            )
            .0,
            vec![
                vec![false, false, true, true, false],
                vec![true, false, true, false, false]
            ]
        );
    }

    #[test]
    fn test_get_neighbors() {
        let grid = Grid::build(
            "
            ..@@.
            @.@..
            ",
        );
        assert_eq!(grid.get_neighbors(0, 0), 1);
        assert_eq!(grid.get_neighbors(0, 2), 2);
        assert_eq!(grid.get_neighbors(1, 3), 3);
    }
}
