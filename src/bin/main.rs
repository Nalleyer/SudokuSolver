extern crate sudoku_solver;

use sudoku_solver::{Sudoku};
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let s : Sudoku = Sudoku::new(&contents).unwrap();
    println!("{}", s);
}
