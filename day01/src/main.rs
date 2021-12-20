use std::collections::HashSet;
use std::io::{self, Read};

fn find_xy(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut complements = HashSet::new();

    nums.iter().find_map(|n| {
        if let Some(c) = complements.get(&(target - n)) {
            Some((*n, *c))
        } else {
            complements.insert(*n);
            None
        }
    })
}

fn part_one(expenses: &[i32]) {
    println!(
        "Part One: {:?}",
        find_xy(expenses, 2020).map(|(x, y)| x * y)
    );
}

fn part_two(expenses: &[i32]) {
    let mut attempted = HashSet::new();
    let mut exp_iter = expenses.iter();

    let ans = loop {
        if let Some(z) = exp_iter.next().filter(|z| attempted.insert(*z)) {
            if let Some((x, y)) = find_xy(expenses, 2020 - z) {
                break Some(x * y * z);
            }
        } else {
            break None;
        }
    };
    println!("Part Two: {:?}", ans);
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
