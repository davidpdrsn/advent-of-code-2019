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

fn validate_length(n: u64, len: u64) -> Option<u64> {
    (digit_length(n) == len).then(n)
}

fn validate_two_identical_adjacent_digits(n: u64) -> Option<u64> {
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

fn validate_two_identical_adjacent_digits_small_group(n: u64) -> Option<u64> {
    break_into_groups(n)
        .iter()
        .any(|group| group.len() == 2)
        .then(n)
}

fn break_into_groups(n: u64) -> Vec<Vec<u64>> {
    let digits = digits(n);
    let mut groups = vec![vec![]];
    let mut prev = None::<u64>;

    for digit in digits {
        match prev {
            None => {
                let group = groups.last_mut().unwrap();
                group.push(digit);
            }
            Some(prev) => {
                if prev == digit {
                    let group = groups.last_mut().unwrap();
                    group.push(digit);
                } else {
                    groups.push(vec![digit]);
                }
            }
        }

        prev = Some(digit);
    }

    groups
}

fn validate_each_digit_increases(n: u64) -> Option<u64> {
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

fn digit_length(n: u64) -> u64 {
    digits(n).len() as u64
}

fn digits(n: u64) -> Vec<u64> {
    n.to_string()
        .split("")
        .filter(|digit| digit != &"")
        .map(|digit| digit.parse::<u64>().unwrap())
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

#[test]
#[ignore]
fn validate_two_identical_adjacent_digits_small_group_test() {
    assert!(
        validate_two_identical_adjacent_digits_small_group(112233).is_some(),
        "112233"
    );

    assert!(
        validate_two_identical_adjacent_digits_small_group(123444).is_none(),
        "123444"
    );

    assert!(
        validate_two_identical_adjacent_digits_small_group(111122).is_some(),
        "111122"
    );

    assert!(
        validate_two_identical_adjacent_digits_small_group(11112212222).is_some(),
        "11112212222"
    );
}

#[test]
fn break_into_groups_test() {
    assert_eq!(
        break_into_groups(11112212222),
        vec![vec![1, 1, 1, 1], vec![2, 2], vec![1], vec![2, 2, 2, 2],],
    );
}
