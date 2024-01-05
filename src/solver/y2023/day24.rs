use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{ControlFlow, DerefMut, Div, Index, IndexMut, Mul, RangeBounds};
use std::rc::Rc;

use anyhow::{Context, Result};
use derive_more::{Add, AddAssign, Deref, From, FromStr, Into, Sub, SubAssign};
use derive_new::new;
use itertools::Itertools;

use crate::solver::{ProblemSolver, share_struct_solver};

share_struct_solver!(Day24, Day24Part1, Day24Part2);

type BitSet = bit_set::BitSet<usize>;

#[derive(new, Deref, Debug)]
pub struct Day24Part1(Vec<Line>);

#[derive(Deref, Debug)]
pub struct Day24Part2(Rc<Day24Part1>);

#[derive(Copy, Clone, From, Into, Debug)]
pub struct Line {
    pos: Position,
    vel: Velocity,
}

#[derive(Copy, Clone, From, Into, Debug)]
pub struct Line2D {
    pos: Position2D,
    vel: Velocity2D,
}

type Position = Vec3D;
type Velocity = Vec3D;

type Position2D = Vec2D;
type Velocity2D = Vec2D;

#[derive(Copy, Clone, From, Into, Debug)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, From, Into, Debug)]
struct Vec2D {
    x: f64,
    y: f64,
}

impl Line {
    fn project_x_y(&self) -> Line2D {
        Line2D { pos: self.pos.project_x_y(), vel: self.vel.project_x_y() }
    }

    fn project_y_z(&self) -> Line2D {
        Line2D { pos: self.pos.project_y_z(), vel: self.vel.project_y_z() }
    }

    fn project_z_x(&self) -> Line2D {
        Line2D { pos: self.pos.project_z_x(), vel: self.vel.project_z_x() }
    }
}

impl Vec3D {
    fn project_x_y(&self) -> Vec2D {
        Vec2D { x: self.x, y: self.y }
    }

    fn project_y_z(&self) -> Vec2D {
        Vec2D { x: self.y, y: self.z }
    }

    fn project_z_x(&self) -> Vec2D {
        Vec2D { x: self.z, y: self.x }
    }

    fn no_zero(&self) -> bool {
        self.x.abs() > EPSILON && self.y.abs() > EPSILON && self.z.abs() > EPSILON
    }

    fn cross_product(&self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.project_y_z().cross_product(&other.project_y_z()),
            y: self.project_z_x().cross_product(&other.project_z_x()),
            z: self.project_x_y().cross_product(&other.project_x_y()),
        }
    }
}

impl Line2D {
    fn at_time(&self, t: f64) -> Vec2D {
        let mut res = (&self.vel) * t;
        res += &self.pos;
        res
    }
}

impl Vec2D {
    fn cross_product(&self, other: &Vec2D) -> f64 {
        (self.x * other.y) - (other.x * self.y)
    }
}

impl<'a> Add<&'a Vec2D> for &'a Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: &'a Vec2D) -> Self::Output {
        Vec2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a> Sub<&'a Vec2D> for &'a Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: &'a Vec2D) -> Self::Output {
        Vec2D { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<'a> AddAssign<&'a Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: &'a Vec2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a> SubAssign<&'a Vec2D> for Vec2D {
    fn sub_assign(&mut self, rhs: &'a Vec2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a> Mul<f64> for &'a Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2D { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<'a> Div<f64> for &'a Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2D { x: self.x / rhs, y: self.y / rhs }
    }
}

impl FromStr for Day24Part1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.lines()
            .map(|line| {
                let context_fn = || format!("Cannot parse line {line:?}");
                let (pos, vel) = line.split_once('@').with_context(context_fn)?;
                let mut part_iter = pos.trim().splitn(3, ',').map(|s| <f64>::from_str(s.trim()));
                let x = part_iter.next().with_context(context_fn)??;
                let y = part_iter.next().with_context(context_fn)??;
                let z = part_iter.next().with_context(context_fn)??;
                let pos = (x, y, z).into();
                let mut part_iter = vel.trim().splitn(3, ',').map(|s| <f64>::from_str(s.trim()));
                let x = part_iter.next().with_context(context_fn)??;
                let y = part_iter.next().with_context(context_fn)??;
                let z = part_iter.next().with_context(context_fn)??;
                let vel = (x, y, z).into();
                Ok((pos, vel).into())
            })
            .collect::<Result<Vec<_>>>()
            .map(Day24Part1)
    }
}

impl ProblemSolver for Day24Part1 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        Ok(self.intersect_in_area_count(&(200000000000000.0..=400000000000000.0)))
    }
}

impl Day24Part1 {
    fn intersect_in_area_count<R: RangeBounds<f64>>(&self, x_y_bound: &R) -> usize {
        self
            .iter()
            .map(Line::project_x_y)
            .tuple_combinations()
            .filter_map(|(left, right)| {
                Self::intersect_time_2d(&left, &right)
                    .filter(|(left_t, right_t)| *left_t >= 0.0 && *right_t >= 0.0)
                    .map(|(left_t, _)| left.at_time(left_t))
            })
            .filter(|pos| x_y_bound.contains(&pos.x) && x_y_bound.contains(&pos.y))
            .count()
    }

