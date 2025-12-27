use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_ranges<P>(path: P) -> Vec<(u64, u64)>
where
    P: AsRef<Path>,
{
    let mut f = BufReader::new(File::open(path).expect("open failed"));
    let mut buf = Vec::<u8>::new();

    let mut ranges = Vec::<(u64, u64)>::new();
    while f.read_until(b',', &mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("failed to convert to utf8");

        let parts = s
            .split('-')
            .map(|v| v.trim())
            .map(|v| v.trim_matches(','))
            .collect::<Vec<&str>>();
        ranges.push((
            parts[0].parse::<u64>().expect("Failed to parse"),
            parts[1].parse::<u64>().expect("Failed to parse"),
        ));

        // this returns the ownership of the read data to buf
        // there is no allocation
        // https://stackoverflow.com/a/37189758
        buf = s.into_bytes();
        buf.clear();
    }
    ranges
}

fn get_invalid_ids(id_ranges: Vec<(u64, u64)>, mode: RepeatMode) -> Vec<u64> {
    id_ranges
        .iter()
        .flat_map(|(start, end)| {
            println!("Reviewing range {}-{}", start, end);
            (*start..=*end).filter_map(|num: u64| {
                let num_str = num.to_string();

                if matches!(mode, RepeatMode::Twice) {
                    if num_str.len() % 2 != 0 {
                        return None;
                    }
                    let (a, b) = num_str.split_at(num_str.len() / 2);
                    if a != b {
                        return None;
                    }
                    Some(num)
                } else {
                    if has_patterns(num_str.clone()) {
                        println!(" - match: {}", num);
                        Some(num)
                    } else {
                        None
                    }
                }
            })
        })
        .collect()
}

fn has_patterns(val: String) -> bool {
    if val.len() < 2 {
        return false;
    }
    let range_end = match val.len() % 2 {
        0 => val.len() / 2,
        _ => val.len() / 2 + 1,
    };
    for i in 1..range_end + 1 {
        let chunks = chunk_string(&val, i);
        if all_elements_are_equal(&chunks) {
            return true;
        }
    }
    false
}

fn all_elements_are_equal(vec: &Vec<String>) -> bool {
    let first_val = &vec[0];
    for val in vec {
        if *val != *first_val {
            return false;
        }
    }
    true
}

fn chunk_string(string: &str, n: usize) -> Vec<String> {
    let mut buf = string.chars().peekable();
    let mut out = Vec::<String>::new();
    while buf.peek().is_some() {
        out.push(buf.by_ref().take(n).collect());
    }
    out
}

fn main() {
    let mode = RepeatMode::Multi;
    let id_ranges = get_ranges("input.txt");
    let total = get_invalid_ids(id_ranges, mode).iter().sum::<u64>();
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example_invalid_ids() {
        let mode = RepeatMode::Twice;
        assert_eq!(get_invalid_ids(vec![(11, 22)], mode.clone()), vec![11, 22]);
        assert_eq!(get_invalid_ids(vec![(95, 115)], mode.clone()), vec![99]);
        assert_eq!(get_invalid_ids(vec![(998, 1012)], mode.clone()), vec![1010]);
        assert_eq!(
            get_invalid_ids(vec![(1188511880, 1188511890)], mode.clone()),
            vec![1188511885]
        );
        assert_eq!(
            get_invalid_ids(vec![(222220, 222224)], mode.clone()),
            vec![222222]
        );
        assert_eq!(
            get_invalid_ids(vec![(1698522, 1698528)], mode.clone()),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(vec![(446443, 446449)], mode.clone()),
            vec![446446]
        );
        assert_eq!(
            get_invalid_ids(vec![(38593856, 38593862)], mode.clone()),
            vec![38593859]
        );
    }

    #[test]
    fn test_pt1_example_sum() {
        assert_eq!(
            get_invalid_ids(
                vec![
                    (11, 22),
                    (95, 115),
                    (998, 1012),
                    (1188511880, 1188511890),
                    (222220, 222224),
                    (1698522, 1698528),
                    (446443, 446449),
                    (38593856, 38593862),
                ],
                RepeatMode::Twice
            )
            .iter()
            .sum::<u64>(),
            1227775554
        )
    }

    #[test]
    fn test_pt2_example_invalid_ids() {
        let mode = RepeatMode::Multi;
        assert_eq!(get_invalid_ids(vec![(11, 22)], mode.clone()), vec![11, 22]);
        assert_eq!(
            get_invalid_ids(vec![(95, 115)], mode.clone()),
            vec![99, 111]
        );
        assert_eq!(
            get_invalid_ids(vec![(998, 1012)], mode.clone()),
            vec![999, 1010]
        );
        assert_eq!(
            get_invalid_ids(vec![(1188511880, 1188511890)], mode.clone()),
            vec![1188511885]
        );
        assert_eq!(
            get_invalid_ids(vec![(222220, 222224)], mode.clone()),
            vec![222222]
        );
        assert_eq!(
            get_invalid_ids(vec![(1698522, 1698528)], mode.clone()),
            vec![]
        );
        assert_eq!(
            get_invalid_ids(vec![(446443, 446449)], mode.clone()),
            vec![446446]
        );
        assert_eq!(
            get_invalid_ids(vec![(38593856, 38593862)], mode.clone()),
            vec![38593859]
        );
        assert_eq!(
            get_invalid_ids(vec![(565653, 565659)], mode.clone()),
            vec![565656]
        );
        assert_eq!(
            get_invalid_ids(vec![(824824821, 824824827)], mode.clone()),
            vec![824824824]
        );
        assert_eq!(
            get_invalid_ids(vec![(2121212118, 2121212124)], mode.clone()),
            vec![2121212121]
        );
    }

    #[test]
    fn test_pt2_example_sum() {
        assert_eq!(
            get_invalid_ids(
                vec![
                    (11, 22),
                    (95, 115),
                    (998, 1012),
                    (1188511880, 1188511890),
                    (222220, 222224),
                    (1698522, 1698528),
                    (446443, 446449),
                    (38593856, 38593862),
                    (565653, 565659),
                    (824824821, 824824827),
                    (2121212118, 2121212124)
                ],
                RepeatMode::Multi
            )
            .iter()
            .sum::<u64>(),
            4174379265
        )
    }

    #[test]
    fn test_chunk_string() {
        assert_eq!(
            chunk_string("AAABBBCCC", 1),
            vec!["A", "A", "A", "B", "B", "B", "C", "C", "C"]
        );
        assert_eq!(
            chunk_string("AAABBBCCC", 2),
            vec!["AA", "AB", "BB", "CC", "C"]
        );
        assert_eq!(chunk_string("AAABBBCCC", 3), vec!["AAA", "BBB", "CCC"]);
        assert_eq!(chunk_string("AAABBBCCC", 4), vec!["AAAB", "BBCC", "C"]);
    }

    #[test]
    fn test_all_elements_are_equal() {
        assert_eq!(
            all_elements_are_equal(&vec!["A".to_string(), "A".to_string(), "A".to_string()]),
            true
        );
        assert_eq!(
            all_elements_are_equal(&vec!["AA".to_string(), "A".to_string(), "A".to_string()]),
            false
        );
        assert_eq!(
            all_elements_are_equal(&vec!["A".to_string(), "A".to_string(), "B".to_string()]),
            false
        );
    }

    #[test]
    fn test_has_patterns() {
        assert_eq!(has_patterns("121121".to_string()), true);
        assert_eq!(has_patterns("1".to_string()), false);
    }
}

#[derive(Clone)]
enum RepeatMode {
    // Pt. 1
    Twice,
    // Pt. 2
    Multi,
}
