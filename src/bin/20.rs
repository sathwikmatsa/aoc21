use std::{collections::HashSet, path::Path};

#[derive(Clone)]
struct Image {
    pixels: HashSet<(i64, i64)>,
    kind: PixelKind,
}

impl Image {
    fn new_with_kind(kind: PixelKind) -> Self {
        Self {
            pixels: HashSet::new(),
            kind,
        }
    }

    fn is_tracking_lit_pixels(&self) -> bool {
        self.kind.is_lit()
    }

    fn lit_pixels(&self) -> Result<usize, ()> {
        match self.kind.is_lit() {
            true => Ok(self.pixels.len()),
            false => Err(()),
        }
    }

    fn bounding_box(&self) -> [(i64, i64); 2] {
        let mut min_max_x = (i64::MAX, i64::MIN);
        let mut min_max_y = (i64::MAX, i64::MIN);

        for &(px, py) in &self.pixels {
            if px < min_max_x.0 {
                min_max_x.0 = px;
            }
            if px > min_max_x.1 {
                min_max_x.1 = px;
            }
            if py < min_max_y.0 {
                min_max_y.0 = py;
            }
            if py > min_max_y.1 {
                min_max_y.1 = py;
            }
        }

        [min_max_x, min_max_y]
    }

    fn is_pixel_lit(&self, pixel: (i64, i64)) -> bool {
        self.is_tracking_lit_pixels() == self.pixels.contains(&pixel)
    }

    fn neighbour_aggregate(&self, pixel: (i64, i64)) -> u16 {
        let (i, j) = pixel;
        let mut exp = 8;
        let mut acc = 0;
        for row in [i - 1, i, i + 1] {
            for col in [j - 1, j, j + 1] {
                if self.is_pixel_lit((row, col)) {
                    acc += 2u16.pow(exp as u32);
                }
                exp -= 1;
            }
        }
        acc
    }

    fn enhance(&self, algorithm: &[bool; 512]) -> Self {
        let [(minx, maxx), (miny, maxy)] = self.bounding_box();
        let mut output = Image::new_with_kind(self.kind.flip());
        for i in minx - 1..=maxx + 1 {
            for j in miny - 1..=maxy + 1 {
                let val = self.neighbour_aggregate((i, j));
                let pixel_lit = algorithm[val as usize];

                if pixel_lit == output.is_tracking_lit_pixels() {
                    output.pixels.insert((i, j));
                }
            }
        }
        output
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PixelKind {
    Lit,
    Off,
    AlwaysLit,
}

impl PixelKind {
    fn flip(&self) -> Self {
        match self {
            Self::Off => Self::Lit,
            Self::Lit => Self::Off,
            Self::AlwaysLit => Self::AlwaysLit,
        }
    }

    fn is_lit(&self) -> bool {
        matches!(self, Self::Lit | Self::AlwaysLit)
    }
}

fn main() {
    let (algo, input) = algo_and_input("input/20.txt");
    let twice = enhance_n(&input, algo, 2);
    println!("part 1: {}", twice.lit_pixels().unwrap());
    let times50 = enhance_n(&input, algo, 50);
    println!("part 2: {}", times50.lit_pixels().unwrap());
}

fn enhance_n(original: &Image, algorithm: [bool; 512], n: usize) -> Image {
    let mut image = original.to_owned();
    for _ in 0..n {
        image = image.enhance(&algorithm);
    }
    image
}

fn algo_and_input(path: impl AsRef<Path>) -> ([bool; 512], Image) {
    let content = std::fs::read_to_string(path).unwrap();
    let (algo, input) = content.split_once("\n\n").unwrap();
    let algo = algo
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let input = input
        .lines()
        .enumerate()
        .map(|(row_id, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(col_id, c)| match c {
                    '#' => Some((row_id as i64, col_id as i64)),
                    _ => None,
                })
        })
        .flatten()
        .collect();
    (
        algo,
        Image {
            pixels: input,
            kind: match algo[0] {
                false => PixelKind::AlwaysLit,
                _ => PixelKind::Lit,
            },
        },
    )
}

#[cfg(test)]
mod problem20 {
    use super::*;

    #[test]
    fn part1() {
        let (algo, input) = algo_and_input("input/20.test.txt");
        let twice = enhance_n(&input, algo, 2);
        assert_eq!(35, twice.lit_pixels().unwrap());
    }
}
