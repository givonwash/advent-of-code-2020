use std::{
    collections::HashMap,
    io::{self, Read},
};

fn part_one(adapters: &[u32]) {
    let mut diff_freq = HashMap::new();

    for win in adapters.windows(2) {
        diff_freq
            .entry(win[1] - win[0])
            .and_modify(|freq| {
                *freq += 1;
            })
            .or_insert(1);
    }

    let ans = diff_freq
        .get(&1)
        .zip(diff_freq.get(&3))
        .map(|(diff_1, diff_3)| diff_1 * diff_3);

    println!("Part One: {:?}", ans);
}

fn part_two(adapters: &[u32]) {
    let mut paths = 1;
    // buffer for keeping track of newly discovered paths while iterating over available adapters
    let mut heads = Vec::new();

    for (i, adp) in adapters.iter().take(adapters.len() - 1).enumerate() {
        // get the adapters that are reachable from the current adapter
        let span = adapters[(i + 1)..]
            .iter()
            .copied()
            .take_while(|r| *r <= adp + 3)
            .collect::<Vec<_>>();

        match (span.len(), heads.len()) {
            // if span is 1 and heads is non-empty:
            // 1. count how many new paths are known and mulassign paths with that count
            // 2. empty the heads buffer
            // NOTE: a span of 1 will always be encountered on the penultimate entry in adapters
            (1, 1..) => {
                paths *= heads.drain(..).count();
            }
            // if span is >1, then add the heads of newly known paths to heads
            (2.., _) => {
                if heads.is_empty() {
                    heads.extend(span);
                } else {
                    // to avoid unnecessary removals from heads:
                    // 1. use the first adapter from span to overwrite all heads equal to adp
                    // 2. count how many overwrites from (1.) occur
                    // 3. append the rest of the adaptors count-from-(2.)-times to heads
                    let mut count = 0;
                    let (first, rest) = span.split_at(1);
                    for head in heads.iter_mut().filter(|h| *h == adp) {
                        *head = first[0];
                        count += 1;
                    }
                    heads.extend(rest.iter().cycle().take(count * rest.len()));
                }
            }
            (_, _) => {}
        }
    }

    println!("Part Two: {}", paths);
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
