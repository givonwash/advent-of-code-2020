use std::io::{self, Read};
use std::str::FromStr;

struct Seat(u32);

#[derive(Debug)]
enum ParseSeatError {
    InvalidChars,
    InvalidLength,
}

fn parse_binary(s: &str, one: char, zero: char) -> Result<u32, ParseSeatError> {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if c == one {
                Ok(2u32.pow(i as u32))
            } else if c == zero {
                Ok(0)
            } else {
                Err(ParseSeatError::InvalidChars)
            }
        })
        .try_fold(0, |acc, res| res.map(|n| acc + n))
}

impl FromStr for Seat {
    type Err = ParseSeatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, column) = s.split_at(7);
        if column.len() > 3 {
            Err(Self::Err::InvalidLength)
        } else {
            let row = parse_binary(row, 'B', 'F')?;
            let column = parse_binary(column, 'R', 'L')?;

            Ok(Self((row * 8) + column))
        }
    }
}

fn part_one(seats: &[Seat]) {
    let answer = seats.iter().map(|s| s.0).max();

    println!("Part One: {answer:?}");
}

fn part_two(seats: &[Seat]) {
    let (min, max, sum): (Option<u32>, Option<u32>, u32) =
        seats
            .iter()
            .map(|s| s.0)
            .fold((None, None, 0), |(amin, amax, asum), id| {
                let min = amin.map(|mn| mn.min(id)).or(Some(id));
                let max = amax.map(|mx| mx.max(id)).or(Some(id));
                let sum = asum + id;
                (min, max, sum)
            });

    let answer = match (min, max, sum) {
        (Some(min), Some(max), sum) => {
            let to_max = max * (max + 1) / 2;
            let to_min = min * (min - 1) / 2;
            Some(to_max - to_min - sum)
        }
        _ => None,
    };

    println!("Part Two: {answer:?}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let seats = input
        .lines()
        .map(|seat| seat.parse().expect("Invalid Boarding Pass given"))
        .collect::<Vec<_>>();

    part_one(&seats);
    part_two(&seats);

    Ok(())
}
