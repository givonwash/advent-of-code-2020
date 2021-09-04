use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref PASSWORD_RECORD_RE: Regex =
        Regex::new(r"^(?P<start>\d+)-(?P<end>\d+)\s+(?P<pattern>\w):\s+(?P<password>.*)$").unwrap();
}

struct PasswordRecord<'a> {
    start: usize,
    end: usize,
    pattern: &'a str,
    password: &'a str,
}

impl<'a> PasswordRecord<'a> {
    fn is_valid_pattern_count(&self) -> bool {
        let pattern_count = self.password.match_indices(self.pattern).count();
        (self.start..=self.end).contains(&pattern_count)
    }

    fn is_valid_pattern_position(&self) -> bool {
        let valid_count = self
            .password
            .match_indices(self.pattern)
            .filter(|(i, _)| (i + 1) == self.start || (i + 1) == self.end)
            .count();
        valid_count == 1
    }
}

impl<'a> TryFrom<&'a str> for PasswordRecord<'a> {
    type Error = Box<dyn Error>;

    fn try_from(s: &'a str) -> ::std::result::Result<Self, Self::Error> {
        let captures = PASSWORD_RECORD_RE
            .captures(s)
            .ok_or("Failed to parse password record")?;

        let start = captures
            .name("start")
            .ok_or("No 'start' found in PasswordRecord")?
            .as_str()
            .parse()?;
        let end = captures
            .name("end")
            .ok_or("No 'end' in PasswordRecord")?
            .as_str()
            .parse()?;
        let pattern = captures
            .name("pattern")
            .ok_or("No 'pattern' in PasswordRecord")?
            .as_str();
        let password = captures
            .name("password")
            .ok_or("No 'password' in PasswordRecord")?
            .as_str();

        Ok(PasswordRecord {
            start,
            end,
            pattern,
            password,
        })
    }
}

fn part_one(input: &str) -> Result<()> {
    let mut count = 0;
    for record in input.lines().map(|l| PasswordRecord::try_from(l)) {
        if record?.is_valid_pattern_count() {
            count += 1;
        }
    }

    println!("Part one: {}", count);
    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let mut count = 0;
    for record in input.lines().map(|l| PasswordRecord::try_from(l)) {
        if record?.is_valid_pattern_position() {
            count += 1;
        }
    }

    println!("Part two: {}", count);
    Ok(())
}

fn main() -> Result<()> {
    println!("Solving for day 02.");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part_one(&input)?;
    part_two(&input)?;

    Ok(())
}
