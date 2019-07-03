extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref DIGITSET: HashSet<u8> = {
        let vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        vec.iter().cloned().collect()
    };
}
