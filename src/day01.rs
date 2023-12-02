use std::fs;
use std::collections::HashMap;
use regex::Regex;

const PATTERN1: &str = r"(\d)";
const PATTERN2: &str = r"(\d|one|two|three|four|five|six|seven|eight|nine)"; 

fn parse_str(s: &str, pattern: &str) -> isize {
    // rust regex engine doesn't support overlap, so manually "de-overlap"
    let s = &s.replace("twone", "twoone");
    let s = &s.replace("oneight", "oneeight");
    let s = &s.replace("eightwo", "eighttwo");

    let re = Regex::new(pattern).unwrap();
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert("one", "1");
    hm.insert("two", "2");
    hm.insert("three", "3");
    hm.insert("four", "4");
    hm.insert("five", "5");
    hm.insert("six", "6");
    hm.insert("seven", "7");
    hm.insert("eight", "8");
    hm.insert("nine", "9");

    let mut it = re.find_iter(s);
    let first = it.next().unwrap().as_str();
    let mut last = first;
    while let Some(m) = it.next() {
        last = m.as_str();
    };
    let first = match hm.get(first) {
        Some(s) => s,
        None => first,
    };
    let last = match hm.get(last) {
        Some(s) => s,
        None => last,
    };
    let mut res = String::from(first);
    res.push_str(last);
    res.parse::<isize>().unwrap()
}

pub fn main() {
    println!("main from day01!");
    let contents = fs::read_to_string("inputs/day01.txt").expect("unable to read file contents");
    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        part1 += parse_str(line, PATTERN1);
        part2 += parse_str(line, PATTERN2);
    }
    println!("part1: {part1}");
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str("1abc2", PATTERN1), 12);
        assert_eq!(parse_str("pqr3stu8vwx", PATTERN1), 38);
        assert_eq!(parse_str("a1b2c3d4e5f", PATTERN1), 15);
        assert_eq!(parse_str("treb7uchet", PATTERN1), 77);
        assert_eq!(parse_str("two1nine", PATTERN2), 29);
        assert_eq!(parse_str("eightwothree", PATTERN2), 83);
        assert_eq!(parse_str("abcone2threexyz", PATTERN2), 13);
        assert_eq!(parse_str("xtwone3four", PATTERN2), 24);
        assert_eq!(parse_str("4nineeightseven2", PATTERN2), 42);
        assert_eq!(parse_str("zoneight234", PATTERN2), 14);
        assert_eq!(parse_str("7pqrstsixteen", PATTERN2), 76);
        assert_eq!(parse_str("326sevenfivenseven1kctgmnqtwonefq", PATTERN2), 31);
        assert_eq!(parse_str("pqfphhcgxz8eightwohv", PATTERN2), 82);
    }
}
