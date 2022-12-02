extern crate itertools;

use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.parse::<usize>())
        .group_by(|elt| elt.is_ok())
        .into_iter()
        .filter_map(|(key, results)| if key {Some(results)} else {None})
        .map(|results| results.into_iter().map(|r| r.unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(elves: &[Vec<usize>]) -> usize {
    elves.iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(elves: &[Vec<usize>]) -> usize {
    let mut totals: Vec<usize> = elves.iter()
        .map(|elf| elf.iter().sum())
        .collect();
    totals.sort_unstable();
    totals.iter().rev().take(3).sum()
}
