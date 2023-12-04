use std::fs;
use regex::Regex;

// represent a number, storing line and span
#[derive(Debug)]
struct Number {
    value: i32,
    line: i32,
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Symbol {
    value: String,
    line: i32,
    pos: i32,
}

impl Number {
    fn adj_to(&self, s: &Symbol) -> bool {
        s.pos >= (&self.start - 1) && s.pos <= (&self.end + 1) && (&self.line - s.line).abs() <= 1
    }
}

fn parse_line(s: &str, line: i32) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let number_re: &str = r"\d+";
    let symbol_re: &str = r"[^0-9.]";
    let re = Regex::new(number_re).unwrap();
    let it = re.find_iter(s);
    for m in it {
        let number = Number {
            value: m.as_str().parse::<i32>().unwrap(),
            line: line,
            start: m.start() as i32,
            end: (m.end() - 1) as i32,
        };
        numbers.push(number);
    }
    let re = Regex::new(symbol_re).unwrap();
    let it = re.find_iter(s);
    for m in it {
        let symbol = Symbol {
            value: String::from(m.as_str()),
            line: line,
            pos: m.start() as i32,
        };
        symbols.push(symbol);
    }
    (numbers, symbols)
}

fn solve_part1(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> i32 {
    let mut res = 0;
    for number in numbers {
        if symbols.iter().any(|symbol| number.adj_to(symbol)) {
            // println!("adding {:?} to solution", number);
            res += number.value;
        };
        // for symbol in symbols {
        //     if number.adj_to(symbol) { println!("matched {:?} to {:?}", number, symbol); };
        // };
    };
    res
}

fn solve_part2(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> i32 {
    let mut res = 0;
    for symbol in symbols {
        if symbol.value != "*" { continue; };
        let mut i = 0;
        let mut value = 1;
        for number in numbers {
            if number.adj_to(symbol) {
                i += 1;
                value *= number.value;
            };
        };
        if i == 2 { res += value; };
    };
    res
}



pub fn main() {
    println!("main from day03!");
    let contents = fs::read_to_string("inputs/day03.txt").expect("unable to read file contents");
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (i, line) in contents.lines().enumerate() {
        let (mut n, mut s) = parse_line(line, i as i32);
        numbers.append(&mut n);
        symbols.append(&mut s);
    }
    let part1 = solve_part1(&numbers, &symbols);
    let part2 = solve_part2(&numbers, &symbols);
    //println!("{:?}", numbers);
    //println!("{:?}", symbols);
    println!("part1: {part1}");
    println!("part2: {part2}");
}
