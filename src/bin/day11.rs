//! # --- Day 11: Seating System ---
//!
//! Your plane lands with plenty of time to spare. The final leg of your journey
//! is a ferry that goes directly to the tropical island where you can finally
//! start your vacation. As you reach the waiting area to board the ferry, you
//! realize you're so early, nobody else has even arrived yet!
//!
//! By modeling the process people use to choose (or abandon) their seat in the
//! waiting area, you're pretty sure you can predict the best place to sit. You
//! make a quick map of the seat layout (your puzzle input).
//!
//! The seat layout fits neatly on a grid. Each position is either floor (`.`), an
//! empty seat (`L`), or an occupied seat (`#`). For example, the initial seat
//! layout might look like this:
//!
//! ```
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! Now, you just need to model the people who will be arriving shortly.
//! Fortunately, people are entirely predictable and always follow a simple set
//! of rules. All decisions are based on the **number of occupied seats** adjacent
//! to a given seat (one of the eight positions immediately up, down, left,
//! right, or diagonal from the seat). The following rules are applied to every
//! seat simultaneously:
//!
//! - If a seat is **empty** (`L`) and there are **no** occupied seats adjacent
//!   to it, the seat becomes **occupied**.
//! - If a seat is **occupied** (`#`) and **four or more** seats adjacent to it
//!   are also occupied, the seat becomes **empty**.
//! - Otherwise, the seat's state does not change.
//!
//! Floor (`.`) never changes; seats don't move, and nobody sits on the floor.
//!
//! After one round of these rules, every seat in the example layout becomes
//! occupied:
//!
//! ```
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! After a second round, the seats with four or more occupied adjacent seats
//! become empty again:
//!
//! ```
//! #LLLLLL.L#
//! L.L.L..L..
//! #LLL.LL.L#
//! #.LL.LL.LL
//! #.LLLL#.##
//! ..L.L.....
//! #LLLLLLLL#
//! #.LLLLLL.L
//! #.#LLLL.##
//! ```
//!
//! This process continues for three more rounds:
//!
//! ```
//! #.##.L#.##
//! #L###LL.L#
//! L.#.#..#..
//! #L##.##.L#
//! #.##.LL.LL
//! #.###L#.##
//! ..#.#.....
//! #L######L#
//! #.LL###L.L
//! #.#L###.##
//! ```
//!
//! ```
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.L.L..#..
//! #LLL.##.L#
//! #.LL.LL.LL
//! #.LL#L#.##
//! ..L.L.....
//! #L#LLLL#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! ```
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.#.L..#..
//! #L##.##.L#
//! #.#L.LL.LL
//! #.#L#L#.##
//! ..L.L.....
//! #L#L##L#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! At this point, something interesting happens: the chaos stabilizes and
//! further applications of these rules cause no seats to change state! Once
//! people stop moving around, you count `37` occupied seats.
//!
//! Simulate your seating area by applying the seating rules repeatedly until no
//! seats change state. **How many seats end up occupied?**
//!
//! ## --- Part Two ---
//!
//! As soon as people start to arrive, you realize your mistake. People don't
//! just care about adjacent seats - they care about **the first seat they can see**
//! in each of those eight directions!
//!
//! Now, instead of considering just the eight immediately adjacent seats,
//! consider the **first seat** in each of those eight directions. For
//! example, the empty seat below would see **eight** occupied seats:
//!
//! ```
//! .......#.
//! ...#.....
//! .#.......
//! .........
//! ..#L....#
//! ....#....
//! .........
//! #........
//! ...#.....
//! ```
//!
//! The leftmost empty seat below would only see **one** empty seat, but cannot
//! see any of the occupied ones:
//!
//! ```
//! .............
//! .L.L.#.#.#.#.
//! .............
//! ```
//!
//! The empty seat below would see **no** occupied seats:
//!
//! ```
//! .##.##.
//! #.#.#.#
//! ##...##
//! ...L...
//! ##...##
//! #.#.#.#
//! .##.##.
//! ```
//!
//! Also, people seem to be more tolerant than you expected: it now takes
//! **five or more** visible occupied seats for an occupied seat to become
//! empty (rather than **four or more** from the previous rules). The other
//! rules still apply: empty seats that see no occupied seats become occupied,
//! seats matching no rule don't change, and floor never changes.
//!
//! Given the same starting layout as above, these new rules cause the seating
//! area to shift around as follows:
//!
//! ```
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! ```
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! ```
//! #.LL.LL.L#
//! #LLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLLL.L
//! #.LLLLL.L#
//! ```
//!
//! ```
//! #.L#.##.L#
//! #L#####.LL
//! L.#.#..#..
//! ##L#.##.##
//! #.##.#L.##
//! #.#####.#L
//! ..#.#.....
//! LLL####LL#
//! #.L#####.L
//! #.L####.L#
//! ```
//!
//! ```
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##LL.LL.L#
//! L.LL.LL.L#
//! #.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.#L.L#
//! #.L####.LL
//! ..#.#.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.LL.L#
//! #.LLLL#.LL
//! ..#.L.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! Again, at this point, people stop shifting around and the seating area
//! reaches equilibrium. Once this occurs, you count 26 occupied seats.
//!
//! Given the new visibility method and the rule change for occupied seats
//! becoming empty, once equilibrium is reached,
//! **how many seats end up occupied?**
use std::collections::HashSet;
use std::env;
use std::fs;
use std::mem;
use std::time::Instant;
use std::{fmt, fmt::Formatter};

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        };
        write!(f, "{}", c)
    }
}

