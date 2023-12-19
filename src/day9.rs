use num::Zero;
use std::{num::ParseIntError, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/day9.txt");

pub enum Side {
  Front, // generating elements at the front of the rows (p2)
  Back,  // generating elements at the end of the rows (p2)
}

#[derive(Debug)]
struct Report {
  values: Vec<isize>,
}

impl FromStr for Report {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s.split_whitespace()
      .map(|v| v.parse())
      .collect::<Result<_, _>>()
      .map(|values| Self { values })
  }
}

impl Report {
  pub fn load_all(lines: &str) -> Result<Vec<Report>, ParseIntError> {
    lines.lines().map(|line| line.parse()).collect()
  }

  pub fn solution(&self, side: Side) -> isize {
    let mut rows = Vec::new();
    let is_done = |item: &Vec<isize>| item.iter().all(|&value| value.is_zero());

    // generate rows until done, starting with the given row
    rows.push(self.values.clone());

    while !is_done(&rows.last().unwrap()) {
      rows.push(Report::step(&rows.last().unwrap()));
    }

    rows
      .iter()
      .rev()
      .map(|row| {
        *match side {
          Side::Front => row.first(),
          Side::Back => row.last(),
        }
        .unwrap()
      })
      .reduce(|prev, this| match side {
        Side::Front => this - prev,
        Side::Back => this + prev,
      })
      .unwrap()
  }

  // given a list of numbers, return a new list that's the difference
  // between each subsequent pair
  fn step(values: &[isize]) -> Vec<isize> {
    values
      .windows(2)
      .map(|pair| pair[1] - pair[0])
      .collect::<Vec<isize>>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

  #[test]
  fn part1_example() {
    let reports: Vec<Report> = Report::load_all(EXAMPLE_INPUT).expect("Failed to parse lines");

    let ans: Vec<isize> = reports.iter().map(|r| r.solution(Side::Back)).collect();
    assert_eq!(&ans, &[18, 28, 68]);
  }

  #[test]
  fn part1_solution() {
    let reports: Vec<Report> = Report::load_all(INPUT).expect("Failed to parse lines");

    let ans: isize = reports.iter().map(|r| r.solution(Side::Back)).sum();
    assert_eq!(ans, 2175229206);
  }

  #[test]
  fn part2_example() {
    let reports: Vec<Report> = Report::load_all(EXAMPLE_INPUT).expect("Failed to parse lines");

    let ans: Vec<isize> = reports.iter().map(|r| r.solution(Side::Front)).collect();
    assert_eq!(&ans, &[-3, 0, 5]);
  }

  #[test]
  fn part2_solution() {
    let reports: Vec<Report> = Report::load_all(INPUT).expect("Failed to parse lines");

    let ans: isize = reports.iter().map(|r| r.solution(Side::Front)).sum();
    assert_eq!(ans, 942);
  }
}
