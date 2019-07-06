use crate::sudoku::{Sudoku, Value, View};

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solver<'s> {
    sudoku: &'s mut Sudoku,
    sudoku_last: Option<Sudoku>,
}

impl<'s> Solver<'s> {
    pub fn new(sudoku: &'s mut Sudoku) -> Solver<'s> {
        Solver {
            sudoku,
            sudoku_last: None,
        }
    }

    fn is_finished(&self) -> bool {
        self.sudoku.vec.iter().all(|p_value| match &p_value.value {
            Value::Just(_) => true,
            Value::Blank => false,
            Value::Unknown(v) => v.len() == 1,
        })
    }

    fn scan_views(&mut self) {
        self.update_last();
        println!("scan view a");
        for i in 0..9 {
            let fills = self.sudoku.get_area(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
        println!("scan view r");
        self.update_last();
        for i in 0..9 {
            let fills = self.sudoku.get_row(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
        println!("scan view c");
        self.update_last();
        for i in 0..9 {
            let fills = self.sudoku.get_column(i).fill();
            self.sudoku.fill(fills)
        }
        println!("{}", self.sudoku);
        self.unknown_to_just();
    }

    fn update_last(&mut self) {
        self.sudoku_last = Some(self.sudoku.clone());
    }

    fn is_same_to_last(&self) -> bool {
        match &self.sudoku_last {
            None => false,
            Some(sudoku_last) => *self.sudoku == *sudoku_last,
        }
    }

    fn scan_views_unique_area(&mut self) {
        self.update_last();
        for i in 0..9 {
            let fills = self.sudoku.get_area(i).find_unique();
            self.sudoku.fill(fills)
        }
        println!("unique a");
        println!("{}", self.sudoku);
    }

    fn scan_views_unique_row(&mut self) {
        self.update_last();
        for i in 0..9 {
            let fills = self.sudoku.get_row(i).find_unique();
            self.sudoku.fill(fills)
        }
        println!("unique r");
        println!("{}", self.sudoku);
    }

    fn scan_views_unique_column(&mut self) {
        self.update_last();
        for i in 0..9 {
            let fills = self.sudoku.get_column(i).find_unique();
            self.sudoku.fill(fills)
        }
        println!("unique c");
        println!("{}", self.sudoku);
    }

    fn unknown_to_just(&mut self) {
        println!("u t j");
        self.update_last();
        self.sudoku.unknown_to_just();
        println!("{}", self.sudoku);
    }

    fn scan_views_until_same(&mut self) {
        loop {
            self.scan_views();
            if self.is_same_to_last() {
                return;
            }
        }
    }

    pub fn solve(&mut self) {
        // TODO: add backtracer to avoid the max limit
        for _ in 1..10 {
            self.scan_views_until_same();
            self.scan_views_unique_area();
            self.scan_views_until_same();
            self.scan_views_unique_row();
            self.scan_views_until_same();
            self.scan_views_unique_column();

            if self.is_finished() {
                break;
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

    #[test]
    fn same_sudoku() {
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
        let mut sdk2 = Sudoku::new(&s_str).unwrap();
        assert_eq!(true, sdk == sdk2);
    }
}
