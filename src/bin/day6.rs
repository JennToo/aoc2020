use aoc2020::{advent_main, group_lines_by_blank};
use std::collections::HashSet;
use std::hash::Hash;

advent_main!(day6, "data/day6_input.txt");

fn day6(lines: &[String]) -> (usize, usize) {
    let grouped_lines = group_lines_by_blank(&lines);
    let groups: Vec<_> = grouped_lines
        .iter()
        .map(Vec::as_slice)
        .map(QuestionGroup::from_lines)
        .collect();

    let part1: usize = groups.iter().map(|group| group.someone.len()).sum();
    let part2: usize = groups.iter().map(|group| group.everyone.len()).sum();

    (part1, part2)
}

struct QuestionGroup {
    someone: HashSet<char>,
    everyone: HashSet<char>,
}

impl QuestionGroup {
    fn from_lines(lines: &[&str]) -> QuestionGroup {
        let responses: Vec<HashSet<char>> =
            lines.iter().map(|line| line.chars().collect()).collect();

        QuestionGroup {
            someone: responses.iter().fold(HashSet::new(), union_sets),
            everyone: responses.iter().fold(responses[0].clone(), intersect_sets),
        }
    }
}

fn union_sets<T>(set1: HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    set1.union(set2).cloned().collect()
}

fn intersect_sets<T>(set1: HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    set1.intersection(set2).cloned().collect()
}
