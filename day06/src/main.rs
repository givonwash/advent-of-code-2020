use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn part_one(groups: &[&str]) {
    let answer: usize = groups
        .iter()
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();
    println!("Part One: {answer}");
}

fn part_two(groups: &[&str]) {
    let answer: usize = groups
        .iter()
        .map(|group| {
            let (people, question_frequency) = group.lines().fold(
                (0, HashMap::new()),
                |(people, mut question_frequency), questions| {
                    for question in questions.chars() {
                        question_frequency
                            .entry(question)
                            .and_modify(|freq| {
                                *freq += 1;
                            })
                            .or_insert(1);
                    }
                    (people + 1, question_frequency)
                },
            );

            question_frequency
                .values()
                .filter(|freq| **freq == people)
                .count()
        })
        .sum();
    println!("Part Two: {answer}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let groups = input.split("\n\n").collect::<Vec<_>>();

    part_one(&groups);
    part_two(&groups);
    Ok(())
}
