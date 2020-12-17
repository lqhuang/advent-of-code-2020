//! # --- Day 12: Rain Risk ---
//!
//! Your ferry made decent progress toward the island, but the storm came in
//! faster than anyone expected. The ferry needs to take **evasive actions**!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning;
//! rather than giving a route directly to safety, it produced extremely
//! circuitous instructions. When the captain uses the PA system to ask if
//! anyone can help, you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of
//! single-character **actions** paired with integer input **values**. After staring at
//! them for a few minutes, you work out what they probably mean:
//!
//! - Action `N` means to move **north** by the given value.
//! - Action `S` means to move **south** by the given value.
//! - Action `E` means to move **east** by the given value.
//! - Action `W` means to move **west** by the given value.
//! - Action `L` means to turn **left** the given number of degrees.
//! - Action `R` means to turn **right** the given number of degrees.
//! - Action `F` means to move **forward** by the given value in the direction the
//!   ship is currently facing.
//!
//! The ship starts by facing **east**. Only the `L` and `R`actions change the
//! direction the ship is facing. (That is, if the ship is facing east and the
//! next instruction is `N10`, the ship would move north 10 units, but would
//! still move east if the following action were `F`.)
//!
//! For example:
//!
//! ```
//! F10
//! N3
//! F7
//! R90
//! F11
//! ```
//!
//! These instructions would be handled as follows:
//!
//! - `F10` would move the ship 10 units east (because the ship starts by facing east) to **east 10, north 0**.
//! - `N3` would move the ship 3 units north to **east 10, north 3**.
//! - `F7` would move the ship another 7 units east (because the ship is still facing east) to **east 17, north 3**.
//! - `R90` would cause the ship to turn right by 90 degrees and face south; it remains at **east 17, north 3**.
//! - `F11` would move the ship 11 units south to **east 17, south 8**.
//!
//! At the end of these instructions, the ship's Manhattan distance (sum of the
//! absolute values of its east/west position and its north/south position)
//! from its starting position is `17 + 8 = 25`.
//!
//! Figure out where the navigation instructions lead.
//! **What is the Manhattan distance between that location and the ship's starting position?**
//!
//! ## --- Part Two ---
//!
//! Before you can give the destination to the captain, you realize that the
//! actual action meanings were printed on the back of the instructions the
//! whole time.
//!
//! Almost all of the actions indicate how to move a **waypoint** which is
//! relative to the ship's position:
//!
//! - Action `N` means to move the waypoint **north** by the given value.
//! - Action `S` means to move the waypoint **south** by the given value.
//! - Action `E` means to move the waypoint **east** by the given value.
//! - Action `W` means to move the waypoint **west** by the given value.
//! - Action `L` means to rotate the waypoint around the ship **left** (**counter-clockwise**) the given number of degrees.
//! - Action `R` means to rotate the waypoint around the ship **right** (**clockwise**) the given number of degrees.
//! - Action `F` means to move **forward** to the waypoint a number of times equal to the given value.
//!
//! The waypoint starts **10 units east and 1 unit north** relative to the ship.
//! The waypoint is relative to the ship; that is, if the ship moves, the
//! waypoint moves with it.
//!
//! For example, using the same instructions as above:
//!
//! - `F10` moves the ship to the waypoint 10 times (a total of **100 units east and 10 units north**), leaving the ship at **east 100, north 10**. The waypoint stays 10 units east and 1 unit north of the ship.
//! - `N3` moves the waypoint 3 units north to **10 units east and 4 units north** of the ship. The ship remains at **east 100, north 10**.
//! - `F7` moves the ship to the waypoint 7 times (a total of **70 units east and 28 units north**), leaving the ship at **east 170, north 38**. The waypoint stays 10 units east and 4 units north of the ship.
//! - `R90` rotates the waypoint around the ship clockwise 90 degrees, moving it to **4 units east and 10 units south** of the ship. The ship remains at **east 170, north 38**.
//! - `F11` moves the ship to the waypoint 11 times (a total of **44 units east and 110 units south**), leaving the ship at **east 214, south 72**. The waypoint stays 4 units east and 10 units south of the ship.
//!
//! After these operations, the ship's Manhattan distance from its starting
//! position is `214 + 72 = 286`.
//!
//! Figure out where the navigation instructions actually lead.
//! **What is the Manhattan distance between that location and the ship's starting position?**
use std::env;
use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::{branch::alt, sequence::tuple, IResult};

