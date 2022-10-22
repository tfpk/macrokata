# Exercise 5: A more complex example

In this exercise, we'll be implementing code to make the 
following syntax possible:

```rust,ignore
# fn main() {
for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
    // code
});

let values = [1, 3, 5];

for_2d!(x <u16> in values, y <u16> in values, {
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
let values = [1, 3, 5];
for x in values {
    let x: u16 = x;
    for y in values {
        let y: u16 = y;
        // code
    }
}
# }
```

`

To complete this task, there are a few more metavariables you will
need to know about:

 - `ident`: an "identifier", like a variable name. `ident` metavariables
    Can be followed by anything.
 - `block`: a "block expression" (anything inside curly braces).
    Can be followed by anything.
 - `ty`: a type. Can only be followed by `=>`, `,`, `=`, `|`, `;`,
    `:`, `>`, `>>`, `[`, `{`, `as`, `where`, or a `block` metavariable.
