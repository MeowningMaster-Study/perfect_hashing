pub mod errors;
pub mod options;
pub mod primary;
mod secondary;
use super::globals;
use options::HashTableOptions;
use rand::Rng;

pub use primary::PrimaryHashTable;

pub type Stored<'a> = &'a String;
pub type HashFunction<T> = fn(T) -> usize;

pub fn get_index(options: HashTableOptions, key: usize) -> usize {
    ((options.a * key + options.b) % globals::P) % options.m
}

pub fn gen_b() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range((globals::P / 10)..globals::P)
}
