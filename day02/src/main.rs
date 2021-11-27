use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

lazy_static! {
    static ref PASSWORD_RECORD_RE: Regex =
        Regex::new(r"^(?P<lhs>\d+)-(?P<rhs>\d+)\s(?P<pattern>.):\s(?P<password>.*)$").unwrap();
}

enum PolicyKind {
    PatternCount,
    PatternPosition,
}

struct Policy<'a> {
    lhs: usize,
    rhs: usize,
    pattern: &'a str,
}

struct PasswordRecord<'a> {
    policy: Policy<'a>,
    password: &'a str,
}

impl<'a> PasswordRecord<'a> {
    fn meets_policy(&self, kind: PolicyKind) -> bool {
        let policy = &self.policy;

        match kind {
            PolicyKind::PatternCount => {
                let pcount = self.password.matches(policy.pattern).count();
                (policy.lhs..=policy.rhs).contains(&pcount)
            }
            PolicyKind::PatternPosition => {
                let in_lhs = self
                    .password
                    .get((policy.lhs - 1)..policy.lhs)
                    .map(|s| s == policy.pattern)
                    .unwrap_or(false);
                let in_rhs = self
                    .password
                    .get((policy.rhs - 1)..policy.rhs)
                    .map(|s| s == policy.pattern)
                    .unwrap_or(false);

                in_lhs ^ in_rhs
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for PasswordRecord<'a> {
    type Error = &'static str;

    fn try_from(record: &'a str) -> Result<Self, Self::Error> {
        let caps = PASSWORD_RECORD_RE
            .captures(record)
            .ok_or("Password Regex Failed")?;

        let lhs = caps
            .name("lhs")
            .ok_or("`lhs` capture group missing")?
            .as_str()
            .parse::<usize>()
            .or(Err("Could not parse `lhs` into usize"))?;
        let rhs = caps
            .name("rhs")
            .ok_or("`rhs` capture group missing")?
            .as_str()
            .parse::<usize>()
            .or(Err("Could not parse `rhs` into usize"))?;
        let pattern = caps
            .name("pattern")
            .ok_or("`pattern` capture group missing")?
            .as_str();
        let password = caps
            .name("password")
            .ok_or("`password` capture group missing")?
            .as_str();

        Ok(PasswordRecord {
            policy: Policy { lhs, rhs, pattern },
            password,
        })
    }
}

fn part_one<'a, I>(records: I)
where
    I: Iterator<Item = PasswordRecord<'a>>,
{
    let answer = records
        .filter(|rec| rec.meets_policy(PolicyKind::PatternCount))
        .count();

    println!("Part One: {}", answer);
}

fn part_two<'a, I>(records: I)
where
    I: Iterator<Item = PasswordRecord<'a>>,
{
    let answer = records
        .filter(|rec| rec.meets_policy(PolicyKind::PatternPosition))
        .count();

    println!("Part Two: {}", answer);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let records = input.lines().map(TryFrom::try_from).filter_map(Result::ok);

    part_one(records.clone());
    part_two(records);

    Ok(())
}
