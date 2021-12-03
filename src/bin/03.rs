use std::path::Path;

type BitVec = Vec<bool>;

trait Bits {
    fn width(&self) -> usize;
    fn to_decimal(&self) -> usize;
    fn from_str(s: &str) -> Self;
    fn invert(&self) -> Self;
    fn to_string(&self) -> String;
}

impl Bits for BitVec {
    fn width(&self) -> usize {
        self.len()
    }

    fn to_decimal(&self) -> usize {
        self.iter()
            .rev()
            .enumerate()
            .map(|(idx, set)| match set {
                true => 2usize.pow(idx as u32),
                false => 0,
            })
            .sum()
    }

    fn from_str(s: &str) -> Self {
        s.chars()
            .map(|c| match c {
                '1' => true,
                '0' => false,
                x => panic!("invalid char: {} in binary string", x),
            })
            .collect()
    }

    fn invert(&self) -> Self {
        self.iter().map(|x| !x).collect()
    }

    fn to_string(&self) -> String {
        self.iter()
            .map(|bit| match bit {
                true => '1',
                false => '0',
            })
            .collect()
    }
}

fn main() {
    let diagnostics = get_report("input/03.txt");

    let gamma_rate = calculate_gamma_rate(&diagnostics);
    let epsilon_rate = gamma_rate.invert();
    let o2_rate = get_rating_value(&diagnostics, true);
    let co2_rate = get_rating_value(&diagnostics, false);

    println!(
        "part 1: {}",
        gamma_rate.to_decimal() * epsilon_rate.to_decimal()
    );
    println!("part 2: {}", o2_rate.to_decimal() * co2_rate.to_decimal());
}

fn get_report(p: impl AsRef<Path>) -> Vec<BitVec> {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    text.lines().map(|line| Bits::from_str(line)).collect()
}

#[allow(dead_code)]
fn print_bitvec(b: &[BitVec]) {
    println!("-----------------");
    b.iter().for_each(|v| println!("{}", v.to_string()));
    println!("-----------------");
}

fn calculate_gamma_rate(diagnostics: &[BitVec]) -> BitVec {
    let bits_width = diagnostics[0].width();
    let n_vecs_half = diagnostics.len() as f64 / 2.0f64;
    (0..bits_width)
        .into_iter()
        .map(|i| {
            diagnostics
                .iter()
                .filter_map(|d| match d[i] {
                    true => Some(()),
                    _ => None,
                })
                .count() as f64
                > n_vecs_half
        })
        .collect()
}

fn filter_records(diagnostics: &mut Vec<BitVec>, most_common: bool, bit_pos: usize) {
    let n_vecs_half = diagnostics.len() as f64 / 2.0f64;
    let set_bit_freq = diagnostics
        .iter()
        .filter_map(|d| match d[bit_pos] {
            true => Some(()),
            _ => None,
        })
        .count() as f64;

    let select_bit = match most_common {
        true => set_bit_freq >= n_vecs_half,
        false => set_bit_freq < n_vecs_half,
    };

    diagnostics.retain(|v| v[bit_pos] == select_bit);
}

fn get_rating_value(diagnostics: &[BitVec], o2: bool) -> BitVec {
    let bits_width = diagnostics[0].width();
    let mut dr = diagnostics.to_owned();
    for i in 0..bits_width {
        filter_records(&mut dr, o2, i);
        if dr.len() == 1 {
            break;
        }
    }
    dr[0].to_owned()
}

#[cfg(test)]
mod problem03 {
    use super::*;

    #[test]
    fn to_decimal() {
        let bitvec: BitVec = Bits::from_str("10110");
        assert_eq!(bitvec.to_decimal(), 22);
    }

    #[test]
    fn part1() {
        let diagnostics = get_report("input/03.test.txt");
        let gamma_rate = calculate_gamma_rate(&diagnostics);
        let epsilon_rate = gamma_rate.invert();
        assert_eq!(gamma_rate.to_decimal(), 22);
        assert_eq!(epsilon_rate.to_decimal(), 9);
    }

    #[test]
    fn part2() {
        let diagnostics = get_report("input/03.test.txt");
        let o2_rate = get_rating_value(&diagnostics, true);
        let co2_rate = get_rating_value(&diagnostics, false);
        assert_eq!(o2_rate.to_decimal(), 23);
        assert_eq!(co2_rate.to_decimal(), 10);
    }
}
