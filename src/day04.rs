use std::fs;
use regex::Regex;

struct Card {
    tickets: Vec<i32>,
    winners: Vec<i32>,
}

impl Card {
    fn new(tickets: Vec<i32>, winners: Vec<i32>) -> Self {
       Self {
           tickets,
           winners,
        }
    }

    fn num_winners(&self) -> i32 {
        self.tickets.iter().fold(0, |acc, ticket| {
            if self.winners.iter().any(|winner| winner == ticket) {
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

fn parse_line(s: &str) -> Card {
    let mut tickets: Vec<i32> = vec![];
    let mut winners: Vec<i32> = vec![];
    let s = &s[2+s.find(": ").unwrap()..];
    let mut rounds = s.split(" | ");
    let number_re: &str = r"\d+";
    let re = Regex::new(number_re).unwrap();
    let it = re.find_iter(rounds.next().unwrap());
    for m in it {
        winners.push(m.as_str().parse::<i32>().unwrap());
    }
    let it = re.find_iter(rounds.next().unwrap());
    for m in it {
        tickets.push(m.as_str().parse::<i32>().unwrap());
    }
    Card::new(winners, tickets)
}

pub fn main() {
    println!("main from day04!");
    let contents = fs::read_to_string("inputs/day04.txt").expect("unable to read file contents");
    let mut part1 = 0;
    let mut cards: Vec<Card> = vec![];
    for line in contents.lines() {
        let card = parse_line(line);
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
