use aoc2020::{advent_main, group_lines_by_blank};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

advent_main!(day4, "data/day4_input.txt");

fn day4(lines: &[String]) -> (usize, usize) {
    let line_groups = group_lines_by_blank(&lines);
    let passports: Vec<_> = line_groups
        .iter()
        .map(Vec::as_slice)
        .map(Passport::from_lines)
        .collect();

    let part1 = passports
        .iter()
        .filter(|passport| passport.has_required_fields())
        .count();
    let part2 = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    (part1, part2)
}

#[derive(Debug)]
struct Passport(HashMap<String, String>);

lazy_static! {
    static ref RECORD_REGEX: Regex = Regex::new(r"(\w+):([\w#]+)").unwrap();
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref HEX_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref YEAR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
}

impl Passport {
    fn from_lines(lines: &[&str]) -> Passport {
        Passport(
            lines
                .iter()
                .flat_map(|line| {
                    RECORD_REGEX
                        .captures_iter(line)
                        .map(|captures| (captures[1].into(), captures[2].into()))
                })
                .collect(),
        )
    }

    fn has_required_fields(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|field| self.0.get(*field).is_some())
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && self.check_field_range("byr", 1920, 2002)
            && self.check_field_range("iyr", 2010, 2020)
            && self.check_field_range("eyr", 2020, 2030)
            && self.check_field_height()
            && self.check_field_regex("hcl", &HEX_REGEX)
            && self.check_field_in_list("ecl", &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
            && self.check_field_regex("pid", &PID_REGEX)
    }

    fn check_field_regex(&self, field: &str, regex: &Regex) -> bool {
        regex.is_match(self.0[field].as_ref())
    }

    fn check_field_in_list(&self, field: &str, values: &[&str]) -> bool {
        values.iter().any(|value| self.0[field].as_str() == *value)
    }

    fn check_field_range(&self, field: &str, min: i64, max: i64) -> bool {
        if !YEAR_REGEX.is_match(self.0[field].as_str()) {
            false
        } else if let Ok(value) = self.0[field].parse::<i64>() {
            value >= min && value <= max
        } else {
            false
        }
    }

    fn check_field_height(&self) -> bool {
        if let Some(captures) = HEIGHT_REGEX.captures(self.0["hgt"].as_ref()) {
            if let Ok(value) = captures[1].parse::<i64>() {
                match &captures[2] {
                    "in" => value >= 59 && value <= 76,
                    "cm" => value >= 150 && value <= 193,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[test]
fn test_passports_from_lines() {
    let lines: Vec<String> = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"
    .lines()
    .map(String::from)
    .collect();

    let line_groups = group_lines_by_blank(&lines);
    let result: Vec<_> = line_groups
        .iter()
        .map(Vec::as_slice)
        .map(Passport::from_lines)
        .collect();

    println!("{:?}", result);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0].0["iyr"], "2017");

    assert_eq!(result[0].has_required_fields(), true);
    assert_eq!(result[1].has_required_fields(), false);
    assert_eq!(result[2].has_required_fields(), true);
    assert_eq!(result[3].has_required_fields(), false);
}
