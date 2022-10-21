macro_rules! broken {
    ("$val:literal") => { $val };
    ($val:expr) => { $val };
}

fn main() {
    let my_expr = broken!("c");
    println!("{my_expr}");
}
