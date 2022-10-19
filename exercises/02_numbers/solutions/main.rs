////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! num {
    (one) => {1};
    (two) => {2};
    (three) => {3};
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(num!(one) + num!(two) + num!(three));
}
