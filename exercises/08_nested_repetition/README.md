# Exercise 8: Nested Repetition

In this exercise, you will need to use nested repetition. That's where you
write a repetition inside another one, for example: `( $( $( $val:expr ),+ );+ )`
would let you specify at least one value, but seperate them with either `;` and `,`.

The only oddity about nested repetition is that you must ensure that you use
meta-variables in a context where it's clear you're only referring to one of them.
In other words, the `$val` metavariable in the last paragraph *must* be used within
a nested repetition.

# Starting Exercise 8

In this exercise, you will be building a macro to load a data structure with
an adjacency list from a graph. As a refresher, graphs are data-structures
that describe how different nodes are connected.
  * [ ] 
In this example, each node is a literal, and you will be specifying, for each node,
which nodes it connects to. For example:

```rust,ignore
graph!{
    1 -> (2, 3, 4, 5);
    2 -> (1, 3);
    3 -> (2);
    4 -> ();
    5 -> (1, 2, 3);
}
```

Should get translated into a `Vec` containing the pairs `(1, 2)`, `(1, 3)`, ... `(2, 1)`, ... `(5, 3)`

You're now ready to complete `08_nested_repetition`.


