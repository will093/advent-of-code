//!
//! Tools for parsing input text into formats commonly used in Advent of Code solutions.
//!

use std::{marker::PhantomData};
use crate::utils::integer::{Signed, Unsigned};

pub trait AocParseExt<'a> {
    fn as_unsigned_iter<T:Unsigned<T>>(&'a self) -> IntParser<'a, T>;
    fn as_signed_iter<T:Signed<T>>(&'a self) -> IntParser<'a, T>;
    fn to_char_grid(&'a self) -> Vec<Vec<char>>;
}

impl <'a>AocParseExt<'a> for &str {
    fn as_unsigned_iter<T>(&'a self) -> IntParser<'a, T> {
        IntParser::new(self, IntParserType::Unsigned)
    }
    fn as_signed_iter<T>(&'a self) -> IntParser<'a, T> {
        IntParser::new(self, IntParserType::Signed)
    }
    fn to_char_grid(&'a self) -> Vec<Vec<char>> {
        self
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}

/// A parser which allows for iterating through all unsigned integers in a &str.
///
/// # Examples
///
/// ```
/// use advent::utils::parse::{AocParseExt, IntParser};
///
/// let mut it: IntParser<u32> = "1,2,a b c,3".as_unsigned_iter();
/// assert_eq!(it.next(), Some(1));
/// assert_eq!(it.next(), Some(2));
/// assert_eq!(it.next(), Some(3));
/// assert_eq!(it.next(), None);
/// ```
/// 
/// ```
/// use advent::utils::parse::{AocParseExt, IntParser};
///
/// let mut it: IntParser<i32> = "1,-2,a b c,3".as_signed_iter();
/// assert_eq!(it.next(), Some(1));
/// assert_eq!(it.next(), Some(-2));
/// assert_eq!(it.next(), Some(3));
/// assert_eq!(it.next(), None);
/// ```
pub struct IntParser<'a, T> {
    parser_type: IntParserType,
    input: std::slice::Iter<'a, u8>,
    _marker: std::marker::PhantomData<T>,
}

#[derive(PartialEq)]
pub enum IntParserType {
    Signed,
    Unsigned,
}


impl<'a, T> IntParser<'a, T> {
    fn new(input: &'a str, parser_type: IntParserType) -> Self {
        Self { input: input.as_bytes().iter(), parser_type, _marker: PhantomData }
    }
}

impl<'a, T> Iterator for IntParser<'a, T>
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

        match self.parser_type {
            IntParserType::Unsigned => {
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
            },
            IntParserType::Signed => {
                let is_valid_signed_int = |digits: &Vec<u8>| {
                    !digits.is_empty() && digits.iter().any(|d| d.is_ascii_digit())
                };
                while let Some(b) = self.input.next() {
                    if b.is_ascii_digit() || (digits.is_empty() && *b == b'-') {
                        digits.push(*b);
                    } else if is_valid_signed_int(&digits) {
                        return Some(parse_digits(&digits));
                    }
                }

                if is_valid_signed_int(&digits) {
                    return Some(parse_digits(&digits));
                }
            }
        }
        None   
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_iter_empty_str() {
        let s = "";
        let vals: Vec<_> = s.as_unsigned_iter::<u32>().collect();
        assert_eq!(vals, vec![]);
    }


    #[test]
    fn unsigned_iter_no_vals() {
        let s = "abc-r";
        let vals: Vec<_> = s.as_unsigned_iter::<u32>().collect();
        assert_eq!(vals, vec![]);
    }

    #[test]
    fn unsigned_iter_single() {
        let s = "1";
        let vals: Vec<_> = s.as_unsigned_iter::<u32>().collect();
        assert_eq!(vals, vec![1]);
    }

    #[test]
    fn unsigned_iter_singl_with_noise() {
        let s = "b1a";
        let vals: Vec<_> = s.as_unsigned_iter::<u32>().collect();
        assert_eq!(vals, vec![1]);
    }


    #[test]
    fn unsigned_iter_multi() {
        let s = "b1a34c-7";
        let vals: Vec<_> = s.as_unsigned_iter::<u32>().collect();
        assert_eq!(vals, vec![1,34,7]);
    }

    #[test]
    fn signed_iter_empty_str() {
        let s = "";
        let vals: Vec<_> = s.as_signed_iter::<i32>().collect();
        assert_eq!(vals, vec![]);
    }

    #[test]
    fn signed_iter_no_vals() {
        let s = "abc-r";
        let vals: Vec<_> = s.as_signed_iter::<i32>().collect();
        assert_eq!(vals, vec![]);
    }

    #[test]
    fn signed_iter_single() {
        let s = "-1";
        let vals: Vec<_> = s.as_signed_iter::<i32>().collect();
        assert_eq!(vals, vec![-1]);
    }

    #[test]
    fn signed_iter_single_with_noise() {
        let s = "b--1a";
        let vals: Vec<_> = s.as_signed_iter::<i32>().collect();
        assert_eq!(vals, vec![-1]);
    }

    #[test]
    fn signed_iter_multi() {
        let s = "-1,-3,-4,g6,h23-";
        let vals: Vec<_> = s.as_signed_iter::<i32>().collect();
        assert_eq!(vals, vec![-1,-3,-4,6,23]);
    }
}