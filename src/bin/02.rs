use std::{path::Path, str::FromStr};

#[derive(Copy, Clone, Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let command_type = split.next().unwrap();
        let command_value = split.next().unwrap().parse().unwrap();
        match command_type {
            "forward" => Ok(Self::Forward(command_value)),
            "up" => Ok(Self::Up(command_value)),
            "down" => Ok(Self::Down(command_value)),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Submarine {
    horizontal_pos: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn exec_one(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.horizontal_pos += x,
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x,
        }
    }

    fn exec_two(&mut self, command: Command) {
        match command {
            Command::Forward(x) => {
                self.horizontal_pos += x;
                self.depth += self.aim * x
            }
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
        }
    }

    fn multiply_hd(&self) -> usize {
        self.horizontal_pos * self.depth
    }

    fn reset(&mut self) {
        self.horizontal_pos = 0;
        self.aim = 0;
        self.depth = 0;
    }
}

fn main() {
    let commands = get_commands("input/02.txt");
    let mut submarine = Submarine::default();

    commands
        .iter()
        .for_each(|&command| submarine.exec_one(command));
    println!("part 1: {}", submarine.multiply_hd());

    submarine.reset();

    commands
        .iter()
        .for_each(|&command| submarine.exec_two(command));
    println!("part 2: {}", submarine.multiply_hd());
}

fn get_commands(p: impl AsRef<Path>) -> Vec<Command> {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    text.lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect()
}

#[cfg(test)]
mod problem02 {
    use super::*;

    #[test]
    fn part1() {
        let test_commands = get_commands("input/02.test.txt");
        let mut submarine = Submarine::default();
        test_commands
            .iter()
            .for_each(|&command| submarine.exec_one(command));
        assert_eq!(submarine.multiply_hd(), 150);
    }

    #[test]
    fn part2() {
        let test_commands = get_commands("input/02.test.txt");
        let mut submarine = Submarine::default();
        test_commands
            .iter()
            .for_each(|&command| submarine.exec_two(command));
        assert_eq!(submarine.multiply_hd(), 900);
    }
}
