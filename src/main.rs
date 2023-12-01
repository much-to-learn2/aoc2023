use std::env;

pub fn main() {
     let args: Vec<String> = env::args().collect();
     let day = match args[1].parse::<u32>() {
         Ok(n) => n,
         Err(_) => {panic!("enter a valid day number")}
     };
     match day {
         1 => { aoc2023::day01::main() },
         _ => { println!("day number {day} not solved") },
     };
}
