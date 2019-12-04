use super::{read_file, Error, Part, Result};
use crate::BoolThenExt;
use std::{collections::HashSet, ops::RangeBounds};

mod part_1;
mod part_2;

pub fn main(part: Part) -> Result<()> {
    let input = (347312..=805915);

    match part {
        Part::One => part_1::main(input),
        Part::Two => part_2::main(input),
    }
}

fn validate_length(n: u32, len: u32) -> Option<u32> {
    (digit_length(n) == len).then(n)
}

fn validate_two_identical_adjacent_digits(n: u32) -> Option<u32> {
    let digits = digits(n);

    digits
        .iter()
        .enumerate()
        .any(|(idx, digit)| {
            digits
                .get(idx + 1)
                .map(|next| next == digit)
                .unwrap_or(false)
        })
        .then(n)
}

fn validate_each_digit_increases(n: u32) -> Option<u32> {
    let digits = digits(n);

    digits
        .iter()
        .enumerate()
        .all(|(idx, digit)| {
            digits
                .get(idx + 1)
                .map(|next| next >= digit)
                .unwrap_or(true)
        })
        .then(n)
}

fn digit_length(n: u32) -> u32 {
    digits(n).len() as u32
}

fn digits(n: u32) -> Vec<u32> {
    n.to_string()
        .split("")
        .filter(|digit| digit != &"")
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect()
}

#[test]
fn digits_test() {
    assert_eq!(digits(7), vec![7], "7");
    assert_eq!(digits(10), vec![1, 0], "10");
    assert_eq!(digits(102), vec![1, 0, 2], "102");
}

#[test]
fn validate_two_identical_adjacent_digits_test() {
    assert!(validate_two_identical_adjacent_digits(123).is_none());
    assert!(validate_two_identical_adjacent_digits(1233).is_some());
}

#[test]
fn validate_each_digit_increases_test() {
    assert!(validate_each_digit_increases(123).is_some(), "123");
    assert!(validate_each_digit_increases(1233).is_some(), "1233");
    assert!(validate_each_digit_increases(1232).is_none(), "1232");
}
