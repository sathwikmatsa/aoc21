use std::{path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    East,
    South,
    Empty,
}

impl FromStr for Cell {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::East),
            "v" => Ok(Self::South),
            "." => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

fn step(region: &mut Vec<Vec<Cell>>) -> usize {
    let mut moves = 0;
    let row_size = region[0].len();
    let col_size = region.len();
    let mut swaps = Vec::new();

    // move east herd
    (0..col_size).for_each(|j| {
        for i in 0..row_size {
            if region[j][i] == Cell::East && region[j][(i + 1) % row_size] == Cell::Empty {
                swaps.push(((j, (i + 1) % row_size), (j, i)));
                moves += 1;
            }
        }
    });

    while let Some((m, n)) = swaps.pop() {
        let temp = region[m.0][m.1];
        region[m.0][m.1] = region[n.0][n.1];
        region[n.0][n.1] = temp;
    }

    // move south herd
    for i in 0..row_size {
        for j in 0..col_size {
            if region[j][i] == Cell::South && region[(j + 1) % col_size][i] == Cell::Empty {
                swaps.push((((j + 1) % col_size, i), (j, i)));
                moves += 1;
            }
        }
    }

    while let Some((m, n)) = swaps.pop() {
        let temp = region[m.0][m.1];
        region[m.0][m.1] = region[n.0][n.1];
        region[n.0][n.1] = temp;
    }

    moves
}

fn main() {
    let mut region = get_region_map("input/25.txt");
    let mut steps = 1;

    while step(&mut region) != 0 {
        steps += 1;
    }
    println!("part 1: {}", steps);
}

fn get_region_map(path: impl AsRef<Path>) -> Vec<Vec<Cell>> {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}
