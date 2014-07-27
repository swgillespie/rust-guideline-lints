## Rust Guideline lints ##

This repository will contain some lints that try to enforce some of the guidelines documented at http://aturon.github.io/.
These lints aren't super useful, they are more interesting as learning projects instead of providing useful information.

Right now the only lint provided is a lint that warns upon dereferencing within a match expression, as shown here:
http://aturon.github.io/features/match.html. I picked this guideline as a simple target for learning the compiler lint
system. It's still kinda cool, though! 

### Running the example ###
This repository is Cargo enabled:

```
git clone https://github.com/swgillespie/rust-guideline-lints.git
cd rust-guideline-lints
cargo build
```

There's an example in src/main.rs that imports and triggers this lint, which will print upon `cargo build`:. 

```   
   Compiling rust_guideline_lints v0.0.1 (file:/home/sean/Documents/workspace/rust/rust-lints)
/home/sean/Documents/workspace/rust/rust-lints/src/main.rs:7:11: 7:16 warning: Dereferencing in a match expression is discouraged, #[warn(match_dereference)] on by default
/home/sean/Documents/workspace/rust/rust-lints/src/main.rs:7     match *five {
                                                                       ^~~~~
/home/sean/Documents/workspace/rust/rust-lints/src/main.rs:8:9: 8:18 note: Consider using a 'box' pattern here and all other match arms.
/home/sean/Documents/workspace/rust/rust-lints/src/main.rs:8         5 | 6 | 7 => println!("It's 5, 6, or 7!"),
                                                                     ^~~~~~~~~
```
