use itertools::Itertools;
use std::path::Path;

#[derive(Clone, Copy, Debug, Default)]
struct BoardCell {
    value: usize,
    marked: bool,
}

impl BoardCell {
    fn new(value: usize) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

type Board = [[BoardCell; 5]; 5];
type DrawNumbers = Vec<usize>;

trait Bingo {
    fn draw(&mut self, number: usize);
    fn won(&self) -> bool;
    fn unmarked_sum(&self) -> usize;
}

impl Bingo for Board {
    fn draw(&mut self, number: usize) {
        self.iter_mut().flatten().for_each(|cell| {
            if cell.value == number {
                cell.marked = true;
            }
        });
    }
    fn won(&self) -> bool {
        let any_row = self.iter().any(|row| row.iter().all(|cell| cell.marked));
        let any_col = (0..5)
            .into_iter()
            .any(|idx| self.iter().map(|row| row[idx]).all(|cell| cell.marked));
        any_row || any_col
    }
    fn unmarked_sum(&self) -> usize {
        self.iter()
            .flatten()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.value)
            .sum()
    }
}

fn main() {
    let (numbers, boards) = get_draw_boards("input/04.txt");

    let (first_winning_board, last_draw) = get_first_winning_board(&numbers, &boards);
    println!("part 1: {}", first_winning_board.unmarked_sum() * last_draw);

    let (last_winning_board, last_draw) = get_last_winning_board(&numbers, &boards);
    println!("part 2: {}", last_winning_board.unmarked_sum() * last_draw);
}

fn get_draw_boards(p: impl AsRef<Path>) -> (DrawNumbers, Vec<Board>) {
    let text = std::fs::read_to_string(p.as_ref()).unwrap();
    let mut lines = text.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards = lines
        .filter(|l| !l.is_empty())
        .chunks(5)
        .into_iter()
        .map(|input| {
            input
                .into_iter()
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|n| BoardCell::new(n.parse().unwrap()))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    (numbers, boards)
}

fn get_first_winning_board(numbers: &[usize], boards: &[Board]) -> (Board, usize) {
    let mut boards = boards.to_owned();
    for number in numbers {
        boards.iter_mut().for_each(|board| board.draw(*number));
        if let Some(won_board) = boards.iter().find(|board| board.won()) {
            return (won_board.to_owned(), *number);
        }
    }
    unreachable!()
}

fn get_last_winning_board(numbers: &[usize], boards: &[Board]) -> (Board, usize) {
    let mut boards = boards.to_owned();
    for number in numbers {
        boards.iter_mut().for_each(|board| board.draw(*number));
        if boards.len() == 1 && boards[0].won() {
            return (boards[0], *number);
        }
        boards.retain(|board| !board.won());
    }
    unreachable!()
}

#[cfg(test)]
mod problem03 {
    use super::*;

    #[test]
    fn part1() {
        let (numbers, boards) = get_draw_boards("input/04.test.txt");
        let (first_winning_board, last_draw) = get_first_winning_board(&numbers, &boards);

        assert_eq!(first_winning_board.unmarked_sum(), 188);
        assert_eq!(last_draw, 24);
    }

    #[test]
    fn part2() {
        let (numbers, boards) = get_draw_boards("input/04.test.txt");
        let (last_winning_board, last_draw) = get_last_winning_board(&numbers, &boards);

        assert_eq!(last_winning_board.unmarked_sum(), 148);
        assert_eq!(last_draw, 13);
    }
}
