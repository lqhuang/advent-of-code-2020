//! # --- Day 6: Custom Customs ---
//!
//! As your flight approaches the regional airport where you'll switch to a
//! much larger plane, customs declaration forms are distributed to the
//! passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked `a` through `z`. All
//! you need to do is identify the questions for which **anyone in your group**
//! answers "yes". Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a
//! language barrier and asks if you can help. For each of the people in their
//! group, you write down the questions for which they answer "yes", one per
//! line. For example:
//!
//! ```
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are `6` questions to which anyone answered "yes":
//! `a`, `b`, `c`, `x`, `y`, and `z`. (Duplicate answers to the same question
//! don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've
//! collected answers from every group on the plane (your puzzle input). Each
//! group's answers are separated by a blank line, and within each group, each
//! person's answers are on a single line. For example:
//!
//! ```
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - The first group contains one person who answered "yes" to `3` questions:
//!   `a`, `b`, and `c`.
//! - The second group contains three people; combined, they answered "yes" to
//!   `3` questions: `a`, `b`, and `c`.
//! - The third group contains two people; combined, they answered "yes" to
//!   `3` questions: `a`, `b`, and `c`.
//! - The fourth group contains four people; combined, they answered "yes" to
//!   only `1` question, `a`.
//! - The last group contains one person who answered "yes" to only `1`
//!   question, `b`.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1 = 11`.
//!
//! For each group, count the number of questions to which anyone answered
//! "yes". **What is the sum of those counts?**
//!
//! ## --- Part Two ---
//!
//! As you finish the last group's customs declaration, you notice that you
//! misread one word in the instructions:
//!
//! You don't need to identify the questions to which **anyone** answered
//! "yes"; you need to identify the questions to which **everyone** answered
//! "yes"!
//!
//! Using the same example as above:
//!
//! ```
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - In the first group, everyone (all 1 person) answered "yes" to `3`
//!   questions: `a`, `b`, and `c`.
//! - In the second group, there is **no** question to which everyone answered
//!   "yes".
//! - In the third group, everyone answered yes to only `1` question, `a`.
//!   Since some people did not answer "yes" to `b` or `c`, they don't count.
//! - In the fourth group, everyone answered yes to only `1` question, `a`.
//! - In the fifth group, everyone (all 1 person) answered "yes" to `1`
//!   question, `b`.
//!
//! In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1 = 6`.
//!
//! For each group, count the number of questions to which **everyone** answered
//! "yes". **What is the sum of those counts?**
use std::collections::HashSet;
use std::{env, fs};

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn count_sum<F>(answers: &[Vec<&str>], init: HashSet<char>, func: F) -> usize
where
    F: FnMut(HashSet<char>, &str) -> HashSet<char> + Copy,
{
    answers
        .iter()
        .map(|group| group.iter().copied().fold(init.clone(), func).len())
        .sum::<usize>()
}

fn inter_func(acc: HashSet<char>, x: &str) -> HashSet<char> {
    let hashset = (*x).chars().collect::<HashSet<char>>();
    acc.intersection(&hashset)
        .copied()
        .collect::<HashSet<char>>()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}.", filename);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let answers = parse_input(&input);

    let union_func = |acc: HashSet<char>, x: &str| {
        let hashset = (*x).chars().collect::<HashSet<char>>();
        acc.into_iter().chain(hashset).collect::<HashSet<char>>()
    };

    let first_count = count_sum(&answers, HashSet::new(), union_func);
    println!(
        "The sum of those counts by first instruction is {}",
        first_count
    );

    let second_count = count_sum(
        &answers,
        "abcdefghijklmnopqrstuvwxyz".chars().collect(),
        inter_func,
    );
    println!(
        "The sum of those counts by second instruction is {}",
        second_count
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_union() {
        let answers = parse_input(INPUT);
        let union_func = |acc: HashSet<char>, x: &str| {
            let hashset = (*x).chars().collect::<HashSet<char>>();
            acc.into_iter().chain(hashset).collect::<HashSet<char>>()
        };
        let count = count_sum(&answers, HashSet::new(), union_func);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_intersection() {
        let answers = parse_input(INPUT);

        let count = count_sum(
            &answers,
            "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            inter_func,
        );
        assert_eq!(count, 6);
    }
}
