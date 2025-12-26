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

fn get_invalid_ids(id_ranges: Vec<(u64, u64)>) -> Vec<u64> {
    id_ranges
        .iter()
        .flat_map(|(start, end)| {
            (*start..=*end).filter_map(|num: u64| {
                let num_str = num.to_string();
                if num_str.len() % 2 != 0 {
                    return None;
                }
                let (a, b) = num_str.split_at(num_str.len() / 2);
                if a != b {
                    return None;
                }
                Some(num)
            })
        })
        .collect()
}

fn main() {
    let id_ranges = get_ranges("input.txt");
    let invalid_ids = get_invalid_ids(id_ranges);
    let total = invalid_ids.iter().sum::<u64>();
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example_invalid_ids() {
        assert_eq!(get_invalid_ids(vec![(11, 22)]), vec![11, 22]);
        assert_eq!(get_invalid_ids(vec![(95, 115)]), vec![99]);
        assert_eq!(get_invalid_ids(vec![(998, 1012)]), vec![1010]);
        assert_eq!(
            get_invalid_ids(vec![(1188511880, 1188511890)]),
            vec![1188511885]
        );
        assert_eq!(get_invalid_ids(vec![(222220, 222224)]), vec![222222]);
        assert_eq!(get_invalid_ids(vec![(1698522, 1698528)]), vec![]);
        assert_eq!(get_invalid_ids(vec![(446443, 446449)]), vec![446446]);
        assert_eq!(get_invalid_ids(vec![(38593856, 38593862)]), vec![38593859]);
    }

    #[test]
    fn test_pt1_example_sum() {
        assert_eq!(
            get_invalid_ids(vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
            ])
            .iter()
            .sum::<u64>(),
            1227775554
        )
    }
}
