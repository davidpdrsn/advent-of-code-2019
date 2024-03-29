use super::{read_file, Error, Part, Result};
use std::collections::HashSet;

mod part_1;
mod part_2;

pub fn main(part: Part) -> Result<()> {
    match part {
        Part::One => part_1::main(),
        Part::Two => part_2::main(),
    }
}

fn parse_moves(line: &str) -> Result<Vec<Move>> {
    line.split(',')
        .map(|s| s.replace("\n", ""))
        .map(|s| parse_move(&s))
        .collect::<Result<Vec<_>>>()
}

fn parse_move(m: &str) -> Result<Move> {
    let parts = m.split("").filter(|s| !s.is_empty()).collect::<Vec<_>>();

    let direction = &parts[0];
    let maginutde = parts[1..].join("").parse::<i32>()?;

    let move_ = match direction {
        &"U" => Move::Up(maginutde),
        &"D" => Move::Down(maginutde),
        &"R" => Move::Right(maginutde),
        &"L" => Move::Left(maginutde),
        dir => return Err(Error::msg(format!("Invalid movement direction {}", dir))),
    };

    Ok(move_)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn into_single_moves(self) -> Vec<Self> {
        let range = |n| (1..=n).map(|_| 1);

        match self {
            Move::Up(maginutde) => range(maginutde).map(Move::Up).collect(),
            Move::Down(maginutde) => range(maginutde).map(Move::Down).collect(),
            Move::Left(maginutde) => range(maginutde).map(Move::Left).collect(),
            Move::Right(maginutde) => range(maginutde).map(Move::Right).collect(),
        }
    }
}

#[derive(Debug)]
struct WirePath {
    current_position: Pos,
    previous_positions: Vec<Pos>,
}

impl WirePath {
    fn new() -> Self {
        Self {
            current_position: Pos { x: 0, y: 0 },
            previous_positions: Vec::new(),
        }
    }

    fn current_position(&self) -> Pos {
        self.current_position
    }

    fn all_positions(&self) -> Vec<Pos> {
        let mut positions = self.previous_positions.clone();
        positions.push(self.current_position());
        positions
    }

    fn apply(&mut self, move_: Move) {
        self.previous_positions.push(self.current_position());

        let mut next_pos = self.current_position;

        match move_ {
            Move::Up(maginutde) => next_pos.y += maginutde,
            Move::Down(maginutde) => next_pos.y -= maginutde,
            Move::Right(maginutde) => next_pos.x += maginutde,
            Move::Left(maginutde) => next_pos.x -= maginutde,
        }

        self.current_position = next_pos;
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn manhattan_distance(p: Pos, q: Pos) -> i32 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

#[test]
fn manhattan_distance_test() {
    let p = Pos { x: 0, y: 0 };

    let q = Pos { x: 3, y: 3 };
    assert_eq!(manhattan_distance(p, q), 6);

    let q = Pos { x: 6, y: 5 };
    assert_eq!(manhattan_distance(p, q), 11);
}

#[test]
fn test_parse_move() {
    use Move::*;
    let moves = parse_moves("L1008,D451,L146,D628").unwrap();
    assert_eq!(moves, vec![Left(1008), Down(451), Left(146), Down(628)]);
}
