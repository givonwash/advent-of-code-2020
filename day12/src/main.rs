use std::{
    io::{self, Read},
    ops::Neg,
    str::FromStr,
};

#[derive(Clone, Copy)]
#[repr(u8)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug)]
struct ParseDirectionError;

#[derive(Clone, Copy)]
#[repr(u8)]
enum Rotation {
    Flip = 2,
    Left = 3,
    Right = 1,
}

#[derive(Clone, Copy)]
enum Instruction {
    Shift {
        direction: Direction,
        magnitude: i64,
    },
    Push(i64),
    Turn(Rotation),
}

#[derive(Debug)]
struct ParseInstructionError;

#[derive(Clone, Copy, Debug)]
struct Point(i64, i64);

#[derive(Clone, Copy, Debug)]
struct Ship {
    location: Point,
}

struct OrientedShipRide<I>
where
    I: Iterator<Item = Instruction>,
{
    instructions: I,
    ship: Ship,
    orientation: Direction,
}

struct WaypointShipRide<I>
where
    I: Iterator<Item = Instruction>,
{
    instructions: I,
    ship: Ship,
    waypoint: Point,
}

impl Point {
    fn rotate(&mut self, rotation: Rotation) {
        use Rotation::*;

        let Point(x, y) = self;

        *self = match rotation {
            Right => Point(*y, x.neg()),
            Flip => Point(x.neg(), y.neg()),
            Left => Point(y.neg(), *x),
        };
    }
}

impl Direction {
    fn rotate(&mut self, n: u8) {
        let start = *self as u8;
        let finish = ((start + n) % 4).try_into().unwrap();
        *self = finish;
    }
}

impl TryFrom<u8> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Direction::*;

        match value {
            0 => Ok(North),
            1 => Ok(East),
            2 => Ok(South),
            3 => Ok(West),
            _ => Err(ParseDirectionError),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use Instruction::*;
        use Rotation::*;

        let (operation, args) = instruction.split_at(1);

        match operation {
            "F" => Ok(Push(args.parse().unwrap())),
            "N" => Ok(Shift {
                direction: North,
                magnitude: args.parse().unwrap(),
            }),
            "E" => Ok(Shift {
                direction: East,
                magnitude: args.parse().unwrap(),
            }),
            "S" => Ok(Shift {
                direction: South,
                magnitude: args.parse::<i64>().unwrap().neg(),
            }),
            "W" => Ok(Shift {
                direction: West,
                magnitude: args.parse::<i64>().unwrap().neg(),
            }),
            "L" => match args {
                "90" => Ok(Turn(Left)),
                "180" => Ok(Turn(Flip)),
                "270" => Ok(Turn(Right)),
                _ => Err(ParseInstructionError),
            },
            "R" => match args {
                "90" => Ok(Turn(Right)),
                "180" => Ok(Turn(Flip)),
                "270" => Ok(Turn(Left)),
                _ => Err(ParseInstructionError),
            },
            _ => Err(ParseInstructionError),
        }
    }
}

impl Ship {
    fn ride_with_orientation<I>(
        self,
        instructions: I,
        orientation: Direction,
    ) -> OrientedShipRide<I>
    where
        I: Iterator<Item = Instruction>,
    {
        OrientedShipRide {
            instructions,
            orientation,
            ship: self,
        }
    }

    fn ride_with_waypoint<I>(self, instructions: I, waypoint: Point) -> WaypointShipRide<I>
    where
        I: Iterator<Item = Instruction>,
    {
        WaypointShipRide {
            instructions,
            ship: self,
            waypoint,
        }
    }
}

impl<I> Iterator for OrientedShipRide<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = Ship;

    fn next(&mut self) -> Option<Self::Item> {
        self.instructions.next().map(|instruction| {
            use Direction::*;
            use Instruction::*;

            let OrientedShipRide {
                orientation, ship, ..
            } = self;

            match instruction {
                Shift {
                    direction,
                    magnitude,
                } => {
                    match direction {
                        North | South => {
                            ship.location.1 += magnitude;
                        }
                        East | West => {
                            ship.location.0 += magnitude;
                        }
                    };
                }
                Push(magnitude) => {
                    match orientation {
                        North => {
                            ship.location.1 += magnitude;
                        }
                        South => {
                            ship.location.1 -= magnitude;
                        }
                        East => {
                            ship.location.0 += magnitude;
                        }
                        West => {
                            ship.location.0 -= magnitude;
                        }
                    };
                }
                Turn(rotation) => {
                    orientation.rotate(rotation as u8);
                }
            };

            *ship
        })
    }
}

impl<I> Iterator for WaypointShipRide<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = Ship;

    fn next(&mut self) -> Option<Self::Item> {
        self.instructions.next().map(|instruction| {
            use Direction::*;
            use Instruction::*;

            let WaypointShipRide { ship, waypoint, .. } = self;

            match instruction {
                Shift {
                    direction,
                    magnitude,
                } => {
                    match direction {
                        North | South => {
                            waypoint.1 += magnitude;
                        }
                        East | West => {
                            waypoint.0 += magnitude;
                        }
                    };
                }
                Push(magnitude) => {
                    ship.location.0 += waypoint.0 * magnitude;
                    ship.location.1 += waypoint.1 * magnitude;
                }
                Turn(rotation) => {
                    waypoint.rotate(rotation);
                }
            };

            *ship
        })
    }
}

fn part_one(instructions: impl Iterator<Item = Instruction>) {
    let ship = Ship {
        location: Point(0, 0),
    };

    let answer = ship
        .ride_with_orientation(instructions, Direction::East)
        .last()
        .map(|s| s.location.0.abs() + s.location.1.abs());

    println!("Part One: {answer:?}")
}

fn part_two(instructions: impl Iterator<Item = Instruction>) {
    let ship = Ship {
        location: Point(0, 0),
    };

    let answer = ship
        .ride_with_waypoint(instructions, Point(10, 1))
        .last()
        .map(|s| s.location.0.abs() + s.location.1.abs());

    println!("Part Two: {answer:?}")
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instructions = input.lines().map(|line| line.parse().unwrap());

    part_one(instructions.clone());
    part_two(instructions);

    Ok(())
}
