/*
 * Tools for parsing input text into formats commonly used in Advent of Code solutions.
 */

use std::{marker::PhantomData};

pub trait AocParseExt<'a> {
    fn as_unsigned_iter<T>(&'a self) -> UnsignedIntParser<'a, T>;
}

impl <'a>AocParseExt<'a> for &str {
    fn as_unsigned_iter<T>(&'a self) -> UnsignedIntParser<'a, T> {
        UnsignedIntParser::new(self)
    }
}

/*
 * A parser which allows for iterating through all unsigned integers in a &str.
 * 
 * let it = "1,2,a b c,3".as_unsigned_iter();
 * it.next() // Some(1);
 * it.next() // Some(2);
 * it.next() // Some(3);
 * it.next() // None;
 * 
 */
pub struct UnsignedIntParser<'a, T> {
    input: std::slice::Iter<'a, u8>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> UnsignedIntParser<'a, T> {
    fn new(input: &'a str) -> Self {
        Self { input: input.as_bytes().iter(), _marker: PhantomData }
    }
}

impl<'a, T> Iterator for UnsignedIntParser<'a, T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut digits: Vec<u8> = vec![];

        let parse_digits = |digits: &[u8]| {
            let integer = String::from_utf8_lossy(digits);
            integer.parse::<T>().expect(&format!("{} should be a valid integer", integer))
        };

        while let Some(b) = self.input.next() {
            if b.is_ascii_digit() {
                digits.push(*b);
            } else if !digits.is_empty() {
                return Some(parse_digits(&digits));
            }
        }
        if !digits.is_empty() {
            return Some(parse_digits(&digits));
        }
        None   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_int_parser_parses() {
        let mut iter: UnsignedIntParser<u32> = "123,456,abc 789".as_unsigned_iter();

        assert_eq!(iter.next(), Some(123));
        assert_eq!(iter.next(), Some(456));
        assert_eq!(iter.next(), Some(789));
        assert_eq!(iter.next(), None);
    }
}