use aoc2020::{group_lines_by_blank, read_lines};
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let lines = read_lines("data/day6_input.txt").expect("Could not open file");
    let grouped_lines = group_lines_by_blank(&lines);
    let groups: Vec<_> = grouped_lines
        .iter()
        .map(QuestionGroup::from_lines)
        .collect();

    let part1: usize = groups.iter().map(|group| group.someone.len()).sum();
    let part2: usize = groups.iter().map(|group| group.everyone.len()).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

struct QuestionGroup {
    someone: HashSet<char>,
    everyone: HashSet<char>,
}

impl QuestionGroup {
    fn from_lines(lines: &Vec<&str>) -> QuestionGroup {
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
