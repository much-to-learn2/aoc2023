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

struct Card {
    tickets: Vec<Number>,
    winners: Vec<Number>,
}

impl Card {
    fn new(tickets: Vec<Number>, winners: Vec<Number>) -> Self {
       Self {
           tickets,
           winners,
        }
    }

    fn num_winners(&self) -> i32 {
        self.tickets.iter().fold(0, |acc, ticket| {
            if self.winners.iter().any(|winner| winner.value == ticket.value) {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn score(&self) -> i32 {
        if self.num_winners() == 0 {
            0
        } else {
            2_i32.pow((self.num_winners() - 1) as u32)
        }
    }
}

struct Game {
    cards: Vec<Card>,
    counts: Vec<i32>,
}

impl Game {
    fn new(cards: Vec<Card>) -> Self {
        let counts: Vec<i32> = vec![1; cards.len()];
        Self {
            cards,
            counts,
        }
    }

    // propagate a round of play for card `i`
    fn update_counts(&mut self, i: usize) {
        let score = self.cards[i].num_winners() as usize;
        println!("before round {i} (score={score}): {:?}", self.counts);
        for j in i+1..std::cmp::min(i+1+score, self.cards.len()) {
            self.counts[j] += self.counts[i];
        };
        println!("after round {i}: {:?}", self.counts);
    }

    fn round(&mut self) {
        for i in 0..self.cards.len() {
            self.update_counts(i);
        }
    }

    fn score(&self) -> i32 {
        self.counts.iter().sum()
    }

}


fn parse_line(s: &str, line: i32) -> (Vec<Number>, Vec<Number>) {
    let mut tickets: Vec<Number> = vec![];
    let mut winners: Vec<Number> = vec![];
    let s = &s[2+s.find(": ").unwrap()..];
    let mut rounds = s.split(" | ");
    let number_re: &str = r"\d+";
    let re = Regex::new(number_re).unwrap();
    let it = re.find_iter(rounds.next().unwrap());
    for m in it {
        let number = Number {
            value: m.as_str().parse::<i32>().unwrap(),
            line: line,
            start: m.start() as i32,
            end: (m.end() - 1) as i32,
        };
        winners.push(number);
    }
    let it = re.find_iter(rounds.next().unwrap());
    for m in it {
        let number = Number {
            value: m.as_str().parse::<i32>().unwrap(),
            line: line,
            start: m.start() as i32,
            end: (m.end() - 1) as i32,
        };
        tickets.push(number);
    }
    (winners, tickets)
}

pub fn main() {
    println!("main from day04!");
    let contents = fs::read_to_string("inputs/day04.txt").expect("unable to read file contents");
    let mut part1 = 0;
    let mut cards: Vec<Card> = vec![];
    for (i, line) in contents.lines().enumerate() {
        let (mut w, mut t) = parse_line(line, i as i32);
        let card = Card::new(w, t);
        part1 += card.score();
        cards.push(card);
    }
    let mut game: Game = Game::new(cards);
    game.round();
    let part2 = game.score();
    println!("part1: {part1}");
    println!("{:?}", game.counts);
    println!("part2: {part2}");
}
