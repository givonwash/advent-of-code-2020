use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    ops::{Deref, DerefMut},
};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RuleParser;

#[derive(Clone)]
struct BagRule<'a> {
    quantity: u32,
    bag: &'a str,
}

#[derive(Default)]
struct Bags<'a>(HashMap<&'a str, Vec<BagRule<'a>>>);

impl<'a> BagRule<'a> {
    fn scaled_by(&self, factor: u32) -> Self {
        Self {
            quantity: self.quantity * factor,
            bag: self.bag,
        }
    }
}

impl<'a> Deref for Bags<'a> {
    type Target = HashMap<&'a str, Vec<BagRule<'a>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Bags<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> FromIterator<(&'a str, BagRule<'a>)> for Bags<'a> {
    fn from_iter<T: IntoIterator<Item = (&'a str, BagRule<'a>)>>(iter: T) -> Self {
        let mut bags = Self::default();

        for (parent, child) in iter {
            let children = bags.entry(parent).or_insert_with(Default::default);
            children.push(child);
        }

        bags
    }
}

fn part_one<'a, I>(rules: I)
where
    I: Iterator<Item = (&'a str, Pairs<'a, Rule>)>,
{
    let bags: Bags<'a> = rules
        .flat_map(|(parent, children)| {
            children.filter_map(|child_pair| match child_pair.as_rule() {
                Rule::non_empty_rule => {
                    let mut child_rule = child_pair.into_inner();
                    let quantity = child_rule.next().unwrap().as_str().parse().unwrap();
                    let child = child_rule
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    Some((
                        child,
                        BagRule {
                            quantity,
                            bag: parent,
                        },
                    ))
                }
                Rule::empty_rule => None,
                _ => unreachable!(),
            })
        })
        .collect();

    let mut explored = HashSet::new();
    let mut unexplored = bags
        .get("shiny gold")
        .expect("Could not find shiny gold bag")
        .iter()
        .map(|br| br.bag)
        .collect::<Vec<_>>();

    while let Some(child) = unexplored.pop() {
        explored.insert(child);
        if let Some(parents) = bags.get(child) {
            unexplored.extend(
                parents
                    .iter()
                    .filter_map(|p| (!explored.contains(p.bag)).then(|| p.bag)),
            );
        }
    }

    println!("Part One: {}", explored.len());
}

fn part_two<'a, I>(rules: I)
where
    I: Iterator<Item = (&'a str, Pairs<'a, Rule>)>,
{
    let bags: Bags<'a> = rules
        .flat_map(|(parent, children)| {
            children.filter_map(move |child_pair| match child_pair.as_rule() {
                Rule::non_empty_rule => {
                    let mut child_rule = child_pair.into_inner();
                    let quantity = child_rule.next().unwrap().as_str().parse().unwrap();
                    let child = child_rule
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    Some((
                        parent,
                        BagRule {
                            quantity,
                            bag: child,
                        },
                    ))
                }
                Rule::empty_rule => None,
                _ => unreachable!(),
            })
        })
        .collect();

    let mut total = 0;
    let mut bag_stack = bags
        .get("shiny gold")
        .cloned()
        .expect("Could not find shiny gold bag");

    while let Some(rule) = bag_stack.pop() {
        total += rule.quantity;
        if let Some(children) = bags.get(rule.bag) {
            bag_stack.extend(children.iter().map(|br| br.scaled_by(rule.quantity)))
        }
    }

    println!("Part Two: {total}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bags = input.lines().map(|line| {
        let mut definition = RuleParser::parse(Rule::definition, line)
            .expect("Failed to parse bag definition")
            .next()
            .unwrap()
            .into_inner();
        let parent = definition
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str();
        let children = definition.next().unwrap().into_inner();
        (parent, children)
    });

    part_one(bags.clone());
    part_two(bags);
    Ok(())
}
