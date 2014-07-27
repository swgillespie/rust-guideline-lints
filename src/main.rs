#![feature(phase)]

#[phase(plugin)] extern crate rust_guideline_lints;

fn main() {
    let five = box 5i;
    match *five {
        5 | 6 | 7 => println!("It's 5, 6, or 7!"),
        _ => println!("It's not 5, 6, or 7!")
    };
}

