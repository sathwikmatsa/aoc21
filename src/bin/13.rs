use std::{collections::HashSet, path::Path, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('=').unwrap();
        let val = right.parse().unwrap();
        Ok(match left.contains('x') {
            true => Fold::X(val),
            false => Fold::Y(val),
        })
    }
}

impl Fold {
    fn fold(&self, points: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        points
            .iter()
            .map(|point| {
                let mut tpoint = point.to_owned();
                match self {
                    Self::X(x) => {
                        if tpoint.0 > *x {
                            tpoint.0 = x - (tpoint.0 - x);
                        }
                    }
                    Self::Y(y) => {
                        if tpoint.1 > *y {
                            tpoint.1 = y - (tpoint.1 - y);
                        }
                    }
                };
                tpoint
            })
            .collect()
    }
}

fn main() {
    let (dots, folds) = load_thermal_imaging("input/13.txt");
    let visible_dots = folds.first().unwrap().fold(&dots).len();
    println!("part 1: {}", visible_dots);
    let code = folds.iter().fold(dots, |acc, f| f.fold(&acc));
    println!("part 2: ");
    print_map(&code);
}

fn print_map(dots: &HashSet<(usize, usize)>) {
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();

    for j in 0..=max_y {
        for i in 0..=max_x {
            print!(
                "{}",
                match dots.contains(&(i, j)) {
                    true => "#",
                    false => ".",
                },
            );
        }
        println!();
    }
    println!();
}

fn load_thermal_imaging(path: impl AsRef<Path>) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    let coords = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    let folds = lines.map(|line| line.parse().unwrap()).collect();
    (coords, folds)
}

#[cfg(test)]
mod problem13 {
    use super::*;

    #[test]
    fn part1() {
        let (dots, folds) = load_thermal_imaging("input/13.test.txt");
        let visible_dots = folds.first().unwrap().fold(&dots).len();
        assert_eq!(17, visible_dots);
    }
}
