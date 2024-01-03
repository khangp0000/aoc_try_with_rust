use std::ops::Index;

use derive_more::Display;
use enumset::EnumSetType;

pub mod grid_2d_bitvec;
pub mod grid_2d_vec;

pub trait Grid2d<T>: Index<(usize, usize), Output = T> {
    fn height(&self) -> usize;

    fn width(&self) -> usize;

    fn size(&self) -> usize {
        self.width() * self.height()
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.contains(x, y) {
            return Some(self.index((x, y)));
        }
        None
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    fn north_coordinate_from(&self, x: usize, y: usize, step: usize) -> Option<(usize, usize)> {
        y.checked_sub(step).filter(|y| self.contains(x, *y)).map(|y| (x, y))
    }

    fn south_coordinate_from(&self, x: usize, y: usize, step: usize) -> Option<(usize, usize)> {
        y.checked_add(step).filter(|y| self.contains(x, *y)).map(|y| (x, y))
    }

    fn west_coordinate_from(&self, x: usize, y: usize, step: usize) -> Option<(usize, usize)> {
        x.checked_sub(step).filter(|x| self.contains(*x, y)).map(|x| (x, y))
    }

    fn east_coordinate_from(&self, x: usize, y: usize, step: usize) -> Option<(usize, usize)> {
        x.checked_add(step).filter(|x| self.contains(*x, y)).map(|x| (x, y))
    }

    fn north_west_coordinate_from(
        &self,
        x: usize,
        y: usize,
        step: usize,
    ) -> Option<(usize, usize)> {
        x.checked_sub(step)
            .and_then(|x| y.checked_sub(step).map(|y| (x, y)))
            .filter(|(x, y)| self.contains(*x, *y))
    }

    fn north_east_coordinate_from(
        &self,
        x: usize,
        y: usize,
        step: usize,
    ) -> Option<(usize, usize)> {
        x.checked_add(step)
            .and_then(|x| y.checked_sub(step).map(|y| (x, y)))
            .filter(|(x, y)| self.contains(*x, *y))
    }

    fn south_west_coordinate_from(
        &self,
        x: usize,
        y: usize,
        step: usize,
    ) -> Option<(usize, usize)> {
        x.checked_sub(step)
            .and_then(|x| y.checked_add(step).map(|y| (x, y)))
            .filter(|(x, y)| self.contains(*x, *y))
    }

    fn south_east_coordinate_from(
        &self,
        x: usize,
        y: usize,
        step: usize,
    ) -> Option<(usize, usize)> {
        x.checked_add(step)
            .and_then(|x| y.checked_add(step).map(|y| (x, y)))
            .filter(|(x, y)| self.contains(*x, *y))
    }

    fn move_from_coordinate_to_direction(
        &self,
        x: usize,
        y: usize,
        step: usize,
        direction: GridDirection,
    ) -> Option<(usize, usize)> {
        match direction {
            GridDirection::North => self.north_coordinate_from(x, y, step),
            GridDirection::South => self.south_coordinate_from(x, y, step),
            GridDirection::East => self.east_coordinate_from(x, y, step),
            GridDirection::West => self.west_coordinate_from(x, y, step),
            GridDirection::SouthWest => self.south_west_coordinate_from(x, y, step),
            GridDirection::SouthEast => self.south_east_coordinate_from(x, y, step),
            GridDirection::NorthEast => self.north_east_coordinate_from(x, y, step),
            GridDirection::NorthWest => self.north_west_coordinate_from(x, y, step),
        }
    }

    fn move_from_coordinate_to_direction_with_value(
        &self,
        x: usize,
        y: usize,
        step: usize,
        direction: GridDirection,
    ) -> Option<(usize, usize, &T)> {
        match direction {
            GridDirection::North => self.north_coordinate_from(x, y, step),
            GridDirection::South => self.south_coordinate_from(x, y, step),
            GridDirection::East => self.east_coordinate_from(x, y, step),
            GridDirection::West => self.west_coordinate_from(x, y, step),
            GridDirection::SouthWest => self.south_west_coordinate_from(x, y, step),
            GridDirection::SouthEast => self.south_east_coordinate_from(x, y, step),
            GridDirection::NorthEast => self.north_east_coordinate_from(x, y, step),
            GridDirection::NorthWest => self.north_west_coordinate_from(x, y, step),
        }
        .map(|(x, y)| (x, y, &self[(x, y)]))
    }
}

#[derive(EnumSetType, Hash, Display, Debug)]
pub enum GridDirection {
    North,
    South,
    East,
    West,
    SouthWest,
    SouthEast,
    NorthEast,
    NorthWest,
}

impl GridDirection {
    pub const fn reverse(&self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::South,
            GridDirection::South => GridDirection::North,
            GridDirection::East => GridDirection::West,
            GridDirection::West => GridDirection::East,
            GridDirection::SouthWest => GridDirection::NorthEast,
            GridDirection::SouthEast => GridDirection::NorthWest,
            GridDirection::NorthEast => GridDirection::SouthWest,
            GridDirection::NorthWest => GridDirection::SouthEast,
        }
    }

    pub fn clock_wise_90(&self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::East,
            GridDirection::South => GridDirection::West,
            GridDirection::East => GridDirection::South,
            GridDirection::West => GridDirection::North,
            GridDirection::SouthWest => GridDirection::NorthWest,
            GridDirection::SouthEast => GridDirection::SouthWest,
            GridDirection::NorthEast => GridDirection::SouthEast,
            GridDirection::NorthWest => GridDirection::NorthEast,
        }
    }
}
