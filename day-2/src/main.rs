use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_ranges<P>(path: P) -> Vec<(String, String)>
where
    P: AsRef<Path>,
{
    let mut f = BufReader::new(File::open(path).expect("open failed"));
    let mut buf = Vec::<u8>::new();

    let mut ranges = Vec::<(String, String)>::new();
    while f.read_until(b',', &mut buf).expect("read_until failed") != 0 {
        let s = String::from_utf8(buf).expect("failed to convert to utf8");

        let parts = s
            .split('-')
            .map(|v| v.trim())
            .map(|v| v.trim_matches(','))
            .collect::<Vec<&str>>();
        ranges.push((parts[0].into(), parts[1].into()));

        // this returns the ownership of the read data to buf
        // there is no allocation
        // https://stackoverflow.com/a/37189758
        buf = s.into_bytes();
        buf.clear();
    }
    ranges
}

fn main() {
    let mut total = 0;
    for (start, end) in get_ranges("input.txt") {
        println!("Range: {} - {}", start, end);
        for num in start.parse::<u64>().expect("Failed to parse")
            ..=end.parse::<u64>().expect("Failed to parse")
        {
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
                continue;
            }
            let (a, b) = num_str.split_at(num_str.len() / 2);
            if a != b {
                continue;
            }
            println!(" Adding {:?}", num_str);
            total += num;
        }
    }
    println!("Total: {}", total);
}
