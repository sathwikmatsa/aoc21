use std::path::Path;

fn main() {
    let positions = get_positions("input/07.txt");
    let (_best_pos, fuel) = best_alignment(&positions);
    println!("part 1: {}", fuel);
    let (_best_pos, fuel) = best_alignment_v2(&positions);
    println!("part 2: {}", fuel);
}

fn get_positions(p: impl AsRef<Path>) -> Vec<usize> {
    std::fs::read_to_string(p)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn best_alignment(positions: &[usize]) -> (usize, usize) {
    let least = *positions.iter().min().unwrap();
    let highest = *positions.iter().max().unwrap();
    (least..=highest)
        .into_iter()
        .map(|pos| {
            let sum = positions
                .iter()
                .map(|p| i32::abs(*p as i32 - pos as i32) as usize)
                .sum();
            (pos, sum)
        })
        .min_by_key(|pair| pair.1)
        .unwrap()
}

fn best_alignment_v2(positions: &[usize]) -> (usize, usize) {
    let fuel_for_steps = |n: usize| n * (n + 1) / 2;
    let least = *positions.iter().min().unwrap();
    let highest = *positions.iter().max().unwrap();
    (least..=highest)
        .into_iter()
        .map(|pos| {
            let sum = positions
                .iter()
                .map(|p| {
                    let n = i32::abs(*p as i32 - pos as i32) as usize;
                    fuel_for_steps(n)
                })
                .sum();
            (pos, sum)
        })
        .min_by_key(|pair| pair.1)
        .unwrap()
}

#[cfg(test)]
mod problem07 {
    use super::*;

    #[test]
    fn part1() {
        let positions = get_positions("input/07.test.txt");
        let (best_pos, fuel) = best_alignment(&positions);
        assert_eq!(2, best_pos);
        assert_eq!(37, fuel);
    }

    #[test]
    fn part2() {
        let positions = get_positions("input/07.test.txt");
        let (best_pos, fuel) = best_alignment_v2(&positions);
        assert_eq!(5, best_pos);
        assert_eq!(168, fuel);
    }
}
