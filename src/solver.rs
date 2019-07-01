use crate::soduko::{Sudoku, Value};

pub struct Solver<'s> {
    sudoku : &'s mut Sudoku,
}