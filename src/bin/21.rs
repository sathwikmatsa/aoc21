use std::collections::HashMap;

const BOARD_START_POS: usize = 1;
const BOARD_LAST_POS: usize = 10;

trait Die {
    fn roll(&mut self) -> usize;
    fn roll_thrice(&mut self) -> [usize; 3];
    fn rolls(&self) -> usize;
}

#[derive(Clone, Copy, Debug)]
struct DeterministicDie {
    start: usize,
    wrap_back_after: usize,
    current: usize,
    rolls: usize,
}

impl DeterministicDie {
    fn new(start: usize, wrap_back_after: usize) -> Self {
        Self {
            start,
            wrap_back_after,
            current: start - 1,
            rolls: 0,
        }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        self.current =
            self.start + (self.current + 1 - self.start) % (self.wrap_back_after + 1 - self.start);
        self.rolls += 1;
        self.current
    }

    fn roll_thrice(&mut self) -> [usize; 3] {
        [self.roll(), self.roll(), self.roll()]
    }

    fn rolls(&self) -> usize {
        self.rolls
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Pawn {
    score: usize,
    pos: usize,
}

impl Pawn {
    fn new(start: usize) -> Self {
        Self {
            score: 0,
            pos: start,
        }
    }

    fn forward(&mut self, times: usize) {
        self.pos = BOARD_START_POS
            + (self.pos + times - BOARD_START_POS) % (BOARD_LAST_POS + 1 - BOARD_START_POS);
        self.score += self.pos;
    }
}

fn game<'a>(p1: &'a mut Pawn, p2: &'a mut Pawn, die: &mut impl Die) -> (&'a Pawn, &'a Pawn) {
    if p1.score < 1000 && p2.score < 1000 {
        loop {
            let times = die.roll_thrice().iter().sum();
            p1.forward(times);
            if p1.score >= 1000 {
                break;
            }
            let times = die.roll_thrice().iter().sum();
            p2.forward(times);
            if p2.score >= 1000 {
                break;
            }
        }
    }

    match p1.score >= 1000 {
        true => (p1, p2),
        false => (p2, p1),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Chance {
    P1,
    P2,
}

fn dirac_game(p1: Pawn, p2: Pawn) -> [usize; 2] {
    fn play(
        p1: Pawn,
        p2: Pawn,
        chance: Chance,
        cache: &mut HashMap<(Pawn, Pawn, Chance), [usize; 2]>,
    ) -> [usize; 2] {
        if let Some(out) = cache.get(&(p1, p2, chance)) {
            return out.to_owned();
        }
        let res = {
            if p1.score >= 21 {
                [1, 0]
            } else if p2.score >= 21 {
                [0, 1]
            } else {
                let mut wins = [0, 0];
                for i in [1, 2, 3] {
                    for j in [1, 2, 3] {
                        for k in [1, 2, 3] {
                            let this_wins = match chance {
                                Chance::P1 => {
                                    let mut p1_prime = p1;
                                    p1_prime.forward(i + j + k);
                                    play(p1_prime, p2, Chance::P2, cache)
                                }
                                Chance::P2 => {
                                    let mut p2_prime = p2;
                                    p2_prime.forward(i + j + k);
                                    play(p1, p2_prime, Chance::P1, cache)
                                }
                            };
                            wins[0] += this_wins[0];
                            wins[1] += this_wins[1];
                        }
                    }
                }
                wins
            }
        };
        cache.insert((p1, p2, chance), res);
        res
    }

    play(p1, p2, Chance::P1, &mut HashMap::new())
}

fn main() {
    let mut die = DeterministicDie::new(1, 100);
    let mut pawn1 = Pawn::new(7);
    let mut pawn2 = Pawn::new(2);
    let (_won, lost) = game(&mut pawn1, &mut pawn2, &mut die);
    println!("part 1: {}", lost.score * die.rolls());
    let [p1_wins, p2_wins] = dirac_game(Pawn::new(7), Pawn::new(2));
    println!("part 2: {}", std::cmp::max(p1_wins, p2_wins));
}

#[cfg(test)]
mod problem21 {
    use super::*;

    #[test]
    fn pawn() {
        let mut pawn = Pawn::new(7);
        pawn.forward(5);
        assert_eq!(pawn.score, 2);
        assert_eq!(pawn.pos, 2);
    }

    #[test]
    fn deterministic_die() {
        let mut die = DeterministicDie::new(1, 10);
        assert_eq!(die.roll_thrice(), [1, 2, 3]);
        assert_eq!(die.roll_thrice(), [4, 5, 6]);
        assert_eq!(die.roll_thrice(), [7, 8, 9]);
        assert_eq!(die.roll_thrice(), [10, 1, 2]);
    }

    #[test]
    fn part1() {
        let mut die = DeterministicDie::new(1, 100);
        let mut pawn1 = Pawn::new(4);
        let mut pawn2 = Pawn::new(8);
        let (won, lost) = game(&mut pawn1, &mut pawn2, &mut die);
        assert_eq!(won.score, 1000);
        assert_eq!(lost.score, 745);
        assert_eq!(die.rolls(), 993);
    }

    #[test]
    fn part2() {
        let [p1_wins, p2_wins] = dirac_game(Pawn::new(4), Pawn::new(8));
        assert_eq!(p1_wins, 444356092776315);
        assert_eq!(p2_wins, 341960390180808);
    }
}
