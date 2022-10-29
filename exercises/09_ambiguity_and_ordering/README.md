# Exercise 9: Ambiguity and Ordering

Up until this point, we've mostly been dealing with macros with a single rule.
We saw earlier that macros can require more than one rule, but so far we've never
had ambiguity in which rule should be followed.

There are, however, multiple circumstances where rules could have ambiguity,
so it's important to understand how macros deal with that ambiguity.

The following is adapted from the [rust documentation on
macros](https://doc.rust-lang.org/reference/macros-by-example.html#transcribing):

 - When a macro is invoked (i.e. someone writes `my_macro!()`), the compiler
   looks for a macro with that name, and tries each rule in turn.
 - To try a rule, it reads through each token in the parser in turn. There are
   three possibilities:

   1. The token found matches the matcher. In this case, it keeps parsing the
      next token. If there are no tokens left, and the matcher is complete, then
      the rule matches.
   2. The token found does not match the matcher. In this case, Rust tries the
      next rule. If there are no rules left, an error is raised as the macro
      cannot be expanded.
   3. The rule is ambiguous. In other words, it's not clear from *just this
      token* what to do. If this happens, this is an error.

 - If it finds a rule that matches the tokens inside the brackets; it starts
   transcribing. *Once a match is found, no more rules are examined*.

Let's have a look at some examples:

```rust,ignore
macro_rules! ambiguity {
    ($($i:ident)* $j:ident) => { };
}

# fn main() {
ambiguity!(error);
# }
```

This example fails because Rust is not able to determine what `$j` should be just by looking at
the current token. If Rust could look forward, it would see that `$j` must be followed by a `)`,
but it cannot, so it causes an error.

```rust
macro_rules! ordering {
    ($j:expr) => { "This was an expression" };
    ($j:literal) => { "This was a literal" };
}

# fn main() {
let expr1 = ordering!('a');  // => "This was an expression".
let expr1 = ordering!(3 + 5);  // => "This was an expression".
# }
```

This example demonstrates an example where Rust macros can behave strangely due to
ordering rules: even though `literal` is a much stricter condition than `expr`,
because `literal`s are `expr`s, the first rule will always match.

## Exercise 9: Ambiguity and Ordering

This task is a little bit different to previous tasks: we have given you
a partially functional macro already, along with some invocations of that macro.

You should adjust the macro's rules and syntax to make sure that you
achieve the correct behaviour without any ambiguity.

 - `sum!()` should sum together two or more expressions together.
 - `get_number_type!()` should determine what sort of Rust syntax is being used:
    a positive literal, a negative literal, a block, or an expression.

You may not edit the `main` function, but it should eventually look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 09_ambiguity_and_ordering -->
```
