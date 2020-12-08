use aoc2020::advent_main;
use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};

advent_main!(day7, "data/day7_input.txt");

fn day7(lines: &[String]) -> (usize, usize) {
    let rules: BagRules = lines
        .iter()
        .map(String::as_str)
        .map(bag_rule_from_line)
        .collect();
    let part1 = find_bags_that_eventually_contain("shiny gold", &rules).len();
    let part2 = recursive_bag_count("shiny gold", &rules) - 1;
    (part1, part2)
}

type BagRules<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn bag_rule_from_line<'a>(line: &'a str) -> (&'a str, HashMap<&'a str, usize>) {
    let result = RuleParser::parse(Rule::toplevel, line)
        .unwrap()
        .next()
        .unwrap();
    let id = result.clone().into_inner().next().unwrap().as_str();
    let sub_bags = result
        .clone()
        .into_inner()
        .nth(1)
        .unwrap()
        .into_inner()
        .map(|sub_bag| {
            let mut iter = sub_bag.into_inner();
            let count = iter.next().unwrap().as_str().parse::<usize>().unwrap();
            let id = iter.next().unwrap().as_str();
            (id, count)
        })
        .collect();
    (id, sub_bags)
}

fn find_bags_that_eventually_contain<'a>(id: &str, rules: &'a BagRules) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let mut new_ones: HashSet<&str> = HashSet::new();

    loop {
        for (candidate_id, candidate_contents) in rules {
            if result.contains(candidate_id) {
                continue;
            }

            if candidate_contents.contains_key(id) {
                new_ones.insert(candidate_id);
            }
            for recursive_option in result.iter() {
                if candidate_contents.contains_key(recursive_option) {
                    new_ones.insert(candidate_id);
                }
            }
        }

        if new_ones.len() == 0 {
            break;
        }

        result = result.union(&new_ones).cloned().collect();
        new_ones = HashSet::new();
    }

    result
}

fn recursive_bag_count(id: &str, rules: &BagRules) -> usize {
    1 + rules[id]
        .iter()
        .map(|(id, count)| count * recursive_bag_count(id, rules))
        .sum::<usize>()
}

#[derive(Parser)]
#[grammar = "day7_grammar.pest"]
struct RuleParser;

#[test]
fn test_parser() {
    RuleParser::parse(Rule::bag_identifier, "plaid beige").unwrap();
    RuleParser::parse(Rule::count, "100").unwrap();
    RuleParser::parse(Rule::content_record, "1 bright white bag").unwrap();
    RuleParser::parse(Rule::content_record, "2 muted yellow bags").unwrap();
    RuleParser::parse(Rule::bag_contents, "2 muted yellow bags").unwrap();
    RuleParser::parse(
        Rule::bag_contents,
        "1 bright white bag, 2 muted yellow bags",
    )
    .unwrap();
    RuleParser::parse(Rule::bag_contents, "no other bags").unwrap();
    RuleParser::parse(
        Rule::toplevel,
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
    )
    .unwrap();
}

#[test]
fn test_bag_rule_from_line() {
    let result =
        bag_rule_from_line("dotted magenta bags contain 2 mirrored fuchsia bags, 3 pale purple bags, 5 shiny yellow bags.");
    assert_eq!(
        result,
        (
            "dotted magenta",
            vec![
                ("mirrored fuchsia", 2),
                ("pale purple", 3),
                ("shiny yellow", 5)
            ]
            .iter()
            .cloned()
            .collect()
        )
    );
}
