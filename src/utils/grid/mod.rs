pub mod grid_2d_vec;

use derive_more::Display;
use enumset::EnumSetType;
use std::ops::Index;

pub trait Grid2d<T>: Index<(usize, usize), Output = T> {
    fn height(&self) -> usize;

    fn width(&self) -> usize;

    fn size(&self) -> usize {
        self.width() * self.height()
    }

    fn get(&self, x: &usize, y: &usize) -> Option<&T> {
        if self.contains(x, y) {
            return Some(self.index((*x, *y)));
        }
        None
    }

    fn contains(&self, &x: &usize, &y: &usize) -> bool {
        x < self.width() && y < self.height()
    }

    fn north_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        if y == 0_usize || y > self.height() {
            return None;
        }
        Some((x, y - 1))
    }

    fn south_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        let height = self.height();
        if height == 0 || y >= height - 1 {
            return None;
        }
        Some((x, y + 1))
    }

    fn west_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        if x == 0_usize || x > self.width() {
            return None;
        }
        Some((x - 1, y))
    }

    fn east_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        let width = self.width();
        if width == 0 || x >= width - 1 {
            return None;
        }
        Some((x + 1, y))
    }

    fn north_west_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        if y == 0_usize || y > self.height() || x == 0_usize || x > self.width() {
            return None;
        }
        Some((x - 1, y - 1))
    }

    fn north_east_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        let width = self.width();
        if y == 0_usize || y > self.height() || width == 0 || x >= width - 1 {
            return None;
        }
        Some((x + 1, y - 1))
    }

    fn south_west_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        let height = self.height();
        if height == 0 || y >= height - 1 || x == 0_usize || x > self.width() {
            return None;
        }
        Some((x - 1, y + 1))
    }

    fn south_east_coordinate_from(&self, &x: &usize, &y: &usize) -> Option<(usize, usize)> {
        let width = self.width();
        let height = self.height();
        if height == 0 || y >= height - 1 || width == 0 || x >= width - 1 {
            return None;
        }
        Some((x + 1, y + 1))
    }

    fn move_from_coordinate_to_direction(
        &self,
        x: &usize,
        y: &usize,
        direction: &GridDirection,
    ) -> Option<(usize, usize)> {
        match direction {
            GridDirection::North => self.north_coordinate_from(x, y),
            GridDirection::South => self.south_coordinate_from(x, y),
            GridDirection::East => self.east_coordinate_from(x, y),
            GridDirection::West => self.west_coordinate_from(x, y),
            GridDirection::SouthWest => self.south_west_coordinate_from(x, y),
            GridDirection::SouthEast => self.south_east_coordinate_from(x, y),
            GridDirection::NorthEast => self.north_east_coordinate_from(x, y),
            GridDirection::NorthWest => self.north_west_coordinate_from(x, y),
        }
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
    pub fn reverse(&self) -> GridDirection {
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
