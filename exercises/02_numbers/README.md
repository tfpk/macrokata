# Exercise 2: Numbers

As a reminder, to complete this exercise:

* [ ] Read the `README.md` file, to understand what your goal is.
* [ ] Try and complete the `main.rs` file.
* [ ] Run the binary in this file, `cargo run 01_my_first_macro`.
* [ ] Get your code to compile, run, and look the same as the expected expansion.


## Macros With Arguments

Macros would be pretty useless if you couldn't modify their behaviour based on the circumstances.
To this end, let's see how we can vary what our macro does based on the contents of it's brackets.

The simplest way of doing this is to have our macro behave differently if different tokens are placed
in-between the matcher. As a reminder, the matcher is the bit in each rule before the `=>`.

Below we see a macro which will replace itself with `true` if the letter
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
# fn main() {
let _true = torf!(t);
let _false = torf!(f);
# }
```

You'll note the syntax has changed slightly -- we've gone from having one of the `() => {}` blocks (which is called a rule),
to having two. Macros try to find the first rule that matches, and replaces the macro with the
contents of the `{}` block.

It's very similar to a `match` statement, but it's important to note that you're not matching
on *variables*, you're matching on tokens.

## But what is a "token"

Up until now, we've spoken about "tokens",
without explaining what we mean further than a handwavey "it's text".

When Rust code is compiled, one of the first steps of parsing
is turning bytes of text into a "token tree", which is a
data-structure representing the text-fragments of a line of code (so
`(3 + 4)` becomes a token tree containing `3`, `+` and `4`.

This means that macro matchers ignore whitesapce, and can deal with
brackets in code specially.

As you've seen above macros let you capture all the tokens inside their brackets,
and then modify the code the write back out based on those tokens.
This ability to use code that's only been partially parsed lets you
create powerful extensions to the rust language, using your own syntax.

Further advanced reading about what tokens are can be found [here](https://doc.rust-lang.org/reference/tokens.html)

## Starting Exercise 02

You're now ready to complete `02_numbers`. Your task is to create a macro which replaces the words `one`, `two` and `three`
with the relevant numbers.
