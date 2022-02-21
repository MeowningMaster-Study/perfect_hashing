use super::{
    errors::NonUniqueError, gen_b, get_index, options::HashTableOptions,
    secondary::SecondaryHashTable, HashFunction, Stored,
};
use itertools::Itertools;
use std::vec::Vec;

pub struct PrimaryHashTable<'a> {
    options: HashTableOptions,
    secondaries: Vec<SecondaryHashTable<'a>>,
    hash_func: HashFunction<Stored<'a>>,
}

impl<'a> PrimaryHashTable<'a> {
    pub fn new(
        data: &[Stored<'a>],
        hash_func: HashFunction<Stored<'a>>,
    ) -> Result<PrimaryHashTable<'a>, NonUniqueError> {
        for i in 0..data.len() {
            for k in (i + 1)..data.len() {
                if data[i] == data[k] {
                    return Err(NonUniqueError {});
                }
            }
        }

        let keys: Vec<usize> = data.iter().map(|x| hash_func(x)).collect();

        let options = HashTableOptions {
            a: 7,
            b: gen_b(),
            m: keys.iter().unique().count(),
        };

        let mut buffers: Vec<Vec<&String>> = (0..options.m).map(|_| vec![]).collect();

        for x in data {
            let key = hash_func(x);
            let i = get_index(options, key);
            buffers[i].push(x);
        }

        let secondaries = buffers
            .into_iter()
            .map(|x| SecondaryHashTable::new(&x, hash_func))
            .collect();

        Ok(PrimaryHashTable {
            options,
            secondaries,
            hash_func,
        })
    }

    pub fn get(&self, x: Stored<'a>) -> &Option<Stored<'_>> {
        let key = (self.hash_func)(x);
        let i = get_index(self.options, key);
        self.secondaries[i].get(key)
    }
}
