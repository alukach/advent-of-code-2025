use std::fs::read_to_string;

fn main() {
    env_logger::init();
    let input = read_to_string("input.txt").expect("failed to read");
    println!("Sum: {}", process(input))
}

fn process(input: impl AsRef<str>) -> u64 {
    let mut problems = Vec::<Problem>::new();
    let mut values = Vec::new();
    for line in input.as_ref().lines() {
        for (i, val) in line.split_whitespace().enumerate() {
            // Buildout vector to hold values (only on first pass)
            if i == values.len() {
                values.push(Vec::new());
            }

            // Get mutable vector to store values
            let i_values = values
                .get_mut(i)
                .expect(&format!("Index {} not set in values vector", i));

            // Attempt to parse column value...
            if let Ok(val) = val.parse::<u64>() {
                // if successful, place in vector of values
                i_values.push(val);
            } else {
                // If fail, assuming it's an operator (and thus end of lines)
                problems.push(Problem {
                    values: i_values.to_vec(),
                    operator: match val {
                        "*" => Operator::Multiplied,
                        "+" => Operator::Added,
                        _ => panic!("Unexpected operator {}", val),
                    },
                });
            };
        }
    }

    problems
        .iter()
        .map(|p| match p.operator {
            Operator::Added => p.values.iter().sum::<u64>(),
            Operator::Multiplied => p.values.iter().product(),
        })
        .sum()
}

struct Problem {
    values: Vec<u64>,
    operator: Operator,
}

enum Operator {
    Added,
    Multiplied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt_1_ex() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = "
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   +
        ";
        assert_eq!(process(input), 4277556)
    }
}
