use aoc2020::advent_main;
use itertools::Itertools;

advent_main!(day1, "data/day1_input.txt");

fn day1(lines: &[String]) -> (i64, i64) {
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
    (part1, part2)
}

fn find_items_that_sum_to(items: &[i64], selection_count: usize, sum_to: i64) -> Option<Vec<i64>> {
    items
        .iter()
        .cloned()
        .combinations(selection_count)
        .find(|candidate| candidate.iter().sum::<i64>() == sum_to)
}
