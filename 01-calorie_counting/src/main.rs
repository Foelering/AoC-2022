use std::fs::File;
use std::io::{BufRead, BufReader};
struct Elf {
    calories: i64,
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut elf = Elf { calories: 0 };
    let mut best_elf = Elf { calories: 0 };
    for line in lines {
        match line.unwrap().parse::<i64>() {
            Ok(y) => {
                elf.calories += y;
            }
            Err(err) => {
                if elf.calories > best_elf.calories {
                    best_elf.calories = elf.calories;
                }
                elf.calories = 0;
                println!("Done elf {:?}", err);
            }
        }
    }

    if elf.calories > best_elf.calories {
        best_elf.calories = elf.calories;
        elf.calories = 0;
    }

    println!("Best elf has {} calories", best_elf.calories)
}
