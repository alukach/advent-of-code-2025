// https://adventofcode.com/2025/day/3

/*
NOTES:

- batteries labelled with joltage, 1-9
- batteries arranged in banks
- turn on 2 batteries in each bank, joltage equals number formed by those batteries joltage
- find largest joltage of each bank
*/

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() {
    let banks = get_banks("input.txt").expect("Failed to read banks");
    let sum = banks
        .map(|bank| get_largest_joltage(bank.expect("Failed to read line").into()))
        .sum::<u32>();
    println!("{}", sum);
}

fn get_banks<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_largest_joltage(bank: String) -> u32 {
    // Find largest first digit
    let mut max_val_1 = 0;
    let mut max_i = 0;
    for (i, c) in bank[0..bank.len() - 1].chars().into_iter().enumerate() {
        let val = c.to_string().parse::<u32>().expect("Failed to parse value");
        if val > max_val_1 {
            max_val_1 = val;
            max_i = i;
        }
    }

    // Find largest second digit
    let mut max_val_2 = 0;
    for c in bank[max_i + 1..bank.len()].chars().into_iter() {
        let val = c.to_string().parse::<u32>().expect("Failed to parse value");
        max_val_2 = val.max(max_val_2);
    }
    format!("{}{}", max_val_1, max_val_2)
        .parse()
        .expect("Failed to parse value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_largest_joltage() {
        assert_eq!(get_largest_joltage("987654321111111".into()), 98);
        assert_eq!(get_largest_joltage("811111111111119".into()), 89);
        assert_eq!(get_largest_joltage("234234234234278".into()), 78);
        assert_eq!(get_largest_joltage("818181911112111".into()), 92);
    }
}
