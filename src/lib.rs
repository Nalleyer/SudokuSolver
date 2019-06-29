use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Just(u8),
    Nothing,
    Unkown(Vec<u8>)
}

#[derive(Clone, Debug)]
pub struct Sudoku {
    vec: Vec<Value>
}


fn make_line(line : &str) -> Result<Vec<Value>, &'static str> {
    let mut vec : Vec<Value> = vec!();
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
                vec.push(Value::Nothing);
            }
        }
    }
    if vec.len() == 9 {
        Ok(vec)
    }
    else {
        Err("parsing line failed")
    }
}

impl Sudoku {
    pub fn new(str: &str) -> Result<Sudoku, Box<dyn Error>> {
        let mut vec : Vec<Value> = vec!();
        for line in str.lines() {
            if !line.is_empty() {
                vec.append(&mut make_line(line).unwrap());
            }
        }

        Ok(Sudoku {vec})
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+\n")?;
        for i in 0..9 {
            write!(f, "|")?;
            for j in 0..9 {
                match self.vec[i * 9 + j] {
                    Value::Just(u) => { write!(f, "{}", u)?; },
                    Value::Nothing => { write!(f, " ")?; },
                    Value::Unkown(_) => { write!(f, "?")?; }
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