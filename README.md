# MacroKata

Welcome to MacroKata, a set of exercises which you can use to learn how to write
macros in Rust. When completing each task, there are three goals:

 - Get your code to compile without warnings or errors.
 - Get your code to "work correctly" (i.e. produce the same output)
 - Importantly, *generate the same code* as what the sample solution does.

You should complete the kata in order, as they increase in
difficulty, and depend on previous kata.

This set of exercises is written for people who have already spent some time
programming in Rust. Before completing this, work through a Rust tutorial
and build some small programs yourself. 

## Getting Started

Clone this repository:

``` sh
$ git clone https://www.github.com/tfpk/macrokata/
```

You will also need to install the Rust "nightly" toolchain, so that we can show
expanded macros:

``` sh
$ rustup toolchain install nightly
```

Next, install `cargo-expand`:

``` sh
$ cargo install cargo-expand
```

Build the main binary provided with this repo:

``` sh
$ cargo build --bin macrokata
```

You can find the first kata (`my_first_macro`) inside `exercises/01_my_first_macro`.
Read the [first chapter of the book](https://tfpk.github.io/macrokata/01_README.html)
and get started by editing the `main.rs` file.

To compare your expanded code to the "goal", use the `test` subcommand:

``` sh
$ cargo run -- test 01_my_first_macro
```

You can run your own code as follows:

``` sh
$ cargo run --bin 01_my_first_macro
```

## How To Learn About Procedural Macros

I was originally planning to expand `macrokata` into discussing procedural
macros as well. As I was researching that, I found dtolnay's superlative [Proc
Macro Workshop](https://github.com/dtolnay/proc-macro-workshop).
[Jon Gjengset's video on proc-macros](https://www.youtube.com/watch?v=geovSK3wMB8)
is also a phenomenal resource (despite its length).

I've put my attempt to write something like that on hold because I think the
above is better in every way. Do file an issue if there's something that we
could do here to complement that workshop though.
