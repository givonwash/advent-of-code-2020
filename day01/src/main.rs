use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn find_xy<P: FnMut(&(usize, &str)) -> bool>(
    input: &str,
    target: i32,
    predicate: P,
) -> Result<Option<(i32, i32)>> {
    let mut seen = HashSet::new();
    for (_, line) in input.lines().enumerate().filter(predicate) {
        let entry = line.parse::<i32>()?;
        let complement = target - entry;
        if let Some(complement) = seen.get(&complement) {
            return Ok(Some((entry, *complement)));
        } else {
            seen.insert(entry);
        }
    }

    Ok(None)
}

fn part_one(input: &str) -> Result<()> {
    if let Some((x, y)) = find_xy(input, 2020, |_| true)? {
        println!("Part One: {}", x * y);
    } else {
        println!("Part One: Could not find answer");
    }
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let mut known_targets = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        let entry = line.parse::<i32>()?;
        let target = 2020 - entry;
        if known_targets.get(&target).is_some() {
            continue;
        } else if let Some((x, y)) = find_xy(input, target, |iline| iline.0 != i)? {
            println!("Part Two: {}", entry * x * y);
            return Ok(());
        } else {
            known_targets.insert(target);
        }
    }

    println!("Part Two: Could not find answer");
    Ok(())
}

fn main() -> Result<()> {
    println!("Solving for day 01.");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part_one(&input)?;
    part_two(&input)?;

    Ok(())
}
