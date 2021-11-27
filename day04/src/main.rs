use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

lazy_static! {
    static ref KEY_VALUE_RE: Regex = Regex::new(r"\b(?P<key>\w{3}):(?P<value>#?\w+)").unwrap();
}

enum DistanceUnit {
    Inches,
    Centimeters,
}

struct Height(usize, DistanceUnit);

impl FromStr for Height {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hgt, units) = s.split_at(s.len() - 2);

        match units {
            "in" => Ok(Self(
                hgt.parse()
                    .ok()
                    .filter(|hgt| (59..=76).contains(hgt))
                    .ok_or(Self::Err::InvalidField(Field::Height))?,
                DistanceUnit::Inches,
            )),
            "cm" => Ok(Self(
                hgt.parse()
                    .ok()
                    .filter(|hgt| (150..=193).contains(hgt))
                    .ok_or(Self::Err::InvalidField(Field::Height))?,
                DistanceUnit::Centimeters,
            )),
            _ => Err(Self::Err::InvalidField(Field::Height)),
        }
    }
}

enum ParsePassportError {
    MissingField(Field),
    InvalidField(Field),
}

enum Field {
    PassportID,
    CountryID,
    BirthYear,
    ExpirationYear,
    IssueYear,
    Height,
    HairColor,
    EyeColor,
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(Self::Err::InvalidField(Field::EyeColor)),
        }
    }
}

#[allow(dead_code)]
struct Passport<'a> {
    id: usize,
    country_id: Result<usize, ParsePassportError>,
    birth_year: usize,
    expiration_year: usize,
    issue_year: usize,
    height: Height,
    hair_color: &'a str,
    eye_color: EyeColor,
}

impl<'a> TryFrom<HashMap<&'a str, &'a str>> for Passport<'a> {
    type Error = ParsePassportError;

    fn try_from(fields: HashMap<&'a str, &'a str>) -> Result<Self, Self::Error> {
        let id = fields
            .get("pid")
            .ok_or(Self::Error::MissingField(Field::PassportID))
            .map(|pid| {
                (pid.len() == 9)
                    .then(|| pid.parse().ok())
                    .flatten()
                    .ok_or(Self::Error::InvalidField(Field::PassportID))
            })?;

        let country_id = fields
            .get("cid")
            .ok_or(Self::Error::MissingField(Field::CountryID))
            .map(|cid| {
                cid.parse()
                    .map_err(|_| Self::Error::InvalidField(Field::CountryID))
            });

        let birth_year = fields
            .get("byr")
            .ok_or(Self::Error::MissingField(Field::BirthYear))
            .map(|byr| {
                (byr.len() == 4)
                    .then(|| byr.parse().ok().filter(|y| (1920..=2002).contains(y)))
                    .flatten()
                    .ok_or(Self::Error::InvalidField(Field::BirthYear))
            })?;

        let expiration_year = fields
            .get("eyr")
            .ok_or(Self::Error::MissingField(Field::ExpirationYear))
            .map(|eyr| {
                (eyr.len() == 4)
                    .then(|| eyr.parse().ok().filter(|y| (2020..=2030).contains(y)))
                    .flatten()
                    .ok_or(Self::Error::InvalidField(Field::ExpirationYear))
            })?;

        let issue_year = fields
            .get("iyr")
            .ok_or(Self::Error::MissingField(Field::IssueYear))
            .map(|iyr| {
                (iyr.len() == 4)
                    .then(|| iyr.parse().ok().filter(|y| (2010..=2020).contains(y)))
                    .flatten()
                    .ok_or(Self::Error::InvalidField(Field::IssueYear))
            })?;

        let height = fields
            .get("hgt")
            .ok_or(Self::Error::MissingField(Field::Height))?
            .parse();

        let hair_color = fields
            .get("hcl")
            .ok_or(Self::Error::MissingField(Field::HairColor))
            .map(|hcl| {
                let (prefix, hex) = hcl.split_at(1);
                if prefix == "#"
                    && hex.len() == 6
                    && hex.chars().all(|c| matches!(c, 'a'..='f' | '0'..='9'))
                {
                    Ok(hex)
                } else {
                    Err(Self::Error::InvalidField(Field::HairColor))
                }
            })?;

        let eye_color = fields
            .get("ecl")
            .ok_or(Self::Error::MissingField(Field::EyeColor))?
            .parse();

        Ok(Self {
            id: id?,
            country_id: match country_id {
                Ok(cid @ Ok(_)) => cid,
                Ok(err @ Err(_)) => err,
                Err(err) => Err(err),
            },
            birth_year: birth_year?,
            expiration_year: expiration_year?,
            issue_year: issue_year?,
            height: height?,
            hair_color: hair_color?,
            eye_color: eye_color?,
        })
    }
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = ParsePassportError;

    fn try_from(passport: &'a str) -> Result<Self, Self::Error> {
        KEY_VALUE_RE
            .captures_iter(passport)
            .map(|caps| {
                let key = caps.name("key").unwrap().as_str();
                let value = caps.name("value").unwrap().as_str();
                (key, value)
            })
            .collect::<HashMap<&'a str, &'a str>>()
            .try_into()
    }
}

fn part_one<'a, I>(passports: I)
where
    I: Iterator<Item = Result<Passport<'a>, ParsePassportError>>,
{
    let answer = passports
        .filter(|p| !matches!(p, Err(ParsePassportError::MissingField(_))))
        .count();

    println!("Part One: {}", answer);
}

fn part_two<'a, I>(passports: I)
where
    I: Iterator<Item = Result<Passport<'a>, ParsePassportError>>,
{
    let answer = passports.filter(Result::is_ok).count();

    println!("Part Two: {}", answer);
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = input.split("\n\n").map(TryFrom::try_from);

    part_one(passports.clone());
    part_two(passports);
    Ok(())
}
