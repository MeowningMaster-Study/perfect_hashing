use super::{gen_b, get_index, options::HashTableOptions};
use std::vec::Vec;

type HashedData<'a, Value> = Vec<Option<(usize, &'a Value)>>;

pub struct SecondaryHashTable<'a, Value> {
    pub options: HashTableOptions,
    pub hashed_data: HashedData<'a, Value>,
}

impl<'a, Value> SecondaryHashTable<'a, Value> {
    pub fn new(map: &[(usize, &'a Value)]) -> SecondaryHashTable<'a, Value> {
        let len = map.len();
        let mut options = HashTableOptions {
            a: 2,
            b: gen_b(),
            m: len.pow(2),
        };
        let mut hashed_data: Vec<Option<(usize, &Value)>> = vec![None; options.m];

        let mut iter = map.iter().enumerate();

        loop {
            for (i, pair) in iter.by_ref() {
                let key = pair.0;
                let index = get_index(options, key);
                // if place is empty
                if hashed_data[index].is_none() {
                    hashed_data[index] = Some(map[i]);
                } else {
                    break;
                }
            }

            // if all keys hashed
            if iter.next().is_none() {
                break;
            } else {
                // try another options
                options.a += 1;
                if options.a > 9 {
                    options.a = 2;
                    options.b = gen_b();
                }
                hashed_data = vec![None; options.m];
            }
        }

        println!("secondary hash table options: {:?}", &options);

        SecondaryHashTable {
            options,
            hashed_data,
        }
    }

    pub fn get(&self, key: usize) -> Option<&Value> {
        if self.options.m == 0 {
            return None;
        }
        let index = get_index(self.options, key);
        let pair = self.hashed_data[index];
        if pair.is_none() || pair.unwrap().0 != key {
            None
        } else {
            Some(pair.unwrap().1)
        }
    }
}
