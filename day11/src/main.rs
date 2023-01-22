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

struct AdjacentTileSimulator {
    grid: Grid,
    /// the (inclusive) upper bound on how many occupied adjacent tiles cause an empty seat to flip
    empty_threshold: usize,
    /// the inclusive lower bound on how many occupied adjacent tiles cause an occupied seat to flip
    occupied_threshold: usize,
    /// how many tiles away can a tile be from another to consider them adjacent?
    radius: usize,
}

struct VisibleTileSimulator {
    grid: Grid,
    /// the inclusive lower bound on how many occupied visible tiles cause an occupied seat to flip
    empty_threshold: usize,
    /// the (inclusive) upper bound on how many occupied visible tiles cause an empty seat to flip
    occupied_threshold: usize,
}

#[derive(Debug)]
struct ParseTileError;

#[derive(Debug)]
enum ParseGridError {
    InconsistentWidths,
}

impl Tile {
    fn is_seat(&self) -> bool {
        matches!(self, Self::Empty | Self::Occupied)
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Empty),
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Occupied),
            _ => Err(ParseTileError),
        }
    }
}

impl Grid {
    fn new(tiles: Vec<Tile>, width: usize) -> Self {
        Self { tiles, width }
    }

    fn contains_index(&self, index: &usize) -> bool {
        (0..self.tiles.len()).contains(index)
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
        if self.contains_index(&index) {
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

    fn simulate_using_adjacent_tiles(
        self,
        empty_threshold: usize,
        occupied_threshold: usize,
        radius: usize,
    ) -> AdjacentTileSimulator {
        AdjacentTileSimulator {
            grid: self,
            radius,
            empty_threshold,
            occupied_threshold,
        }
    }

    fn simulate_using_visible_tiles(
        self,
        empty_threshold: usize,
        occupied_threshold: usize,
    ) -> VisibleTileSimulator {
        VisibleTileSimulator {
            grid: self,
            empty_threshold,
            occupied_threshold,
        }
    }

    fn rows(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn traverse<S, T>(&self, start: (usize, usize), predicate: S, next: T) -> Option<(usize, Tile)>
    where
        S: Fn(&Tile) -> bool,
        T: Fn((usize, usize)) -> Option<(usize, usize)>,
    {
        self.get_index(start).and_then(|idx| {
            let tile = self.tiles[idx];

            if predicate(&tile) {
                Some((idx, tile))
            } else {
                next(start).and_then(|loc| self.traverse(loc, predicate, next))
            }
        })
    }
}

impl AdjacentTileSimulator {
    fn occupied_neighbors(&self) -> Vec<usize> {
        use Tile::*;

        let Self { grid, radius, .. } = self;

        grid.tiles
            .iter()
            .enumerate()
            .map(|(idx, tile)| match tile {
                Floor => 0,
                _ => {
                    let (row, col) = grid.get_location(idx).unwrap();
                    let from = (row.saturating_sub(*radius), col.saturating_sub(*radius));
                    let to = (
                        (row + *radius).min(grid.rows() - 1),
                        (col + *radius).min(grid.width - 1),
                    );

                    grid.get_ranges(from, to)
                        .unwrap()
                        .map(|range| {
                            if range.contains(&idx) {
                                let normalized_idx = idx - range.start();
                                grid.tiles[range]
                                    .iter()
                                    .enumerate()
                                    .filter(|(i, t)| *i != normalized_idx && matches!(t, Occupied))
                                    .count()
                            } else {
                                grid.tiles[range]
                                    .iter()
                                    .filter(|t| matches!(t, Occupied))
                                    .count()
                            }
                        })
                        .sum::<usize>()
                }
            })
            .collect()
    }
}

impl VisibleTileSimulator {
    const LINE_OF_SIGHT_TRANSFORMATIONS: [fn((usize, usize)) -> Option<(usize, usize)>; 8] = [
        |(row, col)| row.checked_add(1).zip(Some(col)),
        |(row, col)| row.checked_add(1).zip(col.checked_add(1)),
        |(row, col)| Some(row).zip(col.checked_add(1)),
        |(row, col)| row.checked_sub(1).zip(col.checked_add(1)),
        |(row, col)| row.checked_sub(1).zip(Some(col)),
        |(row, col)| row.checked_sub(1).zip(col.checked_sub(1)),
        |(row, col)| Some(row).zip(col.checked_sub(1)),
        |(row, col)| row.checked_add(1).zip(col.checked_sub(1)),
    ];

    fn occupied_neighbors(&self) -> Vec<usize> {
        use Tile::*;

        let Self { grid, .. } = self;

        grid.tiles
            .iter()
            .enumerate()
            .map(|(idx, tile)| match tile {
                Floor => 0,
                _ => {
                    let location = grid.get_location(idx).unwrap();
                    Self::LINE_OF_SIGHT_TRANSFORMATIONS
                        .into_iter()
                        .filter_map(|f| {
                            let visible_seat =
                                f(location).and_then(|seed| grid.traverse(seed, Tile::is_seat, f));
                            visible_seat.filter(|(_, tile)| matches!(tile, Occupied))
                        })
                        .count()
                }
            })
            .collect()
    }
}

impl Iterator for AdjacentTileSimulator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        use Tile::*;

        let &mut Self {
            empty_threshold,
            occupied_threshold,
            ..
        } = self;

        let (changed, occupied) = self.occupied_neighbors().into_iter().enumerate().fold(
            (0, 0),
            |(changed, occupied), (idx, adjacent_occupied)| match &mut self.grid.tiles[idx] {
                tile @ Occupied => {
                    if adjacent_occupied >= occupied_threshold {
                        *tile = Empty;
                        (changed + 1, occupied)
                    } else {
                        (changed, occupied + 1)
                    }
                }
                tile @ Empty => {
                    if adjacent_occupied <= empty_threshold {
                        *tile = Occupied;
                        (changed + 1, occupied + 1)
                    } else {
                        (changed, occupied)
                    }
                }
                Floor => (changed, occupied),
            },
        );

        (changed > 0).then(|| (changed, occupied))
    }
}

impl Iterator for VisibleTileSimulator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        use Tile::*;

