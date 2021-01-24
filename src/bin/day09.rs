//! # --- Day 9: Encoding Error ---
//!
//! With your neighbor happily enjoying their video game, you turn your
//! attention to an open data port on the little screen in the seat in front of
//! you.
//!
//! Though the port is non-standard, you manage to connect it to your computer
//! through the clever use of several paperclips. Upon connection, the port
//! outputs a series of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System
//! (XMAS) which, conveniently for you, is an old cypher with an important
//! weakness.
//!
//! XMAS starts by transmitting a **preamble** of 25 numbers. After that, each
//! number you receive should be the sum of any two of the 25 immediately
//! previous numbers. The two numbers will have different values, and there
//! might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers 1 through 25 in a
//! random order. To be valid, the next number must be the sum of two of those
//! numbers:
//!
//! - `26` would be a **valid** next number, as it could be `1` plus `25` (or
//!   many other pairs, like `2` and `24`).
//! - `49` would be a **valid** next number, as it is the sum of `24` and `25`.
//! - `100` would **not** be valid; no two of the previous `25` numbers sum to
//!   `100`.
//! - `50` would also **not** be valid; although `25` appears in the previous
//!   `25` numbers, the two numbers in the pair must be different.
//!
//! Suppose the 26th number is `45`, and the first number (no longer an option,
//! as it is more than 25 numbers ago) was `20`. Now, for the next number to be
//! valid, there needs to be some pair of numbers among `1-19`, `21-25`, or `45`
//! that add up to it:
//!
//! - `26` would still be a **valid** next number, as `1` and `25` are still
//!   within the previous `25` numbers.
//! - `65` would **not** be valid, as no two of the available numbers sum to it.
//! - `64` and `66` would both be **valid**, as they are the result of `19+45`
//!   and `21+45` respectively.
//!
//! Here is a larger example which only considers the previous `5` numbers (and
//! has a preamble of length 5):
//!
//! ```
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum
//! of two of the previous 5 numbers; the only number that does not follow this
//! rule is **`127`**.
//!
//! The first step of attacking the weakness in the XMAS data is to find the
//! first number in the list (after the preamble) which is not the sum of two of
//! the 25 numbers before it. **What is the first number that does not have this
//! property?**
//!
//! ## --- Part Two ---
//!
//! The final step in breaking the XMAS encryption relies on the invalid number
//! you just found: you must **find a contiguous set of at least two numbers**
//! in your list which sum to the invalid number from step 1.
//!
//! Again consider the above example:
//!
//! ```
//! 35
//! 20
//! **15**
//! **25**
//! **47**
//! **40**
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from `15` through `40` produces
//! the invalid number from step 1, `127`. (Of course, the contiguous set of
//! numbers in your actual list might be much longer.)
//!
//! To find the **encryption weakness**, add together the **smallest** and
//! **largest** number in this contiguous range; in this example, these are `15`
//! and `47`, producing `62`.
//!
//! **What is the encryption weakness in your XMAS-encrypted list of numbers?**
use std::cmp::Ordering;
use std::collections::HashSet;
use std::{env, fs};

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn find_invalid(array: &[u64], preamble: usize) -> Option<u64> {
    if array.len() <= preamble {
        panic!("Length of data list should be greater than preamble size.")
    }
    let mut set: HashSet<u64> = array[0..preamble].iter().copied().collect();

    for (i_shift, &item) in array[preamble..].iter().enumerate() {
        let valid = set.iter().any(|&x| {
            let r = item.saturating_sub(x);
            r != x && set.contains(&r)
        });

        if !valid {
            return Some(item);
        } else {
            set.remove(&array[i_shift]);
            set.insert(item);
        }
    }

    None
}

fn find_encryption_weakness(array: &[u64], target: u64) -> Option<Vec<u64>> {
    let cum_sum: Vec<u64> = array
        .iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();
    let mut i = 0;
    let mut j = 1;
    let (start, end_option) = loop {
        let sub = cum_sum[j] - cum_sum[i];
        match sub.cmp(&target) {
            Ordering::Equal => break (i, Some(j)),
            Ordering::Less => j += 1,
            Ordering::Greater => i += 1,
        }
        if j > cum_sum.len() {
            break (i, None);
        }
    };

    if let Some(end) = end_option {
        let seq = &array[start + 1..=end]; // note: the real sequence is [start + 1, end]
        Some(seq.to_vec())
    } else {
        None
    }
}

fn sum_min_max(array: &[u64]) -> u64 {
    array.iter().min().unwrap() + array.iter().max().unwrap()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}.", filename);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let array = parse_input(&input);
    let first_invalid = find_invalid(&array, 25).unwrap();

    let weakness_seq = find_encryption_weakness(&array, first_invalid).unwrap();
    let weakness = sum_min_max(&weakness_seq);

    println!(
        "First invalid number for XMAS is {} and encryption weakness number is {}",
        first_invalid, weakness
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ARRAY: &[u64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_parse_input() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let array = parse_input(&input);
        assert_eq!(&array, TEST_ARRAY);
    }

    #[test]
    fn test_find_invalid() {
        let first_invalid = find_invalid(TEST_ARRAY, 5);
        assert_eq!(first_invalid, Some(127));
    }

    #[test]
    fn test_encryption_weakness() {
        let weakness_seq = find_encryption_weakness(TEST_ARRAY, 127).unwrap();
        assert_eq!(weakness_seq, vec![15, 25, 47, 40]);

        let weakness = sum_min_max(&weakness_seq);
        assert_eq!(weakness, 62);
    }
}
