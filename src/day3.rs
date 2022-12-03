use std::collections::HashSet;
use itertools::Itertools;

fn score(char: char) -> usize {
    (match char.is_lowercase() {
        true => (char as u8 - b'a') + 1,
        false => (char as u8 - b'A') + 27,
    }) as usize
}
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(sacks: &Vec<String>) -> usize {
    sacks.into_iter().map(|sack| {
        let (left, right) = sack.split_at(sack.len()/2);
        assert_eq!(left.len(), right.len());
        let right_set: HashSet<char> = right.chars().collect();
        left.chars().filter(|c| right_set.contains(c)).nth(0).unwrap()
    }).map(|c| score(c.clone())).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(sacks: &Vec<String>) -> usize {
    let sack_specials: Vec<char> = sacks.iter().chunks(3).into_iter().map(|troop| {
        let common: HashSet<char> = troop.map(|sack| HashSet::<char>::from_iter(sack.chars()))
            .reduce(|left: HashSet<char>, right: HashSet<char>|
                left.intersection(&right).cloned().collect::<HashSet<char>>()
            ).unwrap();
        common.iter().nth(0).unwrap().clone()
    }).collect();
    sack_specials.iter().map(|c: &char| score(c.clone())).sum()
}