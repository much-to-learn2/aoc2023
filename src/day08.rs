use std::fs;
use std::collections::{HashMap, VecDeque};
use regex::Regex;

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(v: &str, l: &str, r: &str) -> Self {
        Self {
            value: v.to_owned(),
            left: l.to_owned(),
            right: r.to_owned(),
        }
    }

    fn from_str(s: &str) -> Self {
        let pattern: &str = r"([a-zA-Z]{3}) = \(([a-zA-Z]{3}), ([a-zA-Z]{3})\)";
        let re = Regex::new(pattern).unwrap();
        let (_, [v, l, r]) = re.captures(s).unwrap().extract();
        Node::new(v, l, r)
    }   
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
    instr: Vec<char>, 
}

impl Graph {
    fn new(instr: &str) -> Self {
        Graph { 
            nodes: HashMap::new(),
            instr: instr.chars().collect(),
        }
    }

    fn push(&mut self, node: Node) {
        self.nodes.insert(node.value.clone(), node);
    }

    fn num_steps(&self) -> i64 {
        let mut curr = "AAA";
        let mut instr = VecDeque::from(self.instr.clone());
        let mut steps = 0;
        while curr != "ZZZ" {
            // println!("curr= {curr}");
            let curr_instr = instr.pop_front().unwrap();
            instr.push_back(curr_instr);
            curr = if curr_instr == 'L' {
                &self.nodes[curr].left
            } else {
                &self.nodes[curr].right 
            };
            steps += 1;
        };
        steps
    }

    fn num_steps_ghost(&self) -> i64 { 
        // TODO: optimize this by finding num steps for each starting position,
        // and then folding with LCM function
        let mut curr_nodes: Vec<&str> = self.nodes.keys()
            .map(|s| s.as_str())
            .filter(|&s| s.as_bytes()[2] as char == 'A')
            .collect();
        println!("curr_nodes: {:?}", curr_nodes);
        // let curr_nodes = vec!["AAA"]; // should get same answer as part 1
        let steps: Vec<i64> = curr_nodes.iter().map(|&curr| {
            let mut instr = VecDeque::from(self.instr.clone());
            let mut steps = 0;
            let mut curr = curr;
            while curr.as_bytes()[2] as char != 'Z' {
                steps += 1;
                let curr_instr = instr.pop_front().unwrap();
                instr.push_back(curr_instr);
                curr = if curr_instr == 'L' {
                    &self.nodes[curr].left
                } else {
                    &self.nodes[curr].right
                }
            };
            steps
        }).collect();
        println!("steps: {:?}", steps);

        steps.iter().fold(1, |acc, curr| {
            lcm(acc, *curr)
        })
    }
}

fn gcd(mut n: i64, mut m: i64) -> i64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(m: i64, n: i64) -> i64 {
    (m * n).abs() / gcd(m, n)
}


pub fn main() {
    println!("main from day08!");
    let contents = fs::read_to_string("inputs/day08.txt").expect("unable to read file contents");
    let mut it = contents.lines();
    let instructions = it.next().unwrap();
    it.next();
    let mut g = Graph::new(instructions);
    for line in it {
        let node = Node::from_str(line);
        g.push(node);
    };
    let part1 = g.num_steps();
    let part2 = g.num_steps_ghost();

    println!("part1: {part1}");
    println!("part2: {part2}");
}
