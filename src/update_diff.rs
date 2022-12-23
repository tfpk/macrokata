use imara_diff::intern::InternedInput;
use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
use std::error::Error;
use std::path::PathBuf;

use std::fs::File;
use std::io::prelude::*;

pub fn update_diff(exercise: &str) -> Result<(), Box<dyn Error>> {
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

    let exercise_main = std::fs::read_to_string(exercise_main_path)?;
    let exercise_solution = std::fs::read_to_string(exercise_solution_path)?;

    let input = InternedInput::new::<&str>(exercise_main.as_ref(), exercise_solution.as_ref());
    let actual_diff = diff(
        Algorithm::Histogram,
        &input,
        UnifiedDiffBuilder::new(&input),
    );

    let mut file = File::create(exercise_diff_path)?;
    file.write_all(actual_diff.as_bytes())?;

    Ok(())
}
