pub mod graph;
pub mod grid;
pub mod int_range;
pub mod int_trait;

use anyhow::Result;

use crate::solver::ProblemSolver;
use anyhow::Context;
use reqwest::blocking::Client;

use derive_more::{Deref, Display};
use derive_new::new;
use std::fmt::Formatter;
use std::fs;
use std::fs::{create_dir_all, read_to_string, File};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use thiserror::Error;

macro_rules! boxed_try_get_input_and_solve {
    ($solver:ty) => {
        |year, day, base_input_path, session_file_path| {
            crate::utils::try_get_input_and_solve::<$solver, _>(
                year,
                day,
                base_input_path,
                session_file_path,
            )
            .map(|r| Box::new(r) as Box<dyn Display>)
        }
    };
}

pub(crate) use boxed_try_get_input_and_solve;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to split with delimiter {1:?}: {0:?}")]
    FailedToSplit(String, char),
}

#[derive(Debug, Eq, PartialEq, new)]
pub struct Result2Parts<T1: Display, T2: Display> {
    res_1: T1,
    res_2: T2,
}

#[derive(new, Deref, Debug, Eq, PartialEq)]
pub struct WarningResult<T> {
    #[deref]
    res: T,
    warning: &'static str,
}

impl<T1: Display, T2: Display> Display for Result2Parts<T1, T2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<part 1: {}, part 2: {}>", self.res_1, self.res_2)
    }
}

impl<T: Display> Display for WarningResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} --{}--", self.res, self.warning)
    }
}

fn reqwest_client() -> &'static Client {
    static REQWEST_CLIENT: OnceLock<Client> = OnceLock::new();
    return REQWEST_CLIENT.get_or_init(Client::new);
}

fn get_input_path(base_input_path: &Path, year: u16, day: u8) -> PathBuf {
    base_input_path.join(format!("y{}/day{}.txt", year, day))
}

pub fn download_input_if_needed(
    year: u16,
    day: u8,
    target_path: &Path,
    session_cookie_path: &Path,
) -> Result<()> {
    if target_path.exists() {
        if target_path.is_file() {
            return Ok(());
        } else {
            anyhow::bail!(format!("Path is not a file: {:?}", target_path));
        }
    }

    let session = read_to_string(session_cookie_path)
        .with_context(|| format!("Failed to read session file: {:?}", session_cookie_path))?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut response = reqwest_client()
        .get(&url)
        .header("cookie", format!("session={}", session))
        .send()
        .with_context(|| format!("Failed to send get request to {}", url))?
        .error_for_status()?;
    create_dir_all(
        target_path
            .parent()
            .with_context(|| format!("Failed to get parent for path {:?}", target_path))?,
    )
    .with_context(|| format!("Failed to create parent dir for path {:?}", target_path))?;

    let mut output = File::create(target_path)
        .with_context(|| format!("Failed to create file path {:?}", target_path))?;
    let write_result = response.copy_to(&mut output);
    match write_result {
        Ok(_) => Ok(()),
        Err(e) => {
            fs::remove_file(target_path).with_context(|| {
                format!("Input file write failed but cannot delete for file path {:?}", target_path)
            })?;
            Err(e).with_context(|| format!("Input file write failed {:?}", target_path))?
        }
    }
}

pub trait GetInputAndSolver<T: Display> {
    fn try_get_input_and_solve(
        year: u16,
        day: u8,
        base_input_path: &Path,
        session_file_path: &Path,
    ) -> Result<T>;
}

pub fn try_get_input_and_solve<P: ProblemSolver<SolutionType = T>, T: Display>(
    year: u16,
    day: u8,
    base_input_path: &Path,
    session_file_path: &Path,
) -> Result<T> {
    let input = get_input(year, day, base_input_path, session_file_path)?;
    P::from_str(&input)?.solve()
}

pub fn get_input(
    year: u16,
    day: u8,
    base_input_path: &Path,
    session_file_path: &Path,
) -> Result<String> {
    let input_path = get_input_path(base_input_path, year, day);
    download_input_if_needed(year, day, &input_path, session_file_path)?;
    Ok(read_to_string(&input_path)?)
}
