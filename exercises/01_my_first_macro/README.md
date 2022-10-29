# Exercise 1: My First Macro

Welcome to this introduction to Rust's Macro system.
To complete each exercise (including this one), you should:

* [ ] Read this file to understand the theory being tested, and what
      task you will be asked to complete.
* [ ] Try and complete the `main.rs` file.
* [ ] Test to see if your macro creates the same code we have, using
      `cargo run -- test 01_my_first_macro`.
* [ ] Run your code, using `cargo run --bin 01_my_first_macro`, to see what it does.


## What are Macros?

Rust's macros are a way of using code to generate code before compilation.
Because the generation happens before the compiler does anything, you are given
much more flexibility in what you can write.

This allows you to break many of the syntax rules Rust imposes on you. For
example, Rust does not allow "variadic" functions: functions with variable
numbers of arguments. This makes a `println` function impossible -- it would
have to take any number of arguments (`println("hello")` and `println("{}",
123)`, for example).

Rust gets around this rule by using a `println!` macro. Before `println!` is
compiled, Rust rewrites the macro into a function which takes a single array of
arguments. That way, even though it looks to you like there are multiple
arguments, once it's compiled there's always just one array.

Macros can range from simple (e.g. reducing duplicated code) to complex (e.g.
implementing HTML parsing inside of Rust). This guide aims to build you up from
the simple to the complex.

As mentioned, you've already used macros: `println!` for example, is a macro.
`vec![]` is as well. Macros always have a name. To run a macro, call its name
with a bang (`!`) afterwards, and then brackets (any of `()`, `[]` or `{}`)
containing arguments.

In other words, to run the macro `my_macro`, you'd say `my_macro!()` or
`my_macro![]` or `my_macro!{}`.

## Macro Rules vs. Procedural Macros

Rust has two macros systems, but this guide will only focus on one.
`macro_rules!` are a special language to describe how to transform
code into valid Rust code: this is the system we will focus on.
Procedural macros (proc-macros) are a method of writing a Rust function
which transforms an input piece of Rust code into an output piece.

Proc Macros are useful, but complex, and not the subject of this guide.
[You can read more about them here.](https://doc.rust-lang.org/reference/procedural-macros.html)

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

The `macro_rules!` instructs the compiler that there is a new macro you are
defining. It is followed by the name of the macro, `my_macro`. The next line
specifies a "rule". Inside the normal brackets is a "matcher" -- some text
(formally, we refer to the text as "tokens") -- which Rust will use to decide
which rule to execute. Inside the curly brackets is a "transcriber", which is
what Rust will replace `my_macro!()` with.

So, `my_macro!()` will be replaced by `3`.


## Exercise 1: My First Macro

Your task is to write a macro named `show_output!()` which calls the
`show_output()` function.

You may not edit the `main` function, but it should eventually look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 01_my_first_macro -->
```
