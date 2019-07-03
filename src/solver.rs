use crate::sudoku::{Sudoku, Value, View};

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solver<'s> {
    sudoku: &'s mut Sudoku,
}

impl<'s> Solver<'s> {
    pub fn new(sudoku: &'s mut Sudoku) -> Solver<'s> {
        Solver { sudoku }
    }

    fn is_finished(&self) -> bool {
        self.sudoku.vec.iter().all(|p_value| match &p_value.value {
            Value::Just(_) => true,
            Value::Blank => false,
            Value::Unknown(v) => v.len() == 1,
        })
    }

    fn scan_views(&mut self) {
        println!("scan view a");
        for i in 0..9 {
            let fills = self.sudoku.get_area(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
        println!("scan view r");
        for i in 0..9 {
            let fills = self.sudoku.get_row(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
        println!("scan view c");
        for i in 0..9 {
            let fills = self.sudoku.get_column(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
    }

    fn scan_views_unique(&mut self) {
        self.scan_views();
        println!("scan view unique a");
        for i in 0..9 {
            let fills = self.sudoku.get_area(i).find_unique();
            self.sudoku.fill(fills)
        }
        self.scan_views();
        println!("{}", self.sudoku);
        println!("scan view unique r");
        for i in 0..9 {
            let fills = self.sudoku.get_row(i).find_unique();
            self.sudoku.fill(fills)
        }
        self.scan_views();
        println!("{}", self.sudoku);
        println!("scan view unique c");
        for i in 0..9 {
            let fills = self.sudoku.get_column(i).find_unique();
            self.sudoku.fill(fills)
        }
        self.scan_views();
        println!("{}", self.sudoku);
    }

    fn unknown_to_just(&mut self) {
        println!("u t j");
        self.sudoku.unknown_to_just();
        println!("{}", self.sudoku);
    }

    pub fn solve(&mut self) {
        for _ in 1..40 {
            self.scan_views();
            self.scan_views_unique();
            if self.sudoku.is_solved() {
                break
            }
        }
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
