use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn part_one<'a, I>(groups: I)
where
    I: Iterator<Item = &'a str>,
{
    let answer: usize = groups
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();
    println!("Part One: {}", answer);
}

fn part_two<'a, I>(groups: I)
where
    I: Iterator<Item = &'a str>,
{
    let answer: usize = groups
        .map(|group| {
            let mut char_frequency = HashMap::new();
            let mut people = 1;
            for c in group.trim_end().chars() {
                if c == '\n' {
                    people += 1;
                } else {
                    let count = char_frequency.entry(c).or_insert(0);
                    *count += 1;
                }
            }
            char_frequency
                .into_values()
                .filter(|val| *val == people)
                .count()
        })
        .sum();
    println!("Part Two: {}", answer);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let groups = input.split("\n\n");
    part_one(groups.clone());
    part_two(groups);
    Ok(())
}
