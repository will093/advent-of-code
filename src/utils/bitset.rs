// Based off of https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/bitset.rs

//! Add `biterator` method that treats an integer as a set, iterating over each element where
//! the respective bit is set. For example `1101` would return 0, 2 and 3.
use crate::utils::integer::*;

pub trait BitOps<T> {
    fn biterator(self) -> Bitset<T>;
}

impl<T: Integer<T>> BitOps<T> for T {
    fn biterator(self) -> Bitset<T> {
        Bitset { t: self }
    }
}

pub struct Bitset<T> {
    t: T,
}

impl<T: Integer<T>> Iterator for Bitset<T> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.t == T::ZERO {
            None
        } else {
            let tz = self.t.trailing_zeros();
            self.t = self.t ^ (T::ONE << tz);
            Some(tz as usize)
        }
    }
}
