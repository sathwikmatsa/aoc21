use std::{
    collections::{BinaryHeap, HashMap},
    path::Path,
};

fn main() {
    let density_map = get_chitons_density("input/15.txt");
    println!("part 1: {}", a_star(&density_map));
    let density_map5x = extend_5x(&density_map);
    println!("part 2: {}", a_star(&density_map5x));
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    node: (usize, usize),
    distance_from_start: u32,
    heuristic: usize,
    prev_node: Option<(usize, usize)>,
}

impl State {
    fn cost(&self) -> u32 {
        self.distance_from_start + self.heuristic as u32
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost()
            .cmp(&self.cost())
            .then_with(|| other.distance_from_start.cmp(&self.distance_from_start))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn make_state(
    pos: (usize, usize),
    dist: u32,
    end: (usize, usize),
    prev: Option<(usize, usize)>,
) -> State {
    State {
        node: pos,
        distance_from_start: dist,
        heuristic: (end.0 - pos.0) + (end.1 - pos.1),
        prev_node: prev,
    }
}

fn a_star(map: &[Vec<u32>]) -> u32 {
    let n_rows = map.len();
    let n_cols = map[0].len();
    let end = (n_rows - 1, n_cols - 1);
    let start = make_state((0, 0), 0, end, None);
    let mut pq = BinaryHeap::new();
    pq.push(start);
    let mut dist = HashMap::new();
    dist.insert((0, 0), 0);

    while let Some(hp) = pq.pop() {
        if hp.node == end {
            return hp.distance_from_start;
        }
        if let Some(d) = dist.get(&hp.node) {
            if *d < hp.distance_from_start {
                continue;
            }
        }
        for nextp in neighbours(&hp, n_rows, n_cols) {
            let new_state = make_state(
                nextp,
                hp.distance_from_start + map[nextp.0][nextp.1],
                end,
                Some(hp.node),
            );
            let d = dist.entry(new_state.node).or_insert(u32::MAX);
            if new_state.distance_from_start < *d {
                pq.push(new_state);
                *d = new_state.distance_from_start;
            }
        }
    }
    unreachable!("cannot reach end");
}

fn extend_5x(map: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let pattern_width = map[0].len();
    let pattern_height = map.len();
    let n_rows = pattern_height * 5;
    let n_cols = pattern_width * 5;
    let mut ext_map = vec![vec![0; n_cols]; n_rows];
    map.iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(move |(j, &val)| ((i, j), val)))
        .flatten()
        .for_each(|((i, j), val)| {
            for m in 0..5 {
                for n in 0..5 {
                    ext_map[i + pattern_height * m][j + pattern_width * n] =
                        ((val as usize + m + n - 1) % 9 + 1) as u32;
                }
            }
        });
    ext_map
}

fn get_chitons_density(path: impl AsRef<Path>) -> Vec<Vec<u32>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbours(state: &State, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    const NEIGHBOURS: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

    let pos = state.node;

    NEIGHBOURS
        .iter()
        .filter_map(|[r, c]| {
            let row = pos.0 as i32 + r;
            let col = pos.1 as i32 + c;
            match (0..n_rows as i32).contains(&row) && (0..n_cols as i32).contains(&col) {
                true => Some((row as usize, col as usize)),
                _ => None,
            }
        })
        .filter(|pos| Some(*pos) != state.prev_node)
        .collect()
}

#[cfg(test)]
mod problem15 {
    use super::*;

    #[test]
    fn chitons_low_risk() {
        let density_map = get_chitons_density("input/15.test.txt");
        assert_eq!(40, a_star(&density_map));
        let density_map5x = extend_5x(&density_map);
        assert_eq!(315, a_star(&density_map5x));
    }
}
