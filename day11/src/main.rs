use std::{
    io::{self, Read},
    ops::RangeInclusive,
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Floor,
    Occupied,
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
}

enum SimulationMethod {
    Adjacent {
        /// how many tiles away can a tile be from another to consider them adjacent?
        radius: usize,
        /// the inclusive lower bound on how many occupied adjacent tiles cause an occupied seat to flip
        occupied_threshold: usize,
        /// the (inclusive) upper bound on how many occupied adjacent tiles cause an empty seat to flip
        empty_threshold: usize,
    },
    Visible {
        /// the inclusive lower bound on how many occupied visible tiles cause an occupied seat to flip
        occupied_threshold: usize,
        /// the (inclusive) upper bound on how many occupied visible tiles cause an empty seat to flip
        empty_threshold: usize,
    },
}

struct GridSimulator {
    /// method to use for simulation
    method: SimulationMethod,
    /// has the simulation reached a steady state?
    in_equilibrium: bool,
    /// the current simulated grid
    grid: Grid,
}

type Transformation = fn((usize, usize)) -> Option<(usize, usize)>;

const TRANSFORMATIONS: [Transformation; 8] = [
    |(row, col)| row.checked_add(1).zip(Some(col)),
    |(row, col)| row.checked_add(1).zip(col.checked_add(1)),
    |(row, col)| Some(row).zip(col.checked_add(1)),
    |(row, col)| row.checked_sub(1).zip(col.checked_add(1)),
    |(row, col)| row.checked_sub(1).zip(Some(col)),
    |(row, col)| row.checked_sub(1).zip(col.checked_sub(1)),
    |(row, col)| Some(row).zip(col.checked_sub(1)),
    |(row, col)| row.checked_add(1).zip(col.checked_sub(1)),
];

#[derive(Debug)]
struct ParseTileError;

#[derive(Debug)]
enum ParseGridError {
    InconsistentWidths,
}

impl Tile {
    fn is_seat(&self) -> bool {
        matches!(self, Tile::Empty | Tile::Occupied)
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Tile::Empty),
            '.' => Ok(Tile::Floor),
            '#' => Ok(Tile::Occupied),
            _ => Err(ParseTileError),
        }
    }
}

impl Grid {
    fn new(tiles: Vec<Tile>, width: usize) -> Self {
        Self { tiles, width }
    }

    fn contains_index(&self, index: usize) -> bool {
        (0..self.tiles.len()).contains(&index)
    }

    fn get_index(&self, location: (usize, usize)) -> Option<usize> {
        if self.contains_location(location) {
            Some(Self::make_index(self.width, location))
        } else {
            None
        }
    }

    fn make_index(width: usize, (row, col): (usize, usize)) -> usize {
        (width * row) + col
    }

    fn get_ranges(
        &self,
        (frow, fcol): (usize, usize),
        (trow, tcol): (usize, usize),
    ) -> Option<impl Iterator<Item = RangeInclusive<usize>>> {
        match (self.get_index((frow, fcol)), self.get_index((trow, tcol))) {
            (Some(from), Some(to)) => Some(Self::make_ranges(self.width, from, to)),
            (_, _) => None,
        }
    }

    fn make_ranges(
        width: usize,
        from: usize,
        to: usize,
    ) -> impl Iterator<Item = RangeInclusive<usize>> {
        let range_width = (to % width) - (from % width);

        (from..=to)
            .map(move |idx| idx..=(idx + range_width))
            .step_by(width)
    }

    fn contains_location(&self, (row, col): (usize, usize)) -> bool {
        row < self.rows() && col < self.width
    }

    fn get_location(&self, index: usize) -> Option<(usize, usize)> {
        if self.contains_index(index) {
            Some(Self::make_location(self.width, index))
        } else {
            None
        }
    }

    fn make_location(width: usize, index: usize) -> (usize, usize) {
        let row = index / width;
        let col = index % width;
        (row, col)
    }

    fn simulate(self, method: SimulationMethod) -> GridSimulator {
        GridSimulator {
            method,
            in_equilibrium: false,
            grid: self,
        }
    }

