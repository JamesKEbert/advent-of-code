use std::fs;

#[macro_use]
extern crate log;

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use day1::{day1_cli_command_processing, Day1Commands};
use day2::{day2_cli_command_processing, Day2Commands};
use day3::{day3_cli_command_processing, Day3Commands};
use day4::{day4_cli_command_processing, Day4Commands};
use day5::{day5_cli_command_processing, Day5Commands};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify level of logs emitted
    #[arg(long, default_value_t = log::LevelFilter::Info)]
    loglevel: log::LevelFilter,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run Day1 methods against input files
    Day1 {
        #[command(subcommand)]
        command: Day1Commands,
    },
    /// Run Day2 methods against input files
    Day2 {
        #[command(subcommand)]
        command: Day2Commands,
    },
    /// Run Day3 methods against input files
    Day3 {
        #[command(subcommand)]
        command: Day3Commands,
    },
    /// Run Day4 methods against input files
    Day4 {
        #[command(subcommand)]
        command: Day4Commands,
    },
    /// Run Day5 methods against input files
    Day5 {
        #[command(subcommand)]
        command: Day5Commands,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut builder = colog::default_builder();
    builder.filter(None, cli.loglevel);
    builder.init();

    match &cli.command {
        Commands::Day1 { command } => day1_cli_command_processing(command),
        Commands::Day2 { command } => day2_cli_command_processing(command),
        Commands::Day3 { command } => day3_cli_command_processing(command),
        Commands::Day4 { command } => day4_cli_command_processing(command),
        Commands::Day5 { command } => day5_cli_command_processing(command),
    }
}

pub fn read_file(file_path: Utf8PathBuf) -> String {
    info!("Reading File...");
    // Using expect here, not doing file validation. If the process fails here, we'll consider that a user error. Obviously validation/handling would be ideal, but I don't care in this context.
    let contents = fs::read_to_string(file_path).expect("Content to be parsed correctly");
    info!("Read File!");
    trace!("File Contents: {:?}", contents);
    contents
}

#[cfg(test)]
fn test_init() {
    env_logger::builder().is_test(true).try_init().ok();
}
