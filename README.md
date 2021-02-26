# Rust_Book
My notes for going through the Rust_Book

# Chapter 1: Guessing Game
- The std::io can be extended like C#'s using statement.
- You can either call stdin using io::stdin or by saying `use std::io::stdin` and calling it stdin in your code.
- The .expect outputs to the console the string you put it in it.
- To add a dependency just add a new package to the `[dependencies]`
- An example would be like this:
```rust
[dependencies]
rand = "0.5.5"
```
- `0.5.5 is actually shorthand for ^0.5.5`