use imara_diff::intern::InternedInput;
use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

pub fn test(exercise: String) -> Result<(), Box<dyn Error>> {
    let color = if atty::is(atty::Stream::Stdout) {
        "always"
    } else {
        "never"
    };

    let main_output = Command::new("cargo")
        .arg("expand")
        .arg("--color")
        .arg(color)
        .arg("--bin")
        .arg(&exercise)
        .arg("main")
        .output()
        .unwrap();

    if !main_output.status.success() {
        println!("Got some errors when expanding the macro:");
        println!();
        io::stderr().write_all(&main_output.stderr)?;
    } else {
        println!("This is the expansion you produced:");
        println!();
        io::stdout().write_all(&main_output.stdout)?;

        let soln_output = Command::new("cargo")
            .arg("expand")
            .arg("--color")
            .arg(color)
            .arg("--bin")
            .arg(format!("{exercise}_soln"))
            .arg("main")
            .output()
            .unwrap();

        println!("\nThe expansion we expected is:\n");

        io::stdout().write_all(&soln_output.stdout)?;

        let before = String::from_utf8_lossy(&main_output.stdout);
        let after = String::from_utf8_lossy(&soln_output.stdout);
        let input = InternedInput::new(before.as_ref(), after.as_ref());
        let the_diff = diff(
            Algorithm::Histogram,
            &input,
            UnifiedDiffBuilder::new(&input),
        );

        if the_diff.is_empty() {
            println!("\nCongratulations! You solved it.\n");
        } else {
            println!("\nThe diff is:\n");
            println!("{the_diff}");
        }
    }

    Ok(())
}
