use aoc2020::read_lines;
use itertools::Itertools;

fn main() {
    let lines = read_lines("data/day1_input.txt").expect("Could not read input file");
    let items = lines
        .iter()
        .map(|line| line.parse::<i64>().expect("Invalid number"))
        .collect::<Vec<i64>>();
    let part1: i64 = find_items_that_sum_to(&items, 2, 2020)
        .unwrap()
        .iter()
        .product();
    let part2: i64 = find_items_that_sum_to(&items, 3, 2020)
        .unwrap()
        .iter()
        .product();
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn find_items_that_sum_to(
    items: &Vec<i64>,
    selection_count: usize,
    sum_to: i64,
) -> Option<Vec<i64>> {
    items
        .iter()
        .cloned()
        .combinations(selection_count)
        .find(|candidate| candidate.iter().sum::<i64>() == sum_to)
}
