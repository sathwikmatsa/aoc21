///  aaaa    Assign unique value to each segment.
/// b    c   a = 2 b = 6 c = 3 d = 6
/// b    c   e = 7 f = 4 g = 5
///  dddd
/// e    f   E segments lits up 4 times in all 10 digits
/// e    f   similary B 6 times F 9 times.
///  gggg    Find E, B, F right mappings by counting occurences
///
/// For showing 1, it uses 2 segments (CF), similarly for 7 -> 3 segs, for 4 -> 4, 8 -> 7
/// start with smallest and solve for the unknown for eg: for 1, i.e., find signal with len of 2
/// correlate to CF (original mapping), we already know mapping for F
/// we find C by solving eq c * 4 (value assigned to f) = 3 * 4 (multiply values of segments for 1)
/// repeat the process for 7, 4, 8 and all mappings will be known
#[macro_use]
extern crate lazy_static;
use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

lazy_static! {
    static ref SIGNAL_MAP: HashMap<BTreeSet<usize>, usize> = {
        let mut m = HashMap::new();
        m.insert(BTreeSet::from([3, 4]), 1);
        m.insert(BTreeSet::from([2, 3, 6, 7, 5]), 2);
        m.insert(BTreeSet::from([2, 3, 6, 4, 5]), 3);
        m.insert(BTreeSet::from([8, 6, 3, 4]), 4);
        m.insert(BTreeSet::from([2, 8, 6, 4, 5]), 5);
        m.insert(BTreeSet::from([2, 8, 7, 5, 4, 6]), 6);
        m.insert(BTreeSet::from([2, 3, 4]), 7);
        m.insert(BTreeSet::from([2, 3, 4, 5, 6, 7, 8]), 8);
        m.insert(BTreeSet::from([6, 8, 2, 3, 4, 5]), 9);
        m.insert(BTreeSet::from([2, 3, 4, 5, 7, 8]), 0);
        m
    };
}

const E_REPS: usize = 4;
const B_REPS: usize = 6;
const F_REPS: usize = 9;
const E_UNIQV: usize = 7;
const B_UNIQV: usize = 8;
const F_UNIQV: usize = 4;

#[derive(Debug, Clone)]
struct SegmentDisplay {
    uniq: HashMap<char, usize>,
}

impl SegmentDisplay {
    fn deduce_from(patterns: &str) -> Self {
        let occurences = patterns.split(' ').fold(HashMap::new(), |mut map, p| {
            p.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
            map
        });

        let mut uniq = HashMap::new();
        for (key, value) in occurences {
            match value {
                E_REPS => uniq.insert(key, E_UNIQV),
                B_REPS => uniq.insert(key, B_UNIQV),
                F_REPS => uniq.insert(key, F_UNIQV),
                _ => None,
            };
        }

        let mut disp = Self { uniq };

        let one = patterns.split(' ').find(|p| p.len() == 2).unwrap();
        Self::solve_single_unknown(&mut disp, one, 3 * 4);
        let seven = patterns.split(' ').find(|p| p.len() == 3).unwrap();
        Self::solve_single_unknown(&mut disp, seven, 2 * 3 * 4);
        let four = patterns.split(' ').find(|p| p.len() == 4).unwrap();
        Self::solve_single_unknown(&mut disp, four, 8 * 6 * 3 * 4);
        let eight = patterns.split(' ').find(|p| p.len() == 7).unwrap();
        Self::solve_single_unknown(&mut disp, eight, 8 * 2 * 3 * 7 * 6 * 4 * 5);

        disp
    }

    fn solve_single_unknown(&mut self, pattern: &str, pattern_val: usize) {
        let mut unknown_val = pattern_val;
        pattern.chars().for_each(|c| {
            if let Some(val) = self.uniq.get(&c) {
                unknown_val /= val;
            }
        });
        let unknown = pattern
            .chars()
            .find(|c| self.uniq.get(c).is_none())
            .unwrap();
        self.uniq.insert(unknown, unknown_val);
    }

    fn signal(&self, signal: &str) -> String {
        signal
            .split_ascii_whitespace()
            .map(|sigd| -> String {
                let input = sigd
                    .chars()
                    .map(|c| *self.uniq.get(&c).unwrap())
                    .collect::<BTreeSet<_>>();
                format!("{}", *SIGNAL_MAP.get(&input).unwrap())
            })
            .collect()
    }
}
fn main() {
    let signals = get_patterns_output("input/08.txt");
    println!("part 1: {}", count_easy_digits(&signals));
    println!("part 2: {}", output_sum(&signals));
}

fn get_patterns_output(p: impl AsRef<Path>) -> Vec<(String, String)> {
    std::fs::read_to_string(p)
        .unwrap()
        .lines()
        .map(|line| {
            let mut input = line.split('|');
            let patterns = input.next().unwrap().trim().to_owned();
            let output = input.next().unwrap().trim().to_owned();
            (patterns, output)
        })
        .collect()
}

fn output_sum(input: &[(String, String)]) -> usize {
    input
        .iter()
        .map(|(patterns, output)| {
            let disp = SegmentDisplay::deduce_from(patterns);
            disp.signal(output).parse::<usize>().unwrap()
        })
        .sum()
}

fn count_easy_digits(input: &[(String, String)]) -> usize {
    input
        .iter()
        .map(|(patterns, output)| {
            let disp = SegmentDisplay::deduce_from(patterns);
            disp.signal(output)
                .chars()
                .filter(|c| ['1', '4', '7', '8'].contains(c))
                .collect::<String>()
        })
        .collect::<String>()
        .len()
}

#[cfg(test)]
mod problem08 {
    use super::*;

    #[test]
    fn part1() {
        let signals = get_patterns_output("input/08.test.txt");
        assert_eq!(count_easy_digits(&signals), 26);
    }

    #[test]
    fn part2() {
        let display = SegmentDisplay::deduce_from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
        );
        assert_eq!(display.signal("cdfeb fcadb cdfeb cdbaf"), "5353");
    }
}
