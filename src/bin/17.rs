use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Probe {
    velocity: (i32, i32),
    position: (i32, i32),
}

impl Probe {
    fn new(initial_velocity: (i32, i32), start_pos: (i32, i32)) -> Self {
        Self {
            velocity: initial_velocity,
            position: start_pos,
        }
    }

    fn step(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        self.velocity.1 -= 1;
        self.velocity.0 = match self.velocity.0 {
            x if x == 0 => 0,
            x if x > 0 => x - 1,
            x if x < 0 => x + 1,
            _ => unreachable!(),
        }
    }

    fn can_probably_hit(
        &self,
        target_x: &RangeInclusive<i32>,
        target_y: &RangeInclusive<i32>,
    ) -> bool {
        self.position.0 < *target_x.end() && self.position.1 > *target_y.start()
    }

    fn hits(&self, target_x: &RangeInclusive<i32>, target_y: &RangeInclusive<i32>) -> bool {
        target_x.contains(&self.position.0) && target_y.contains(&self.position.1)
    }
}

type HighestY = i32;

struct Launcher {
    target_x: RangeInclusive<i32>,
    target_y: RangeInclusive<i32>,
}

impl Launcher {
    fn launch(&self, probe: &mut Probe) -> Option<HighestY> {
        let mut highest_y = probe.position.1;
        while probe.can_probably_hit(&self.target_x, &self.target_y) {
            probe.step();
            highest_y = std::cmp::max(probe.position.1, highest_y);
            if probe.hits(&self.target_x, &self.target_y) {
                return Some(highest_y);
            }
        }
        None
    }
}

fn main() {
    let launcher = Launcher {
        target_x: 25..=67,
        target_y: -260..=-200,
    };

    let mut highest_ys = Vec::new();

    // I just tweaked the for-loop ranges until the solution is accepted
    // TODO: Find a way to reliably determine the search space for velocity
    for vx in 1..300 {
        for vy in -300..300 {
            let mut probe = Probe::new((vx, vy), (0, 0));
            if let Some(hy) = launcher.launch(&mut probe) {
                highest_ys.push(hy);
            }
        }
    }

    println!("part 1: {}", highest_ys.iter().max().unwrap());
    println!("part 2: {}", highest_ys.len());
}

#[cfg(test)]
mod problem17 {
    use super::*;

    #[test]
    fn part1() {
        let launcher = Launcher {
            target_x: 20..=30,
            target_y: -10..=-5,
        };

        let mut highest_ys = Vec::new();

        for vx in 1..30 {
            for vy in -10..30 {
                let mut probe = Probe::new((vx, vy), (0, 0));
                if let Some(hy) = launcher.launch(&mut probe) {
                    highest_ys.push(hy);
                }
            }
        }

        assert_eq!(highest_ys.iter().max(), Some(&45));
    }
}