        let &mut Self {
            empty_threshold,
            occupied_threshold,
            ..
        } = self;

        let (changed, occupied) = self.occupied_neighbors().into_iter().enumerate().fold(
            (0, 0),
            |(changed, occupied), (idx, adjacent_occupied)| match &mut self.grid.tiles[idx] {
                tile @ Occupied => {
                    if adjacent_occupied >= occupied_threshold {
                        *tile = Empty;
                        (changed + 1, occupied)
                    } else {
                        (changed, occupied + 1)
                    }
                }
                tile @ Empty => {
                    if adjacent_occupied <= empty_threshold {
                        *tile = Occupied;
                        (changed + 1, occupied + 1)
                    } else {
                        (changed, occupied)
                    }
                }
                Floor => (changed, occupied),
            },
        );

        (changed > 0).then(|| (changed, occupied))
    }
}

fn part_one(grid: Grid) {
    let occupied = grid
        .simulate_using_adjacent_tiles(0, 4, 1)
        .last()
        .map(|(_, o)| o);

    println!("Part One: {occupied:?}");
}

fn part_two(grid: Grid) {
    let occupied = grid
        .simulate_using_visible_tiles(0, 5)
        .last()
        .map(|(_, o)| o);

    println!("Part Two: {occupied:?}");
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
            grid.traverse((3, 1), Tile::is_seat, |(row, col)| row
                .checked_sub(1)
                .zip(Some(col))),
            Some((13, Empty))
        );
        assert_eq!(
            grid.traverse(
                (3, 2),
                |tile| matches!(tile, Occupied),
                |(row, col)| row.checked_sub(1).zip(col.checked_sub(1))
            ),
            None
        );
        assert_eq!(
            grid.traverse(
                (0, 0),
                |tile| matches!(tile, Floor),
                |(row, col)| Some(row).zip(col.checked_add(1))
            ),
            Some((2, Floor))
        );
        assert_eq!(
            grid.traverse((4, 4), Tile::is_seat, |(row, col)| row
                .checked_add(1)
                .zip(col.checked_add(1))),
            None
        );
        assert_eq!(
            grid.traverse((0, 15), Tile::is_seat, |(row, col)| Some(row)
                .zip(col.checked_sub(1))),
            None
        );
    }
}