    fn rows(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn find_neighbor<S, T>(
        &self,
        starting_at: (usize, usize),
        is_neighbor: S,
        try_again_at: T,
    ) -> Option<(usize, Tile)>
    where
        S: Fn(&Tile) -> bool,
        T: Fn((usize, usize)) -> Option<(usize, usize)>,
    {
        let mut location = starting_at;

        while let Some(idx) = self.get_index(location) {
            let tile = &self.tiles[idx];

            if is_neighbor(tile) {
                return Some((idx, *tile));
            } else if let Some(next_location) = try_again_at(location) {
                location = next_location;
            } else {
                return None;
            }
        }

        None
    }
}

impl Iterator for GridSimulator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.in_equilibrium {
            None
        } else {
            let old_grid = self.grid.clone();
            let width = self.grid.width;
            let rows = old_grid.rows();

            let (total_changes, total_occupied) = match self.method {
                SimulationMethod::Adjacent {
                    radius,
                    occupied_threshold,
                    empty_threshold,
                } => self
                    .grid
                    .tiles
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(idx, tile)| match tile {
                        Tile::Floor => None,
                        _ => {
                            let (row, col) = old_grid.get_location(idx).unwrap();
                            let from = (row.saturating_sub(radius), col.saturating_sub(radius));
                            let to = ((row + radius).min(rows - 1), (col + radius).min(width - 1));
                            Some((idx, tile, old_grid.get_ranges(from, to).unwrap()))
                        }
                    })
                    .fold(
                        (0, 0),
                        |(total_changes, total_occupied), (idx, tile, ranges)| {
                            let adjacent_occupied = ranges
                                .map(|range| {
                                    if range.contains(&idx) {
                                        let normalized_idx = idx - *range.start();
                                        old_grid.tiles[range]
                                            .iter()
                                            .enumerate()
                                            .filter(|(i, t)| {
                                                *i != normalized_idx && matches!(t, Tile::Occupied)
                                            })
                                            .count()
                                    } else {
                                        old_grid.tiles[range]
                                            .iter()
                                            .filter(|t| matches!(t, Tile::Occupied))
                                            .count()
                                    }
                                })
                                .sum::<usize>();

                            match tile {
                                Tile::Occupied => {
                                    if adjacent_occupied >= occupied_threshold {
                                        *tile = Tile::Empty;
                                        (total_changes + 1, total_occupied)
                                    } else {
                                        (total_changes, total_occupied + 1)
                                    }
                                }
                                Tile::Empty => {
                                    if adjacent_occupied <= empty_threshold {
                                        *tile = Tile::Occupied;
                                        (total_changes + 1, total_occupied + 1)
                                    } else {
                                        (total_changes, total_occupied)
                                    }
                                }
                                Tile::Floor => unreachable!(),
                            }
                        },
                    ),
                SimulationMethod::Visible {
                    occupied_threshold,
                    empty_threshold,
                } => self
                    .grid
                    .tiles
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(idx, tile)| match tile {
                        Tile::Floor => None,
                        _ => {
                            let location = old_grid.get_location(idx).unwrap();

                            let occupied = TRANSFORMATIONS
                                .into_iter()
                                .filter_map(|transform| {
                                    transform(location)
                                        .and_then(|location| {
                                            old_grid.find_neighbor(
                                                location,
                                                Tile::is_seat,
                                                transform,
                                            )
                                        })
                                        .filter(|neigbor| matches!(neigbor, (_, Tile::Occupied)))
                                })
                                .count();

                            Some((tile, occupied))
                        }
                    })
                    .fold(
                        (0, 0),
                        |(total_changes, total_occupied), (tile, visibly_occupied)| match tile {
                            Tile::Occupied => {
                                if visibly_occupied >= occupied_threshold {
                                    *tile = Tile::Empty;
                                    (total_changes + 1, total_occupied)
                                } else {
                                    (total_changes, total_occupied + 1)
                                }
                            }
                            Tile::Empty => {
                                if visibly_occupied <= empty_threshold {
                                    *tile = Tile::Occupied;
                                    (total_changes + 1, total_occupied + 1)
                                } else {
                                    (total_changes, total_occupied)
                                }
                            }
                            Tile::Floor => unreachable!(),
                        },
                    ),
            };

            if total_changes == 0 {
                self.in_equilibrium = true;
                None
            } else {
                Some(total_occupied)
            }
        }
    }
}

