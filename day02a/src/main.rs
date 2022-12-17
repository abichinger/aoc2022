#[derive(Eq, PartialEq)]
enum Symbol {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Symbol {
    pub fn from_char(s: &str) -> Symbol {
        match s {
            "A" | "X" => Symbol::ROCK,
            "B" | "Y" => Symbol::PAPER,
            _ => Symbol::SCISSORS, 
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Symbol::ROCK => 1,
            Symbol::PAPER => 2,
            Symbol::SCISSORS => 3,
        }
    }

    pub fn against(&self, other: Symbol) -> i32 {
        if *self == other {
            return 3; //draw
        }
        if self.score() == other.score() - 1 || self.score()%3 == other.score() - 1 {
            return 0; //lost
        }
        return 6; //won
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut score = 0;
    for line in input.lines() {
        let symbols = line.split(" ").collect::<Vec<&str>>();
        let opponent = Symbol::from_char(symbols.get(0).unwrap());
        let me = Symbol::from_char(symbols.get(1).unwrap());
        score += me.against(opponent);
        score += me.score();
    }
    print!("{}", score);
}


