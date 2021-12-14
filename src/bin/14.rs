use std::{collections::HashMap, path::Path, str::FromStr};

use itertools::Itertools;

type Rules = HashMap<(char, char), char>;

struct Polymer {
    pairwise: HashMap<(char, char), usize>,
    composition: HashMap<char, usize>,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairwise = s
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, pair| {
                *acc.entry(pair).or_insert(0) += 1;
                acc
            });
        let composition = s
            .chars()
            .into_group_map_by(|c| *c)
            .into_iter()
            .map(|(c, v)| (c, v.len()))
            .collect();

        Ok(Self {
            pairwise,
            composition,
        })
    }
}

impl Polymer {
    fn max_minus_min(&self) -> usize {
        let max = self.composition.values().max().unwrap();
        let min = self.composition.values().min().unwrap();
        max - min
    }

    fn step(&self, rules: &Rules) -> Polymer {
        let mut new_pairwise = HashMap::new();
        let mut new_composition = self.composition.clone();
        self.pairwise.iter().for_each(|(&pair, freq)| {
            if let Some(&insert) = rules.get(&pair) {
                *new_composition.entry(insert).or_insert(0) += freq;
                *new_pairwise.entry((pair.0, insert)).or_insert(0) += freq;
                *new_pairwise.entry((insert, pair.1)).or_insert(0) += freq;
            }
        });
        Self {
            pairwise: new_pairwise,
            composition: new_composition,
        }
    }
}

fn main() {
    let (mut poly, rules) = get_poly_rules("input/14.txt");
    for _ in 0..10 {
        poly = poly.step(&rules);
    }
    println!("part 1: {}", poly.max_minus_min());
    for _ in 10..40 {
        poly = poly.step(&rules);
    }
    println!("part 2: {}", poly.max_minus_min());
}

fn get_poly_rules(path: impl AsRef<Path>) -> (Polymer, Rules) {
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    let poly = lines.next().unwrap().parse().unwrap();
    lines.next();
    let rules = lines
        .by_ref()
        .map(|line| {
            let (pattern, insertion) = line.split_once(" -> ").unwrap();
            let left = pattern.chars().next().unwrap();
            let right = pattern.chars().nth(1).unwrap();
            let insert = insertion.chars().next().unwrap();
            ((left, right), insert)
        })
        .collect();
    (poly, rules)
}

#[cfg(test)]
mod problem14 {
    use super::*;

    #[test]
    fn polymerization() {
        let (mut poly, rules) = get_poly_rules("input/14.test.txt");
        for _ in 0..10 {
            poly = poly.step(&rules);
        }
        assert_eq!(1588, poly.max_minus_min());
        for _ in 10..40 {
            poly = poly.step(&rules);
        }
        assert_eq!(2188189693529, poly.max_minus_min());
    }
}
