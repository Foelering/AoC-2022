use std::fs::File;
use std::io::{BufRead, BufReader};

enum Hand {
    Rock,
    Paper,
    Scissors
}

struct Game {
    other: Hand,
    me: Hand
}

enum GameError {
    Parsing
}

impl TryFrom<String> for Game {
    type Error = GameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        /*
        match value.as_str() {
            "A X" => Ok(Game { other: Hand::Rock, me: Hand::Rock}),
            "A Y" => Ok(Game { other: Hand::Rock, me: Hand::Paper}),
            "A Z" => Ok(Game { other: Hand::Rock, me: Hand::Scissors}),
            "B X" => Ok(Game { other: Hand::Paper, me: Hand::Rock}),
            "B Y" => Ok(Game { other: Hand::Paper, me: Hand::Paper}),
            "B Z" => Ok(Game { other: Hand::Paper, me: Hand::Scissors}),
            "C X" => Ok(Game { other: Hand::Scissors, me: Hand::Rock}),
            "C Y" => Ok(Game { other: Hand::Scissors, me: Hand::Paper}),
            "C Z" => Ok(Game { other: Hand::Scissors, me: Hand::Scissors}),
            _ => Err(GameError::Parsing)
        }
        */

        match value.as_str() {
            "A X" => Ok(Game { other: Hand::Rock, me: Hand::Scissors}),
            "A Y" => Ok(Game { other: Hand::Rock, me: Hand::Rock}),
            "A Z" => Ok(Game { other: Hand::Rock, me: Hand::Paper}),
            "B X" => Ok(Game { other: Hand::Paper, me: Hand::Rock}),
            "B Y" => Ok(Game { other: Hand::Paper, me: Hand::Paper}),
            "B Z" => Ok(Game { other: Hand::Paper, me: Hand::Scissors}),
            "C X" => Ok(Game { other: Hand::Scissors, me: Hand::Paper}),
            "C Y" => Ok(Game { other: Hand::Scissors, me: Hand::Scissors}),
            "C Z" => Ok(Game { other: Hand::Scissors, me: Hand::Rock}),
            _ => Err(GameError::Parsing)
        }
    }
}

impl From<Game> for i64 {
    
    fn from(game: Game) -> i64 {
        
        match (game.me, game.other) {
            (Hand::Rock, Hand::Rock) => 1 + 3,
            (Hand::Rock, Hand::Scissors) => 1 + 6,
            (Hand::Rock, Hand::Paper) => 1 + 0,
            (Hand::Scissors, Hand::Rock) => 3 + 0,
            (Hand::Scissors, Hand::Scissors) => 3 + 3,
            (Hand::Scissors, Hand::Paper) => 3 + 6,
            (Hand::Paper, Hand::Rock) => 2 + 6,
            (Hand::Paper, Hand::Scissors) => 2 + 0,
            (Hand::Paper, Hand::Paper) => 2 + 3
        }
    }
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = BufReader::new(file).lines();
    let mut res:u64 = 0;
    for line in lines {
        let decoded = line.unwrap();
        match Game::try_from(decoded.clone()) {
            Ok(game) => res += i64::from(game) as u64,
            Err(_) => println!("Error! {}", decoded),
        }
        println!("{} {}", decoded, res)
    }
    println!("Total points: {}", res);
}
