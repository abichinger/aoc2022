#[derive(Eq, PartialEq, Clone, Copy)]
enum Symbol {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Symbol {
    pub fn from_char(s: &str) -> Symbol {
        match s {
            "A" => Symbol::ROCK,
            "B" => Symbol::PAPER,
            _ => Symbol::SCISSORS, 
        }
    }

    pub fn from_outcome(s: &str, opponent: &Symbol) -> Symbol {
        match s {
            "Y" => opponent.clone(), //draw
            "X" => Symbol::from_score(opponent.score() - 1),//lose
            _ => Symbol::from_score(opponent.score() + 1) //win
        }
    }

    pub fn from_score(score: i32) -> Symbol {

        let i = (score - 1) % 3 + 1;

        match i {
            1 => Symbol::ROCK,
            2 => Symbol::PAPER,
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
        let me = Symbol::from_outcome(symbols.get(1).unwrap(), &opponent);
        score += me.against(opponent);
        score += me.score();
    }
    print!("{}", score);
}

