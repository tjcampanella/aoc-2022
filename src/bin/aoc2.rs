use std::fs;

const fn main() {}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

enum Result {
    Win,
    Lose,
    Tie,
}

impl Result {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Tie => 3,
        }
    }
}

impl RockPaperScissors {
    fn from_letter_code(code: &str) -> Self {
        match code {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => Self::Invalid,
        }
    }

    fn to_letter_code(&self) -> String {
        match self {
            Self::Rock => String::from("X"),
            Self::Paper => String::from("Y"),
            Self::Scissors => String::from("Z"),
            Self::Invalid => String::new(),
        }
    }

    fn get_choice_from_code(rps: &Self, code: &str) -> Self {
        match (rps, code) {
            (Self::Rock, "X") => Self::Scissors,
            (Self::Rock, "Y") => Self::Rock,
            (Self::Rock, "Z") => Self::Paper,

            (Self::Paper, "X") => Self::Rock,
            (Self::Paper, "Y") => Self::Paper,
            (Self::Paper, "Z") => Self::Scissors,

            (Self::Scissors, "X") => Self::Paper,
            (Self::Scissors, "Y") => Self::Scissors,
            (Self::Scissors, "Z") => Self::Rock,
            _ => Self::Invalid,
        }
    }

    fn check_result(&self, r: &Self) -> Result {
        match (self, r) {
            (Self::Rock, Self::Scissors) => Result::Win,
            (Self::Rock, Self::Paper) => Result::Lose,
            (Self::Rock, Self::Rock) => Result::Tie,
            (Self::Paper, Self::Rock) => Result::Win,
            (Self::Paper, Self::Scissors) => Result::Lose,
            (Self::Paper, Self::Paper) => Result::Tie,
            (Self::Scissors, Self::Paper) => Result::Win,
            (Self::Scissors, Self::Rock) => Result::Lose,
            (Self::Scissors, Self::Scissors) => Result::Tie,
            _ => Result::Lose,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Self::Invalid => 0,
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn aoc2p1() -> u32 {
    let cipher = fs::read_to_string("aoc2-input.txt").map_or_else(
        |_| Vec::new(),
        |contents| {
            contents
                .split('\n')
                .map(|line| {
                    line.split(char::is_whitespace)
                        .map(RockPaperScissors::from_letter_code)
                        .collect::<Vec<RockPaperScissors>>()
                })
                .collect::<Vec<Vec<_>>>()
        },
    );

    let mut score: u32 = 0;
    for value in &cipher {
        match &value[..] {
            [first, second] => {
                let result = second.check_result(first);
                score += result.value() + second.value();
            }
            _ => score += 0,
        };
    }

    score
}

fn aoc2p2() -> u32 {
    let cipher = fs::read_to_string("aoc2-input.txt").map_or_else(
        |_| Vec::new(),
        |contents| {
            contents
                .split('\n')
                .map(|line| {
                    line.split(char::is_whitespace)
                        .map(RockPaperScissors::from_letter_code)
                        .collect::<Vec<RockPaperScissors>>()
                })
                .collect::<Vec<Vec<_>>>()
        },
    );

    let mut score: u32 = 0;
    for value in &cipher {
        match &value[..] {
            [first, second] => {
                let choice =
                    RockPaperScissors::get_choice_from_code(first, &second.to_letter_code());
                let result = choice.check_result(first);
                score += result.value() + choice.value();
            }
            _ => score += 0,
        };
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::{aoc2p1, aoc2p2};

    #[test]
    fn aoc2p1_test() {
        assert_eq!(8392, aoc2p1());
    }

    #[test]
    fn aoc2p2_test() {
        assert_eq!(10116, aoc2p2());
    }
}
