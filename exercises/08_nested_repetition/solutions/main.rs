////////// DO NOT CHANGE BELOW HERE /////////

fn print_vec<V: std::fmt::Debug>(vec: &Vec<V>) {
    println!("{vec:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! graph {
    ( $($from:literal -> ( $( $to:literal),* );)*  ) => {
        {
            let mut vec = Vec::new();
            $( $( vec.push(($from, $to)); )* )*

            vec
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let my_graph = graph!(
        1 -> (2, 3, 4, 5);
        2 -> (1, 3);
        3 -> (2);
        4 -> ();
        5 -> (1, 2, 3);
    );

    print_vec(&my_graph);
}
