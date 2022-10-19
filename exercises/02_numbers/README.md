# Exercise 2: Numbers

As a reminder, to complete this exercise:

* [ ] Read the `README.md` file, to understand what your goal is.
* [ ] Try and complete the `main.rs` file.
* [ ] Run the binary in this file, `cargo run 01_my_first_macro`.
* [ ] Get your code to compile, run, and have the main file look the same as `result.rs`


## Macros With Arguments

Macros would be pretty useless if you couldn't modify their behaviour based on the circumstances.
To this end, let's see how we can vary what our macro does based on how it's called.

The simplest way of doing this is to have our macro behave differently if different letters are placed
in-between the brackets. Below we see a macro which will replace itself with `true` if the letter
`t` is inside the brackets; and `f` otherwise.


``` rust
macro_rules! torf {
    (t) => {
        true
    };
    (f) => {
        false
    };
}
```

You'll note the syntax has changed slightly -- we've gone from having one of the `() => {}` blocks,
to having two. Macros try to find the first block that matches, and replaces the macro with the
contents of the `{}` block.

It's very similar to a `match` statement, but it's important to note that you're not matching
on *variables*, you're matching on literal text.

You're now ready to complete `02_numbers`.
