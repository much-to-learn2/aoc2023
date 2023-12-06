use std::fs;
use regex::Regex;

fn distance(time: i64, held: i64) -> i64 {
    // s/o lauren pasek
    // y = -(x)(x-time)
    -held * (held - time)
}

fn beat_distance(time: i64, record: i64) -> (i64, i64) {
    // distance < -(x)(x-time), solve for x
    let mut i = 0;
    while distance(time, i) <= record {
        i += 1
    };
    (i, time - i)
}

fn solve_part1(times: &Vec<i64>, dists: &Vec<i64>) -> i64 {
    let mut res = 1;
    for i in 0..times.len() {
        let (min, max) = beat_distance(times[i], dists[i]);
        let ways = max - min + 1;
        res *= ways;
    };
    res
}

fn parse_line(s: &str) -> Vec<i64> {
    let s: Vec<i64> = s[s.find(|c: char| c.is_ascii_digit()).unwrap()..]
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    s
}

fn concat(vec: Vec<i64>) -> i64 {
    vec.iter().fold(String::from(""), |acc, curr| {
        acc + &curr.to_string()
    }).parse::<i64>().unwrap()
}


pub fn main() {
    println!("main from day06!");
    let contents = fs::read_to_string("inputs/day06.txt").expect("unable to read file contents");
    let mut it = contents.lines();
    let times = parse_line(it.next().unwrap());
    let distances = parse_line(it.next().unwrap());
    let part1 = solve_part1(&times, &distances);
    println!("part1: {part1}");

    let time = concat(times);
    let distance = concat(distances);
    println!("time: {time}, distance: {distance}");

    let (min, max) = beat_distance(time, distance);
    let part2 = max - min + 1;
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula() {
        assert_eq!(beat_distance(7, 9), (2, 5));
    }

    #[test]
    fn test_part1() {
        let times = vec![7, 15, 30];
        let dists = vec![9, 40, 200];
        let res = solve_part1(&times, &dists);
        assert_eq!(res, 288);
    }
}
