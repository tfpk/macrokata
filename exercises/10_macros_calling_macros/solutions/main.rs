macro_rules! digit {
    (zero) => {
        "0"
    };
    (one) => {
        "1"
    };
    (two) => {
        "2"
    };
    (three) => {
        "3"
    };
    (four) => {
        "4"
    };
    (five) => {
        "5"
    };
    (six) => {
        "6"
    };
    (seven) => {
        "7"
    };
    (eight) => {
        "8"
    };
    (nine) => {
        "9"
    };
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! number {
    ($($num:ident )+) => (concat!($(digit!($num)),+))
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let my_number = number!(nine three seven two zero).parse::<u32>().unwrap();
    let my_other_number = number!(one two four six eight zero).parse::<u32>().unwrap();
    println!("{}", my_number + my_other_number); // = 218400
}
