extern crate sudoku_solver;

use std::env;
use std::fs;
use sudoku_solver::{Solver, Sudoku, Value};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut s: Sudoku = Sudoku::new(&contents).unwrap();
    println!("{}", s);
    println!("{:?}", s);
    println!("{}", s.get_row(8));
    println!("{}", s.get_column(8));
    println!("{}", s.get_area(5));
    let mut solver = Solver::new(&mut s);
    solver.solve();
}
