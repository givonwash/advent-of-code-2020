use std::collections::HashSet;
use std::io::{self, Read};

fn find_xy(mut nums: impl Iterator<Item = i32>, target: i32) -> Option<(i32, i32)> {
    let mut complements = HashSet::new();

    nums.find_map(|n| {
        if let Some(c) = complements.get(&(target - n)) {
            Some((n, *c))
        } else {
            complements.insert(n);
            None
        }
    })
}

fn part_one(expenses: &[i32]) {
    let answer = find_xy(expenses.iter().copied(), 2020).map(|(x, y)| x * y);

    println!("Part One: {answer:?}");
}

fn part_two(expenses: &[i32]) {
    let mut seen = HashSet::new();

    let answer = expenses
        .iter()
        .enumerate()
        .filter(|(_, e)| seen.insert(**e))
        .find_map(|(i, e)| {
            find_xy(
                expenses
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(j, e)| (i != j).then(|| e)),
                2020 - e,
            )
            .map(|(x, y)| x * y * e)
        });

    println!("Part Two: {answer:?}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let expenses = input
        .lines()
        .map(|l| l.parse().expect("Failed to parse entry in input into i32"))
        .collect::<Vec<_>>();

    part_one(&expenses);
    part_two(&expenses);

    Ok(())
}
