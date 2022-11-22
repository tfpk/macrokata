use clap::{Parser, Subcommand};
use std::error::Error;

mod check;
mod goal;
mod test;
mod update_diff;

use check::check_all;
use goal::goal;
use test::test;
use update_diff::update_diff;

#[derive(Subcommand, Debug)]
enum SubCommands {
    Test {
        /// The name of the exercise to run.
        exercise: String,
    },
    Goal {
        /// The name of the exercise to run.
        exercise: String,
    },
    UpdateDiff {
        /// The name of the exercise to create a diff for.
        exercise: String,
    },
    CheckAll,
}

/// MacroKata is a set of exercises to learn how to use
/// macros well.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Choose what MacroKata does.
    #[clap(subcommand)]
    command: SubCommands,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        SubCommands::Test { exercise } => test(exercise),
        SubCommands::Goal { exercise } => goal(exercise),
        SubCommands::UpdateDiff { exercise } => update_diff(&exercise),
        SubCommands::CheckAll => {
            check_all();
            Ok(())
        }
    }
}
