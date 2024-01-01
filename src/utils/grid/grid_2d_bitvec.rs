use std::cell::OnceCell;
use std::collections::Bound;
use std::fmt::Debug;
use std::ops::{
    Index, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use anyhow::Result;
use bitvec::bitvec;
use bitvec::order::{BitOrder, Lsb0};
use bitvec::prelude::BitSlice;
use bitvec::slice::Chunks;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use thiserror::Error;

use crate::utils::grid::Grid2d;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Widths are not the same")]
    InvalidWidth,
}

#[derive(Debug)]
pub struct Grid2dBitVec<S: BitStore = usize, O: BitOrder = Lsb0> {
    grid: BitVec<S, O>,
    grid_x_significant: OnceCell<BitVec<S, O>>,
    height: usize,
    width: usize,
}

impl<S: BitStore, O: BitOrder> Index<(usize, usize)> for Grid2dBitVec<S, O> {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(y < self.height);
        assert!(x < self.width);
        &self.grid[y * self.width + x]
    }
}

/// Implements `Index`
macro_rules! index {
	($($t:ty),+ $(,)?) => { $(
        impl<S: BitStore, O: BitOrder> Index<($t, usize)> for Grid2dBitVec<S, O> {
            type Output = BitSlice<S, O>;

            fn index(&self, (x_range, y): ($t, usize)) -> &Self::Output {
                assert!(y < self.height);
                let x_start = match x_range.start_bound() {
                    Bound::Included(x) => *x,
                    Bound::Excluded(x) => *x + 1,
                    Bound::Unbounded => 0
                };
                let x_end = match x_range.end_bound() {
                    Bound::Included(x) => *x + 1,
                    Bound::Excluded(x) => *x,
                    Bound::Unbounded => self.width
                };
                assert!(x_start <= x_end);
                assert!(x_end <= self.width);
                let start_idx = y * self.width;
                &self.grid[(start_idx + x_start)..(start_idx + x_end)]
            }
        }

        impl<S: BitStore, O: BitOrder> Index<(usize, $t)> for Grid2dBitVec<S, O> {
            type Output = BitSlice<S, O>;

            fn index(&self, (x, y_range): (usize, $t)) -> &Self::Output {
                assert!(x < self.width);
                let y_start = match y_range.start_bound() {
                    Bound::Included(y) => *y,
                    Bound::Excluded(y) => *y + 1,
                    Bound::Unbounded => 0
                };
                let y_end = match y_range.end_bound() {
                    Bound::Included(x) => *x + 1,
                    Bound::Excluded(x) => *x,
                    Bound::Unbounded => self.width
                };
                assert!(y_start <= y_end);
                assert!(y_end <= self.height);
                let start_idx = x * self.height;
                &self.get_grid_x_significant()[(start_idx + y_start)..(start_idx + y_end)]
            }
        }
	)+ };
}

index! {
    Range<usize>,
    RangeFrom<usize>,
    RangeFull,
    RangeInclusive<usize>,
    RangeTo<usize>,
    RangeToInclusive<usize>,
}

impl<S: BitStore, O: BitOrder> Grid2d<bool> for Grid2dBitVec<S, O> {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl<S: BitStore, O: BitOrder> Grid2dBitVec<S, O> {
    pub fn try_new<I: IntoIterator<Item = Result<bool>>, II: IntoIterator<Item = I>>(
        into_iter: II,
    ) -> Result<Self> {
        let predict_width = OnceCell::default();

        let grid = into_iter
            .into_iter()
            .map(|i| i.into_iter())
            .map(Iterator::collect::<Result<BitVec<S, O>>>)
            .map(|line_res| {
                let line = line_res?;
                if line.len() == *predict_width.get_or_init(|| line.len()) {
                    Ok(line)
                } else {
                    Err(Error::InvalidWidth)?
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let height = grid.len();
        let &width = predict_width.get_or_init(|| 0_usize);
        let grid = grid
            .into_iter()
            .reduce(|mut l, mut r| {
                l.append(&mut r);
                l
            })
            .unwrap_or_else(BitVec::default);

        Ok(Self { grid, grid_x_significant: OnceCell::default(), height, width })
    }

    pub fn rows(&self) -> Chunks<'_, S, O> {
        return self.grid.chunks(self.width);
    }

    pub fn get_row(&self, idx: usize) -> &BitSlice<S, O> {
        &self.grid[idx * self.width..(idx + 1) * self.width]
    }

    pub fn cols(&self) -> Chunks<'_, S, O> {
        return self.get_grid_x_significant().chunks(self.height);
    }

    pub fn get_col(&self, idx: usize) -> &BitSlice<S, O> {
        &self.get_grid_x_significant()[idx * self.height..(idx + 1) * self.height]
    }

    fn get_grid_x_significant(&self) -> &BitVec<S, O> {
        self.grid_x_significant.get_or_init(|| {
            self.rows().enumerate().fold(bitvec!(S, O; 0; self.size()), |mut acc, (y, slice)| {
                slice.iter_ones().for_each(|x| acc.set(x * self.height + y, true));
                acc
            })
        })
    }

    pub fn flatten_idx(&self, x: usize, y: usize) -> usize {
        assert!(y < self.height);
        assert!(x < self.width);
        y * self.width + x
    }

    fn flatten_idx_x_significant(&self, x: usize, y: usize) -> usize {
        assert!(y < self.height);
        assert!(x < self.width);
        x * self.height + y
    }

    #[allow(dead_code)]
    pub fn get_internal_bitvec(&self) -> &BitVec<S, O> {
        &self.grid
    }
}