fn part_one(grid: Grid) {
    let final_occupied = grid
        .simulate(SimulationMethod::Adjacent {
            radius: 1,
            occupied_threshold: 4,
            empty_threshold: 0,
        })
        .last();

    println!("Part One: {final_occupied:?}");
}

fn part_two(grid: Grid) {
    let final_occupied = grid
        .simulate(SimulationMethod::Visible {
            occupied_threshold: 5,
            empty_threshold: 0,
        })
        .last();

    println!("Part Two: {final_occupied:?}");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (width, tiles): (Option<usize>, Vec<Tile>) = input
        .lines()
        .map(|row| (row.len(), row))
        .try_fold(
            (None, Vec::with_capacity(input.len())),
            |(width, mut tiles), (len, row)| match width {
                Some(w) => {
                    if w != len {
                        Err(ParseGridError::InconsistentWidths)
                    } else {
                        tiles.extend(row.chars().map(|c| Tile::try_from(c).unwrap()));
                        Ok((width, tiles))
                    }
                }
                None => {
                    tiles.extend(row.chars().map(|c| Tile::try_from(c).unwrap()));
                    Ok((Some(len), tiles))
                }
            },
        )
        .unwrap();

    let grid = Grid::new(tiles, width.expect("No tiles passed to grid"));

    part_one(grid.clone());
    part_two(grid);

    Ok(())
}

mod test {
    use super::*;
    use Tile::*;

    #[allow(unused)]
    fn create_grid() -> Grid {
        #[rustfmt::skip]
        let tiles = vec![
        //     0         1         2         3
            Occupied, Occupied, Floor,    Occupied, // 0
            Empty,    Occupied, Empty,    Floor,    // 1
            Occupied, Empty,    Occupied, Empty,    // 2
            Floor,    Empty,    Empty,    Occupied  // 3
        ];

        Grid::new(tiles, 4)
    }

    #[test]
    fn test_get_index() {
        let grid = create_grid();
        assert_eq!(grid.get_index((0, 0)), Some(0));
        assert_eq!(grid.get_index((2, 2)), Some(10));
        assert_eq!(grid.get_index((3, 2)), Some(14));
        assert_eq!(grid.get_index((0, 15)), None);
        assert_eq!(grid.get_index((1, 5)), None);
        assert_eq!(grid.get_index((6, 6)), None);
    }

    #[test]
    fn test_get_ranges() {
        let grid = create_grid();
        assert!(grid
            .get_ranges((0, 0), (1, 1))
            .unwrap()
            .eq([0..=1, 4..=5].into_iter()));
        assert!(grid
            .get_ranges((2, 1), (3, 2))
            .unwrap()
            .eq([9..=10, 13..=14].into_iter()));
        assert!(grid.get_ranges((0, 0), (6, 6)).is_none());
    }

    #[test]
    fn test_get_location() {
        let grid = create_grid();
        assert_eq!(grid.get_location(4), Some((1, 0)));
        assert_eq!(grid.get_location(11), Some((2, 3)));
        assert!(grid.get_location(16).is_none());
    }

    #[test]
    fn test_find_neighbor() {
        let grid = create_grid();
        assert_eq!(
            grid.find_neighbor((3, 1), Tile::is_seat, |(row, col)| row
                .checked_sub(1)
                .zip(Some(col))),
            Some((13, Empty))
        );
        assert_eq!(
            grid.find_neighbor(
                (3, 2),
                |tile| matches!(tile, Tile::Occupied),
                |(row, col)| row.checked_sub(1).zip(col.checked_sub(1))
            ),
            None
        );
        assert_eq!(
            grid.find_neighbor(
                (0, 0),
                |tile| matches!(tile, Tile::Floor),
                |(row, col)| Some(row).zip(col.checked_add(1))
            ),
            Some((2, Floor))
        );
        assert_eq!(
            grid.find_neighbor((4, 4), Tile::is_seat, |(row, col)| row
                .checked_add(1)
                .zip(col.checked_add(1))),
            None
        );
        assert_eq!(
            grid.find_neighbor((0, 15), Tile::is_seat, |(row, col)| Some(row)
                .zip(col.checked_sub(1))),
            None
        );
    }
}
