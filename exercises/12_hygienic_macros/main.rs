macro_rules! coord {
    ($x:expr, $y:expr) => {
        $crate::Coordinate { x: $x, y: $y }
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::third_dimension::Coordinate {
            x: $x,
            y: $y,
            z: $z,
        }
    };
    ($x:expr, $y:expr, $z:expr, $t:expr) => {
        $crate::fourth_dimension::Coordinate {
            x: $x,
            y: $y,
            z: $z,
            t: $t,
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

#[derive(Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub mod third_dimension {
    #[derive(Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Coordinate {
        pub fn as_2d(self) -> super::Coordinate {
            coord!(self.x, self.y)
        }
    }
}

pub mod fourth_dimension {
    #[derive(Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub t: i32,
    }

    impl Coordinate {
        pub fn as_3d(self) -> super::third_dimension::Coordinate {
            coord!(self.x, self.y, self.z)
        }
    }
}

fn main() {
    let four_dim = coord!(1, 2, 3, 1000);
    let three_dim = four_dim.as_3d();
    let two_dim = three_dim.as_2d();

    println!("{two_dim:?}");
}
