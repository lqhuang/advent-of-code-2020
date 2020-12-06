use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use itertools::Itertools;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("{}", contents);

    let nums: HashSet<u32> = contents
        .split('\n')
        .filter_map(|x| x.parse().ok())
        .collect();
    // println!("{:?}",nums);
    // two sum
    for x in &nums {
        let rest = 2020 - x;
        if nums.contains(&rest) {
            println!("{} + {} = 2020, prod = {}", x, rest, x * rest);
        }
    }

    // three sum
    let comb = nums.clone().into_iter().combinations(2);
    let sum_map: HashMap<u32, Vec<u32>> = comb.map(|x| (x.iter().sum(), x)).collect();
    // println!("{:?}", sum_map);

    for (x, val) in &sum_map {
        if x < &2020 {
            let rest = 2020 - x;
            if nums.contains(&rest) {
                println!(
                    "{} + {} + {} = 2020, prod = {:?}",
                    rest,
                    val[0],
                    val[1],
                    val.iter().product::<u32>() * rest
                );
            }
        }
    }

    Ok(())
}
