use std::env;

use crate::problems::right_side_view;

pub mod problems;

pub mod utils;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let problem = &args[1];

    // prob use macros here: https://doc.rust-lang.org/book/ch19-06-macros.html
    // also see: https://users.rust-lang.org/t/dynamically-calling-functions/43093 , https://doc.rust-lang.org/std/primitive.fn.html , https://crates.io/crates/phf
    // macros allow metaprogramming, which is basically writing Rust code to generate Rust code before compilation... because, since Rust is compiled, we can't just dynamically call functions from modules
    if problem == "right_side_view" {
        right_side_view::solution(None);
    }
}
