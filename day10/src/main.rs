use std::{
    collections::HashMap,
    io::{self, Read},
};

fn count_arrangements(adapters: &[u32]) -> Option<u64> {
    fn counter(adapters: &[u32], cache: &mut HashMap<u32, Option<u64>>) -> Option<u64> {
        adapters.last().and_then(|head| {
            cache.get(head).copied().unwrap_or_else(|| {
                let arrangements = match adapters {
                    [] | [_] => Some(1),
                    &[prev, _] => head.checked_sub(prev).filter(|d| *d <= 3).and(Some(1)),
                    &[prev2, prev1, _] => {
                        match (head.checked_sub(prev2), head.checked_sub(prev1)) {
                            // `prev2` and `prev1` are both reachable
                            (Some(2..=3), Some(1..=2)) => {
                                counter(&adapters[..2], cache).map(|c| c + 1)
                            }
                            // `prev2` is unreachable & `prev1` is reachable
                            (Some(4..), Some(1..=3)) => counter(&adapters[..2], cache),
                            // neither `prev2` or `prev1` are reachable
                            _ => None,
                        }
                    }
                    &[.., prev3, prev2, prev1, _] if prev2 < prev1 => {
                        let ihead = adapters.len();

                        match (
                            head.checked_sub(prev3),
                            head.checked_sub(prev2),
                            head.checked_sub(prev1),
                        ) {
                            // `prev3`, `prev2`, & `prev1` are reachable
                            (Some(3), Some(2), Some(1)) => counter(&adapters[..ihead - 1], cache)
                                .and_then(|total| {
                                    counter(&adapters[..ihead - 2], cache).map(|c| total + c)
                                })
                                .and_then(|total| {
                                    counter(&adapters[..ihead - 3], cache).map(|c| total + c)
                                }),
                            // `prev3` is unreachable & `prev2` and `prev1` are reachable
                            (Some(4..), Some(2..=3), Some(1..=2)) => {
                                counter(&adapters[..ihead - 1], cache).and_then(|total| {
                                    counter(&adapters[..ihead - 2], cache).map(|c| total + c)
                                })
                            }
                            // `prev3` and `prev2` are unreachable & `prev1` is reachable
                            (Some(4..), Some(4..), Some(1..=3)) => {
                                counter(&adapters[..ihead - 1], cache)
                            }
                            // `prev3`, `prev2`, & `prev1` are unreachable
                            _ => None,
                        }
                    }
                    _ => None,
                };

                cache.insert(*head, arrangements);

                arrangements
            })
        })
    }

    let mut cache = HashMap::new();
    counter(&adapters, &mut cache)
}

fn part_one(adapters: &[u32]) {
    let mut frequencies = HashMap::new();

    for win in adapters.windows(2) {
        frequencies
            .entry(win[1] - win[0])
            .and_modify(|freq| {
                *freq += 1;
            })
            .or_insert(1);
    }

    let answer = frequencies
        .get(&1)
        .zip(frequencies.get(&3))
        .map(|(diff_1, diff_3)| diff_1 * diff_3);

    println!("Part One: {answer:?}");
}

fn part_two(adapters: &[u32]) {
    let answer = count_arrangements(&adapters);

    println!("Part Two: {answer:?}");
}

fn main() -> io::Result<()> {
    // add charger as implicit adapter
    let mut input = String::from("0\n");
    io::stdin().read_to_string(&mut input)?;

    let mut adapters = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();

    // sort adapters in ascending order
    adapters.sort_unstable();

    // add phone as implicit adapter
    adapters.push(adapters.last().copied().unwrap() + 3);

    part_one(&adapters);
    part_two(&adapters);

    Ok(())
}
