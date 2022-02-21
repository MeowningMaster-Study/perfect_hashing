use super::{
    errors::NonUniqueError, gen_b, get_index, options::HashTableOptions,
    secondary::SecondaryHashTable,
};
use itertools::Itertools;
use std::vec::Vec;

type Key<'a> = &'a str;
pub type HashFunction<'a, T> = fn(T) -> usize;

pub struct PrimaryHashTable<'a, Value> {
    options: HashTableOptions,
    secondaries: Vec<SecondaryHashTable<'a, Value>>,
    hash_func: HashFunction<'a, Key<'a>>,
}

impl<'a, Value> PrimaryHashTable<'a, Value> {
    pub fn new(
        map: &'a [(Key<'a>, Value)],
        hash_func: HashFunction<'a, Key<'a>>,
    ) -> Result<PrimaryHashTable<'a, Value>, NonUniqueError> {
        let len = map.len();
        for i in 0..len {
            for k in (i + 1)..len {
                if map[i].0 == map[k].0 {
                    return Err(NonUniqueError {});
                }
            }
        }

        let hashes: Vec<usize> = map.iter().map(|x| hash_func(&x.0)).collect();
        println!("hashes: {:?}", &hashes);

        let options = HashTableOptions {
            a: 7,
            b: gen_b(),
            m: hashes.iter().unique().count(),
        };

        println!("primary hash table options: {:?}", &options);

        let mut buffers: Vec<Vec<(usize, &Value)>> = (0..options.m).map(|_| vec![]).collect();

        for (i, pair) in map.iter().enumerate() {
            let hash = hashes[i];
            let index = get_index(options, hash);
            buffers[index].push((hash, &pair.1));
        }

        let secondaries = buffers
            .into_iter()
            .map(|pair| SecondaryHashTable::new(&pair))
            .collect();

        Ok(PrimaryHashTable {
            options,
            secondaries,
            hash_func,
        })
    }

    pub fn get(&self, key: Key<'a>) -> Option<&Value> {
        let hash = (self.hash_func)(&key);
        let index = get_index(self.options, hash);
        self.secondaries[index].get(hash)
    }
}
