# Exercise 1: My First Macro

Welcome to this introduction to Rust's Macro system.
To complete each exercise (including this one), you should:

* [ ] Read the `README.md` file, to understand what your goal is.
* [ ] Try and complete the `main.rs` file.
* [ ] Run the binary in this file, `cargo run 01_my_first_macro`.
* [ ] Get your code to compile, run, and have the main file look the same as `result.rs`


## What are Macros?

Rust's macros are a way of automatically generating code before the compiler compiles it.
Because the generation happens before the compiler does anything; it doesn't
care about types; and it can output any Rust code you'd like.

This allows you to break many of the syntax rules rust imposes on you. For example,
Rust does not allow "variadic" functions -- functions with variable numbers of 
arguments. This makes a `println` function impossible -- it would have to take any number
of arguments (`println!("hello")` and `println("{}", 123)`, for example).

Rust gets around this rule by using a `println!` macro. Before `println!` is compiled,
Rust rewrites the macro into a function which takes a single array of arguments. That way, even though
it looks to you like there are multiple arguments, once it's compiled there's always
just one array.

Macros can range from very simple -- reducing duplicated code; to very complex, implementing the
whole of HTML inside of Rust. This guide aims to build you up from the simple to the complex.

As you may have already figured out, you've already used macros -- `println!` for example, is a macro.
`vec![]` is as well. Macros always have a name. To run a macro, call it's name
with a bang (`!`) afterwards, and then brackets (any of `()`, `[]` or `{}`) containing 
arguments.

In other words, to run the macro `my_macro`, you'd say `my_macro!()` or `my_macro![]` or `my_macro!{}`.

## How do I create one?

The simplest form of macro looks like this:

```rust
macro_rules! my_macro {
    () => {
        3
    }
}

# fn main() {
let _value = my_macro!();
# }
```

The `macro_rules!` instructs the compiler that there is a new macro you are defining.
It is followed by the name of the macro, `my_macro`.
The next line specifies a "rule". Inside the normal brackets is a "matcher" -- some text (formally, we refer to the text as "tokens").
which Rust will use to decide which rule to execute.  Inside the curly brackets is a 
"transcriber", which is what rust will replace `my_macro!()` with.

So, `my_macro!()` will be replaced by `3`.


## Exercise 1: My First Macro

Your task is to write a macro named `show_output!()` which calls the `show_output()` function.

You may not edit the `main` function; but it should eventually look like the following:

```rust,ignore
<!-- cmdrun cargo run --quiet -- goal 01_my_first_macro  -->
```

