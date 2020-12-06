use aoc2020::read_lines;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let lines = read_lines("data/day6_input.txt").expect("Could not open file");
    let groups = QuestionGroup::from_lines(&lines);

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
    fn from_lines(lines: &Vec<String>) -> Vec<QuestionGroup> {
        let mut groups = Vec::new();
        let mut responses: Vec<HashSet<char>> = Vec::new();

        for line in lines.iter() {
            if line == "" {
                groups.push(Self::from_sets(responses));
                responses = Vec::new();
            } else {
                let mut response = HashSet::new();
                for char_ in line.chars() {
                    response.insert(char_);
                }
                responses.push(response);
            }
        }
        groups.push(Self::from_sets(responses));

        groups
    }

    fn from_sets(sets: Vec<HashSet<char>>) -> QuestionGroup {
        QuestionGroup {
            someone: sets.iter().fold(HashSet::new(), union_sets),
            everyone: sets.iter().fold(sets[0].clone(), intersect_sets),
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
