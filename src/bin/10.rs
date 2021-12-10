use std::{fmt::Display, path::Path, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    OpenBraces,
    OpenBrackets,
    OpenParenthesis,
    OpenAngleBrackets,
    CloseBraces,
    CloseBrackets,
    CloseParenthesis,
    CloseAngleBrackets,
}

impl FromStr for Token {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "{" => Ok(Self::OpenBraces),
            "[" => Ok(Self::OpenBrackets),
            "(" => Ok(Self::OpenParenthesis),
            "<" => Ok(Self::OpenAngleBrackets),
            "}" => Ok(Self::CloseBraces),
            "]" => Ok(Self::CloseBrackets),
            ")" => Ok(Self::CloseParenthesis),
            ">" => Ok(Self::CloseAngleBrackets),
            _ => Err(()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpenBraces => "{",
                Self::OpenBrackets => "[",
                Self::OpenParenthesis => "(",
                Self::OpenAngleBrackets => "<",
                Self::CloseBraces => "}",
                Self::CloseBrackets => "]",
                Self::CloseParenthesis => ")",
                Self::CloseAngleBrackets => ">",
            }
        )
    }
}

impl Token {
    fn is_open(&self) -> bool {
        matches!(
            self,
            &Self::OpenBraces
                | &Self::OpenBrackets
                | &Self::OpenParenthesis
                | &Self::OpenAngleBrackets
        )
    }

    fn matching_close(&self) -> Result<Self, ()> {
        if self.is_open() {
            Ok(match self {
                Self::OpenBraces => Self::CloseBraces,
                Self::OpenBrackets => Self::CloseBrackets,
                Self::OpenParenthesis => Self::CloseParenthesis,
                Self::OpenAngleBrackets => Self::CloseAngleBrackets,
                _ => unreachable!(),
            })
        } else {
            Err(())
        }
    }

    fn tokenize_line(s: &str) -> Vec<Self> {
        s.chars().map(|c| c.to_string().parse().unwrap()).collect()
    }
}

#[derive(Copy, Clone, Debug)]
struct SyntaxError {
    expected: Option<Token>,
    found: Token,
}

impl SyntaxError {
    fn new(expected: Option<Token>, found: Token) -> Self {
        Self { expected, found }
    }

    fn points(&self) -> usize {
        match &self.found {
            Token::CloseBraces => 1197,
            Token::CloseBrackets => 57,
            Token::CloseParenthesis => 3,
            Token::CloseAngleBrackets => 25137,
            _ => unreachable!(),
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expected {}, but found {}",
            self.expected
                .map_or(String::from("no token"), |t| t.to_string()),
            self.found
        )
    }
}

struct SyntaxChecker;

impl SyntaxChecker {
    fn check_line(s: &str) -> Option<SyntaxError> {
        let mut stack = Vec::new();
        let tokens = Token::tokenize_line(s);
        for token in tokens {
            if token.is_open() {
                stack.push(token);
            } else if let Some(top) = stack.last() {
                let matching_close = top.matching_close().ok();
                if matching_close.map_or(false, |matching| matching == token) {
                    stack.pop();
                } else {
                    return Some(SyntaxError::new(matching_close, token));
                }
            } else {
                return Some(SyntaxError::new(None, token));
            }
        }
        None
    }
}

struct AutoComplete;

impl AutoComplete {
    fn check_line(s: &str) -> String {
        let mut stack = Vec::new();
        let tokens = Token::tokenize_line(s);
        for token in tokens {
            if token.is_open() {
                stack.push(token);
            } else if let Some(top) = stack.last() {
                let matching_close = top.matching_close().ok();
                if matching_close.map_or(false, |matching| matching == token) {
                    stack.pop();
                } else {
                    unreachable!("illegal line, filter with syntax checker");
                }
            } else {
                unreachable!("illegal line, filter with syntax checker");
            }
        }
        stack
            .iter()
            .rev()
            .map(|token| token.matching_close().unwrap().to_string())
            .collect()
    }

    fn completion_score(s: &str) -> usize {
        s.chars().fold(0, |mut acc, c| {
            let cval = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!("auto complete only has close tokens"),
            };
            acc = acc * 5 + cval;
            acc
        })
    }
}

fn main() {
    let file = load_file("input/10.txt");
    let errors = file.lines().filter_map(|s| SyntaxChecker::check_line(s));
    let err_score = errors.map(|error| error.points()).sum::<usize>();
    println!("part 1: {}", err_score);

    let autocompletions = file
        .lines()
        .filter(|s| SyntaxChecker::check_line(s).is_none())
        .map(|s| AutoComplete::check_line(s));
    let ac_scores = autocompletions
        .map(|s| AutoComplete::completion_score(s.as_str()))
        .sorted()
        .collect::<Vec<_>>();
    let ac_winner = ac_scores[ac_scores.len() / 2];
    println!("part 2: {}", ac_winner);
}

fn load_file(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod problem09 {
    use super::*;

    #[test]
    fn part1() {
        let file = load_file("input/10.test.txt");
        let errors = file.lines().filter_map(|s| SyntaxChecker::check_line(s));
        let score = errors.map(|error| error.points()).sum::<usize>();
        assert_eq!(26397, score);
    }

    #[test]
    fn part2() {
        let file = load_file("input/10.test.txt");
        let autocompletions = file
            .lines()
            .filter(|s| SyntaxChecker::check_line(s).is_none())
            .map(|s| AutoComplete::check_line(s));
        let ac_scores = autocompletions
            .map(|s| AutoComplete::completion_score(s.as_str()))
            .sorted()
            .collect::<Vec<_>>();
        let ac_winner = ac_scores[ac_scores.len() / 2];
        assert_eq!(288957, ac_winner);
    }
}
