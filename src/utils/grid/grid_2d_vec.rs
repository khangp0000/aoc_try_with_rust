use crate::utils::grid::Grid2d;
use anyhow::Result;
use std::cell::OnceCell;
use std::fmt::Debug;
use std::ops::Index;
use std::slice::Iter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Widths are not the same")]
    InvalidWidth,
}

#[derive(Debug)]
pub struct Grid2dVec<T> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Index<(usize, usize)> for Grid2dVec<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl<T> Grid2d<T> for Grid2dVec<T> {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl<T> Grid2dVec<T> {
    pub fn try_new<I: IntoIterator<Item = Result<T>>, II: IntoIterator<Item = I>>(
        into_iter: II,
    ) -> Result<Self> {
        let predict_width = OnceCell::new();

        let grid = into_iter
            .into_iter()
            .map(|i| i.into_iter())
            .map(Iterator::collect::<Result<Vec<_>>>)
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
        Ok(Self { grid, height, width })
    }

    pub fn rows(&self) -> Iter<'_, Vec<T>> {
        return self.grid.iter();
    }

    pub fn map_out_place<F: FnMut(usize, usize, &T) -> G, G>(&self, mut map_fn: F) -> Grid2dVec<G> {
        Grid2dVec {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(y, v)| {
                    v.iter().enumerate().map(|(x, t)| map_fn(x, y, t)).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            height: self.height,
            width: self.width,
        }
    }
}
