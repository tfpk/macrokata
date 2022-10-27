# MacroKata

Welcome to MacroKata, a set of exercises which you can use to learn how to write
macros in rust. When completing each task, there are three goals:

 - Get your code to compile without warnings or errors.
 - Get your code to "work correctly" (i.e. produce the same output)
 - Importantly, *generate the same code* as what the sample solution does.

You should complete the kata in order, as they increase in
difficulty, and depend on previous kata. 

## Getting Started

Clone this repository:

``` sh
$ git clone https://www.github.com/tfpk/macrokata/
```

You will also need to install the rust "nightly" toolchain, so that we can show
expanded macros.

``` sh
$ rustup toolchain install nightly
```

And install cargo expand.

``` sh
$ cargo install cargo-expand
```

Build the main binary provided with this repo:

``` sh
$ cargo build --bin macrokata
```

You can find the first kata (`my_first_macro`) inside `exercises/01_my_first_macro`.
Read the `README.md` file, and get started by editing the `main.rs` file.

To compare your expanded code to the "goal"; use the `test` subcommand.

``` sh
$ cargo run -- test 01_my_first_macro
```

To run your own code, use:

``` sh
$ cargo run --bin 01_my_first_macro
```

