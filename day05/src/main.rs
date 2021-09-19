use std::convert::{TryFrom, TryInto};
use std::io::{self, Read};

struct Seat {
    id: u32,
}

#[derive(Clone, Copy)]
struct Row(u32);

#[derive(Clone, Copy)]
struct Column(u32);

#[derive(Debug)]
enum ParseBoardingPassError {
    InvalidChars,
    TooManyChars,
}

impl TryFrom<&str> for Row {
    type Error = ParseBoardingPassError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 7 {
            Err(ParseBoardingPassError::TooManyChars)
        } else {
            Ok(Row(value
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    if c == 'B' {
                        Ok(2u32.pow(i as u32))
                    } else if c == 'F' {
                        Ok(0)
                    } else {
                        Err(ParseBoardingPassError::InvalidChars)
                    }
                })
                .try_fold(0, |acc, res| res.map(|x| acc + x))?))
        }
    }
}

impl TryFrom<&str> for Column {
    type Error = ParseBoardingPassError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 3 {
            Err(ParseBoardingPassError::TooManyChars)
        } else {
            Ok(Column(
                value
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == 'R' {
                            Ok(2u32.pow(i as u32))
                        } else if c == 'L' {
                            Ok(0)
                        } else {
                            Err(ParseBoardingPassError::InvalidChars)
                        }
                    })
                    .try_fold(0, |acc, res| res.map(|x| acc + x))?,
            ))
        }
    }
}

impl From<(Row, Column)> for Seat {
    fn from((row, column): (Row, Column)) -> Self {
        Self {
            id: (row.0 * 8) + column.0,
        }
    }
}

impl TryFrom<&str> for Seat {
    type Error = ParseBoardingPassError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (row, column) = value.split_at(7);
        Ok(Seat::from((row.try_into()?, column.try_into()?)))
    }
}

fn part_one<I>(input: I)
where
    I: Iterator<Item = Seat>,
{
    let answer = input.map(|s| s.id).max();
    match answer {
        Some(ans) => {
            println!("Part One: {}", ans);
        }
        None => {
            println!("Part One: Could not find answer");
        }
    }
}

fn part_two<I>(input: I)
where
    I: Iterator<Item = Seat>,
{
    let (min, max, sum) =
        input
            .map(|s| s.id)
            .fold((None, None, 0), |(mut amin, mut amax, mut asum), id| {
                amax = amax.or(Some(id)).map(|mx| mx.max(id));
                amin = amin.or(Some(id)).map(|mn| mn.min(id));
                asum += id;
                (amin, amax, asum)
            });
    match (min, max, sum) {
        (Some(min), Some(max), sum) => {
            // sum of natural numbers up to max
            let upto_max = max * (max + 1) / 2;
            // sum of natural numbers up to (min - 1)
            let upto_min = (min - 1) * min / 2;
            // diff between upto_max and upto_min gives what the sum of all seat IDs should equal
            // if there were no gaps. subtracting the actual sum of seat IDs returns our seat ID
            let answer = upto_max - upto_min - sum;
            println!("Part Two: {}", answer);
        }
        (_, _, _) => {
            println!("Part Two: Could not find answer")
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.lines().map(|l| Seat::try_from(l).unwrap());
    part_one(input.clone());
    part_two(input);
    Ok(())
}
