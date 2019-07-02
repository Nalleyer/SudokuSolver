use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Just(u8),
    Blank,
    Unknown(Vec<u8>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Just(u) => write!(f, "{}", u),
            Value::Blank => write!(f, "_"),
            Value::Unknown(v) => write!(f, "{:?}", v)
        }
    }
}

#[derive(Debug)]
pub struct View<'s>(Vec<&'s Value>);

impl<'s> View<'s> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'s> fmt::Display for View<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{},", self.0[i])?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Sudoku {
    pub vec: Vec<Value>,
}

fn make_line(line: &str) -> Result<Vec<Value>, &'static str> {
    let mut vec: Vec<Value> = vec![];
    for c in line.chars() {
        if c == ' ' {
            continue;
        } else {
            if let Some(digit) = c.to_digit(10) {
                if digit >= 1 && digit <= 9 {
                    vec.push(Value::Just(digit as u8));
                } else {
                    return Err("value must be in 1..9");
                }
            } else {
                vec.push(Value::Blank);
            }
        }
    }
    if vec.len() == 9 {
        Ok(vec)
    } else {
        Err("parsing line failed, expected 9 number per line")
    }
}

impl Sudoku {
    pub fn new(str: &str) -> Result<Sudoku, Box<dyn Error>> {
        let mut vec: Vec<Value> = vec![];
        for line in str.lines() {
            if !line.trim().is_empty() {
                vec.append(&mut make_line(line).unwrap());
            }
        }

        Ok(Sudoku { vec })
    }

    // 0
    // 1
    // 2
    // ...
    // 8
    pub fn get_row(&self, idx: usize) -> View {
        if idx > 8 {
            panic!("invalid idx, getting row")
        } else {
            View(self.vec.iter().skip(idx * 9).take(9).collect())
        }
    }

    // 0 1 2 ... 8
    pub fn get_column(&self, idx: usize) -> View {
        if idx > 8 {
            panic!("invalid idx, getting row")
        } else {
            View(self.vec.iter().skip(idx).step_by(9).collect())
        }
    }

    // 0 1 2
    // 3 4 5
    // 6 7 8
    pub fn get_area(&self, idx: usize) -> View {
        if idx > 8 {
            panic!("invalid idx, getting row")
        } else {
            let big_row = idx / 3;
            let big_column = idx % 3;
            let row_low = big_row * 3;
            let column_low = big_column * 3;
            View(
                self.vec
                    .iter()
                    .enumerate()
                    .filter(move |(i, _)| {
                        let row = i / 9;
                        let column = i - (row * 9);
                        let is_row_ok = row >= row_low && row < row_low + 3;
                        let is_colulmn_ok = column >= column_low && column < column_low + 3;
                        is_row_ok && is_colulmn_ok
                    })
                    .map(|(_, v)| v)
                    .collect(),
            )
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+\n")?;
        for i in 0..9 {
            write!(f, "|")?;
            for j in 0..9 {
                match self.vec[i * 9 + j] {
                    Value::Just(u) => {
                        write!(f, "{}", u)?;
                    }
                    Value::Blank => {
                        write!(f, " ")?;
                    }
                    Value::Unknown(_) => {
                        write!(f, "?")?;
                    }
                }
                if (j + 1) % 3 == 0 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
            if (i + 1) % 3 == 0 {
                write!(f, "+---+---+---+\n")?;
            }
        }
        Ok(())
    }
}
