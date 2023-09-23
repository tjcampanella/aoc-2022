use std::fs;

fn aoc1p1() -> Option<u32> {
    let contents = fs::read_to_string("input.txt").map_or_else(
        |_| Vec::new(),
        |contents| {
            contents
                .split(char::is_whitespace)
                .map(str::parse::<u32>)
                .collect::<Vec<_>>()
        },
    );

    let mut results: Vec<u32> = vec![];
    let mut temp = 0;
    for val in contents {
        if let Ok(val) = val {
            temp += val;
        } else {
            results.push(temp);
            temp = 0;
        }
    }

    return results.iter().max().copied();
}

fn aoc1p2() -> u32 {
    let contents = fs::read_to_string("input.txt").map_or_else(
        |_| Vec::new(),
        |contents| {
            contents
                .split(char::is_whitespace)
                .map(str::parse::<u32>)
                .collect::<Vec<_>>()
        },
    );

    let mut results: Vec<u32> = vec![];
    let mut temp = 0;
    for val in contents {
        if let Ok(val) = val {
            temp += val;
        } else {
            results.push(temp);
            temp = 0;
        }
    }

    results.sort_unstable();
    results.reverse();
    return results[0..=2].to_vec().iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::{aoc1p1, aoc1p2};

    #[test]
    fn aoc1p1_test() {
        assert_eq!(Option::Some(68467), aoc1p1());
    }

    #[test]
    fn aoc1p2_test() {
        assert_eq!(68467 + 68143 + 66810, aoc1p2());
    }
}

fn main() {}
