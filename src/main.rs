pub mod globals;
mod hash_string;
mod hash_tables;

use hash_string::hash_string;

fn main() {
    let albert = String::from("Albert");
    let inga = String::from("Inga");
    let data = vec![&albert, &inga];
    let sc_ht = hash_tables::secondary::SecondaryHashTable::new(&data, hash_string);
    println!("{:?}", &sc_ht.hashed_data);
}
