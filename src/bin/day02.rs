//! --- Day 2: Password Philosophy ---
//!
//! Your flight departs in a few days from the coastal airport; the
//! easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having
//! a bad day. "Something's wrong with our computers; we can't log in!"
//! You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of
//! the passwords wouldn't have been allowed by the Official Toboggan
//! Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle
//! input) of **passwords** (according to the corrupted database) and
//! **the corporate policy when that password was set**.
//!
//! For example, suppose you have the following list:
//!
//! ```
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The
//! password policy indicates the lowest and highest number of times
//! a given letter must appear for the password to be valid. For example,
//! `1-3 a` means that the password must contain `a` at least `1` time
//! and at most `3` times.
//!
//! In the above example, `2` passwords are valid. The middle password,
//! `cdefg`, is not; it contains no instances of `b`, but needs at least
//! `1`. The first and third passwords are valid: they contain one `a` or
//! nine `c`, both within the limits of their respective policies.
//!
//! **How many passwords are valid** according to their policies?
//!
//! --- Part Two ---
//!
//! While it appears you validated the passwords correctly, they don't
//! seem to be what the Official Toboggan Corporate Authentication System
//! is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained
//! the password policy rules from his old job at the sled rental place
//! down the street! The Official Toboggan Corporate Policy actually works
//! a little differently.
//!
//! Each policy actually describes two **positions in the password**,
//! where `1` means the first character, 2 means the second character,
//! and so on. (Be careful; Toboggan Corporate Policies have no concept
//! of "index zero"!) **Exactly one of these positions** must contain
//! the given letter. Other occurrences of the letter are irrelevant for
//! the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! - `1-3 a: abcde` is **valid**: position `1` contains `a` and position `3` does not.
//! - `1-3 b: cdefg` is **invalid**: neither position `1` nor position `3` contains b.
//! - `2-9 c: ccccccccc` is **invalid**: both position `2` and position `9` contain `c`.
//!
//! **How many passwords are valid** according to the new interpretation
//! of the policies?
use std::env;
use std::fs;

trait PolicyVerifier {
    fn verify(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
struct OccurrsPolicy {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PolicyVerifier for OccurrsPolicy {
    fn verify(&self) -> bool {
        let occurrences = self.password.matches(self.letter).count();

        occurrences >= self.min && occurrences <= self.max
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PosMatchPolicy {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

impl PolicyVerifier for PosMatchPolicy {
    fn verify(&self) -> bool {
        let first = self.password.chars().nth(self.first - 1).unwrap();
        let second = self.password.chars().nth(self.second - 1).unwrap();

        (first == self.letter) ^ (second == self.letter)
    }
}

fn verified_by_occurrs_policy(contents: String) -> usize {
    let database: Vec<OccurrsPolicy> = contents
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let (range_str, letter_str, password) = (v[0], v[1], v[2]);

            let min_max: Vec<usize> = range_str.split('-').map(|x| x.parse().unwrap()).collect();

            OccurrsPolicy {
                min: min_max[0],
                max: min_max[1],
                letter: letter_str.chars().next().unwrap(),
                password: password.to_owned(),
            }
        })
        .collect();

    // println!("{:?}", database);

    database.into_iter().filter(OccurrsPolicy::verify).count()
}

fn verified_by_posmatch_policy(contents: String) -> usize {
    let database: Vec<PosMatchPolicy> = contents
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            let (range_str, letter_str, password) = (v[0], v[1], v[2]);

            let pos: Vec<usize> = range_str.split('-').map(|x| x.parse().unwrap()).collect();

            PosMatchPolicy {
                first: pos[0],
                second: pos[1],
                letter: letter_str.chars().next().unwrap(),
                password: password.to_owned(),
            }
        })
        .collect();

    // println!("{:?}", database);

    database.into_iter().filter(PosMatchPolicy::verify).count()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("{:?}", contents);

    let verified_count = verified_by_occurrs_policy(contents.clone());
    println!("Valid password under OccursPolicy: {}", verified_count);

    let second_verified = verified_by_posmatch_policy(contents);
    println!("Valid password under PosMatchPolicy: {}", second_verified);

    Ok(())
}
