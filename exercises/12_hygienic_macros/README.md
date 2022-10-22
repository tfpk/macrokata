# Hygeine

By default, all identifiers referred to in a macro are expanded as-is, and are
looked up at the macro's invocation site. This can lead to issues if a macro
refers to an item or macro which isn't in scope at the invocation site. To
alleviate this, the `$crate` metavariable can be used at the start of a path to
force lookup to occur inside the crate defining the macro.


## Disclaimer

(based on the disclaimer for the brilliant
[cargo-expand](https://github.com/dtolnay/cargo-expand/))

Be aware that macro expansion to text is a lossy process. That means that the
expanded code we show in these kata should be used as a debugging aid only.
There should be no expectation that the expanded code can be compiled
successfully, nor that if it compiles then it behaves the same as the original
code. In these kata, we try to avoid these issues as far as possible.

For instance the following function returns `3` when compiled ordinarily by Rust
but the expanded code compiles and returns `4`.

```rust
fn f() -> i32 {
    let x = 1;

    macro_rules! first_x {
        () => { x }
    }

    let x = 2;

    x + first_x!()
}
```

Refer to [The Little Book Of Rust Macros](https://veykril.github.io/tlborm/decl-macros/minutiae/hygiene.html)
for more on the considerations around macro hygiene.

## Exercise 12

Exercise 12 consists of a file containing multiple modules. Fix the code so
that the macro works correctly in all invocations.

Note that you will need to use the `$crate` metavariable.
