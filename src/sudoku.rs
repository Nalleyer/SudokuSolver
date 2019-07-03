use std::collections::HashSet;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::global::DIGITSET;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Just(u8),
    Blank,
    Unknown(HashSet<u8>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Just(u) => write!(f, "{}", u),
            Value::Blank => write!(f, "_"),
            Value::Unknown(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PValue {
    pub pos: usize,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct View<'s>(Vec<&'s PValue>);

impl<'s> View<'s> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set(&self) -> HashSet<u8> {
        let mut set: HashSet<u8> = HashSet::new();
        for pv in self.0.iter() {
            if let Value::Just(u) = &pv.value {
                set.insert(*u);
            }
        }
        set
    }

    pub fn fill(&self) -> Vec<Fill> {
        let my_set = self.set();
        self.0
            .iter()
            .map(|pv| match &(*pv).value {
                Value::Unknown(set) => {
                    Some(Fill(pv.pos, Value::Unknown(set.difference(&my_set).cloned().collect())))
                }
                Value::Blank => Some(Fill(
                    pv.pos, 
                    Value::Unknown(DIGITSET.difference(&my_set).cloned().collect()),
                )),
                Value::Just(_) => None,
            })
            .filter_map(|x| x)
            .collect()
    }

    pub fn find_unique(&self) -> Vec<Fill> {
        let mut count_map : HashMap<u8, usize> = HashMap::new();
        for pv in self.0.iter() {
            if let Value::Unknown(set) = &pv.value {
                for u in set.iter() {
                    let count = count_map.entry(*u).or_insert(0);
                    *count += 1;
                }
            }
        }

        self.0.iter().filter_map(|pv| {
            if let Value::Unknown(set) = &pv.value {
                for u in set.iter() {
                    let count = count_map.get(u).unwrap();
                    if *count == 1 {
                        return Some(Fill(pv.pos, Value::Just(*u)));
                    }
                }
                None
            } else {
                None
            }
        }).collect()
    }
}

impl<'s> fmt::Display for View<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{},", self.0[i].value)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Fill(usize, Value);

#[derive(Clone, Debug)]
pub struct Sudoku {
    pub vec: Vec<PValue>,
}

fn make_line(line: &str, pos: &mut usize) -> Result<Vec<PValue>, &'static str> {
    let mut vec: Vec<PValue> = vec![];
    for c in line.chars() {
        if c == ' ' {
            continue;
        } else {
            if let Some(digit) = c.to_digit(10) {
                if digit >= 1 && digit <= 9 {
                    vec.push(PValue {
                        pos: *pos,
                        value: Value::Just(digit as u8),
                    });
                } else {
                    return Err("value must be in 1..9");
                }
            } else {
                vec.push(PValue {
                    pos: *pos,
                    value: Value::Blank,
                });
            }
            *pos = *pos + 1;
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
        let mut pos: usize = 0;
        let mut vec: Vec<PValue> = vec![];
        for line in str.lines() {
            if !line.trim().is_empty() {
                vec.append(&mut make_line(line, &mut pos).unwrap());
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

    pub fn fill(&mut self, fills: Vec<Fill>) {
        for Fill(pos, new_value) in fills {
            self.vec[pos].value = new_value;
        }
    }

    pub fn unknown_to_just(&mut self) {
        for pv in &mut self.vec {
            if let Value::Unknown(v) = &mut pv.value {
                if v.len() == 1 {
                    pv.value = Value::Just(*v.iter().collect::<Vec<&u8>>()[0]);
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.vec.iter().all(|pv| {
            if let Value::Just(_) = &pv.value {
                true
            } else {
                false
            }
        })
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", String::from("-").repeat(90 + 12))?;
        for i in 0..9 {
            write!(f, "|")?;
            for j in 0..9 {
                match &self.vec[i * 9 + j].value {
                    Value::Just(u) => {
                        write!(f, "{}{},", u, String::from(" ").repeat(9))?;
                    }
                    Value::Blank => {
                        write!(f, "{},", String::from(" ").repeat(10))?;
                    }
                    Value::Unknown(set) => {
                        write!(f, "☆")?;
                        let mut vec : Vec<u8> = set.iter().cloned().collect::<Vec<u8>>();
                        vec.sort();
                        for v in vec.iter() {
                            write!(f, "{}", v)?;
                        }
                        write!(f, "{},", String::from(" ").repeat(9 - set.len()))?;
                    }
                }
                if (j + 1) % 3 == 0 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
            if (i + 1) % 3 == 0 {
                write!(f, "{}\n", String::from("-").repeat(90 + 12))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fill_function() {
        let mut set : HashSet<u8> = HashSet::new();
        set.insert(4);
        set.insert(6);
        let pv7 = PValue{pos: 7, value: Value::Unknown(set)};
        let view : View = View(vec![
            &PValue{pos: 0, value: Value::Just(5)},
            &PValue{pos: 1, value: Value::Just(2)},
            &PValue{pos: 2, value: Value::Just(3)},
            &PValue{pos: 3, value: Value::Just(8)},
            &PValue{pos: 4, value: Value::Just(1)},
            &PValue{pos: 5, value: Value::Just(7)},
            &PValue{pos: 6, value: Value::Just(9)},
            &pv7,
            &PValue{pos: 8, value: Value::Just(6)}
        ]);

        let mut set : HashSet<u8> = HashSet::new();
        set.insert(4);
        assert_eq!(view.fill(), vec![Fill(7, Value::Unknown(set))]);
    }
}