    fn intersect_time_2d(line1: &Line2D, line2: &Line2D) -> Option<(f64, f64)> {
        let det = line1.vel.cross_product(&line2.vel);
        if det.abs() <= EPSILON {
            None
        } else {
            let p_diff = &line1.pos - &line2.pos;
            let t1 = line2.vel.cross_product(&p_diff) / det;
            let t2 = line1.vel.cross_product(&p_diff) / det;

            Some((t1, t2))
        }
    }
}

static EPSILON: f64 = f64::EPSILON;

impl Day24Part2 {
    fn generate_linear_equation_collision_2d(l1: &Line2D, l2: &Line2D) -> Box<[f64; 5]> {
        let px = l1.vel.y - l2.vel.y;
        let py = l2.vel.x - l1.vel.x;
        let vx = l2.pos.y - l1.pos.y;
        let vy = l1.pos.x - l2.pos.x;
        let rhs = l1.pos.cross_product(&l1.vel) - l2.pos.cross_product(&l2.vel);

        Box::new([px, py, vx, vy, rhs])
    }

    fn generate_linear_equation_collision_3d(l1: &Line, l2: &Line) -> Box<[[f64; 7]; 2]> {
        let first = Self::generate_linear_equation_collision_2d(&l1.project_x_y(), &l2.project_x_y());
        let first = [first[0], first[1], 0.0, first[2], first[3], 0.0, first[4]];

        let second = Self::generate_linear_equation_collision_2d(&l1.project_y_z(), &l2.project_y_z());
        let second = [0.0, second[0], second[1], 0.0, second[2], second[3], second[4]];

        Box::new([first, second])
    }

    fn forward_elimination<T: IndexMut<usize, Output=f64>, M: IndexMut<usize, Output=T> + AsMut<[T]>>(matrix: &mut M, size: usize) -> Option<usize> {
        let matrix = RefCell::new(matrix);
        match (0..size).try_for_each(|k| {
            let mut i_max = k;
            {
                let matrix_ref = matrix.borrow();
                let mut v_max = matrix_ref[i_max][k];
                (k + 1..size).map(|i| (i, matrix_ref[i][k]))
                    .for_each(|(i, v)| {
                        if v.abs() > v_max {
                            v_max = v;
                            i_max = i;
                        }
                    });

                if matrix_ref[k][i_max].abs() <= EPSILON {
                    return ControlFlow::Break(k);
                }
            }

            if i_max != k {
                matrix.borrow_mut().deref_mut().as_mut().swap(i_max, k);
            }

            (k + 1..size).map(|i| {
                let matrix_ref = matrix.borrow();
                (i, matrix_ref[i][k] / matrix_ref[k][k])
            })
                .for_each(|(i, f)| {
                    (k + 1..=size).for_each(|j| {
                        let mut matrix_mut = matrix.borrow_mut();
                        matrix_mut[i][j] -= matrix_mut[k][j] * f
                    });
                    matrix.borrow_mut()[i][k] = 0.0;
                });

            ControlFlow::Continue(())
        }) {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(singular_row_id) => Some(singular_row_id)
        }
    }

    fn back_substitution<T: Index<usize, Output=f64>, M: Index<usize, Output=T> + DerefMut<Target=[T]>>(matrix: &M, size: usize) -> Vec<f64> {
        let mut res = vec![0.0; size];

        (0..size).rev().for_each(|i| {
            res[i] = (matrix[i][size] - (i + 1..size).map(|j| matrix[i][j] * res[j])
                .sum::<f64>()) / matrix[i][i]
        });

        res
    }
}

impl ProblemSolver for Day24Part2 {
    type SolutionType = usize;

    fn solve(&self) -> Result<Self::SolutionType> {
        self.iter().tuple_combinations().map(
            |(v0, v1, v2, v3)| {
                let mut res = Vec::with_capacity(6);
                res.extend(Self::generate_linear_equation_collision_3d(v0, v1).into_iter());
                res.extend(Self::generate_linear_equation_collision_3d(v1, v2).into_iter());
                res.extend(Self::generate_linear_equation_collision_3d(v2, v3).into_iter());

                res
            }
        ).filter_map(|mut matrix| match Self::forward_elimination(&mut matrix, 6) {
            None => Some(matrix),
            Some(_) => None,
        }).next().map(|matrix| Self::back_substitution(&matrix, 6))
            .map(|res| res.into_iter().take(3).map(|v| v.round() as usize).sum())
            .context("Cannot found a valid starting rock position and velocity")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use indoc::indoc;

    use crate::solver::ProblemSolver;
    use crate::solver::y2023::day24::{Day24Part1, Day24Part2};

    const SAMPLE_INPUT_1: &str = indoc! {r"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_solve_1() -> Result<()> {
        assert_eq!(Day24Part1::from_str(SAMPLE_INPUT_1)?.intersect_in_area_count(&(7.0..=27.0)), 2);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> Result<()> {
        assert_eq!(Day24Part2::from_str(SAMPLE_INPUT_1)?.solve()?, 47);
        Ok(())
    }
}
