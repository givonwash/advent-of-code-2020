use std::collections::HashSet;
use std::io::{self, Read};

fn get_complement_or_insert(
    complements: &mut HashSet<i32>,
    target: i32,
    candidate: i32,
) -> Option<i32> {
    complements.get(&(target - candidate)).cloned().or_else(|| {
        complements.insert(candidate);
        None
    })
}

fn part_one<I>(expenses: I)
where
    I: Iterator<Item = i32>,
{
    let mut complements = HashSet::new();

    for expense_x in expenses {
        if let Some(expense_y) = get_complement_or_insert(&mut complements, 2020, expense_x) {
            println!("Part One: {}", (expense_x * expense_y));
            return;
        }
    }

    println!("Part One: No Answer Found");
}

fn part_two<I>(expenses: I)
where
    I: Iterator<Item = i32> + Clone,
{
    let mut attempted = HashSet::new();

    for (i, expense_x) in expenses.clone().enumerate() {
        if attempted.contains(&expense_x) {
            continue;
        } else {
            attempted.insert(expense_x);

            let mut complements = HashSet::new();

            for (_, expense_y) in expenses.clone().enumerate().filter(|(j, _)| i != *j) {
                if let Some(expense_z) =
                    get_complement_or_insert(&mut complements, 2020 - expense_x, expense_y)
                {
                    println!("Part Two: {}", (expense_x * expense_y * expense_z));
                    return;
                }
            }
        }
    }

    println!("Part Two: No Answer Found");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let expenses = input
        .lines()
        .map(|l| l.parse().expect("Failed to parse entry in input into i32"));

    part_one(expenses.clone());
    part_two(expenses);

    Ok(())
}