#[derive(Debug, Eq, PartialEq)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Action {
    fn from_tuple((a, val): (&str, usize)) -> Option<Action> {
        match a {
            "N" => Some(Action::North(val)),
            "S" => Some(Action::South(val)),
            "E" => Some(Action::East(val)),
            "W" => Some(Action::West(val)),
            "L" => Some(Action::Left(val)),
            "R" => Some(Action::Right(val)),
            "F" => Some(Action::Forward(val)),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    waypoint: Waypoint,
}

impl Position {
    fn get_manhattan_distance(&self, x: isize, y: isize) -> isize {
        (self.x - x).abs() + (self.y - y).abs()
    }

    fn do_first_type(self, action: &Action) -> Self {
        let (x, y, waypoint) = (self.x, self.y, self.waypoint);
        match *action {
            Action::North(val) => Position {
                x,
                y: y + val as isize,
                waypoint,
            },
            Action::South(val) => Position {
                x,
                y: y - val as isize,
                waypoint,
            },
            Action::East(val) => Position {
                x: x + val as isize,
                y,
                waypoint,
            },
            Action::West(val) => Position {
                x: x - val as isize,
                y,
                waypoint,
            },
            Action::Left(ang) => Position {
                x,
                y,
                waypoint: waypoint.rotate(ang as isize),
            },
            Action::Right(ang) => Position {
                x,
                y,
                waypoint: waypoint.rotate(-(ang as isize)),
            },
            Action::Forward(val) => Position {
                x: x + (val as isize) * waypoint.dx,
                y: y + (val as isize) * waypoint.dy,
                waypoint,
            },
        }
    }

    fn do_second_type(self, action: &Action) -> Position {
        let (x, y, waypoint) = (self.x, self.y, self.waypoint);
        match *action {
            Action::North(_) | Action::South(_) | Action::East(_) | Action::West(_) => Position {
                x,
                y,
                waypoint: waypoint.move_(action),
            },
            Action::Left(ang) => Position {
                x,
                y,
                waypoint: waypoint.rotate(ang as isize), // counter-clockwise
            },
            Action::Right(ang) => Position {
                x,
                y,
                waypoint: waypoint.rotate(-(ang as isize)), // clockwise
            },
            Action::Forward(val) => Position {
                x: x + (val as isize) * waypoint.dx,
                y: y + (val as isize) * waypoint.dy,
                waypoint,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Waypoint {
    dx: isize,
    dy: isize,
}

impl Waypoint {
    fn move_(&self, action: &Action) -> Waypoint {
        let (dx, dy) = (self.dx, self.dy);
        match *action {
            Action::North(val) => Waypoint {
                dx,
                dy: dy + val as isize,
            },
            Action::South(val) => Waypoint {
                dx,
                dy: dy - val as isize,
            },
            Action::East(val) => Waypoint {
                dx: dx + val as isize,
                dy,
            },
            Action::West(val) => Waypoint {
                dx: dx - val as isize,
                dy,
            },
            _ => panic!("Won't be here"),
        }
    }

    fn rotate(&self, ang: isize) -> Waypoint {
        let theta = (ang as f64).to_radians();
        let x = self.dx as f64;
        let y = self.dy as f64;
        Waypoint {
            dx: (x * theta.cos() - y * theta.sin()).round() as isize,
            dy: (x * theta.sin() + y * theta.cos()).round() as isize,
        }
    }
}

fn action_parser(input: &str) -> IResult<&str, Action> {
    let (input, (act, val)) = tuple((
        alt((
            tag("N"),
            tag("S"),
            tag("E"),
            tag("W"),
            tag("L"),
            tag("R"),
            tag("L"),
            tag("F"),
        )),
        digit1,
    ))(input)?;

    Ok((
        input,
        Action::from_tuple((act, val.parse().unwrap())).unwrap(),
    ))
}

fn parse_input(input: &str) -> Vec<Action> {
    input
        .lines()
        .filter_map(|line| match action_parser(line) {
            Ok((_, action)) => Some(action),
            _ => None,
        })
        .collect()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = &args[1];
    println!("Load input file {}.", filename);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let actions = parse_input(&input);

    let zero = Position {
        x: 0,
        y: 0,
        waypoint: Waypoint { dx: 1, dy: 0 },
    };
    let first_pos = actions.iter().fold(zero, |acc, x| acc.do_first_type(x));
    println!(
        "The first location from starting position is {}",
        first_pos.get_manhattan_distance(0, 0)
    );

    let zero = Position {
        x: 0,
        y: 0,
        waypoint: Waypoint { dx: 10, dy: 1 },
    };
    let second_pos = actions.iter().fold(zero, |acc, x| acc.do_second_type(x));
    println!(
        "The first location from starting position is {}",
        second_pos.get_manhattan_distance(0, 0)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_parser() {
        let input = "F10";
        let (_, a) = action_parser(input).unwrap();
        assert_eq!(a, Action::Forward(10));
    }

    #[test]
    fn test_parse_input() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let actions = parse_input(input);
        assert_eq!(
            actions,
            vec![
                Action::Forward(10),
                Action::North(3),
                Action::Forward(7),
                Action::Right(90),
                Action::Forward(11),
            ]
        );
    }

    #[test]
    fn test_rotation() {
        let raw = Waypoint { dx: 0, dy: 1 };
        assert_eq!(raw.rotate(90), Waypoint { dx: -1, dy: 0 });
        assert_eq!(raw.rotate(180), Waypoint { dx: 0, dy: -1 });
        assert_eq!(raw.rotate(-270), Waypoint { dx: -1, dy: 0 });
        assert_eq!(raw.rotate(360), Waypoint { dx: 0, dy: 1 });
        assert_eq!(raw.rotate(630), Waypoint { dx: 1, dy: 0 });

        let raw = Waypoint { dx: 10, dy: 4 };
        assert_eq!(raw.rotate(-90), Waypoint { dx: 4, dy: -10 });
    }

    #[test]
    fn test_first_instruction() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let actions = parse_input(&input);
        let zero = Position {
            x: 0,
            y: 0,
            waypoint: Waypoint { dx: 1, dy: 0 },
        };
        let pos = actions.iter().fold(zero, |acc, x| acc.do_first_type(x));
        assert_eq!(
            pos,
            Position {
                x: 17,
                y: -8,
                waypoint: Waypoint { dx: 0, dy: -1 },
            },
        );
        assert_eq!(pos.get_manhattan_distance(0, 0), 25)
    }

    #[test]
    fn test_second_instruction() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let actions = parse_input(&input);
        let zero = Position {
            x: 0,
            y: 0,
            waypoint: Waypoint { dx: 10, dy: 1 },
        };
        let pos = actions.iter().fold(zero, |acc, x| acc.do_second_type(x));
        assert_eq!(
            pos,
            Position {
                x: 214,
                y: -72,
                waypoint: Waypoint { dx: 4, dy: -10 },
            },
        );
        assert_eq!(pos.get_manhattan_distance(0, 0), 286)
    }
}
