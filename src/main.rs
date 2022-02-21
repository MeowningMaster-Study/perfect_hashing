pub mod globals;
mod hash_string;
mod hash_tables;

use hash_string::hash_string;

fn main() {
    let albert = String::from("Albert");
    let inga = String::from("Inga");
    let data = vec![&albert, &inga];

    let hash_table = hash_tables::PrimaryHashTable::new(&data, hash_string).unwrap();

    let a = hash_table.get(&albert);
    let b = hash_table.get(&inga);
    println!("{:?}, {:?}", a, b);
}
