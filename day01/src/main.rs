use std::collections::HashSet;
use std::io::{self, Read};

struct ComplementCache {
    cache: HashSet<i32>,
    target: i32,
}

impl ComplementCache {
    fn query(&mut self, x: i32) -> Option<i32> {
        if let Some(y) = self.cache.get(&(self.target - x)) {
            Some(*y)
        } else {
            self.cache.insert(x);
            None
        }
    }
}

fn part_one<I: Iterator<Item = i32>>(input: I) {
    let mut cache = ComplementCache {
        cache: HashSet::new(),
        target: 2020,
    };
    for x in input {
        if let Some(y) = cache.query(x) {
            println!("Part One: {}", x * y);
            return;
        }
    }
    println!("Part One: Could not find answer");
}

fn part_two<I>(input: I)
where
    I: Iterator<Item = i32> + Clone,
{
    let mut seen = HashSet::new();

    for (i, x) in input.clone().enumerate() {
        // if we have already encountered a value before, no need to check it again
        if seen.get(&x).is_some() {
            continue;
        } else {
            seen.insert(x);
            let mut cache = ComplementCache {
                cache: HashSet::new(),
                target: 2020 - x,
            };
            for (_, y) in input.clone().enumerate().filter(|(j, _)| *j != i) {
                if let Some(z) = cache.query(y) {
                    println!("Part Two: {}", (x * y * z));
                    return;
                }
            }
        }
    }

    println!("Part Two: Could not find answer");
}

fn main() -> io::Result<()> {
    println!("Solving for day 01.");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer
        .lines()
        .map(|l| l.parse().expect("Failed to parse entry in input into i32"));
    part_one(input.clone());
    part_two(input);

    Ok(())
}
