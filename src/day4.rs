use std::ops::Range;

fn range_from_str(input: &str) -> Range<u16> {
    let parts: Vec<u16> = input.split('-')
        .filter_map(|p| p.parse::<u16>().ok())
        .collect();
    Range { start: parts[0], end: parts[1] + 1 }
}
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range<u16>, Range<u16>)> {
    input.lines().map(|s| {
        let elves: Vec<&str> = s.split(',').collect();
        (range_from_str(elves[0]), range_from_str(elves[1]))
    }).collect()
}

fn contains_all(left: &Range<u16>, needles: &Range<u16>) -> bool {
    let mut my_needles = needles.clone();
    my_needles.all(|n| left.contains(&n))
}
#[aoc(day4, part1)]
pub fn solve_part1(assignments: &Vec<(Range<u16>, Range<u16>)>) -> usize {
    assignments.into_iter()
        .filter(|(a, b)| contains_all(a, b) || contains_all(b, a))
        .count()
}