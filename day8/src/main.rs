use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone)]
struct Digit {
    letters: HashSet<char>,
}

impl Digit {
    fn from_string(string: &str) -> Self {
        Digit {
            letters: string.chars().collect(),
        }
    }

    fn active_segment_count(&self) -> i32 {
        self.letters.len() as i32
    }

    fn matches(&self, other: &Digit) -> bool {
        self.letters == other.letters
    }
}

struct Line {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
    mapping: HashMap<i32, Digit>,
}

impl Line {
    fn decode(&mut self) {
        let mut one: &Digit = &Digit {
            letters: HashSet::new(),
        };
        let mut four: &Digit = &Digit {
            letters: HashSet::new(),
        };
        for digit in &self.patterns {
            match digit.active_segment_count() {
                2 => {
                    self.mapping.insert(1, digit.clone());
                    one = digit;
                }
                3 => {
                    self.mapping.insert(7, digit.clone());
                }
                4 => {
                    self.mapping.insert(4, digit.clone());
                    four = digit;
                }
                7 => {
                    self.mapping.insert(8, digit.clone());
                }
                _ => {}
            };
        }
        for digit in &self.patterns {
            let active_segment_count = digit.active_segment_count();
            if active_segment_count == 6 {
                if four
                    .letters
                    .iter()
                    .all(|letter| digit.letters.contains(letter))
                {
                    self.mapping.insert(9, digit.clone());
                } else if one
                    .letters
                    .iter()
                    .all(|letter| digit.letters.contains(letter))
                {
                    self.mapping.insert(0, digit.clone());
                } else {
                    self.mapping.insert(6, digit.clone());
                }
            } else if active_segment_count == 5 {
                if one
                    .letters
                    .iter()
                    .all(|letter| digit.letters.contains(letter))
                {
                    self.mapping.insert(3, digit.clone());
                } else if four
                    .letters
                    .iter()
                    .filter(|letter| digit.letters.contains(letter))
                    .count()
                    == 3
                {
                    self.mapping.insert(5, digit.clone());
                } else {
                    self.mapping.insert(2, digit.clone());
                }
            }
        }
    }

    fn get_decoded_output(&self) -> i32 {
        let mut result = 0;
        for (i, digit) in self.output.iter().rev().enumerate() {
            for (value, mapped_digit) in &self.mapping {
                if digit.matches(mapped_digit) {
                    result += value * 10_i32.pow(i as u32)
                }
            }
        }
        return result;
    }
}

fn main() {
    let mut lines = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let split = line.split(" | ").collect::<Vec<&str>>();
            Line {
                patterns: split
                    .get(0)
                    .unwrap()
                    .split(" ")
                    .map(|s| Digit::from_string(s))
                    .collect::<Vec<Digit>>(),
                output: split
                    .get(1)
                    .unwrap()
                    .split(" ")
                    .map(|s| Digit::from_string(s))
                    .collect::<Vec<Digit>>(),
                mapping: HashMap::new(),
            }
        })
        .collect::<Vec<Line>>();

    let unique_segment_count = lines
        .iter()
        .map(|line| {
            line.output
                .iter()
                .map(|output| output.active_segment_count())
                .filter(|segment_count| {
                    *segment_count == 2
                        || *segment_count == 3
                        || *segment_count == 4
                        || *segment_count == 7
                })
                .count() as i32
        })
        .sum::<i32>();
    println!("Unique segment count: {}", unique_segment_count);

    for line in &mut lines {
        line.decode();
    }
    let sum: i32 = lines.iter().map(|l| l.get_decoded_output()).sum();
    println!("Decoded sum: {}", sum);
}
