use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use crate::day2::Move::{PAPER, ROCK, SCISSORS};
use crate::day2::Outcome::{DRAW, LOSE, WIN};

#[derive(PartialEq, Copy, Clone)]
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
#[derive(PartialEq)]
pub enum Outcome {
    LOSE,
    DRAW,
    WIN,
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => LOSE,
            "Y" => DRAW,
            "Z" => WIN,
            _ => panic!("Unknown input {}", input)
        }
    }
}

fn parse(line: &str) -> (Move, Move, Outcome) {
    let pieces = line.split_at(1);
    (Move::from(pieces.0.trim()), Move::from(pieces.1.trim()), Outcome::from(pieces.1.trim()))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Move, Move, Outcome)> {
    input.lines()
        .map(|line| parse(line))
        .collect()
}

fn score_round(left: &Move, right: &Move) -> usize {
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
pub fn solve_part1(plays: &[(Move, Move, Outcome)]) -> usize {
    plays.iter()
        .map(|(l, r, _)| score_round(l, r))
        .sum()
}

fn calculate_round(left: &Move, outcome: &Outcome) -> Move {
    if outcome == &Outcome::DRAW { left.clone() }
    else if outcome == &Outcome::WIN {
        match left {
            ROCK => PAPER,
            PAPER => SCISSORS,
            SCISSORS => ROCK,
        }
    }
    else {
        match left {
            ROCK => SCISSORS,
            PAPER => ROCK,
            SCISSORS => PAPER,
        }
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(plays: &[(Move, Move, Outcome)]) -> usize {
    plays.iter()
        .map(|(l, _, o)| (l, calculate_round(l, o)))
        .map(|(l, r)| score_round(l, &r))
        .sum()
}