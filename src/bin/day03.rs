//! # --- Day 3: Toboggan Trajectory ---
//!
//! With the toboggan login problems resolved, you set off toward the airport.
//! While travel by toboggan might be easy, it's certainly not safe: there's
//! very minimal steering and the area is covered in trees. You'll need to see
//! which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer
//! coordinates in a grid. You make a map (your puzzle input) of the open
//! squares (`.`) and trees (`#`) you can see. For example:
//!
//! ```
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once
//! involving arboreal genetics and biome stability, the same pattern repeats
//! to the right many times:
//!
//! ```
//! |..##.......|..##.........##.........##.........##.........##.......  --->
//! |#...#...#..|#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! |.#....#..#.|.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! |..#.#...#.#|..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! |.#...##..#.|.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! |..#.##.....|..#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! |.#.#.#....#|.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! |.#........#|.#........#.#........#.#........#.#........#.#........#
//! |#.##...#...|#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! |#...##....#|#...##....##...##....##...##....##...##....##...##....#
//! |.#..#...#.#|.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (`.`) in the top-left corner and need to reach
//! the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper
//! model that prefers rational numbers); start by **counting all the trees** you
//! would encounter for the slope **right 3, down 1**:
//!
//! From your starting position at the top-left, check the position that is
//! right 3 and down 1. Then, check the position that is right 3 and down 1
//! from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with `O` where
//! there was an open square and `X` where there was a tree:
//!
//! ```
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to
//! encounter `7` trees.
//!
//! Starting at the top-left corner of your map and following a slope of right
//! 3 and down 1, **how many trees would you encounter?**
//!
//! ## --- Part Two ---
//!
//! Time to check the rest of the slopes - you need to minimize the probability
//! of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the
//! following slopes, you start at the top-left corner and traverse the map all
//! the way to the bottom:
//!
//! - Right 1, down 1.
//! - Right 3, down 1. (This is the slope you already checked.)
//! - Right 5, down 1.
//! - Right 7, down 1.
//! - Right 1, down 2.
//!
//! In the above example, these slopes would find `2`, `7`, `3`, `4`, and `2` tree(s)
//! respectively; multiplied together, these produce the answer 336.
//!
//! **What do you get if you multiply together the number of trees encountered on
//! each of the listed slopes?**
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Open,
    Tree,
}

type Matrix = Vec<Vec<Space>>;
#[derive(Debug, PartialEq, Eq)]
struct Map(Matrix);

struct Slope {
    right: usize,
    down: usize,
}

impl Map {
    fn space_at(&self, i: usize, j: usize) -> Option<&Space> {
        self.0.get(i).and_then(|row| {
            let length = row.len();
            row.get(j % length)
        })
    }

    fn count_tree_hits(&self, trail: &Slope) -> usize {
        (0..self.0.len())
            .step_by(trail.down)
            .zip((0..).step_by(trail.right))
            .fold(0, |acc, (i, j)| {
                let is_tree = match self.space_at(i, j) {
                    Some(&Space::Tree) => 1,
                    Some(&Space::Open) => 0,
                    None => 0,
                };
                acc + is_tree
            })
    }
}

fn parse_input(input: &str) -> Map {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Space::Tree,
                    '.' => Space::Open,
                    _ => Space::Open,
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Matrix>();

    Map(matrix)
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}.", filename);

    let map_str = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let map = parse_input(&map_str);
    let trail = Slope { right: 3, down: 1 };
    let tree_count = map.count_tree_hits(&trail);
    println!("Hit tree {} times.", tree_count);

    let trajectories = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let hits_prod = trajectories
        .iter()
        .map(|x| map.count_tree_hits(x))
        .product::<usize>();
    println!(
        "Total number of trees encountered by multiple slope is {}.",
        hits_prod
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use Space::*;

    #[test]
    fn test_parser() {
        let input = "..##.......\n#...#...#..";
        let map = parse_input(input);

        assert_eq!(
            map,
            Map(vec![
                vec![Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open],
                vec![Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open],
            ])
        );
    }

    #[test]
    fn test_space_at() {
        let map = Map(vec![
            vec![
                Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open,
            ],
            vec![
                Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open,
            ],
        ]);

        assert_eq!(map.space_at(0, 0), Some(&Open));
        assert_eq!(map.space_at(1, 4), Some(&Tree));
        assert_eq!(map.space_at(0, 10), Some(&Open));
        assert_eq!(map.space_at(0, 100), Some(&Open));
        assert_eq!(map.space_at(0, 102), Some(&Tree));
        assert_eq!(map.space_at(3, 0), None);
    }

    #[test]
    fn test_base_map() {
        let input = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#
";
        let map = parse_input(input);
        let count = map.count_tree_hits(&Slope { right: 3, down: 1 });
        assert_eq!(count, 7);
    }
}
