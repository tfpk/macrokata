////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
fn show_output() {
    println!("I should appear as the output.")
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! show_output {
    () => {
        show_output()
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    show_output!()
}
