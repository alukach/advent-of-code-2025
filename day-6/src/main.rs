use std::fs::read_to_string;

fn main() {
    env_logger::init();
    let input = read_to_string("input.txt").expect("failed to read");
    println!(
        "Sum (traditional): {}",
        process(&input, MathStyle::Traditional)
    );
    println!(
        "Sum (cephalopod): {}",
        process(&input, MathStyle::Cephalopod)
    );
}

fn process(input: impl AsRef<str>, style: MathStyle) -> u64 {
    let mut problems = Vec::new();
    let mut values = Vec::new();
    let lines = input
        .as_ref()
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<&str>>();

    match style {
        MathStyle::Traditional => {
            for line in lines {
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
                            operator: Operator::from_str(val).expect("Failed to parse operator"),
                        });
                    };
                }
            }
        }
        MathStyle::Cephalopod => {
            /*
                Rather than line-by-line, we iterate column by column (per character)
            */
            let num_cols = lines.first().expect("Failed to get first line").len();

            for col_idx in 0..num_cols {
                // Create an empty value for each column
                let mut col_val = String::new();

                // Populate value from line
                for line in lines.clone() {
                    let col_char = line
                        .chars()
                        .nth(col_idx)
                        .expect("Failed to get character from column");
                    log::debug!("Got char \"{}\" from col {} of {}", col_char, col_idx, line);

                    // If operator, create problem with empty values
                    if let Some(operator) = Operator::from_str(col_char.to_string()) {
                        log::debug!("Creating problem with {} operator", col_char);
                        problems.push(Problem {
                            values: Vec::new(),
                            operator,
                        });
                    } else if let ' ' = col_char {
                        // Otherwise, must be a part of the value
                    } else {
                        log::trace!(
                            "Pushing \"{}\" into value \"{}\" for col {}",
                            col_char,
                            col_val,
                            col_idx
                        );
                        col_val.push(col_char);
                    }
                }

                // After all the lines, we can now put the value into the problem
                let problem = problems.last_mut().expect("No last problem found");
                log::debug!("Inserting value {}", col_val);
                if col_val.trim().is_empty() {
                    log::debug!("Empty column")
                } else {
                    problem.values.push(
                        col_val
                            .trim()
                            .parse::<u64>()
                            .expect("Failed to parse value"),
                    );
                }
            }
        }
    }

    problems.iter().map(|p| p.solve()).sum()
}

struct Problem {
    values: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Added => self.values.iter().sum::<u64>(),
            Operator::Multiplied => self.values.iter().product(),
        }
    }
}

enum Operator {
    Added,
    Multiplied,
}

impl Operator {
    fn from_str(val: impl AsRef<str>) -> Option<Self> {
        let v = val.as_ref();
        match v {
            "*" => Some(Operator::Multiplied),
            "+" => Some(Operator::Added),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum MathStyle {
    Traditional,
    Cephalopod,
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
        assert_eq!(process(input, MathStyle::Traditional), 4277556)
    }

    #[test]
    fn test_pt_2_ex() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
        ";
        assert_eq!(process(input, MathStyle::Cephalopod), 3263827)
    }
}
