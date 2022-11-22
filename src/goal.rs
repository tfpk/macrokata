use std::error::Error;
use std::process::{Command, Stdio};

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
