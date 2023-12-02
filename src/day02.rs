// TODO: a struct to represent a game might have been a good way to do this

use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn parse_line(s: &str) -> Vec<HashMap<&str, i32>> {
    // remove prefix
    let s = &s[2+s.find(": ").unwrap()..];
    let rounds = s.split("; ");
    let pattern = r"(\d+) (green|blue|red)";
    let re = Regex::new(pattern).unwrap();
    let mut res: Vec<HashMap<&str, i32>> = vec![];
    for round in rounds {
        let mut hm: HashMap<&str, i32> = HashMap::new();
        for (_, [n, color]) in re.captures_iter(round).map(|caps| caps.extract()) {
            hm.insert(color, n.parse::<i32>().unwrap());
        };
        res.push(hm);
    }
    // println!("{:?}", res);
    res
}

fn is_game_possible(game: Vec<HashMap<&str, i32>>) -> bool {
    game.iter().all(|round| {
        let red = match round.get("red") {
            Some(&v) => v,
            None => 0,
        };
        let green = match round.get("green") {
            Some(&v) => v,
            None => 0,
        };
        let blue = match round.get("blue") {
            Some(&v) => v,
            None => 0,
        };
        red <= 12 && green <= 13 && blue <= 14
    })
}

fn min_cubes(game: Vec<HashMap<&str, i32>>) -> HashMap<&str, i32> {
    let mut hm: HashMap<&str, i32> = HashMap::new();
    game.iter().for_each(|round| {
        let colors = vec!["red", "green", "blue"];
        colors.iter().for_each(|color| {
            let round_value = match round.get(color) {
                Some(&v) => v,
                None => 0,
            };
            if *hm.entry(color).or_insert(0) < round_value {
                hm.insert(color, round_value);
            };
        });

    });
    hm
}

pub fn main() {
    println!("main from day02!");
    let contents = fs::read_to_string("inputs/day02.txt").expect("unable to read file contents");
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in contents.lines().enumerate() {
        let game = parse_line(line);
        if is_game_possible(game.clone()) {
            part1 += i+1
        }
        let min_cubes_required = min_cubes(game);
        let mut game_score = 1;
        for cube_score in min_cubes_required.into_values() {
            game_score *= cube_score
        }
        part2 += game_score;
        println!("{line}");
    }
    println!("part1: {part1}");
    println!("part2: {part2}");
}