impl Seat {
    fn to_usize(&self) -> usize {
        match self {
            Seat::Floor => 0,
            Seat::Empty => 0,
            Seat::Occupied => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct SeatGrid {
    grid: Vec<Seat>,
    width: usize,
    height: usize,
}

impl fmt::Display for SeatGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.grid.chunks(self.width).for_each(|x| {
            x.iter().for_each(|s| write!(f, "{:?}", s).unwrap());
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1),
];

impl SeatGrid {
    fn get(&self, x: usize, y: usize) -> Option<&Seat> {
        self.grid.get(y * self.width + x)
    }

    // fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Seat> {
    //     self.grid.get_mut(y * self.width + x)
    // }

    fn set(&mut self, x: usize, y: usize, seat: Seat) {
        let item = self.grid.get_mut(y * self.width + x).unwrap();
        *item = seat;
    }

    fn gen_surrounding_coords(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let xi = x as isize;
        let yi = y as isize;
        ((xi - 1)..=(xi + 1))
            .cartesian_product((yi - 1)..=(yi + 1))
            .map(|(i, j)| {
                (
                    clamp(i, 0, self.width as isize - 1),
                    clamp(j, 0, self.height as isize - 1),
                )
            })
            .filter_map(|(i, j)| {
                if i == xi && j == yi {
                    None
                } else {
                    Some((i as usize, j as usize))
                }
            })
            .collect::<HashSet<(usize, usize)>>()
    }

    fn get_surroundings(&self, x: usize, y: usize) -> Vec<&Seat> {
        self.gen_surrounding_coords(x, y)
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y))
            .collect()
    }

    fn surrounding_occupied(&self, x: usize, y: usize) -> usize {
        self.get_surroundings(x, y)
            .into_iter()
            .map(|item| item.to_usize())
            .sum::<usize>()
    }

    fn sight_occupied(&self, x: usize, y: usize) -> usize {
        DIRS.iter()
            .filter_map(|(dx, dy)| {
                let mut i = x as isize + dx;
                let mut j = y as isize + dy;

                let first_seen = loop {
                    if (i >= 0 && i < self.width as isize) && (j >= 0 && j < self.height as isize) {
                        let seat = self.get(i as usize, j as usize).unwrap();
                        match seat {
                            Seat::Empty | Seat::Occupied => break Some(seat),
                            Seat::Floor => {
                                i += dx;
                                j += dy;
                                continue;
                            }
                        }
                    } else {
                        break None;
                    }
                };

                first_seen
            })
            .map(|item| item.to_usize())
            .sum::<usize>()
    }
}

fn clamp(v: isize, min: isize, max: isize) -> isize {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn flap_by_surroundings(curr: &SeatGrid, next: &mut SeatGrid) {
    let width = next.width;
    // let height = next.height;
    curr.grid.iter().enumerate().for_each(|(idx, seat)| {
        let x = idx % width;
        let y = idx / width;
        match seat {
            // Floor (`.`) never changes.
            Seat::Floor => (),
            // If a seat is empty (`L`) and there are no occupied seats adjacent to it,
            // the seat becomes occupied.
            Seat::Empty => {
                if curr.surrounding_occupied(x, y) == 0 {
                    next.set(x, y, Seat::Occupied);
                } else {
                    next.set(x, y, *seat);
                }
            }
            // If a seat is occupied (`#`) and four or more seats adjacent to it
            // are also occupied, the seat becomes empty.
            Seat::Occupied => {
                if curr.surrounding_occupied(x, y) >= 4 {
                    next.set(x, y, Seat::Empty);
                } else {
                    next.set(x, y, *seat);
                }
            }
        };
    });
}

fn flap_by_linesights(curr: &SeatGrid, next: &mut SeatGrid) {
    let width = next.width;
    // let height = next.height;
    curr.grid.iter().enumerate().for_each(|(idx, seat)| {
        let x = idx % width;
        let y = idx / width;
        match seat {
            // Floor (`.`) never changes.
            Seat::Floor => (),
            // If a seat is empty (`L`) and there are no occupied seats adjacent to it,
            // the seat becomes occupied.
            Seat::Empty => {
                if curr.sight_occupied(x, y) == 0 {
                    next.set(x, y, Seat::Occupied);
                } else {
                    next.set(x, y, *seat);
                }
            }
            // If a seat is occupied (`#`) and four or more seats adjacent to it
            // are also occupied, the seat becomes empty.
            // part 2: it now takes five or more visible occupied seats for an
            // occupied seat to become empty
            Seat::Occupied => {
                if curr.sight_occupied(x, y) >= 5 {
                    next.set(x, y, Seat::Empty);
                } else {
                    next.set(x, y, *seat);
                }
            }
        };
    });
}

fn seat_dynamics<F>(seat_map: &SeatGrid, flat_func: F) -> SeatGrid
where
    F: Fn(&SeatGrid, &mut SeatGrid),
{
    let mut curr = seat_map.clone();
    let mut next = seat_map.clone();

    loop {
        flat_func(&curr, &mut next);
        if next == curr {
            break;
        } else {
            mem::swap(&mut curr, &mut next);
        }
    }

    next
}

fn count_occupied(seat_map: &[Seat]) -> usize {
    seat_map.iter().map(Seat::to_usize).sum::<usize>()
}

fn parse_input(input: &str) -> SeatGrid {
    let map2d: Vec<Vec<Seat>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => panic!("Unknown char {}.", c),
                })
                .collect()
        })
        .collect();

    let height = map2d.len();
    let width = map2d[0].len();

    SeatGrid {
        grid: map2d.into_iter().flatten().collect(),
        width,
        height,
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

    let seat_map = parse_input(&input);

    let start = Instant::now();
    let new_map = seat_dynamics(&seat_map, flap_by_surroundings);
    let duration = start.elapsed();

    println!(
        "The number of seats end up occupied via surrounding rule is {}. Time elapsed is {:?}.",
        count_occupied(&new_map.grid),
        duration,
    );

    let new_map = seat_dynamics(&seat_map, flap_by_linesights);
    println!(
        "The number of seats end up occupied via sight rule is {}. Time elapsed is {:?}.",
        count_occupied(&new_map.grid),
        duration,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Seat::*;

    #[test]
    fn test_parse_input() {
        let input = "#.LL.L#.##\n#LLLLLL.L#";
        let seat_map = parse_input(input);
        assert_eq!(
            seat_map,
            SeatGrid {
                grid: vec![
                    Occupied, Floor, Empty, Empty, Floor, Empty, Occupied, Floor, Occupied,
                    Occupied, // line break
                    Occupied, Empty, Empty, Empty, Empty, Empty, Empty, Floor, Empty, Occupied,
                ],
                width: 10,
                height: 2,
            },
        );
    }

    #[test]
    fn test_count() {
        let input = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";
        let seat_map = parse_input(input);
        assert_eq!(count_occupied(&seat_map.grid), 37);
    }

    #[test]
    fn test_seat_map_display() {
        let input = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";
        let seat_map = parse_input(input);
        let output = format!("{}", seat_map);
        assert_eq!(&output, input)
    }

    #[test]
    // #[ignore]
    fn test_seat_surroundings() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let seat_map = parse_input(input);
        let new_map = seat_dynamics(&seat_map, flap_by_surroundings);
        let left = format!("{}", new_map);

        assert_eq!(
            seat_map.gen_surrounding_coords(9, 0),
            vec![(8, 0), (9, 1), (8, 1)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );

        let right = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"
        .to_string();
        assert_eq!(left, right);
    }

    #[test]
    fn test_seat_linesights() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let seat_map = parse_input(input);
        let new_map = seat_dynamics(&seat_map, flap_by_linesights);
        let left = format!("{}", new_map);
        let right = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
        .to_string();
        assert_eq!(left, right);
    }
}
