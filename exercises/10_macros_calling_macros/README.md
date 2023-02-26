# Exercise 10: Macros Calling Macros

We briefly mentioned in a previous exercise that macros are able to call
other macros. In this exercise we will look at a brief example of that.
Before we do, there are three small notes we should mention.

## Useful built-in macros

There are two useful macros which the standard library provides - `stringify!()`
and `concat!()`. Both of them produce static string slices, made up of tokens.

The `stringify!` macro takes tokens and turns them into a `&str` that
textually represents what those tokens are. For example, `stringify!(1 + 1)`
will become `"1 + 1"`.

The `concat!` macro takes a comma-separated list of literals, and creates a
`&str` which concatenates them. For example, `concat!("test", true, 99)` becomes
`"testtrue99"`.

It's useful to know that if either of these have a macro in their parameter,
(i.e. `stringify!(test!())`), the internal macro will be expanded first.
So, if `test!()` expanded to `1 + 1`, your string would be `"1 + 1"`, not 
`"test!()"`.

## The `tt` fragment specifier

An important macro specifier which we have not, as of yet, discussed,
is the `tt` macro. This captures a "Token Tree", which is any token,
or a group of tokens inside brackets. This is the most flexible
fragment specifier, because it imposes no meaning on what the captured
tokens might be. For example:

``` rust
macro_rules! stringify_number {
    (one) => {"1"};
    (two) => {"2"};
    ($tokens:tt) => { stringify!($tokens)};
}

# fn main() {
stringify_number!(one); // is "1"
stringify_number!(while); // is "while"
stringify_number!(bing_bang_boom); // is "bing_bang_boom"
# }
```

It's really important to keep in mind with `tt` macros that you **must**
ensure that anything after them can be unambiguously parsed.

In other words, the metavariable `$($thing:tt)*` (ending with `*`, `+` OR `?`) *must*
be the last fragment in the parser. Since anything can be a token tree, Rust could
not know what to accept after that parser.

To avoid this issue, you can either match a single `tt`, and make the user wrap multiple tokens
inside brackets, or you can specify a delimiter for your match (i.e. `$($thing:tt),+`, since
two token trees not separated by a `,` could not match).

## Restrictions on "Forwarding Macros"

There is one important restriction when calling a macro using another macro.

When forwarding a matched fragment to another macro-by-example, matchers in the
second macro will be passed an
[AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of the fragment type,
which cannot be matched on except as a fragment of that type. The second macro
can't use literal tokens to match the fragments in the matcher, only a fragment
specifier of the same type. The `ident`, `lifetime`, and `tt` fragment types are
an exception, and *can* be matched by literal tokens. The following illustrates
this restriction:

```rust,ignore
macro_rules! foo {
    ($l:expr) => { bar!($l); }
// ERROR:               ^^ no rules expected this token in macro call
}

macro_rules! bar {
    (3) => {}
}

# fn main() {
foo!(3);
# }
```

The following illustrates how tokens can be directly matched after matching a `tt` fragment:


```rust
// compiles OK
macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

macro_rules! bar {
    (3) => {}
}

# fn main() {
foo!(3);
# }
```

## Exercise 10: Macros Calling Macros

In this exercise, you have already been provided with a macro called `digit`, which
maps the identifiers `zero` through `nine` to a `&str` with their numeric value.

Your task is to write a macro called `number!()` which takes at least one of the identifiers `zero`
through `nine`, and converts them to a string containing numbers.

For example, `number!(one two three)` should expand to `"123"`.

**Note:** previously exercise 10 was about making a hashmap. The exercise has changed, but the old 
code is still available in the `archive/` directory. It will be removed on the next update of this book.
