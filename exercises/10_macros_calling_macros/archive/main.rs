////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;
use std::fmt::Debug;

fn print_pair<K: Debug, V: Debug>(pair: (K, V)) {
    println!("{pair:#?}");
}
fn print_hashmap<K: Debug, V: Debug>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create a `pair!()` macro.

// TODO: Create a `hashmap!()` macro that uses the `pair!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair: (char, u8) = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        String::from("Hash") => "map",
        String::from("Key") => value,
    );

    print_hashmap(&my_hashmap);
}
