# Exercise 4: Expression Metavariables

We've now seen literal metavariables, however there are many fragments of Rust
which can be captured in metavariables. A common fragment specifier is `expr`,
which allows you to capture any rust expression (for example, `(3 * 5)` or
`function_call() + CONSTANT`).

Using this specifier is nearly identical to using the `literal` fragment specifier.
`$x:expr` indicates a metavariable named `x`.

At this point, it's also worth mentioning the fragment specifier `stmt`, which is
very similar to `expr`, but allows Rust statements too, like `let` statements.

# "Follow-set Ambiguity Rules"

The rust parser needs to have some way of telling "where does a
metavariable end". If it didn't, expressions like `$a:expr $b:expr`
would be confusing to parse in some circumstances (for example,
how would you parse `3 * 4 * b` -- is a `3 * 4`, and b `*b`? is
`b` supposed to be empty?).

To avoid this problem entirely, Rust has a set of rules called the
"follow-set ambiguity rules". These tell you what is allowed (and
what isn't) to follow a metavariable.

For `literal`s, this rule is simple -- anything can follow a literal
metavariable.

For `expr` (and it's friend `stmt`) the rules are much more restrictive --
they can only be followed by `=>` or `,` or `;`.

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


As we encounter more expression types, we'll make sure to mention their follow-set rules;
but [this page in the Rust reference](https://doc.rust-lang.org/reference/macros-by-example.html#follow-set-ambiguity-restrictions)
has a comprehensive list of the rules for each fragment specifier type.

YOu are now ready for exercise `04_expression_variables`.
