use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use std::error::Error;

#[derive(Subcommand, Debug)]
enum SubCommands {
    Run {
        /// The name of the exercise to run.
        exercise: String
    }
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

fn run(exercise: String) -> Result<(), Box<dyn Error>> {
    println!("This is the expansion you produced, along with any errors");
    println!();

    Command::new("cargo")
                    .arg("expand")
                    .arg("--bin")
                    .arg(&exercise)
                    .arg("main")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

    println!();
    println!("The expansion we expected is:");
    println!();


    Command::new("cargo")
                    .arg("expand")
                    .arg("--bin")
                    .arg(format!("{exercise}_soln"))
                    .arg("main")
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

    Ok(())
}

fn main()  -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        SubCommands::Run { exercise } => {
            run(exercise)
        }
    }
}
