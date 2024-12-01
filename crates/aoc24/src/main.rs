use std::{env, fs, ops::RangeInclusive};

use anyhow::{Context as _, Result, bail};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use days::*;

mod days;

const YEAR: usize = 2024;
const CLI_DAY_RANGE: RangeInclusive<i64> = 1..=25;
const VALID_DAY_RANGE: RangeInclusive<u32> =
    (*CLI_DAY_RANGE.start() as u32)..=(*CLI_DAY_RANGE.end() as u32);

/// Advent of Code
#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(
            value_parser = clap::value_parser!(u32).range(CLI_DAY_RANGE),
            help = "The number of the day you want to run (1-25)")
        ]
        day: Option<u32>,
        #[arg(short, long, help = "Runs all days sequentially")]
        all: bool,
    },
    Get {
        #[arg(
            value_parser = clap::value_parser!(u32).range(CLI_DAY_RANGE),
            help = "The number of the day you want to get the input for (1-25)")
        ]
        day: Option<u32>,
        #[arg(short, long, help = "Downloads input for all days sequentially")]
        all: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { day, all } => {
            if all {
                return run_all_days();
            }
            if let Some(day) = day {
                return run_day(day);
            }
            println!("No day parameter specified, attempting to run today's code");
            let now_day = get_today()?;
            println!("Running day {now_day}");
            run_day(now_day)
        }
        Commands::Get { day, all } => {
            if all {
                return download_all_inputs();
            }
            if let Some(day) = day {
                return download_input(day);
            }
            println!("No day parameter specified, attempting to download today's input");
            let now_day = get_today()?;
            println!("Getting input for day {now_day}");
            download_input(now_day)
        }
    }
}

fn get_today() -> Result<u32> {
    let now = Local::now();
    let now_day = now.day();
    if now.month() == 12 && VALID_DAY_RANGE.contains(&now_day) {
        Ok(now_day)
    } else {
        bail!("Today is not a valid Advent of Code day. Please specify a day");
    }
}

#[allow(const_item_mutation)]
fn run_all_days() -> Result<()> {
    VALID_DAY_RANGE.try_for_each(run_day)
}

fn run_day(day: u32) -> Result<()> {
    println!("======== DAY {day} ========");
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_file = &format!("{project_root}/inputs/day{day:02}.txt");
    match day {
        1 => day01::Day01::run_day(input_file),
        d => bail!("provided unsupported day {d}"),
    }
}

#[allow(const_item_mutation)]
fn download_all_inputs() -> Result<()> {
    VALID_DAY_RANGE.try_for_each(download_input)
}

fn download_input(day: u32) -> Result<()> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let session_file = format!("{project_root}/.aoc_session");
    let session = match env::var("AOC_SESSION") {
        Ok(session) => session,
        Err(_) => fs::read_to_string(&session_file)
            .context(format!("reading session file {session_file}"))?,
    };
    let session = session.trim();
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={session};"))
        .send()
        .context("sending HTTP request to download input")?
        .error_for_status()
        .with_context(|| {
            format!(
                "retrieving the input for day {day}. Do you have the correct session cookie in \
                 the file {session_file} or the environment variable AOC_SESSION?"
            )
        })?;

    let text = response.text().context("decoding response body as text")?;
    let path = format!("{project_root}/inputs/day{day:02}.txt");
    fs::write(&path, text).context("writing input to file")?;
    println!("Successfully downloaded input to {}", &path);
    Ok(())
}
