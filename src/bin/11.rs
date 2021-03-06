use std::{collections::HashSet, path::Path};

type EnergyLevel = u32;
type OctopusCavern = Vec<Vec<EnergyLevel>>;

trait Energy {
    fn increase(&mut self);
    fn reset(&mut self);
    fn is_flashing(&self) -> bool;
}

impl Energy for EnergyLevel {
    fn increase(&mut self) {
        *self += 1;
    }
    fn reset(&mut self) {
        *self = 0;
    }
    fn is_flashing(&self) -> bool {
        *self > 9u32
    }
}

trait Cavern {
    fn from_str(s: &str) -> Self;
    fn debug_print(&self);
}

impl Cavern for OctopusCavern {
    fn from_str(s: &str) -> Self {
        s.lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
    fn debug_print(&self) {
        for row in self {
            row.iter().for_each(|c| match c {
                0 => print!("\x1b[93m{}\x1b[0m", c),
                _ => print!("{}", c),
            });
            println!();
        }
        println!();
    }
}

fn find_fresh_flashing_octopus(
    cavern: &[Vec<u32>],
    processed: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    cavern
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_dix, e)| ((row_idx, col_dix), e))
        })
        .flatten()
        .find(|(pos, e)| e.is_flashing() && !processed.contains(pos))
        .map(|(pos, _)| pos)
}

fn increase_neighbour_energy_level(pos: (usize, usize), cavern: &mut OctopusCavern) {
    const NEIGHBOURS: [[i32; 2]; 8] = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1],
        [-1, 1],
        [-1, -1],
        [1, 1],
        [1, -1],
    ];
    let n_rows = cavern.len() as i32;
    let n_cols = cavern[0].len() as i32;

    NEIGHBOURS
        .iter()
        .filter_map(|[r, c]| {
            let row = pos.0 as i32 + r;
            let col = pos.1 as i32 + c;
            match (0..n_rows).contains(&row) && (0..n_cols).contains(&col) {
                true => Some((row as usize, col as usize)),
                _ => None,
            }
        })
        .for_each(|(row, col)| {
            cavern[row][col] += 1;
        });
}

fn step(cavern: &mut OctopusCavern) -> usize {
    let mut flashes = 0;
    cavern.iter_mut().flatten().for_each(|level| *level += 1);
    let mut processed_flashes = HashSet::new();
    while let Some((row, col)) = find_fresh_flashing_octopus(cavern, &processed_flashes) {
        increase_neighbour_energy_level((row, col), cavern);
        processed_flashes.insert((row, col));
        flashes += 1;
    }
    cavern.iter_mut().flatten().for_each(|level| {
        if level.is_flashing() {
            level.reset();
        }
    });
    flashes
}

fn all_flash(cavern: &mut OctopusCavern) -> usize {
    let mut steps = 0;
    let n_octopuses = cavern.iter().flatten().count();
    while n_octopuses != step(cavern) {
        steps += 1;
    }
    steps + 1
}

fn main() {
    let mut cavern = cavern_levels("input/11.txt");
    let total_flashes = (0..100)
        .into_iter()
        .map(|_| step(&mut cavern))
        .sum::<usize>();
    println!("part 1: {}", total_flashes);
    let all_flash = all_flash(&mut cavern);
    println!("part 2: {}", all_flash + 100);
}

fn cavern_levels(path: impl AsRef<Path>) -> OctopusCavern {
    Cavern::from_str(std::fs::read_to_string(path).unwrap().as_str())
}

#[cfg(test)]
mod problem11 {
    use super::*;

    #[test]
    fn test_step() {
        let mut cavern = cavern_levels("input/11.test.txt");
        let s1_flashes = step(&mut cavern);
        assert_eq!(s1_flashes, 0);
        let after_s1 = r#"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"#;
        let after_s1_cavern: OctopusCavern = Cavern::from_str(after_s1);
        assert_eq!(cavern, after_s1_cavern);
        let s2_flashes = step(&mut cavern);
        assert_eq!(s2_flashes, 35);
        let after_s2 = r#"8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"#;
        let after_s2_cavern: OctopusCavern = Cavern::from_str(after_s2);
        assert_eq!(cavern, after_s2_cavern);
    }

    #[test]
    fn part1() {
        let mut cavern = cavern_levels("input/11.test.txt");
        let total_flashes = (0..100)
            .into_iter()
            .map(|_| step(&mut cavern))
            .sum::<usize>();
        assert_eq!(1656, total_flashes);
    }

    #[test]
    fn part2() {
        let mut cavern = cavern_levels("input/11.test.txt");
        let all_flash = all_flash(&mut cavern);
        assert_eq!(all_flash, 195);
    }
}
