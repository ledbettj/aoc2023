use std::{collections::HashMap, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/day1.txt");

pub struct Document {
  values: (u32, u32),
}

impl Document {
  pub fn calibration(&self) -> u32 {
    self.values.0 * 10 + self.values.1
  }

  pub fn lookup() -> HashMap<String, u32> {
    let m = [
      ("0", 0),
      ("1", 1),
      ("2", 2),
      ("3", 3),
      ("4", 4),
      ("5", 5),
      ("6", 6),
      ("7", 7),
      ("8", 8),
      ("9", 9),
      ("one", 1),
      ("two", 2),
      ("three", 3),
      ("four", 4),
      ("five", 5),
      ("six", 6),
      ("seven", 7),
      ("eight", 8),
      ("nine", 9),
    ];

    m.into_iter().map(|(a, b)| (a.to_string(), b)).collect()
  }

  pub fn part1(s: &str) -> Result<Self, &'static str> {
    let values: Vec<u32> = s.chars().filter_map(|ch| ch.to_digit(10)).collect();

    let left = *values.first().ok_or("No digit found")?;
    let right = *values.last().ok_or("No digit found")?;

    Ok(Document {
      values: (left, right),
    })
  }

  pub fn part2(s: &str, lookup: &HashMap<String, u32>) -> Result<Self, &'static str> {
    let r =
      regex::Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").expect("Bad regex");

    let left = r.captures(s).ok_or("No digit found")?.get(0).unwrap();
    let left = *lookup.get(left.as_str()).unwrap();

    let mut last = None;
    let mut n = 0;

    while let Some(cap) = r.captures_at(s, n) {
      n += 1;
      last = Some(cap);
    }

    let last = last.unwrap().get(0).unwrap().as_str();
    let right = *lookup.get(last).unwrap();

    Ok(Document {
      values: (left, right),
    })
  }
}

impl FromStr for Document {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Document::part1(s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
  const EXAMPLE_INPUT2: &'static str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

  #[test]
  fn part1_example() {
    let docs: Vec<Document> = EXAMPLE_INPUT
      .lines()
      .map(|line| line.parse())
      .collect::<Result<Vec<_>, _>>()
      .expect("Failed to parse lines");

    let sum: u32 = docs.iter().map(|d| d.calibration()).sum();

    assert_eq!(sum, 142);
  }

  #[test]
  fn part1_solution() {
    let docs: Vec<Document> = INPUT
      .lines()
      .map(|line| line.parse())
      .collect::<Result<Vec<_>, _>>()
      .expect("Failed to parse lines");

    let sum: u32 = docs.iter().map(|d| d.calibration()).sum();

    assert_eq!(sum, 53921);
  }

  #[test]
  fn part2_example() {
    let lookup = Document::lookup();
    let docs: Vec<Document> = EXAMPLE_INPUT2
      .lines()
      .map(|line| Document::part2(line, &lookup))
      .collect::<Result<Vec<_>, _>>()
      .expect("Failed to parse lines");

    let sum: u32 = docs.iter().map(|d| d.calibration()).sum();

    assert_eq!(sum, 281);
  }

  #[test]
  fn part2_solution() {
    let lookup = Document::lookup();
    let docs: Vec<Document> = INPUT
      .lines()
      .map(|line| Document::part2(line, &lookup))
      .collect::<Result<Vec<_>, _>>()
      .expect("Failed to parse lines");

    let sum: u32 = docs.iter().map(|d| d.calibration()).sum();

    assert_eq!(sum, 54676);
  }
}
