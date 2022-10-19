# MacroKata

MacroKata is a set of exercises which you can use to learn how
to write macros in rust. Unlike most exercises, not only is your goal
to produce the same output with your code as with the files
in solution.rs; but it should *also* be to have your expanded code be the same
as `result.rs`.

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

Build the main binary provided with this repo:

``` sh
$ cargo build
```

You can find the first kata (`my_first_macro`) inside `tasks/01_my_first_macro`.
Read the `README.md` file, and get started by editing the `main.rs` file.

To test your code, run:

``` sh
$ cargo run 01_my_first_macro
```
