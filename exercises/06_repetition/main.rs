////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `if_any!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {

    if_any!(false, 0 == 1, true, {
        p
    })
}
