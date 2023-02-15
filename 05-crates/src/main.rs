struct Stack {
    crates: Vec<char>,
    index: String
}

impl Stack {
    fn push(&mut self, elem: char) {
        self.crates.push(elem);
    }

    fn pop(&mut self)-> Option<char> {
        self.crates.pop()
    }
}

enum Error {
    unknown_index(String),
    duplicated_indexes,
    empty_stack
}

struct Dockyard {
    stacks: Vec<Stack>
}

impl Dockyard {

    fn has (&self, index: &String) -> bool {
        let ss = &self.stacks[..];
        for stack in ss {
            if stack.index == *index {
                return true;
            }
        }
        return false;
    }

    fn get(&self, index: &String) -> Option<&Stack> {
        for stack in &self.stacks[..] {
            if stack.index == *index {
                return Some(stack);
            }
        }
        return None;
    }

    fn try_move(&mut self, from: &String, to: &String) -> Result<(), Error> {
        match (self.get(from), self.get(to)) {
            (Some(x), Some(y)) => {
                if let Some(elem) = x.pop() {
                    y.push(elem);
                    Ok(())
                } else {
                    Err(Error::empty_stack)
                }
            },
            (None, _) => Err(Error::unknown_index(from.clone())),
            (_, None) => Err(Error::unknown_index(to.clone())),
        }
    }
}

impl TryFrom<Vec<Stack>> for Dockyard {
    type Error = Error;

    fn try_from(value: Vec<Stack>) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(tests)]
mod tests {

    #[test]
    fn try_move() {

    }
}
