////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_pair<K, V>(pair: (K, V)) {
    println!("{pair:#?}");
}
fn print_hashmap<K, V>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create a `pair!()` macro.

// TODO: Create a `hashmap!()` macro that uses the `pair!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);

}
