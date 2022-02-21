pub mod errors;
pub mod options;
pub mod primary;
mod secondary;
use std::num::Wrapping;

use super::globals;
use options::HashTableOptions;
use rand::Rng;

pub use primary::PrimaryHashTable;

pub type Stored<'a> = &'a String;
pub type HashFunction<T> = fn(T) -> usize;

pub fn get_index(options: HashTableOptions, key: usize) -> usize {
    ((Wrapping(options.a) * Wrapping(key) + Wrapping(options.b)) % globals::P % Wrapping(options.m))
        .0
}

pub fn gen_b() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range((globals::P / Wrapping(10)).0..globals::P.0)
}
