use super::globals::P;
use super::options::HashTableOptions;
use super::HashFunction;
use rand::Rng;
use std::vec::Vec;

type T<'a> = &'a String;
type HashedData<T> = Vec<Option<T>>;

fn get_index(options: HashTableOptions, key: usize) -> usize {
    ((options.a * key + options.b) % P) % options.m
}

fn get_item<T>(hashed_data: &HashedData<T>, options: HashTableOptions, key: usize) -> &Option<T> {
    &hashed_data[get_index(options, key)]
}

pub struct SecondaryHashTable<'a> {
    pub options: HashTableOptions,
    pub hashed_data: HashedData<T<'a>>,
    pub hash_func: HashFunction<T<'a>>,
}

fn gen_b() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range((P / 10)..P)
}

impl<'a> SecondaryHashTable<'_> {
    pub fn new(data: &[T<'a>], hash_func: HashFunction<T<'a>>) -> SecondaryHashTable<'a> {
        let mut options = HashTableOptions {
            a: 2,
            b: gen_b(),
            m: data.len().pow(2),
        };
        let mut hashed_data = vec![None; options.m];
        let keys = (0..data.len())
            .map(|i| hash_func(&data[i]))
            .collect::<Vec<_>>();

        loop {
            let mut iter = keys.iter().enumerate();
            for (i, key) in iter.by_ref() {
                let item = get_item(&hashed_data, options, *key);
                // if place is empty
                if item.is_none() {
                    hashed_data[i] = Some(data[i]);
                } else {
                    break;
                }
            }

            // if all keys hashed
            if iter.next() == None {
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

        SecondaryHashTable {
            options,
            hashed_data,
            hash_func,
        }
    }
}