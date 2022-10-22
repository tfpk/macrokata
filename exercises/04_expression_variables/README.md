# Exercise 4: Expression Metavariables

We can now capture fragments of Rust code that are literals, however there are
other fragments of rust code which can be captured in metavariables. In general,
every metavariable is of the form `$<NAME>:<FRAGSPEC>`. `<NAME>` is replaced
with the name of the metavariable; but `FRAGSPEC` is more interesting. It means
"Fragment Specifier", and it tells you what sort of fragment of Rust code you
intend to match. We've already seen `literal`, but another common fragment
specifier is `expr`, which allows you to capture any rust expression (for
example, `(3 * 5)` or `function_call() + CONSTANT`).

Using this specifier is nearly identical to using the `literal` fragment
specifier -- `$x:expr` indicates a metavariable, which is an expression, named
`x`.

It's also worth mentioning the fragment specifier `stmt`, which is similar to
`expr`, but allows Rust statements too, like `let` statements.

# Macros and the Precedence of Operators

Macros do not affect the order of operations. If the expression `3 * math!(4,
plus, 2)` expands to `3 * 4 + 2`; the answer will be 14, not 18 (as you might
expect from the brackets).

# "Follow-set Ambiguity Rules"

The rust parser needs to have some way of telling "where does a metavariable
end". If it didn't, expressions like `$first:expr $second:expr` would be confusing to
parse in some circumstances (for example, how would you parse `a * b * c * d`?
Would `first` be `a`, and `second` be `*b * c * d`? Or would `first` be `a * b * c`,
and `second` be `* d`?

To avoid this problem entirely, Rust has a set of rules called the "follow-set
ambiguity rules". These tell you which tokens are allowed to follow a
metavariable (and which aren't).

For `literal`s, this rule is simple -- anything can follow a literal
metavariable.

For `expr` (and it's friend `stmt`) the rules are much more restrictive -- they
can only be followed by `=>` or `,` or `;`.

This means that building a matcher like this:

``` rust,ignore
macro_rules! broken_macro {
    ($a:expr please) => $a
}

fn main() {
    // Fails to compile!
    let value = broken_macro!(3 + 5 please);
}
```

will give you this compiler error:

``` rust,ignore
error: `$a:expr` is followed by `please`, which is not allowed for `expr` fragments
 --> broken_macro.rs:2:14
  |
2 |     ($a:expr please) => { $a }
  |              ^^^^^^ not allowed after `expr` fragments
  |
  = note: allowed there are: `=>`, `,` or `;`
```


As we encounter more expression types, we'll make sure to mention their
follow-set rules; but [this page in the Rust
reference](https://doc.rust-lang.org/reference/macros-by-example.html#follow-set-ambiguity-restrictions)
has a comprehensive list of the rules for each fragment specifier type.


## Exercise 4: Expression Variables

In this task, you will be completing a similar task to the previous one.
Last time, your macro should have worked with any *literal*, but now we would
like a macro which works with any *expression*.


 - The syntax `math!(3, plus, (5 + 6))` should expand to `3 + (5 + 6)`, where
   `3` and `(5 + 6)` could be any expression.
 - The syntax `math!(square my_expression)` should expand to `my_expression *
   my_expression`, where `my_expression` could be any expression.

You may not edit the `main` function; but it should eventually look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 04_expression_variables -->
```
