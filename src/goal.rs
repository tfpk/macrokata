use std::process::{Command, Stdio};
use std::error::Error;

pub fn goal(exercise: String) -> Result<(), Box<dyn Error>> {
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
