use std::io::{self, Read};
use std::ops::AddAssign;

#[derive(Clone, Copy)]
struct Point(usize, usize);

struct Hill<'a> {
    pattern: Vec<&'a str>,
    width: usize,
}

struct Toboggan {
    slope: Point,
}

struct TobogganRide<'r, 'h> {
    toboggan: &'r Toboggan,
    cursor: Point,
    hill: &'r Hill<'h>,
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

impl Point {
    fn normalize_x(&mut self, divisor: usize) {
        self.0 %= divisor;
    }
}

impl From<&(usize, usize)> for Point {
    fn from((x, y): &(usize, usize)) -> Self {
        Self(*x, *y)
    }
}

impl<'h> FromIterator<&'h str> for Hill<'h> {
    fn from_iter<T: IntoIterator<Item = &'h str>>(iter: T) -> Self {
        let pattern = iter.into_iter().collect::<Vec<_>>();
        let width = pattern[0].len();

        Self { pattern, width }
    }
}

impl Toboggan {
    fn ride<'r, 'h>(&'r self, start: Point, hill: &'r Hill<'h>) -> TobogganRide<'r, 'h> {
        TobogganRide {
            toboggan: self,
            cursor: start,
            hill,
        }
    }
}

impl<'r, 'h> TobogganRide<'r, 'h> {
    fn str_at_cursor(&self) -> Option<&'h str> {
        let cursor = self.cursor;

        self.hill
            .pattern
            .get(cursor.1)?
            .get(cursor.0..(cursor.0 + 1))
    }

    fn advance(&mut self) {
        self.cursor += self.toboggan.slope;
        self.cursor.normalize_x(self.hill.width);
    }
}

impl<'r, 'h> Iterator for TobogganRide<'r, 'h> {
    type Item = &'h str;

    fn next(&mut self) -> Option<Self::Item> {
        let str_at_cursor = self.str_at_cursor();
        self.advance();
        str_at_cursor
    }
}

fn part_one(hill: &Hill<'_>) {
    let toboggan = Toboggan { slope: Point(3, 1) };

    let origin = Point(0, 0);
    let trees_hit = toboggan
        .ride(origin, hill)
        .filter(|tile| *tile == "#")
        .count();

    println!("Part One: {trees_hit}");
}

fn part_two(hill: &Hill<'_>) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let origin = Point(0, 0);
    let tree_product: usize = slopes
        .iter()
        .map(|s| {
            let toboggan = Toboggan { slope: s.into() };
            toboggan
                .ride(origin, hill)
                .filter(|tile| *tile == "#")
                .count()
        })
        .product();

    println!("Part Two: {tree_product}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let hill = input.lines().collect();

    part_one(&hill);
    part_two(&hill);

    Ok(())
}
