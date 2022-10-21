macro_rules! curry_unwrapper {
    ($block:block) => {
        $block
    };
    (
        $argname:ident: $argtype:ty,
        $($argnames:ident: $argtypes:ty,)*
        $block:block
    ) => {
        Box::new(move |$argname : $argtype | {
            curry_unwrapper!($($argnames: $argtypes,)* $block)
        })
    }
}

macro_rules! box_type {
    (=> $type:ty) => {
        $type
    };
    ($type:ty $(,$argtypes:ty )* => $restype:ty) => {
        Box<dyn Fn($type) -> box_type!($($argtypes ),* => $restype)>
    }
}

macro_rules! curry_fn {
    (
        $ident:ident,
        ($argname:ident: $argtype:ty)
            -> $(($argnames:ident: $argtypes:ty))->*
            => $restype:ty, $block:block
    ) => {
        fn $ident($argname: $argtype) -> box_type!($($argtypes ),* => $restype) {
            curry_unwrapper!($($argnames: $argtypes,)* $block)
        }
    }
}

fn main() {
    curry_fn!(add, (a: i32) -> (b: i32) -> (c: i32) -> (d: i32) => i32, {
        a + b + c + d
    });

    let res = add(3)(2)(3)(4);


    println!("{res}");
}
