use bimap::BiHashMap;
use std::{path::Path, str::FromStr};

#[derive(Debug)]
struct Graph {
    idx_map: BiHashMap<String, usize>,
    adj_matrix: Vec<Vec<bool>>,
}

fn expand_matrix<T: Default>(matrix: &mut Vec<Vec<T>>) {
    let width = if !matrix.is_empty() {
        matrix[0].len()
    } else {
        0
    };
    // add extra row
    let new_row = (0..width).map(|_| T::default()).collect();
    matrix.push(new_row);
    // add extra column
    matrix.iter_mut().for_each(|row| row.push(T::default()));
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut idx_map = BiHashMap::new();
        let mut adj_matrix = Vec::new();

        s.lines()
            .map(|l| {
                let mut l = l.split('-');
                let node1 = l.next().unwrap();
                let node2 = l.next().unwrap();
                (node1, node2)
            })
            .for_each(|(node1, node2)| {
                if !idx_map.contains_left(node1) {
                    idx_map.insert(node1.to_owned(), adj_matrix.len());
                    expand_matrix(&mut adj_matrix);
                }
                if !idx_map.contains_left(node2) {
                    idx_map.insert(node2.to_owned(), adj_matrix.len());
                    expand_matrix(&mut adj_matrix);
                }
                let n1idx = idx_map.get_by_left(node1).unwrap();
                let n2idx = idx_map.get_by_left(node2).unwrap();
                adj_matrix[*n1idx][*n2idx] = true;
                adj_matrix[*n2idx][*n1idx] = true;
            });

        Ok(Self {
            idx_map,
            adj_matrix,
        })
    }
}

impl Graph {
    fn all_paths(&self, cond: fn(&str, &[&str]) -> bool) -> usize {
        let start_idx = self
            .idx_map
            .get_by_left("start")
            .expect("no node with name 'start'");

        let path = vec!["start"];
        self.paths_to_end(*start_idx, path, cond)
    }

    fn paths_to_end(
        &self,
        start: usize,
        path: Vec<&str>,
        cond: fn(&str, &[&str]) -> bool,
    ) -> usize {
        self.connected_nodes(&start)
            .iter()
            .map(|n| (self.idx_map.get_by_right(n).unwrap(), *n))
            .filter(|(name, _)| cond(*name, &path))
            .map(|(name, id)| {
                if name.eq("end") {
                    1
                } else {
                    let mut p = path.clone();
                    p.push(name.as_str());
                    self.paths_to_end(id, p, cond)
                }
            })
            .sum()
    }

    fn connected_nodes(&self, index: &usize) -> Vec<usize> {
        self.adj_matrix[*index]
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                true => Some(idx),
                false => None,
            })
            .collect()
    }
}

fn main() {
    let graph = load_graph("input/12.txt");
    println!(
        "part 1: {}",
        graph
            .all_paths(|node, path| !(node.starts_with(|c: char| c.is_lowercase())
                && path.contains(&node)))
    );
    println!(
        "part 2: {}",
        graph.all_paths(|node, path| {
            // Big caves can be visited any no. of times
            if node.starts_with(|c: char| c.is_uppercase()) {
                return true;
            }
            let smol_cave_repeated = path
                .iter()
                .filter(|n| n.starts_with(|c: char| c.is_lowercase()))
                .map(|n| path.iter().filter(|s| s.eq(&n)).count())
                .any(|repeats| repeats == 2);

            match smol_cave_repeated {
                // don't allow repeat if any cave is visited twice
                true => !path.contains(&node),
                _ => !matches!(node, "start"),
            }
        })
    );
}

fn load_graph(path: impl AsRef<Path>) -> Graph {
    std::fs::read_to_string(path).unwrap().parse().unwrap()
}

#[cfg(test)]
mod problem12 {
    use super::*;

    #[test]
    fn graph() {
        let graph_s = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let graph = Graph::from_str(graph_s).unwrap();
        let adj_matrix = vec![
            vec![false, true, true, false, false, false],
            vec![true, false, true, true, false, true],
            vec![true, true, false, false, true, true],
            vec![false, true, false, false, false, false],
            vec![false, false, true, false, false, false],
            vec![false, true, true, false, false, false],
        ];

        assert_eq!(graph.adj_matrix, adj_matrix);
    }
    #[test]
    fn part1() {
        let graph = load_graph("input/12.test.txt");
        assert_eq!(
            226,
            graph.all_paths(|node, path| !(node.starts_with(|c: char| c.is_lowercase())
                && path.contains(&node)))
        );
    }

    #[test]
    fn part2() {
        let graph = load_graph("input/12.test.txt");
        assert_eq!(
            3509,
            graph.all_paths(|node, path| {
                if node.starts_with(|c: char| c.is_uppercase()) {
                    return true;
                }
                let smol_cave_repeated = path
                    .iter()
                    .filter(|n| n.starts_with(|c: char| c.is_lowercase()))
                    .map(|n| path.iter().filter(|s| s.eq(&n)).count())
                    .any(|repeats| repeats == 2);

                match smol_cave_repeated {
                    true => !path.contains(&node),
                    _ => !matches!(node, "start"),
                }
            })
        );
    }
}
