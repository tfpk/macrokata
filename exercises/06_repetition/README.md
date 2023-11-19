# Exercise 6: Repetition

Hopefully, you're now feeling pretty confident with metavariables. One of
the first justifications we gave for macros was their ability to simulate
"variadic" functions (functions which have a variable number of arguments). In
this exercise, we'll have a look at how you can implement them yourself.

A simple approach might be to write a rule for each number of arguments. For
example, one might write

```rust
macro_rules! listing_literals {
    (the $e1:literal) => {
        {
            let mut my_vec = Vec::new();
            my_vec.push($e1);
            my_vec
        }
    };
    (the $e1:literal and the $e2:literal) => {
        {
            let mut my_vec = Vec::new();
            my_vec.push($e1);
            my_vec.push($e2);
            my_vec
        }
    };
    (the $e1:literal and the $e2:literal and the $e3:literal) => {
        {
            let mut my_vec = Vec::new();
            my_vec.push($e1);
            my_vec.push($e2);
            my_vec.push($e3);
            my_vec
        }
    }
}

fn main() {
    let vec: Vec<&str> = listing_literals!(the "lion" and the "witch" and the "wardrobe");
    assert_eq!(vec, vec!["lion", "witch", "wardrobe"]);
    let vec: Vec<i32> = listing_literals!(the 9 and the 5);
    assert_eq!(vec, vec![9, 5]);
}
```

This is very clunky, and involves a large amount of repeated code. Imagine doing
this for 10 arguments! What if we could say that we want *a variable number* of
a particular patterns? That would let us say "give me any number of `$e:expr`
tokens, and I'll tell you what to do with them'".

Macro repetitions let us do just that. They consist of three things:
 - A group of tokens that we want to match repeatedly.
 - Optionally, a separator token (which tells the parser what to look for between each match).
 - Either `+`, `*` or `?`, which says how many times to expect a match. `+` means "at least once".
   `*` means "any number of times, including 0 times". `?` means "either 0 times, or 1 time".

Let's look at an example of a macro repetition, to parse the exact macro
we showed above.

The matcher we would use for this is `$(the $my_literal:literal)and+`.
To break that down:

 - `$(` means that we're starting a repetition.
 - Inside the brackets, `the $my_literal:literal` is the pattern we're matching. We'll match the exact text "the", and then a literal token.
 - The `)` means that we're done describing the pattern to match.
 - The `and` is optional, but it is the "separator": a token you can use to separate multiple repetitions. Commonly it's `,` to comma-separate things.
 - Here, we use `+`, which means the repetition must happen at least once. `*` would have worked just as well if we were okay with an empty `Vec`.

What's now left is to use the matched values. To do this, the rule would be something like:

```rust,ignore
($(the $my_literal:literal)and+) => {
    {
        let mut my_vec = Vec::new();
        $(my_vec.push($my_literal));+;
        my_vec
    }
}
```

The line `$(my_vec.push($my_literal));+;` is nearly identical to the repetition we saw above, but to break it down:

 - `$(` tells us that we're starting a repetition.
 - `my_vec.push($my_literal)` is the code that will be transcribed. `$my_literal` will be replaced with each of the literals specified in the matcher.
 - The `)` means that we're done describing the code that will be transcribed.
 - The `;` means we're separating these lines with semicolons. Note that if you want, this could also be empty (to indicate they should be joined without anything in the middle).
 - The `+` ends the repetition.
 - The `;` adds a final semicolon after the expansion of everything.

So this will expand into the same code we saw above!

It's worth noting that we've used an extra set of curly braces in our transcriber. This is because if you don't
put the code in a block, the code will look like `let whatever = let mut my_vec = Vec::new();`, which doesn't make sense.

If you put the code in a curly brace, then the right-hand side of the `=` sign will be a block which returns `my_vec`.

## Exercise 6: Repetition

In this task, you will be creating an `if_any!` macro. If any of the first arguments are true,
it should execute the block which is the last argument.

You may not edit the `main` function, but once you have completed the exercise, your `if_any!` macro should expand to look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 06_repetition -->
```
