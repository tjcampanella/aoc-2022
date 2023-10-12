#![allow(dead_code)]

use std::{collections::HashMap, fs};

const fn main() {}

fn aoc3p1() -> usize {
    let lowers = ('a'..='z').collect::<Vec<char>>();
    let mut uppers = ('A'..='Z').collect::<Vec<char>>();
    let mut letters = lowers;
    letters.append(&mut uppers);

    let input = fs::read_to_string("aoc3-input.txt").map_or_else(
        |_| Vec::new(),
        |contents| {
            contents
                .split('\n')
                .map(|line| line.split_at(line.len() / 2))
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .map(|(s1, s2)| {
                    let mut freqs = HashMap::<char, u32>::new();

                    let mut s1chars = s1.chars().collect::<Vec<_>>();
                    s1chars.sort_unstable();
                    s1chars.dedup();

                    let mut s2chars = s2.chars().collect::<Vec<_>>();
                    s2chars.sort_unstable();
                    s2chars.dedup();

                    s1chars.append(&mut s2chars);

                    let chars = s1chars;

                    for char in chars {
                        freqs.entry(char).and_modify(|val| *val += 1).or_insert(1);
                    }

                    let mut dups = Vec::<(char, u32)>::new();
                    for (key, value) in freqs {
                        if value > 1 {
                            dups.push((key, value));
                        }
                    }

                    dups
                })
                .collect::<Vec<_>>()
        },
    );

    input
        .iter()
        .flatten()
        .map(|(c, _)| letters.iter().position(|val| val == c).unwrap_or(0) + 1)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::aoc3p1;

    #[test]
    fn aoc3p1_test() {
        assert_eq!(7737, aoc3p1());
    }
}
