////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create `for_2d!` macro here.
macro_rules! for_2d {
    (
        $row: ident <$row_type: ty> in $row_items: expr,
        $col: ident <$col_type: ty> in $col_items: expr,
        $block: block
    ) => {
        for $row in $row_items {
            let $row: $row_type = $row;
            for $col in $col_items {
                let $col: $col_type = $col;
                $block
            }
        }
    };
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
