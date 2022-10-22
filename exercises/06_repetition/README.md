# Exercise 6: Repetition

Hopefully, you're now feeling pretty confident with metavariables.
But, one of the first justifications we gave for macros was their
ability to simulate "variadic" functions (functions which have a
variable number of arguments).

A simple approach might be to write a rule for each number of arguments;
for example:

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
    let vec: Vec<i32> = listing_literals!(the 9 and the 5);
}
```

This is very clunky, and involves a large amount of repeated code.
What if we could say that we want *a variable number* of a particular patterns.
That would let us say "give me any number of `$e:expr` tokens, and I'll tell you what to do with them'".

Macro repetitions let us do just that. They consist of three things:
 - a list of tokens that we want to match repeatedly.
 - optionally, a separator token (which tells the parser what to look for between each match)
 - either `+`, `*` or `?`; which says how many times to expect a match. `+` means "at least once".
   `*` means "any number of times, including 0 times". `?` means "either 0 times, or 1 time".

Let's look at an example of a macro repetition, to parse the exact macro
we showed above.

The matcher we would use for this is: `$(the $my_literal:literal)and+`.
Let's break that down:

 - `$(` tells us that we're starting a repetition.
 - `the $my_literal:literal` are the tokens we are matching. We'll match the exact text "the", and then a literal token.
 - the `)` means that we're done describing the pattern to match.
 - the `and` is optional, but is the "separator" -- a token you can use to seperate multiple repetitions.
 - Here, we use `+`, which means the repetition must happen at least once. `*` would have worked just as well if we were okay with an empty `Vec`.

What's now left is to use the matched values. To do this, the rule would be something like:

```rust,ignore
 $(the $my_literal:literal)and+ => {
    {
        let mut my_vec = Vec::new();
        $(my_vec.push($my_literal));+;
        my_vec
    }
}
```

The line `$(my_vec.push($my_literal));+;` is nearly identical to the repetition we saw above, but to break it down:

 - `$(` tells us that we're starting a repetition.
 - `my_vec.push($my_literal)` that will be transcribed. `$my_literal` will be replaced with each of the literals specified in the matcher.
 - the `)` means that we're done describing the pattern to match.
 - the `;` means we're seperating these lines with semicolons
 - The `+` ends the repetition.
 - The `;` adds a final semicolon after the expansion of everything.

So this will expand into the same code we saw above!

It's worth noting that we've used an extra set of curly braces in our transcriber. This is because if you don't
put the code in a block, the `let` block will be on the right hand side of the `=` sign; which doesn't make sense.

If you put the code in a curly brace, then the right-hand side of the `=` sign will be a block which returns `my_vec`.

# Starting Exercise 6

You're now ready to complete `06_repetition`.
Your task is to make a macro called `if_any!`, which takes a variable amount of boolean
expressions; and if any of them are true, runs the block which is the last argument.
