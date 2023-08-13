# Exercise 12: Hygiene

To quote [the reference](https://doc.rust-lang.org/reference/macros-by-example.html#hygiene):

> By default, all identifiers referred to in a macro are expanded as-is, and are
> looked up at the macro's invocation site. This can lead to issues if a macro
> refers to an item (i.e. function/struct/enum/etc.) or macro which isn't in scope at the invocation site. To
> alleviate this, the `$crate` metavariable can be used at the start of a path to
> force lookup to occur inside the crate defining the macro.

Here is an example to illustrate (again, taken from the reference linked above):

```rust,ignore
// Definitions in the `helper_macro` crate.
#[macro_export]
macro_rules! helped {
    /*
    () => { helper!() }
         // ^^^^^^ This might lead to an error due to 'helper' not being in scope.
    */
    () => { $crate::helper!() }
}

#[macro_export]
macro_rules! helper {
    () => { () }
}

// Usage in another crate.
// Note that `helper_macro::helper` is not imported!

use helper_macro::helped;

fn unit() {
    helped!();
}
```

In other words, this means that a macro needs to have all the functions/structs/macros it uses in scope
at its *call site*, not at the place where it is defined. This is fine if the macro is used within a single file, but if a
macro is exported, then it makes things complicated.

The `$crate` metavariable lets you refer to things that are in the crate the macro was defined in (as opposed to the 
crate the macro was called in). If the macro was defined in crate `foo`, and used in crate `bar`, then a reference
to a struct `Widget` like: `Widget::new()` is creating a new `bar::Widget` (and if one doesn't exist, you'll get an error).
If you called `$crate::Widget::new()`, then you're always talking about `foo::Widget`, no matter what crate you're in.

## A footnote on how expanding macros into text is misleading

(Based on the disclaimer for the brilliant
[cargo-expand](https://github.com/dtolnay/cargo-expand/))

Be aware that macro expansion to text is a lossy process. That means that the
expanded code we show in these kata should be used as a debugging aid only.
There should be no expectation that the expanded code can be compiled
successfully, nor that if it compiles then it behaves the same as the original
code. In these kata, we try to avoid these issues as far as possible.

For instance, `answer = 3` when compiled ordinarily by Rust,
but the expanded code, when compiled, would set `answer = 4`.

```rust
fn main() {
    let x = 1;

    macro_rules! first_x {
        () => { x }
    }

    let x = 2;

    let answer = x + first_x!();
}

```

Refer to [The Little Book Of Rust Macros](https://veykril.github.io/tlborm/decl-macros/minutiae/hygiene.html)
for more on the considerations around macro hygiene.

## Exercise 12

Exercise 12 consists of a file containing multiple modules. Fix the code so
that the macro works correctly in all invocations.

Note that you will need to use the `$crate` metavariable.
