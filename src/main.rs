use anyhow::bail;
use anyhow::Result;
use clap::Parser;
use solver::AOC_PROBLEMS_SOLVER;
use std::path::PathBuf;

mod solver;
mod utils;

/// Solve advent of code with command line.
#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help = true)]
struct Args {
    /// Path to session file, "cookie: session={session_file_content}" will be
    /// used to get input data.
    #[arg(short, long, default_value = "data/session.txt")]
    session_file: PathBuf,

    /// Input path folder, used to store downloaded input data. Will not
    /// re-download if file already exists. File path is
    /// "{input_folder}/y{year}/day{day}.txt"
    #[arg(short, long, default_value = "data")]
    input_folder: PathBuf,

    /// Which year are you looking at.
    #[arg(short, long)]
    year: u16,

    /// Which days are you looking at.
    #[arg(short, long, value_delimiter = ',')]
    days: Vec<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let solvers = AOC_PROBLEMS_SOLVER.get_entry(&args.year);
    let (day_mapper_solvers, mut days) = match &solvers {
        None => bail!(format!("There is no solver for selected year {}", args.year)),
        Some(entry) => {
            if args.days.is_empty() {
                (*entry.1, entry.1.keys().copied().collect::<Vec<u8>>())
            } else {
                (*entry.1, args.days)
            }
        }
    };
    days.sort();

    let mut failed = false;
    for day in days {
        if let Some((_, solver_fn)) = day_mapper_solvers.get_entry(&day) {
            let result = solver_fn(args.year, day, &args.input_folder, &args.session_file)?;
            println!("{0}.{1}. Result for year {0} day {1} is:", args.year, day);
            println!("    {result}");
        } else {
            eprintln!("{0}.{1}. There is no solver for year {0} day {1}.", args.year, day);
            failed = true;
        }
    }
    if failed {
        bail!("At least one error occurred.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solver::AOC_PROBLEMS_SOLVER;
    use anyhow::Result;
    use std::path::PathBuf;

    const SESSION_PATH: &str = "data/session.txt";
    const INPUT_FOLDER_PATH: &str = "data";

    fn run(year: u16, day: u8) -> Result<()> {
        let result = AOC_PROBLEMS_SOLVER[&year][&day](
            year,
            day,
            &PathBuf::from(&INPUT_FOLDER_PATH),
            &PathBuf::from(&SESSION_PATH),
        )?;
        println!("Result for year {year} day {day} is:");
        println!("{}", result);
        Ok(())
    }

    #[test]
    fn day1() -> Result<()> {
        run(2023, 1)
    }

    #[test]
    fn day2() -> anyhow::Result<()> {
        run(2023, 2)
    }

    #[test]
    fn day3() -> anyhow::Result<()> {
        run(2023, 3)
    }

    #[test]
    fn day4() -> anyhow::Result<()> {
        run(2023, 4)
    }
}
