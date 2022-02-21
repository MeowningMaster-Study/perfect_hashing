pub mod options;
mod primary;
pub mod secondary;
use super::globals;

pub type HashFunction<T> = fn(T) -> usize;
