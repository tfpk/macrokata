// This function should be called by the `show_output!()` macro
fn show_output() {
    println!("I should appear as the output.")
}

macro_rules! show_output {
    () => {show_output()}
}

fn main() {
    show_output!()
}
