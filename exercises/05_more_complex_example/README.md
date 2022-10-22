# Exercise 5: A More Complex Example

In this task, we'll be implementing code to make the following syntax possible:

```rust,ignore
# fn main() {
for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
    // code
});
#}
```

This code should translate to (ignoring extra curly braces):

``` rust
# fn main() {
for row in 1..5 {
    let row: i32 = row;
    for col in 2..7 {
        let col: i32 = col;
        // code
    }
}
# }
```

To complete this task, there more fragment specifiers you will need to know
about:

 - `ident`: an "identifier", like a variable name. `ident` metavariables
    Can be followed by anything.
 - `block`: a "block expression" (anything inside curly braces).
    Can be followed by anything.
 - `ty`: a type. Can only be followed by `=>`, `,`, `=`, `|`, `;`,
    `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a `block` metavariable.

As a reminder, you may not edit the `main` function; but it should eventually
look like the following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 05_more_complex_example -->
```
