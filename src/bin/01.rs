use std::path::Path;

fn main() {
    let depths = get_depths("input/01.txt");
    println!("part 1: {}", larger_measurements(&depths));
    println!("part 2: {}", larger_measurements3(&depths));
}

fn get_depths(p: impl AsRef<Path>) -> Vec<usize> {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    text.lines().map(|line| line.parse().unwrap()).collect()
}

fn larger_measurements(depths: &[usize]) -> usize {
    depths
        .windows(2)
        .filter(|x| {
            let previous = x[0];
            let current = x[1];
            current > previous
        })
        .count()
}

fn larger_measurements3(depths: &[usize]) -> usize {
    depths
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|x| {
            let previous = x[0];
            let current = x[1];
            current > previous
        })
        .count()
}

#[cfg(test)]
mod problem01 {
    use super::*;

    #[test]
    fn part1() {
        let test_depths = get_depths("input/01.test.txt");
        assert_eq!(larger_measurements(&test_depths), 7);
    }

    #[test]
    fn part2() {
        let test_depths = get_depths("input/01.test.txt");
        assert_eq!(larger_measurements3(&test_depths), 5);
    }
}
