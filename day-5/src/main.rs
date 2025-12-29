use std::fs::read_to_string;

fn main() {
    env_logger::init();
    let input = read_to_string("input.txt").expect("failed to read");
    let ingredients = IngredientsList::build(input);
    println!("Fresh count: {}", ingredients.fresh_count());
    println!(
        "Possibly fresh count: {}",
        ingredients.possibly_fresh_count()
    );
}

struct IngredientsList {
    fresh: Vec<Range>,
    available: Option<Vec<Ingredient>>,
}

impl IngredientsList {
    fn build(input: impl AsRef<str>) -> Self {
        let mut split_input = input.as_ref().trim().split("\n\n");
        let fresh_ranges_str = split_input.next().expect("Failed to get ranges");

        log::info!("Storing ranges...");
        let mut unmerged_ranges = Vec::new();
        for fresh_range in fresh_ranges_str.split("\n").map(|l| l.trim()) {
            let (start_str, end_str) = fresh_range.split_once('-').expect("Couldn't split range");
            let start = start_str.parse::<u64>().expect("Failed to parse start");
            let end = end_str.parse::<u64>().expect("Failed to parse end");
            unmerged_ranges.push(Range { start, end });
        }

        log::info!("Sorting ranges...");
        unmerged_ranges.sort_by_key(|r| r.start);

        log::info!("Merging ranges...");
        let mut fresh_ranges = Vec::<Range>::new();
        for range in unmerged_ranges {
            if let Some(last) = fresh_ranges.last_mut() {
                if range.start <= last.end {
                    last.end = last.end.max(range.end);
                } else {
                    fresh_ranges.push(range)
                }
            } else {
                fresh_ranges.push(range);
            }
        }

        let available_ids_str = split_input.next();
        let available = match available_ids_str {
            Some(available_ids_str) => {
                log::info!("Storing available...");
                let mut available: Vec<Ingredient> = available_ids_str
                    .split('\n')
                    .map(|l| Ingredient {
                        id: l.trim().parse::<u64>().expect("Failed to parse id"),
                        state: State::Spoiled,
                    })
                    .collect();

                log::info!("Sorting available...");
                available.sort_by_key(|i| i.id);

                log::info!("Update available state from inventory...");
                let mut range_idx = 0;
                for i in &mut available {
                    while range_idx < fresh_ranges.len() {
                        let r = fresh_ranges.get(range_idx).expect("Failed to get range");
                        if i.id < r.start {
                            log::debug!("id {} less than range start {}, not fresh", i.id, r.start);
                            break;
                        } else if i.id > r.end {
                            log::debug!(
                                "id {} greater than range end {}, need next range",
                                i.id,
                                r.end
                            );
                            range_idx += 1;
                            continue;
                        } else {
                            log::debug!(
                                "id {} less than range start {} and greater than range end {}, it's fresh!",
                                i.id,
                                r.start,
                                r.end
                            );
                            i.state = State::Fresh;
                            break;
                        }
                    }
                }
                Some(available)
            }
            None => None,
        };

        Self {
            fresh: fresh_ranges,
            available,
        }
    }

    /// How many ingredients are actually fresh, as per the fresh ranges and available list
    fn fresh_count(&self) -> usize {
        self.available
            .as_ref()
            .expect("Available produce not provided")
            .iter()
            .filter(|i| matches!(i.state, State::Fresh))
            .count()
    }

    /// How many ingredients could be fresh, as per the fresh ranges
    fn possibly_fresh_count(&self) -> u64 {
        self.fresh.iter().map(|r| r.end - r.start + 1).sum()
    }
}

struct Ingredient {
    id: u64,
    state: State,
}

#[derive(Clone, PartialEq, Debug)]
enum State {
    Fresh,
    Spoiled,
}

struct Range {
    start: u64,
    end: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt_1_ex() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(
            IngredientsList::build(
                "
                3-5
                10-14
                16-20
                12-18

                1
                5
                8
                11
                17
                32
                ",
            )
            .fresh_count(),
            3
        )
    }
    #[test]
    fn test_pt_2_ex() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(
            IngredientsList::build(
                "
                3-5
                10-14
                16-20
                12-18
                ",
            )
            .possibly_fresh_count(),
            14
        )
    }
}
