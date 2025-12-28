// https://adventofcode.com/2025/day/3

/*
NOTES:

- batteries labelled with joltage, 1-9
- batteries arranged in banks

pt 1:
- turn on 2 batteries in each bank, joltage equals number formed by those batteries joltage
- find largest joltage of each bank

pt 2:
- turn on 12 batteries in each bank, joltage equals number formed by those batteries joltage
- find largest joltage of each bank
*/

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() {
    let banks = get_banks("input.txt").expect("Failed to read banks");
    let battery_count = 12;
    let sum = banks
        .map(|bank| get_largest_joltage(bank.expect("Failed to read line"), battery_count))
        .sum::<u64>();
    println!("{}", sum);
}

fn get_banks<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_largest_joltage(bank: String, n: u8) -> u64 {
    /*
    Let's get the top 3 numbers out of this bank: 111111
    1111XX - 1st number (i=0) must ignore first 0 values (i) & ignore last 2 values (n - i - 1)
    X1111X - 2nd number (i=1) must ignore first 1 values (i) & ignore last 1 values (n - i - 1)
    XX1111 - 3rd number (i=2) must ignore first 2 values (i) & ignore last 0 values (n - i - 1)
    We also must ignore any values that are before or equal to the last selected index by starting at last_selected_idx + 1
    */
    let mut max_vals = Vec::<u8>::new();
    let mut last_selected_idx: isize = -1;
    println!("Bank {}, need {} vals", bank, n);
    for i in 0..n {
        let start = (i as usize).max((last_selected_idx + 1) as usize);
        let end = bank.len() - (n - i) as usize;
        println!(
            "Searching {} for char {}/{} between indexes {} and {}",
            bank, i, n, start, end
        );
        let search_subset = &bank[start..=end];
        println!("  searching subset: \"{}\"", search_subset.to_string());
        for (mut j, c) in search_subset.chars().into_iter().enumerate() {
            j = j + start;
            let val = c.to_string().parse::<u8>().expect("Failed to parse value");

            // Just put something in store on first pass
            print!(" char {}/{}: ", i, n);
            if max_vals.len() == i as usize {
                println!("using \"{}\" as we don't have a value", val);
                max_vals.push(val);
                last_selected_idx = j as isize;
                // Otherwise, only replace if new val is larger than stored value
            } else if val > max_vals[i as usize] {
                println!(
                    "using \"{}\" as it's larger than \"{}\"",
                    val, max_vals[i as usize]
                );
                max_vals[i as usize] = val;
                last_selected_idx = j as isize;
            } else {
                println!(
                    "ignoring \"{}\", as it's smaller than \"{}\"",
                    val, max_vals[i as usize]
                );
            }
        }
        println!(" char {}/{} FINAL: {} (ie {:?})", i, n, max_vals[i as usize], max_vals);
    }
    let s: String = max_vals.iter().map(|&c| format!("{}", c)).collect();
    println!(" -> {}", s);
    s.parse::<u64>().expect("Failed to parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_largest_joltage_pt1() {
        assert_eq!(get_largest_joltage("987654321111111".into(), 2), 98);
        assert_eq!(get_largest_joltage("811111111111119".into(), 2), 89);
        assert_eq!(get_largest_joltage("234234234234278".into(), 2), 78);
        assert_eq!(get_largest_joltage("818181911112111".into(), 2), 92);
    }

    #[test]
    fn test_get_largest_joltage_pt2() {
        assert_eq!(
            get_largest_joltage("987654321111111".into(), 12),
            987654321111
        );
        assert_eq!(
            get_largest_joltage("811111111111119".into(), 12),
            811111111119
        );
        assert_eq!(
            get_largest_joltage("234234234234278".into(), 12),
            434234234278
        );
        assert_eq!(
            get_largest_joltage("818181911112111".into(), 12),
            888911112111
        );
    }
}
