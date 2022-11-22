////////// DO NOT CHANGE BELOW HERE /////////

/// This enum should represent what code the user wrote exactly.
/// Even though to a compiled program there's no difference,
/// this will let the program tell what sort of code the user wrote.
#[derive(Debug)]
enum NumberType {
    /// The user wrote a literal, positive number.
    PositiveNumber(u32),
    /// The user wrote a literal, negative number.
    NegativeNumber(i32),
    /// We can't tell if the user's code is positive or negative because it's a block.
    UnknownBecauseBlock(i32),
    /// We can't tell if the user's code is positive or negative because it's an expression.
    UnknownBecauseExpr(i32),
}

impl NumberType {
    fn show(&self) {
        println!("{self:#?}");
    }
}
////////// DO NOT CHANGE ABOVE HERE /////////

// Sum together at least two expressions.
macro_rules! sum {
    ($firstexpr:expr , $($expr:expr),+) => {
        $firstexpr $( + $expr )+
    }
}

macro_rules! get_number_type {
    ( -$negative:literal ) => {
        NumberType::NegativeNumber(-$negative)
    };
    ( $positive:literal ) => {
        NumberType::PositiveNumber($positive)
    };
    ( $block:block ) => {
        NumberType::UnknownBecauseBlock($block)
    };
    ( $expr:expr ) => {
        NumberType::UnknownBecauseExpr($expr)
    };
}

////////// DO NOT CHANGE BELOW HERE /////////
fn main() {
    // PositiveNumber
    get_number_type!(5).show();

    // NegativeNumber
    get_number_type!(-5).show();

    // UnknownBecauseBlock
    #[allow(clippy::let_and_return)]
    get_number_type!({
        let x = 6;
        x
    })
    .show();

    // UnknownBecauseExpr
    get_number_type!(sum!(1, 2, 3, 4)).show();
    get_number_type!(3 + 5 - 1).show();
}
