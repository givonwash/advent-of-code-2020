mod errors;

use crate::errors::{
    CheckPassportError, ParseEyeColorError, ParseHeightError, ParsePassportError,
    ParsePassportIntError, ParseRgbError, PassportInvalidLogicError,
};

use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
    static ref KEY_VAL_RE: Regex = Regex::new(r"\b(?P<key>\w{3}):(?P<value>#?\w+)").unwrap();
}
/// Passport of a single individual
struct Passport {
    birth_year: PassportField<Year, ParsePassportIntError>,
    issue_year: PassportField<Year, ParsePassportIntError>,
    expiration_year: PassportField<Year, ParsePassportIntError>,
    height: PassportField<Height, ParseHeightError>,
    hair_color: PassportField<Rgb, ParseRgbError>,
    eye_color: PassportField<EyeColor, ParseEyeColorError>,
    passport_id: PassportField<PassportID, ParsePassportIntError>,
    country_id: PassportField<u32, ParseIntError>,
}

/// All states a passport field can be in.
///
/// In part one we are not concerned about the validity of any given field, thus if a given
/// passport has _something_ for all fields (with the exception of the `country_id`) field, then
/// that passport is considered __valid__.
///
/// In part two we _are_ concerned about the validity of all the passports fields, thus a passport
/// field having _something_ is not enough. We need to "check" this field for any parsing or
/// logical errors.
enum PassportField<T, E> {
    Unchecked(Option<Result<T, E>>),
    CheckedValid(T),
    CheckedInvalid(CheckPassportError<E>),
}

/// RGB values from the hex specified with hair color and eye color
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

/// Allowed units for a passport to express length in
#[derive(Clone, Copy)]
enum LengthUnit {
    Centimeters,
    Inches,
}

/// Height of the passport holder
#[derive(Clone, Copy)]
struct Height {
    unit: LengthUnit,
    value: u32,
}

/// Allowed eye colors
#[derive(Clone, Copy)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

#[derive(Clone, Copy)]
struct PassportID(u32);

#[derive(Clone, Copy)]
struct Year(u16);

impl FromStr for Rgb {
    type Err = ParseRgbError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        if let Some(h) = hex.strip_prefix('#') {
            match h.len() {
                n if n > 6 => Err(Self::Err::TooFewDigits),
                n if n < 6 => Err(Self::Err::TooManyDigits),
                _ => {
                    let as_int = u32::from_str_radix(h, 16).or(Err(Self::Err::InvalidChars))?;
                    if as_int <= 0xffffff {
                        let r = (as_int >> 16) as u8;
                        let g = ((as_int >> 8) & (u32::MAX >> 24)) as u8;
                        let b = (as_int & (u32::MAX >> 24)) as u8;
                        Ok(Self { r, g, b })
                    } else {
                        Err(Self::Err::ValueTooLarge)
                    }
                }
            }
        } else if hex.is_empty() {
            Err(Self::Err::NoValue)
        } else {
            Err(Self::Err::MissingHashtag)
        }
    }
}

impl FromStr for LengthUnit {
    type Err = ParseHeightError;

    fn from_str(length: &str) -> Result<Self, Self::Err> {
        match length {
            "in" => Ok(Self::Inches),
            "cm" => Ok(Self::Centimeters),
            "" => Err(Self::Err::NoUnit),
            _ => Err(Self::Err::InvalidUnit),
        }
    }
}

impl FromStr for Height {
    type Err = ParseHeightError;

    fn from_str(height: &str) -> Result<Self, Self::Err> {
        let up_to = height.len() - 2;
        let value = height
            .get(..up_to)
            .ok_or(Self::Err::NoValue)?
            .parse()
            .or(Err(Self::Err::InvalidValue))?;
        let unit = height.get(up_to..).ok_or(Self::Err::NoUnit)?.parse()?;
        Ok(Self { unit, value })
    }
}

impl FromStr for EyeColor {
    type Err = ParseEyeColorError;

    fn from_str(eye_color: &str) -> Result<Self, Self::Err> {
        match eye_color {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            "" => Err(Self::Err::NoValue),
            _ => Err(Self::Err::InvalidValue),
        }
    }
}

impl FromStr for PassportID {
    type Err = ParsePassportIntError;

    fn from_str(pid: &str) -> Result<Self, Self::Err> {
        match pid.parse() {
            Ok(pid_parsed) => match pid.len() {
                n if n < 9 => Err(Self::Err::TooFewDigits),
                n if n > 9 => Err(Self::Err::TooManyDigits),
                _ => Ok(Self(pid_parsed)),
            },
            Err(_) => {
                if pid.is_empty() {
                    Err(Self::Err::NoValue)
                } else {
                    Err(Self::Err::InvalidChars)
                }
            }
        }
    }
}

impl FromStr for Year {
    type Err = ParsePassportIntError;

    fn from_str(year: &str) -> Result<Self, Self::Err> {
        match year.parse() {
            Ok(year_parsed) => match year.len() {
                n if n < 4 => Err(Self::Err::TooFewDigits),
                n if n > 4 => Err(Self::Err::TooManyDigits),
                _ => Ok(Self(year_parsed)),
            },
            Err(_) => {
                if year.is_empty() {
                    Err(Self::Err::NoValue)
                } else {
                    Err(Self::Err::InvalidChars)
                }
            }
        }
    }
}

