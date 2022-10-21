
macro_rules! curry {
    (_, $block:block) => {$block};
    (($argident:ident : $argtype:ty) => $(($argidents:ident: $argtypes:ty) =>)* _, $block:block) => {
        move |$argident: $argtype| curry!($(($argidents: $argtypes) =>)* _, $block)
    };
}

////////// DO NOT CHANGE BELOW HERE /////////
fn main() {
    let is_between = curry!((min: i32) => (max: i32) => (item: &i32) => _, {
        min < *item && *item < max
    });

    let curry_filter_between = curry!((min: i32) => (max:i32) => (vec: &Vec<i32>) => _, {
        let filter_between = is_between(min)(max);
        vec.iter().filter_map(|i| if filter_between(i) { Some(*i) } else { None }).collect()
    });

    let between_3_7 = curry_filter_between(3)(7);
    let between_5_10 = curry_filter_between(5)(10);

    let my_vec = vec![1, 3, 5, 6, 7, 9];

    let some_numbers: Vec<i32> = between_3_7(&my_vec);
    println!("{some_numbers:#?}");

    let more_numbers: Vec<i32> = between_5_10(&my_vec);
    println!("{more_numbers:#?}");

}
