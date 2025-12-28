// https://adventofcode.com/2025/day/1
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_instructions<P>(filename: P) -> io::Result<Vec<Instruction>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        instructions.push(Instruction::from(line?));
    }
    Ok(instructions)
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let direction = match &s[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance: u16 = s[1..]
            .parse()
            .expect("Failed to parse distance as unsigned 8-bit integer");
        Instruction {
            direction,
            distance,
        }
    }
}

fn main() {
    let mut dial = Dial::new(50);
    println!("Initial Dial position: {}", dial.position);
    let instructions = read_instructions("input.txt").expect("Failed to read instructions");
    for instruction in instructions {
        dial.rotate(&instruction);
    }
    println!("Landed on 0 count: {}", dial.landed_on_zero_count);
    println!("Crossed 0 count: {}", dial.crossed_zero_count);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u16,
}

/// Represents the state of the dial
struct Dial {
    position: i16,
    num_positions: i16,
    landed_on_zero_count: u32,
    crossed_zero_count: u64,
}

impl Dial {
    fn new(position: i16) -> Self {
        Dial {
            position,
            num_positions: 100,
            landed_on_zero_count: 0,
            crossed_zero_count: 0,
        }
    }

    fn rotate(&mut self, i: &Instruction) {
        let old_position = self.position;
        let delta = match i.direction {
            Direction::Left => self.position - i.distance as i16,
            Direction::Right => self.position + i.distance as i16,
        };
        let mut crossed = delta.div_euclid(self.num_positions).abs() as u64;
        self.position = delta.rem_euclid(self.num_positions);
        print!(
            "  The dial is rotated {:?}{} to point at {}",
            i.direction, i.distance, self.position
        );
        if crossed > 0 {
            println!("; during this rotation, it points at 0 once.")
        } else {
            println!(".")
        }
        match i.direction {
            Direction::Right => {}
            Direction::Left => {
                if self.position == 0 {
                    crossed += 1; // Landed on 0, div_euclid missed it
                }
                if old_position == 0 && crossed > 0 {
                    crossed -= 1; // Started on 0, didn't actually cross
                }
            }
        }

        self.crossed_zero_count += crossed;
        if self.position == 0 {
            self.landed_on_zero_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossing_zero_neg() {
        let mut dial = Dial::new(2);
        let instruction = Instruction {
            direction: Direction::Left,
            distance: 5,
        };
        dial.rotate(&instruction);
        assert_eq!(dial.position, 97);
    }

    #[test]
    fn test_crossing_zero_pos() {
        let mut dial = Dial::new(97);
        let instruction = Instruction {
            direction: Direction::Right,
            distance: 5,
        };
        dial.rotate(&instruction);
        assert_eq!(dial.position, 2);
    }

    #[test]
    fn test_solution_1() {
        let mut dial = Dial::new(50);
        let instructions = read_instructions("input.txt").expect("Failed to read instructions");
        for instruction in instructions {
            dial.rotate(&instruction);
        }
        assert_eq!(dial.landed_on_zero_count, 1158);
    }

    #[test]
    fn test_solution_2_example() {
        /*f
        The dial starts by pointing at 50.
        The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        The dial is rotated L30 to point at 52.
        The dial is rotated R48 to point at 0.
        The dial is rotated L5 to point at 95.
        The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        The dial is rotated L55 to point at 0.
        The dial is rotated L1 to point at 99.
        The dial is rotated L99 to point at 0.
        The dial is rotated R14 to point at 14.
        The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        */
        let mut dial = Dial::new(50);
        println!("  The dial starts by pointing at {}", dial.position);
        let instructions = Vec::from([
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]);
        for i in instructions {
            dial.rotate(&i.to_string().into());
        }

        assert_eq!(dial.position, 32);
        assert_eq!(dial.crossed_zero_count, 6);
    }

    #[test]
    fn test_crossovers_pos_1000() {
        let mut dial = Dial::new(50);
        dial.rotate(&"R1000".to_string().into());
        assert_eq!(dial.position, 50);
        assert_eq!(dial.crossed_zero_count, 10);
    }

    #[test]
    fn test_crossovers_pos_55() {
        let mut dial = Dial::new(50);
        dial.rotate(&"R55".to_string().into());
        assert_eq!(dial.position, 5);
        assert_eq!(dial.crossed_zero_count, 1);
    }

    #[test]
    fn test_crossovers_neg() {
        let mut dial = Dial::new(50);
        dial.rotate(&"L55".to_string().into());
        assert_eq!(dial.position, 95);
        assert_eq!(dial.crossed_zero_count, 1);
    }

    #[test]
    fn test_crossovers_neg_lands_on_zero() {
        let mut dial = Dial::new(50);
        dial.rotate(&"L50".to_string().into());
        assert_eq!(dial.position, 0);
        assert_eq!(dial.crossed_zero_count, 1);
    }

    #[test]
    fn test_crossovers_neg_starts_on_zero() {
        let mut dial = Dial::new(0);
        dial.rotate(&"L5".to_string().into());
        assert_eq!(dial.position, 95);
        assert_eq!(dial.crossed_zero_count, 0);
    }

    #[test]
    fn test_crossovers_pos_lands_on_zero() {
        let mut dial = Dial::new(50);
        dial.rotate(&"R50".to_string().into());
        assert_eq!(dial.position, 0);
        assert_eq!(dial.crossed_zero_count, 1);
    }
}
