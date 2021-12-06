use std::{path::Path, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Fish {
    /// index corresponds to timer and value corresponds to numbers of fish with that timer
    timers: [usize; 9],
}

impl FromStr for Fish {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timers = [0; 9];
        s.split(',').map(|n| n.parse().unwrap()).for_each(|t: u8| {
            timers[t as usize] += 1;
        });
        Ok(Self { timers })
    }
}

impl Fish {
    const RESET: usize = 6;

    fn next(&mut self) {
        let spawners = self.timers[0];
        self.timers.rotate_left(1);
        self.timers[Self::RESET] += spawners;
    }

    fn next_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    fn count(&self) -> usize {
        self.timers.iter().sum()
    }
}

fn main() {
    let mut fish = get_fish("input/06.txt");
    fish.next_n(80);
    println!("part 1: {}", fish.count());
    fish.next_n(256 - 80);
    println!("part 2: {}", fish.count());
}

fn get_fish(p: impl AsRef<Path>) -> Fish {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    text.parse().unwrap()
}

#[cfg(test)]
mod problem06 {
    use super::*;

    #[test]
    fn next() {
        let mut fish = get_fish("input/06.test.txt");
        fish.next();
        assert_eq!(fish, "2,3,2,0,1".parse().unwrap());
        fish.next();
        assert_eq!(fish, "1,2,1,6,0,8".parse().unwrap());
    }

    #[test]
    fn part1() {
        let mut fish = get_fish("input/06.test.txt");
        fish.next_n(80);
        assert_eq!(fish.count(), 5934);
    }

    #[test]
    fn part2() {
        let mut fish = get_fish("input/06.test.txt");
        fish.next_n(80);
        fish.next_n(256 - 80);
        assert_eq!(fish.count(), 26984457539);
    }
}
