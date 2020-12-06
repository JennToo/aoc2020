use aoc2020::advent_main;
use lazy_static::lazy_static;
use regex::Regex;

advent_main!(day2, "data/day2_input.txt");

fn day2(lines: &[String]) -> (usize, usize) {
    let parsed_lines = lines
        .iter()
        .map(|line| parse_line(line.as_ref()))
        .collect::<Vec<_>>();

    let part1 = parsed_lines
        .iter()
        .filter(|(password, rule)| password_is_valid_min_max(password.as_ref(), rule))
        .count();
    let part2 = parsed_lines
        .iter()
        .filter(|(password, rule)| password_is_valid_indices(password.as_ref(), rule))
        .count();

    (part1, part2)
}

#[derive(Debug, Eq, PartialEq)]
struct PasswordCharRule {
    char_: char,
    min: usize,
    max: usize,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

fn parse_line(line: &str) -> (String, PasswordCharRule) {
    let captures = LINE_REGEX.captures(line).expect("Invalid line");
    (
        captures[4].into(),
        PasswordCharRule {
            char_: captures[3].chars().next().unwrap(),
            min: captures[1].parse::<usize>().unwrap(),
            max: captures[2].parse::<usize>().unwrap(),
        },
    )
}

fn password_is_valid_min_max(password: &str, rule: &PasswordCharRule) -> bool {
    let count = password.chars().filter(|c| *c == rule.char_).count();
    count <= rule.max && count >= rule.min
}

fn password_is_valid_indices(password: &str, rule: &PasswordCharRule) -> bool {
    let first = password.chars().nth(rule.min - 1).unwrap();
    let second = password.chars().nth(rule.max - 1).unwrap();

    (first == rule.char_) != (second == rule.char_)
}

#[test]
fn test_line_parse() {
    let result = parse_line("1-3 a: abcde");
    assert_eq!(
        result,
        (
            "abcde".into(),
            PasswordCharRule {
                char_: 'a',
                min: 1,
                max: 3
            }
        )
    );
}

#[test]
fn test_part2() {
    let lines = [
        ("1-3 a: abcde", true),
        ("1-3 b: cdefg", false),
        ("2-9 c: ccccccccc", false),
    ];
    for (line, expectation) in lines.iter() {
        let (password, rule) = parse_line(line);
        assert_eq!(
            *expectation,
            password_is_valid_indices(password.as_ref(), &rule)
        );
    }
}
