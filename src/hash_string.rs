use super::globals::P;
use ascii::AsciiChar;
const CHARSET_LEN: usize = 255;

pub fn hash_string(s: &String) -> usize {
    let mut hash: usize = 0;
    let mut pow: usize = 1;
    s.chars().for_each(|ch| {
        let ascii_char = AsciiChar::from_ascii(ch).unwrap();
        hash = (hash + ascii_char.as_byte() as usize * pow) % P;
        pow = (pow * CHARSET_LEN) % P;
    });
    return hash;
}
