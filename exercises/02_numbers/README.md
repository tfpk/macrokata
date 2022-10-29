# Exercise 2: Numbers

As a reminder, to complete this exercise:

* [ ] Read this file to understand the theory being tested, and what
      task you will be asked to complete.
* [ ] Try and complete the `main.rs` file.
* [ ] Test to see if your macro creates the same code we have; using
      `cargo run -- test 02_numbers`.
* [ ] Run your code, using `cargo run --bin 02_numbers`, to see what it does.

## Macros With Arguments

Macros would be pretty useless if you couldn't modify their behaviour based on
input from the programmer. To this end, let's see how we can vary what our macro
does.

The simplest way of doing this is to have our macro behave differently if
different tokens are placed in-between the matcher. As a reminder, the matcher
is the bit in each rule before the `=>`.

Below we see a macro which will replace itself with `true` if the letter `t` is
inside the brackets; and `f` otherwise.


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

You'll note the syntax has changed slightly: we've gone from having one of the
`() => {}` blocks (which is called a rule) to having two. Macros try to find
the first rule that matches, and replaces the macro with the contents of the
transcriber block.

Macros are very similar to a `match` statement because they find the first match
and take action based on that; but it's important to note that you're not matching
on *variables*, you're matching on tokens.

## But what is a "token"

Up until now, we've spoken about "tokens" without explaining what we mean,
further than a handwavy "it's text".

When Rust code is compiled, one of the first steps of parsing is turning bytes
of text into a "token tree", which is a data-structure representing the
text-fragments of a line of code (so `(3 + (4 + 5))` becomes a token tree containing
`3`, `+` and another token tree containing `4`, `+` and `5`).

This means that macro matchers aren't restricted to matching exact text, and that
they preserve brackets when matching things.

As you've seen above, macros let you capture all the tokens inside their
brackets, and then modify the code the write back out based on those tokens.
This ability to react to different pieces of code without them having been fully
compiled lets us create powerful extensions to the Rust language, using your own
syntax.

Further advanced reading about what tokens are can be found [here.](https://doc.rust-lang.org/reference/tokens.html)

## Exercise 2: Numbers

Your task is to create a macro called `num` which replaces the words `one`, `two` and `three`
with the relevant numbers.

You may not edit the `main` function, but it should eventually look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 02_numbers -->
```
