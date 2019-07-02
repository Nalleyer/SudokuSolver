use crate::soduko::{Sudoku, Value};

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solver<'s> {
    sudoku: &'s mut Sudoku,
}

impl<'s> Solver<'s> {
    pub fn new(sudoku: &'s mut Sudoku) -> Solver<'s> {
        Solver { sudoku }
    }

    pub fn is_finished(&self) -> bool {
        self.sudoku.vec.iter().all(|value| match value {
            Value::Just(_) => true,
            Value::Blank => false,
            Value::Unknown(v) => v.len() == 1,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_unknown_is_finished() {
        let s_str = r#"
            1 2 3 4 5 6 7 8 9
            4 5 6 7 8 9 1 2 3
            7 8 9 1 2 3 4 5 6
            2 3 4 5 6 7 8 9 1
            5 6 7 8 9 1 2 3 4
            8 9 1 2 3 4 5 6 7
            3 4 5 6 7 8 9 1 2
            6 7 8 9 1 2 3 4 5
            9 1 2 3 4 5 6 7 8
        "#;

        let mut sdk = Sudoku::new(&s_str).unwrap();
        assert_eq!(true, Solver::new(&mut sdk).is_finished())
    }
}
