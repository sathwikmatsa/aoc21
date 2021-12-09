use std::{collections::HashSet, path::Path};

use itertools::Itertools;

fn main() {
    let heightmap = get_height_map("input/09.txt");
    let low_points = low_points(&heightmap);
    let risk_levels = low_points.iter().map(|(p, _)| p + 1);
    println!("part 1: {}", risk_levels.sum::<u32>());

    let basins = basins(&heightmap);
    let three_largest = basins.iter().sorted_by_key(|b| b.len()).rev().take(3);
    println!(
        "part 2: {}",
        three_largest.map(|v| v.len()).product::<usize>()
    );
}

fn get_height_map(p: impl AsRef<Path>) -> Vec<Vec<u32>> {
    std::fs::read_to_string(p)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbours(map: &[Vec<u32>], row: usize, col: usize) -> Vec<(u32, (usize, usize))> {
    let mut neighbours = Vec::new();
    // up
    if let Some(r) = row.checked_sub(1) {
        neighbours.push((map[r][col], (r, col)));
    }
    // down
    if row + 1 < map.len() {
        neighbours.push((map[row + 1][col], (row + 1, col)));
    }
    // left
    if let Some(c) = col.checked_sub(1) {
        neighbours.push((map[row][c], (row, c)));
    }
    // right
    if col + 1 < map[0].len() {
        neighbours.push((map[row][col + 1], (row, col + 1)));
    }
    neighbours
}

fn low_points(map: &[Vec<u32>]) -> Vec<(u32, (usize, usize))> {
    map.iter()
        .enumerate()
        .map(move |(i, row)| row.iter().enumerate().map(move |(j, c)| (i, j, *c)))
        .flatten()
        .filter_map(|(row, col, val)| {
            match neighbours(map, row, col).iter().all(|(n, _)| *n > val) {
                true => Some((val, (row, col))),
                false => None,
            }
        })
        .collect()
}

fn basins(map: &[Vec<u32>]) -> Vec<HashSet<(usize, usize)>> {
    fn basin(
        map: &[Vec<u32>],
        pos: &(usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let pos_height = map[pos.0][pos.1];
        let neighbours = neighbours(map, pos.0, pos.1);
        let filtered_neighbours = neighbours
            .into_iter()
            .filter(|(h, pos)| *h > pos_height && !visited.contains(pos))
            .collect::<Vec<_>>();

        let basin = filtered_neighbours
            .iter()
            .map(|(h, pos)| {
                visited.insert(pos.to_owned());
                match h {
                    9 => HashSet::new(),
                    _ => basin(map, pos, visited),
                }
            })
            .flatten()
            .chain(std::iter::once(pos.to_owned()))
            .collect::<HashSet<_>>();

        basin
    }

    let low_points = low_points(map);
    let mut visited = HashSet::new();
    low_points
        .iter()
        .map(|(_, pos)| basin(map, pos, &mut visited))
        .collect()
}

#[allow(dead_code)]
fn debug_print_basin(basin: &HashSet<(usize, usize)>, map: &[Vec<u32>]) {
    let map_width = map[0].len();
    let map_height = map.len();
    (0..map_height).for_each(|row| {
        (0..map_width).for_each(|col| match basin.contains(&(row, col)) {
            true => print!("\x1b[93m{}\x1b[0m", map[row][col]),
            false => print!("{}", map[row][col]),
        });
        println!();
    });
    println!();
}

#[cfg(test)]
mod problem09 {
    use super::*;

    #[test]
    fn part1() {
        let heightmap = get_height_map("input/09.test.txt");
        let low_points = low_points(&heightmap);
        let risk_levels = low_points.iter().map(|(p, _)| p + 1);
        assert_eq!(risk_levels.sum::<u32>(), 15);
    }

    #[test]
    fn part2() {
        let heightmap = get_height_map("input/09.test.txt");
        let basins = basins(&heightmap);
        let three_largest = basins.iter().sorted_by_key(|b| b.len()).rev().take(3);
        assert_eq!(1134, three_largest.map(|v| v.len()).product::<usize>());
    }
}
