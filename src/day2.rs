use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::day2::Move::{PAPER, ROCK, SCISSORS};

#[derive(PartialEq)]
pub enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}
impl From<&str> for Move {
    fn from(input: &str) -> Self {
        match input {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => panic!("Unknown input {}", input)
        }
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other { Some(Equal) }
        else if (self == &ROCK && other == &PAPER) ||
                 (self == &PAPER && other == &SCISSORS) ||
                 (self == &SCISSORS && other == &ROCK) { Some(Less) }
        else { Some(Greater) }
    }
}

fn parse(line: &str) -> (Move, Move) {
    let pieces = line.split_at(1);
    (Move::from(pieces.0.trim()), Move::from(pieces.1.trim()))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Move, Move)> {
    input.lines()
        .map(|line| parse(line))
        .collect()
}

fn score_part1(left: &Move, right: &Move) -> usize {
    let score = match left.partial_cmp(&right) {
        Some(Less) => 6,
        Some(Equal) => 3,
        Some(Greater) => 0,
        _ => 0,
    };
    let m = match right {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    };
    score + m
}

#[aoc(day2, part1)]
pub fn solve_part1(plays: &[(Move, Move)]) -> usize {
    plays.iter()
        .map(|(l, r)| score_part1(l, r))
        .sum()
}