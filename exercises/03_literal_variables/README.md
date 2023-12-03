# Exercise 3: Literal Metavariables

In the last exercise, we saw how we could change the behaviour of
a macro based on text inside the brackets. This is great, but it's
basically an if statement on the text inside the brackets: it's
very simplistic.

Now we will introduce the concept of a "metavariable". Metavariables capture a
particular part of the text inside the macro's brackets, and let you reuse it.

The syntax for a metavariable is simple. To explain the syntax, see the example
below:

```rust,ignore
macro_rules! do_thing {
    (print $metavar:literal) => {
        println!("{}", $metavar)
    };
}
```

The `$metavar:literal` is saying that you're capturing any `literal` (which is
something like `'a'`, or `3`, or `"hello"`), and naming it `metavar`. Then,
`$metavar` inside the `println!` is saying to "fill in" that space with whatever
`metavar` is.

For an invocation like

```rust
# macro_rules! do_thing {
#     (print $metavar:literal) => {
#         println!("{}", $metavar)
#     };
# }
#
# fn main() {
do_thing!(print 3);
# }
```

Rust understands that `metavar` means `3`. So, when doing substitution,
it starts by writing

```rust,ignore
println!("{}", $metavar);
```

and then substitutes `3` for `$metavar`:

``` rust
# fn main() {
println!("{}", 3);
# }
```

## But what about types?

You might be wondering why we haven't said anything about the *type* of the
literal. It turns out that the type doesn't matter during macro expansion. Rather
than needing the type, Rust just needs to know what sort of syntax to expect. If
you tried to provide a variable name, and you needed a literal, Rust will throw
an error. If you needed a *string* literal, and you provided a *char* literal,
then Rust will happily expand the code. It'll throw an error later on in the
compilation process, as if you had written the expanded code.

## Why do these examples avoid using macros?

The example above uses the `println!` macro inside the `do_thing`
macro. Rust is totally fine with this! However, `macrokata` tries
to avoid (as much as possible) using macros we didn't define inside
the main function. The reason for this is that, if we did use `println!`
you would see its expansion as well. That could be confusing, since

```rust,ignore
print("some text")
```

is much easier to read than

```rust,ignore
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["some text"],
                &[],
            ),
        );
    };

```

## Exercise 3: Literal Meta-Variables

Your task is to create a macro which can perform two small bits of math:

 - The syntax `math!(3 plus 5)` should expand to `3 + 5`, where `3` and `5`
   could be any literal.
 - The syntax `math!(square 2)` should expand to `2 * 2`, where `2` could be any
   literal.

You may not edit the `main` function, but it should eventually look like the
following:

<!-- If you can see this text, it means you're not looking at the book.   -->
<!-- Run the cargo command below (without `cmdrun`) to see the real code. -->
```rust,ignore
<!-- cmdrun cargo run -- goal 03_literal_variables -->
```
