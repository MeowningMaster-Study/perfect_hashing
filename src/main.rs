pub mod globals;
mod hash_string;
mod hash_tables;

use hash_string::hash_string;

fn main() {
    let map = vec![
        ("Lion", "Лев"),
        ("Dog", "Собака"),
        ("Cat", "Кот"),
        ("Manatee", "Ламантин"),
    ];

    let hash_table = hash_tables::PrimaryHashTable::new(&map, hash_string).unwrap();

    let ans = [
        hash_table.get("Dog"),
        hash_table.get("Cat"),
        hash_table.get("Manatee"),
        hash_table.get("Iguana"),
    ];
    println!("{:?}", ans);
}
