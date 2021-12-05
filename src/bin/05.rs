use std::{
    collections::HashMap,
    path::Path,
    str::{FromStr, Split},
};

type Point = [usize; 2];

#[derive(Clone, Copy, Debug, Default)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: [usize; 2], end: [usize; 2]) -> Self {
        Self { start, end }
    }

    fn is_diagonal(&self) -> bool {
        !(self.start[0] == self.end[0] || self.start[1] == self.end[1])
    }

    fn points(&self) -> Vec<Point> {
        fn step(val: i32) -> i32 {
            match val {
                m if m > 0 => -1,
                m if m < 0 => 1,
                _ => 0,
            }
        }
        let step_x = step(self.start[0] as i32 - self.end[0] as i32);
        let step_y = step(self.start[1] as i32 - self.end[1] as i32);

        let mut points = vec![self.start];
        let [mut x, mut y] = self.start;

        while [x, y] != self.end {
            x = (x as i32 + step_x) as usize;
            y = (y as i32 + step_y) as usize;
            points.push([x, y]);
        }

        points
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn coordinate(line: &mut Split<&str>) -> Point {
            line.next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }
        let mut line = s.split(" -> ");
        let start = coordinate(&mut line);
        let end = coordinate(&mut line);
        Ok(Line::new(start, end))
    }
}

fn main() {
    let lines = get_lines_of_vents("input/05.txt");

    let hv_lines = lines.iter().filter(|line| !line.is_diagonal());
    let pf_hv = overlaps(hv_lines);
    let atleast_two_overlap_hv = pf_hv.iter().filter(|(_, &f)| f >= 2).count();
    println!("part 1: {}", atleast_two_overlap_hv);

    let pf = overlaps(lines.iter());
    let atleast_two_overlap = pf.iter().filter(|(_, &f)| f >= 2).count();
    println!("part 2: {}", atleast_two_overlap);
}

fn get_lines_of_vents(p: impl AsRef<Path>) -> Vec<Line> {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    text.lines().map(|line| line.parse().unwrap()).collect()
}

fn overlaps<'a>(lines: impl Iterator<Item = &'a Line>) -> HashMap<Point, usize> {
    lines
        .map(|line| line.points())
        .flatten()
        .fold(HashMap::new(), |mut acc, p| {
            *acc.entry(p).or_insert(0) += 1;
            acc
        })
}

#[cfg(test)]
mod problem05 {
    use super::*;

    #[test]
    fn part1() {
        let lines = get_lines_of_vents("input/05.test.txt");
        let hv_lines = lines.iter().filter(|line| !line.is_diagonal());
        let pf = overlaps(hv_lines);
        let atleast_two_overlap = pf.iter().filter(|(_, &f)| f >= 2).count();
        assert_eq!(atleast_two_overlap, 5);
    }

    #[test]
    fn part2() {
        let lines = get_lines_of_vents("input/05.test.txt");
        let pf = overlaps(lines.iter());
        let atleast_two_overlap = pf.iter().filter(|(_, &f)| f >= 2).count();
        assert_eq!(atleast_two_overlap, 12);
    }
}
