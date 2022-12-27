use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

lazy_static! {
    static ref PASSWORD_RECORD_RE: Regex =
        Regex::new(r"^(?P<lhs>\d+)-(?P<rhs>\d+)\s(?P<pattern>.):\s(?P<password>.*)$").unwrap();
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
    fn is_count_compliant(&self) -> bool {
        let policy = &self.policy;
        let pcount = self.password.matches(policy.pattern).count();
        (policy.lhs..=policy.rhs).contains(&pcount)
    }

    fn is_position_compliant(&self) -> bool {
        let policy = &self.policy;
        (&self.password[(policy.lhs - 1)..policy.lhs] == policy.pattern)
            ^ (&self.password[(policy.rhs - 1)..policy.rhs] == policy.pattern)
    }
}

impl<'a> TryFrom<&'a str> for PasswordRecord<'a> {
    type Error = &'static str;

    fn try_from(record: &'a str) -> Result<Self, Self::Error> {
        let caps = PASSWORD_RECORD_RE
            .captures(record)
            .ok_or("Password Regex Failed")?;

        let lhs = caps.name("lhs").unwrap().as_str().parse().unwrap();
        let rhs = caps.name("rhs").unwrap().as_str().parse().unwrap();
        let pattern = caps.name("pattern").unwrap().as_str();
        let password = caps.name("password").unwrap().as_str();

        Ok(PasswordRecord {
            policy: Policy { lhs, rhs, pattern },
            password,
        })
    }
}

fn part_one(records: &[PasswordRecord<'_>]) {
    let answer = records
        .iter()
        .filter(|rec| rec.is_count_compliant())
        .count();

    println!("Part One: {answer}");
}

fn part_two(records: &[PasswordRecord<'_>]) {
    let answer = records
        .iter()
        .filter(|rec| rec.is_position_compliant())
        .count();

    println!("Part Two: {answer}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let records = input
        .lines()
        .map(TryFrom::try_from)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    part_one(&records);
    part_two(&records);

    Ok(())
}
