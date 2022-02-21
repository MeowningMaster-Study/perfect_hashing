use super::globals::P;
use ascii::AsciiChar;
use std::num::Wrapping;
const CHARSET_LEN: Wrapping<usize> = Wrapping(255);

pub fn hash_string(s: &String) -> usize {
    let mut hash: Wrapping<usize> = Wrapping(0);
    let mut pow: Wrapping<usize> = Wrapping(1);
    s.chars().for_each(|ch| {
        let ascii_char = AsciiChar::from_ascii(ch).unwrap();
        hash = (hash + Wrapping::<usize>(ascii_char.as_byte().into()) * pow) % P;
        pow = (pow * CHARSET_LEN) % P;
    });
    return hash.0;
}
