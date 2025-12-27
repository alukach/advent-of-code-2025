// https://adventofcode.com/2025/day/1

use std::{fs::File, io::BufRead};

fn main() {
    let input = File::open("input").expect("Failed to open input file");
    let mut num_zeros = 0;
    let mut cur_val = 50;
    println!("The dial starts by pointing at {}", cur_val);

    let reader = std::io::BufReader::new(&input);
    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");
        let (new_val, num_passes_over_zero) = get_position_after_rotation(cur_val, &line);
        print!("The dial is rotated {} to point at {}", &line, new_val);
        cur_val = new_val;
        if cfg!(feature = "part-2") {
            if num_passes_over_zero > 0 {
                print!(
                    "; during this rotation, it points at zero {} time(s)",
                    num_passes_over_zero
                );
                num_zeros += num_passes_over_zero;
            }
        } else {
            if cur_val == 0 {
                num_zeros += 1;
            }
        }
        println!(".");
    }
    println!("num zeros: {}", num_zeros);
}

fn get_position_after_rotation(start: i32, rotation: &str) -> (i32, i32) {
    let mut chars = rotation.chars();
    let direction = chars
        .next()
        .map(|c| {
            (if c == 'L' { -1 } else { 1 })
                * chars
                    .as_str()
                    .parse::<i32>()
                    .expect("Failed to parse count")
        })
        .expect("Line is empty");

    let absolute_end = start + direction;
    let end = absolute_end.rem_euclid(100);
    let mut num_passes_over_zero = (absolute_end.div_euclid(100) - start.div_euclid(100)).abs();
    if start.rem_euclid(100) == 0
        && direction < 0
        && absolute_end.div_euclid(100) < start.div_euclid(100) - 1
    {
        num_passes_over_zero -= 1;
    }
    (end, num_passes_over_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_position() {
        assert_eq!(get_position_after_rotation(50, "L68").0, 82);
        assert_eq!(get_position_after_rotation(82, "L30").0, 52);
        assert_eq!(get_position_after_rotation(52, "R48").0, 0);
        assert_eq!(get_position_after_rotation(0, "L5").0, 95);
        assert_eq!(get_position_after_rotation(95, "R60").0, 55);
        assert_eq!(get_position_after_rotation(55, "L55").0, 0);
        assert_eq!(get_position_after_rotation(0, "L1").0, 99);
        assert_eq!(get_position_after_rotation(99, "L99").0, 0);
        assert_eq!(get_position_after_rotation(0, "R14").0, 14);
        assert_eq!(get_position_after_rotation(14, "L82").0, 32);
    }

    #[test]
    fn test_passes_over_zero_50_l68() {
        assert_eq!(get_position_after_rotation(50, "L68").1, 1);
    }
    #[test]
    fn test_passes_over_zero_95_R60() {
        assert_eq!(get_position_after_rotation(95, "R60").1, 1);
    }
    #[test]
    fn test_passes_over_zero_14_l82() {
        assert_eq!(get_position_after_rotation(14, "L82").1, 1);
    }
    #[test]
    fn test_passes_over_zero_50_R1000() {
        assert_eq!(get_position_after_rotation(50, "R1000").1, 10);
    }
    #[test]
    fn test_passes_over_zero_50_l1000() {
        assert_eq!(get_position_after_rotation(50, "L1000").1, 10);
    }
    #[test]
    fn test_passes_over_zero_50_R1050() {
        assert_eq!(get_position_after_rotation(50, "R1050").1, 11);
    }
    #[test]
    fn test_passes_over_zero_50_l1050() {
        assert_eq!(get_position_after_rotation(50, "L1050").1, 11);
    }
    #[test]
    fn test_passes_over_zero_0_l250() {
        assert_eq!(get_position_after_rotation(0, "L250").1, 2);
    }
}
