use glob::glob;
use imara_diff::intern::InternedInput;
use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug)]
pub enum CheckError {
    MainFileDoesNotExist,
    SolutionFileDoesNotExist,
    DiffFileDoesNotExist,
    DiffFileDoesNotMatch { actual: String, expected: String },
    SolutionFileDoesNotClippy,
}

pub fn check_all() {
    let exercise_main_glob: PathBuf = [env!("CARGO_MANIFEST_DIR"), "exercises", "*", "main.rs"]
        .iter()
        .collect();

    let mut is_ok = true;
    for exercise in glob(&exercise_main_glob.to_string_lossy()).expect("Glob must work.") {
        let exercise = exercise.unwrap();
        let exercise_name = exercise
            .parent()
            .expect("the glob has a parent")
            .file_name()
            .expect("the glob has a name");
        if let Err(e) = check(&exercise_name.to_string_lossy()) {
            eprintln!("Check of {exercise_name:#?} failed: {e:?}");
            is_ok = false;
        }
    }

    if !is_ok {
        std::process::exit(1);
    }
}

fn run_cargo_command(exercise: &str, command: &str) -> Result<Output, std::io::Error> {
    let exercise_soln = format!("{exercise}_soln");
    Command::new("cargo")
        .arg(command)
        .arg("--bin")
        .arg(&exercise_soln)
        .output()
}

pub fn check(exercise: &str) -> Result<(), CheckError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let exercise_main_path: PathBuf = [manifest_dir, "exercises", exercise, "main.rs"]
        .iter()
        .collect();
    let exercise_solution_path: PathBuf =
        [manifest_dir, "exercises", exercise, "solutions", "main.rs"]
            .iter()
            .collect();
    let exercise_diff_path: PathBuf = [
        manifest_dir,
        "exercises",
        exercise,
        "solutions",
        "solution.diff",
    ]
    .iter()
    .collect();

    let exercise_main = std::fs::read_to_string(exercise_main_path)
        .map_err(|_| CheckError::MainFileDoesNotExist)?;
    let exercise_solution = std::fs::read_to_string(exercise_solution_path)
        .map_err(|_| CheckError::SolutionFileDoesNotExist)?;
    let exercise_diff = std::fs::read_to_string(exercise_diff_path)
        .map_err(|_| CheckError::DiffFileDoesNotExist)?;

    let input = InternedInput::new::<&str>(exercise_main.as_ref(), exercise_solution.as_ref());
    let actual_diff = diff(
        Algorithm::Histogram,
        &input,
        UnifiedDiffBuilder::new(&input),
    );

    if actual_diff == exercise_diff {
        let clippy = run_cargo_command(exercise, "clippy")
            .map(|o| o.status.success())
            .unwrap_or(false);
        if !clippy {
            Err(CheckError::SolutionFileDoesNotClippy)
        } else {
            Ok(())
        }
    } else {
        Err(CheckError::DiffFileDoesNotMatch {
            actual: actual_diff,
            expected: exercise_diff,
        })
    }
}
