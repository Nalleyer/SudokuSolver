extern crate lazy_static;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DIGITSET: HashSet<u8> =
        { HashSet::from_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned()) };
}