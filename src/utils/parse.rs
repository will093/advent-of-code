/*
 * Tools for parsing input text into formats commonly used in Advent of Code solutions.
 */

use std::{marker::PhantomData};

pub trait AocParseExt<'a> {
    fn as_unsigned_iter<T>(&'a self) -> IntParser<'a, T>;
    fn as_signed_iter<T>(&'a self) -> IntParser<'a, T>;
}

impl <'a>AocParseExt<'a> for &str {
    fn as_unsigned_iter<T>(&'a self) -> IntParser<'a, T> {
        IntParser::new(self, IntParserType::Unsigned)
    }
    fn as_signed_iter<T>(&'a self) -> IntParser<'a, T> {
        IntParser::new(self, IntParserType::Signed)
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
    parser_type: IntParserType, // TODO: instead write SignedInteger and UnsignedInteger traits??
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

        while let Some(b) = self.input.next() {
            if b.is_ascii_digit() 
            || (digits.is_empty() && self.parser_type == IntParserType::Signed && *b == b'-') {
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