# Exercise 3: Literal Meta-Variables

In the last exercise, we saw how we could change the behaviour of
a macro based on text inside the brackets. This is great, but it's
basically an if statement on the text inside the brackets -- it's not
very powerful.

In this exercise, we will introduce the concept of a "metavariable".
Metavariables capture a particular part of the text inside the macro's
brackets; and let you reuse it.

The syntax for a meta-variable is simple. To explain the syntax, see the
example below:

``` rust
macro_rules! print_me {
    ($metavar:literal) => {
        println!("{}", $metavar)
    };
}
```

The `$metavar:literal` is saying that you're capturing any `literal`
(which is something like `'a'`, or `3`, or `"hello"`), and naming it
`metavar`. Then, `$metavar` inside the `println!` is saying to "fill in"
that space with whatever `metavar` is.


## A note on these exercises

The example above uses the `println!` macro inside the `print_me`
macro. Rust is totally fine with this! However, `macrokata` tries 
to avoid (as much as possible) using macros we didn't define inside
the main function. The reason for this is that, if we did use `println!`
you would see it's expansion as well. That could be confusing, since:

``` rust
print("some text")
```

is much easier to read than:

``` rust
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["some text"],
                &[],
            ),
        );
    };

```
`

