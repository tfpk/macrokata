// TODO: Create the `curry!()` macro.

////////// DO NOT CHANGE BELOW HERE /////////

fn print_numbers(nums: &Vec<i32>) {
    println!("Resulting Numbers: {nums:#?}");
}

fn get_example_vec() -> Vec<i32> {
    vec![1, 3, 5, 6, 7, 9]
}

fn print_curried_argument(val: impl std::fmt::Debug) {
    println!("Currying value {:?}.", val);
}

fn main() {
    println!("=== defining functions ===");
    let is_between = curry!((min: i32) => (max: i32) => (item: &i32) => _, {
        min < *item && *item < max
    });

    let curry_filter_between = curry!((min: i32) => (max:i32) => (vec: &Vec<i32>) => _, {
        let filter_between = is_between(min)(max);
        vec.iter().filter_map(|i| if filter_between(i) { Some(*i) } else { None }).collect()
    });

    println!("=== create between_3_7 ===");
    let between_3_7 = curry_filter_between(3)(7);
    println!("=== create between_5_10 ===");
    let between_5_10 = curry_filter_between(5)(10);

    let my_vec = get_example_vec();

    println!("=== call between_3_7 ===");
    let some_numbers: Vec<i32> = between_3_7(&my_vec);
    print_numbers(&some_numbers);

    println!("=== call between_5_10 ===");
    let more_numbers: Vec<i32> = between_5_10(&my_vec);
    print_numbers(&more_numbers);
}
