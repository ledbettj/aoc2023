use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

use crate::parse_error;
use crate::ParseError;

const INPUT: &'static str = include_str!("../inputs/day8.txt");

#[derive(Debug)]
enum Dir {
  Left,
  Right,
}

#[derive(Debug)]
struct Map {
  pattern: Vec<Dir>,
  nodes: HashMap<String, (String, String)>,
}

impl TryFrom<char> for Dir {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      'L' => Ok(Dir::Left),
      'R' => Ok(Dir::Right),
      _ => Err(parse_error!("Invalid direction: {}", value).into()),
    }
  }
}

impl FromStr for Map {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (top, bottom) = s
      .trim()
      .split_once("\n\n")
      .ok_or(parse_error!("No blank line"))?;

    let pattern: Vec<Dir> = top
      .chars()
      .map(|ch| ch.try_into())
      .collect::<Result<_, _>>()?;
    let r = Regex::new(r"(\w+) = \((\w+), (\w+)\)")?;

    let nodes = bottom
      .lines()
      .map(|line| {
        let c = r
          .captures(line)
          .ok_or(parse_error!("Failed to parse line: {}", line))?;
        let e = c.get(1).unwrap().as_str();
        let l = c.get(2).unwrap().as_str();
        let r = c.get(3).unwrap().as_str();
        Ok::<(String, (String, String)), Self::Err>((e.to_string(), (l.to_string(), r.to_string())))
      })
      .collect::<Result<_, _>>()?;

    Ok(Self { pattern, nodes })
  }
}

impl Map {
  pub fn step(&self, from: &str, n: usize) -> &str {
    let (l, r) = self.nodes.get(from).unwrap();
    match self.pattern[n % self.pattern.len()] {
      Dir::Left => l,
      Dir::Right => r,
    }
  }

  pub fn run_p1(&self) -> usize {
    self.steps_to_complete("AAA", |node| node == "ZZZ")
  }

  pub fn steps_to_complete<F>(&self, start: &str, end: F) -> usize
  where
    F: Fn(&str) -> bool,
  {
    let mut current = start;
    let mut steps = 0;

    while !end(current) {
      current = self.step(current, steps);
      steps += 1;
    }

    steps
  }

  pub fn run_p2(&self) -> usize {
    self
      .nodes
      .keys()
      .filter(|key| key.ends_with("A"))
      .map(|node| self.steps_to_complete(node, |n| n.ends_with("Z")))
      .reduce(|a, b| lcm(a, b))
      .unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

  const P2_EXAMPLE_INPUT: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

  #[test]
  fn part1_example() {
    let m: Map = EXAMPLE_INPUT.parse().expect("Failed to parse input");
    assert_eq!(m.run_p1(), 2);
  }

  #[test]
  fn part1_solution() {
    let m: Map = INPUT.parse().expect("Failed to parse input");
    assert_eq!(m.run_p1(), 13771);
  }

  #[test]
  fn part2_example() {
    let m: Map = P2_EXAMPLE_INPUT.parse().expect("Failed to parse input");
    assert_eq!(m.run_p2(), 6);
  }

  #[test]
  fn part2_solution() {
    let m: Map = INPUT.parse().expect("Failed to parse input");
    assert_eq!(m.run_p2(), 13129439557681);
  }
}
