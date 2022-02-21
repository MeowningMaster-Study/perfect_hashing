pub mod errors;
pub mod options;
pub mod primary;
mod secondary;
use std::num::Wrapping;

use super::globals;
use options::HashTableOptions;
use rand::Rng;

pub use primary::PrimaryHashTable;

pub fn get_index(options: HashTableOptions, hash: usize) -> usize {
    let ans = (Wrapping(options.a) * Wrapping(hash) + Wrapping(options.b))
        % globals::P
        % Wrapping(options.m);
    ans.0
}

pub fn gen_b() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range((globals::P / Wrapping(10)).0..globals::P.0)
}
