use std::fs::File;
use std::io::{BufRead, BufReader};

struct Backpack {
    first: u64,
    second: u64
}
#[derive(Debug)]
enum ParsingError {
    WrongLength,
    NonAscii
}

impl TryFrom<String> for Backpack {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() % 2 != 0 {
            return Err(ParsingError::WrongLength);
        }

        let half = value.len() / 2;
        let mut left = 0;
        let mut right = 0;
        for char in value.char_indices() {
            if !char.1.is_ascii_alphabetic() { return Err(ParsingError::NonAscii); }
            let val: u32 = match char.1.is_ascii_uppercase() {
                true => char.1.to_digit(36).unwrap_or_default() as u32 + 17,
                false => char.1.to_digit(36).unwrap_or_default() as u32 - 9
            };

            if char.0 < half {
                left |= u64::pow(2, val);
            } else {
                right |= u64::pow(2, val);
            }
        }

        return Ok(Backpack {
            first: left,
            second: right
        })
    }

    type Error = ParsingError;
}

fn main() {

    let file = File::open("./input").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in lines {

        match Backpack::try_from(line.unwrap()) {
            Ok(b) => sum += (b.first & b.second).ilog2(),
            Err(x) => println!("Error: {:?}", x)
        }
    }

    println!("Sum {}", sum);
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::*;
    
    #[test]
    fn test_on_known_imput () {

        let file = File::open("./test").unwrap();
        let lines = BufReader::new(file).lines();
        let mut sum: i64 = 0;
        let results: Vec<u64> = vec!(16, 38, 42, 22, 20, 19);
        for line in lines.enumerate() {
    
            match Backpack::try_from(line.1.unwrap()) {
                Ok(b) => {
                    let position = (b.first & b.second).ilog2();
                    sum += position as i64;
                    assert_eq!(position as u64, results[line.0], "I got {} instead of {}", position, results[line.0]);
                }
                Err(x) => println!("Error: {:?}", x)
            }

        }
    
        println!("Sum {}", sum);

        assert_eq!(157, sum, "Sum is {}, should be 157", sum)

    }
}
