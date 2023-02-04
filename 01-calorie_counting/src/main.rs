use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Elf {
    calories: i64,
}

fn main() {
    println!("Best elf has {} calories", best_elf().calories);
    let three = best_three();
    println!("Best elves are {:?}", three);
    let sum = {
        let mut sum = 0;
        for elf in three {
            sum += elf.calories
        }
        sum
    };
    println!("Best elves summed {}", sum);
}

fn best_elf() -> Elf {
    let file = File::open("./input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut elf = Elf { calories: 0 };
    let mut best_elf = Elf { calories: 0 };
    for line in lines {
        match line.unwrap().parse::<i64>() {
            Ok(y) => {
                elf.calories += y;
            }
            Err(_) => {
                if elf.calories > best_elf.calories {
                    best_elf.calories = elf.calories;
                }
                elf.calories = 0;
            }
        }
    }

    if elf.calories > best_elf.calories {
        best_elf.calories = elf.calories;
        elf.calories = 0;
    }

    best_elf
}

fn best_three() -> Vec<Elf> {
    let file = File::open("./input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut elf = Elf { calories: 0 };
    let mut best_elf = vec![
        Elf { calories: 0 },
        Elf { calories: 0 },
        Elf { calories: 0 },
    ];
    for line in lines {
        match line.unwrap().parse::<i64>() {
            Ok(y) => {
                elf.calories += y;
            }
            Err(_) => {
                if *best_elf.first().unwrap() < elf {
                    best_elf[0].calories = elf.calories;
                }
                best_elf.sort();
                elf.calories = 0;
            }
        }
    }

    if *best_elf.first().unwrap() < elf {
        best_elf[0].calories = elf.calories;
    }
    best_elf.sort();

    best_elf
}
