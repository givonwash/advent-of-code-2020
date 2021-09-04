use std::io::{self, Read};
use std::iter::{FromIterator, IntoIterator};
use std::ops::AddAssign;

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Hill<'a> {
    pattern: Vec<&'a str>,
    width: usize,
}

struct Toboggan {
    slope: Point,
}

struct TobogganRide<'a> {
    toboggan: &'a Toboggan,
    cursor: Point,
    hill: &'a Hill<'a>,
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Point {
    fn normalize_x(&mut self, divisor: usize) {
        self.x %= divisor;
    }
}

impl From<&(usize, usize)> for Point {
    fn from((x, y): &(usize, usize)) -> Self {
        Self { x: *x, y: *y }
    }
}

impl<'a> FromIterator<&'a str> for Hill<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let pattern = iter.into_iter().collect::<Vec<&'a str>>();
        let width = pattern[0].len();

        Self { pattern, width }
    }
}

impl Toboggan {
    fn ride<'a>(&'a self, start: Point, hill: &'a Hill<'a>) -> TobogganRide<'a> {
        TobogganRide {
            toboggan: self,
            cursor: start,
            hill,
        }
    }
}

impl<'a> TobogganRide<'a> {
    fn str_at_cursor(&self) -> Option<&'a str> {
        let cursor = self.cursor;

        self.hill
            .pattern
            .get(cursor.y)?
            .get(cursor.x..(cursor.x + 1))
    }

    #[inline]
    fn step(&mut self) {
        self.cursor += self.toboggan.slope;
        self.cursor.normalize_x(self.hill.width);
    }
}

impl<'a> Iterator for TobogganRide<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let str_at_cursor = self.str_at_cursor();
        self.step();
        str_at_cursor
    }
}

fn part_one<'a>(hill: &'a Hill<'a>) -> Result<()> {
    let toboggan = Toboggan {
        slope: Point { x: 3, y: 1 },
    };

    let origin = Point { x: 0, y: 0 };
    let trees_hit = toboggan
        .ride(origin, hill)
        .filter(|tile| *tile == "#")
        .count();

    println!("Part One: {}", trees_hit);

    Ok(())
}

fn part_two<'a>(hill: &'a Hill<'a>) -> Result<()> {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let origin = Point { x: 0, y: 0 };
    let tree_product: usize = slopes
        .iter()
        .map(|s| Toboggan { slope: s.into() })
        .map(|toboggan| {
            toboggan
                .ride(origin, hill)
                .filter(|tile| *tile == "#")
                .count()
        })
        .product();

    println!("Part two: {}", tree_product);

    Ok(())
}

fn main() -> Result<()> {
    println!("Solving for day 03.");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let hill = input.lines().collect();
    part_one(&hill)?;
    part_two(&hill)?;
    Ok(())
}
