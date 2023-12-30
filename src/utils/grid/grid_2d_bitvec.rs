use crate::utils::grid::Grid2d;
use std::cell::OnceCell;
use std::fmt::Debug;

use bitvec::order::{BitOrder, Lsb0};
use bitvec::prelude::BitSlice;
use bitvec::slice::Chunks;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use std::ops::Index;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Widths are not the same")]
    InvalidWidth,
}

#[derive(Debug)]
pub struct Grid2dBitVec<S: BitStore = usize, O: BitOrder = Lsb0> {
    grid: BitVec<S, O>,
    height: usize,
    width: usize,
}

impl<S: BitStore, O: BitOrder> Index<(usize, usize)> for Grid2dBitVec<S, O> {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y * self.width + x]
    }
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
    pub fn try_new<I: IntoIterator<Item = anyhow::Result<bool>>, II: IntoIterator<Item = I>>(
        into_iter: II,
    ) -> anyhow::Result<Self> {
        let predict_width = OnceCell::new();

        let grid = into_iter
            .into_iter()
            .map(|i| i.into_iter())
            .map(Iterator::collect::<anyhow::Result<BitVec<S, O>>>)
            .map(|line_res| {
                let line = line_res?;
                if line.len() == *predict_width.get_or_init(|| line.len()) {
                    Ok(line)
                } else {
                    Err(Error::InvalidWidth)?
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let height = grid.len();
        let &width = predict_width.get_or_init(|| 0_usize);
        let grid = grid
            .into_iter()
            .reduce(|mut l, mut r| {
                l.append(&mut r);
                l
            })
            .unwrap_or_else(|| BitVec::new());

        Ok(Self { grid, height, width })
    }

    pub fn rows(&self) -> Chunks<'_, S, O> {
        return self.grid.chunks(self.width);
    }

    pub fn get_row(&self, idx: usize) -> &BitSlice<S, O> {
        &self.grid[idx * self.width..(idx + 1) * self.width]
    }

    pub fn flatten_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[allow(dead_code)]
    pub fn get_internal_bitvec(&self) -> &BitVec<S, O> {
        &self.grid
    }
}
