use super::globals::P;
use std::num::Wrapping;
const CHARSET_LEN: Wrapping<usize> = Wrapping(255);

pub fn hash_string(s: &str) -> usize {
    let mut hash: Wrapping<usize> = Wrapping(0);
    let mut pow: Wrapping<usize> = Wrapping(1);
    for byte in s.as_bytes() {
        hash = (hash + Wrapping(*byte as usize) * pow) % P;
        pow = (pow * CHARSET_LEN) % P;
    }
    return hash.0;
}