impl Default for Passport {
    fn default() -> Self {
        Self {
            birth_year: PassportField::Unchecked(None),
            issue_year: PassportField::Unchecked(None),
            expiration_year: PassportField::Unchecked(None),
            height: PassportField::Unchecked(None),
            hair_color: PassportField::Unchecked(None),
            eye_color: PassportField::Unchecked(None),
            passport_id: PassportField::Unchecked(None),
            country_id: PassportField::Unchecked(None),
        }
    }
}

macro_rules! insert_capture_value {
    ($passport:ident.$attribute:ident , $captures:ident) => {
        let $attribute = $captures.name("value").unwrap().as_str().parse();
        $passport.$attribute = PassportField::Unchecked(Some($attribute));
    };
}

impl FromStr for Passport {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        for captures in KEY_VAL_RE.captures_iter(s) {
            let key = captures.name("key").unwrap().as_str();
            match key {
                "byr" => {
                    insert_capture_value!(passport.birth_year, captures);
                }
                "iyr" => {
                    insert_capture_value!(passport.issue_year, captures);
                }
                "eyr" => {
                    insert_capture_value!(passport.expiration_year, captures);
                }
                "hgt" => {
                    insert_capture_value!(passport.height, captures);
                }
                "hcl" => {
                    insert_capture_value!(passport.hair_color, captures);
                }
                "ecl" => {
                    insert_capture_value!(passport.eye_color, captures);
                }
                "pid" => {
                    insert_capture_value!(passport.passport_id, captures);
                }
                "cid" => {
                    insert_capture_value!(passport.country_id, captures);
                }
                _ => return Err(Self::Err::UnknownKey),
            }
        }
        Ok(passport)
    }
}

impl<T, E> PassportField<T, E> {
    fn validate<V>(&mut self, validator: V)
    where
        V: Fn(&T) -> Result<T, CheckPassportError<E>>,
        E: Clone,
    {
        if let PassportField::Unchecked(unchecked_field) = self {
            match unchecked_field {
                Some(Ok(field)) => match validator(field) {
                    Ok(v) => {
                        *self = PassportField::CheckedValid(v);
                    }
                    Err(e) => *self = PassportField::CheckedInvalid(e),
                },
                Some(Err(err)) => {
                    *self =
                        PassportField::CheckedInvalid(CheckPassportError::ParsingError(err.clone()))
                }
                None => *self = PassportField::CheckedInvalid(CheckPassportError::DoesNotExist),
            }
        }
    }
}

fn part_one<'a, I: Iterator<Item = &'a str>>(text_blocks: I) {
    let valid_passport_count = text_blocks
        .map(|p| Passport::from_str(p))
        .filter(|p| {
            if let Ok(passport) = p {
                matches!(
                    (
                        &passport.birth_year,
                        &passport.issue_year,
                        &passport.expiration_year,
                        &passport.height,
                        &passport.hair_color,
                        &passport.eye_color,
                        &passport.passport_id
                    ),
                    (
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                        PassportField::Unchecked(Some(_)),
                    )
                )
            } else {
                false
            }
        })
        .count();

    println!("Part One: {}", valid_passport_count);
}

fn part_two<'a, I: Iterator<Item = &'a str>>(text_blocks: I) {
    let mut valid_count = 0;
    for block in text_blocks {
        let passport = Passport::from_str(block);
        if let Ok(mut p) = passport {
            p.birth_year.validate(|byr| {
                if (1920..=2002).contains(&byr.0) {
                    Ok(*byr)
                } else {
                    Err(CheckPassportError::LogicError(
                        PassportInvalidLogicError::BirthYear,
                    ))
                }
            });
            p.issue_year.validate(|iyr| {
                if (2010..=2020).contains(&iyr.0) {
                    Ok(*iyr)
                } else {
                    Err(CheckPassportError::LogicError(
                        PassportInvalidLogicError::IssueYear,
                    ))
                }
            });
            p.expiration_year.validate(|eyr| {
                if (2020..=2030).contains(&eyr.0) {
                    Ok(*eyr)
                } else {
                    Err(CheckPassportError::LogicError(
                        PassportInvalidLogicError::ExpirationYear,
                    ))
                }
            });
            p.height.validate(|hgt| {
                if let LengthUnit::Inches = hgt.unit {
                    if (59..=76).contains(&hgt.value) {
                        Ok(*hgt)
                    } else {
                        Err(CheckPassportError::LogicError(
                            PassportInvalidLogicError::Height,
                        ))
                    }
                } else if (150..=193).contains(&hgt.value) {
                    Ok(*hgt)
                } else {
                    Err(CheckPassportError::LogicError(
                        PassportInvalidLogicError::Height,
                    ))
                }
            });
            p.hair_color.validate(|hcl| Ok(*hcl));
            p.eye_color.validate(|ecl| Ok(*ecl));
            p.passport_id.validate(|pid| Ok(*pid));

            if let (
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
                PassportField::CheckedValid(_),
            ) = (
                p.birth_year,
                p.issue_year,
                p.expiration_year,
                p.height,
                p.hair_color,
                p.eye_color,
                p.passport_id,
            ) {
                valid_count += 1;
            }
        }
    }

    println!("Part Two: {}", valid_count);
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Solving for day 04.");

    let text_blocks = input.split("\n\n");

    part_one(text_blocks.clone());
    part_two(text_blocks);

    Ok(())
}
