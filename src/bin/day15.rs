//! # --- Day 15: Rambunctious Recitation ---
//!
//! You catch the airport shuttle and try to book a new flight to your
//! vacation island. Due to the storm, all direct flights have been cancelled,
//! but a route is available to get around the storm. You take it.
//!
//! While you wait for your flight, you decide to check in with the Elves back
//! at the North Pole. They're playing a **memory game** and are ever so
//! excited to explain the rules!
//!
//! In this game, the players take turns saying **numbers**. They begin by
//! taking turns reading from a list of **starting numbers** (your puzzle
//! input). Then, each turn consists of considering the **most recently spoken number**:
//!
//! - If that was the **first** time the number has been spoken, the current
//!   player says `0`.
//! - Otherwise, the number had been spoken before; the current player
//!   announces **how many turns apart** the number is from when it was
//!   previously spoken.
//!
//! So, after the starting numbers, each turn results in that player speaking
//! aloud either `0` (if the last number is new) or an **age** (if the last
//! number is a repeat).
//!
//! For example, suppose the starting numbers are `0,3,6`:
//!
//! - **Turn 1**: The 1st number spoken is a starting number, `0`.
//! - **Turn 2**: The 2nd number spoken is a starting number, `3`.
//! - **Turn 3**: The 3rd number spoken is a starting number, `6`.
//! - **Turn 4**: Now, consider the last number spoken, `6`. Since that was
//!   the first time the number had been spoken, the 4th number spoken is `0`.
//! - **Turn 5**: Next, again consider the last number spoken, `0`. Since it
//!   had been spoken before, the next number to speak is the difference
//!   between the turn number when it was last spoken (the previous turn, `4`)
//!   and the turn number of the time it was most recently spoken before then
//!   (turn `1`). Thus, the 5th number spoken is `4 - 1`, `3`.
//! - **Turn 6**: The last number spoken, `3` had also been spoken before,
//!   most recently on turns `5` and `2`. So, the 6th number spoken is `5 - 2`, `3`.
//! - **Turn 7**: Since `3` was just spoken twice in a row, and the last two
//!   turns are `1` turn apart, the 7th number spoken is `1`.
//! - **Turn 8**: Since `1` is new, the 8th number spoken is `0`.
//! - **Turn 9**: `0` was last spoken on turns `8` and `4`, so the 9th number
//!   spoken is the difference between them, `4`.
//! - **Turn 10**: `4` is new, so the 10th number spoken is `0`.
//!
//! (The game ends when the Elves get sick of playing or dinner is ready,
//! whichever comes first.)
//!
//! Their question for you is: what will be the **2020th** number spoken? In
//! the example above, the 2020th number spoken will be `436`.
//!
//! Here are a few more examples:
//!
//! - Given the starting numbers `1,3,2`, the 2020th number spoken is `1`.
//! - Given the starting numbers `2,1,3`, the 2020th number spoken is `10`.
//! - Given the starting numbers `1,2,3`, the 2020th number spoken is `27`.
//! - Given the starting numbers `2,3,1`, the 2020th number spoken is `78`.
//! - Given the starting numbers `3,2,1`, the 2020th number spoken is `438`.
//! - Given the starting numbers `3,1,2`, the 2020th number spoken is `1836`.
//!
//! Given your starting numbers, **what will be the 2020th number spoken?**
//!
//! ## --- Part Two ---
//!
//! Impressed, the Elves issue you a challenge: determine the `30000000th`
//! number spoken. For example, given the same starting numbers as above:
//!
//! - Given `0,3,6`, the `30000000th` number spoken is `175594`.
//! - Given `1,3,2`, the `30000000th` number spoken is `2578`.
//! - Given `2,1,3`, the `30000000th` number spoken is `3544142`.
//! - Given `1,2,3`, the `30000000th` number spoken is `261214`.
//! - Given `2,3,1`, the `30000000th` number spoken is `6895259`.
//! - Given `3,2,1`, the `30000000th` number spoken is `18`.
//! - Given `3,1,2`, the `30000000th` number spoken is `362`.
//!
//! Given your starting numbers, **what will be the 30000000th number spoken?**
use std::env;
use std::fs;

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect()
}

fn mem_game_solver(starting_nums: &[usize], nth: usize) -> usize {
    let start_len = starting_nums.len();
    let idx = if nth < 1 {
        panic!("Nth should be greater and equal to 1.")
    } else {
        nth - 1
    };

    if idx < start_len {
        starting_nums[nth]
    } else {
        let mut dict = starting_nums
            .iter()
            .enumerate()
            .map(|(i, val)| (*val, i))
            .take(start_len - 1)
            .collect::<HashMap<usize, usize>>();
        let mut prev = *starting_nums.last().unwrap();
        let mut curr: usize = 0;

        for i in start_len..nth {
            if dict.contains_key(&prev) {
                curr = i - 1 - *dict.get(&prev).unwrap();
            } else {
                curr = 0;
            }
            dict.insert(prev, i - 1);
            prev = curr;
            // println!("{} val: {}", i, curr);
        }

        curr
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}.", filename);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let starting_nums = parse_input(&input);

    let num1 = mem_game_solver(&starting_nums, 2020);
    println!("The 2020th number spoken is {}", num1);

    let num2 = mem_game_solver(&starting_nums, 30000000);
    println!("The 30000000th number spoken is {}", num2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_game() {
        let nth = 2020;

        let starting_nums = [0, 3, 6];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 436);

        let starting_nums = [1, 2, 3];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 27);

        let starting_nums = [3, 1, 2];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 1836);
    }

    #[test]
    fn test_mem_game_large_nth() {
        let nth = 30000000;

        let starting_nums = [0, 3, 6];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 175594);

        let starting_nums = [1, 2, 3];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 261214);

        let starting_nums = [3, 1, 2];
        let spoken_num = mem_game_solver(&starting_nums, nth);
        assert_eq!(spoken_num, 362);
    }

    // #[bench]
    // fn bench_mem_game(b: &mut Bencher) {}
}
