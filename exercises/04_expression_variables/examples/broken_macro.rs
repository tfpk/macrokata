macro_rules! broken_macro {
    ($a:expr please) => { $a }
}

fn main() {
    // Fails to compile!
    let value = broken_macro!(3 + 5 please);
}
