use crate::ParseError;
use crate::parse_error;

use colored::Colorize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

const INPUT : &'static str = include_str!("../inputs/day10.txt");

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Pipes {
  Vertical,
  Horizontal,
  NorthEast,
  NorthWest,
  SouthWest,
  SouthEast,
  Start,
}

impl Pipes {
  pub fn next(&self, pos: (isize, isize), last: (isize, isize)) -> (isize, isize) {
    let neighbors = self.neighbors(pos);
    *neighbors.iter().find(|&&p| p != last).unwrap()
  }

  pub fn neighbors(&self, (x, y): (isize, isize)) -> [(isize, isize); 2] {
    match self {
      Pipes::Vertical => [(x, y - 1), (x, y + 1)],
      Pipes::Horizontal => [(x -1, y), (x + 1, y)],
      Pipes::NorthEast => [(x, y - 1), (x + 1, y)],
      Pipes::NorthWest => [(x, y - 1), (x - 1, y)],
      Pipes::SouthWest => [(x, y + 1), (x - 1, y)],
      Pipes::SouthEast => [(x, y + 1), (x + 1, y)],
      Pipes::Start => unreachable!()
    }
  }
}

impl Display for Pipes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Pipes::Vertical => '\u{2502}',
      Pipes::Horizontal => '\u{2500}',
      Pipes::NorthEast => '\u{2514}',
      Pipes::NorthWest => '\u{2518}',
      Pipes::SouthWest => '\u{2510}',
      Pipes::SouthEast => '\u{250c}',
      Pipes::Start => '?',
    })
  }
}

impl TryFrom<char> for Pipes {
  type Error = ParseError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '|' => Ok(Pipes::Vertical),
      '-' => Ok(Pipes::Horizontal),
      'L' => Ok(Pipes::NorthEast),
      'J' => Ok(Pipes::NorthWest),
      '7' => Ok(Pipes::SouthWest),
      'F' => Ok(Pipes::SouthEast),
      'S' => Ok(Pipes::Start),
      ch  => Err(parse_error!("Invalid pipe character: {}", ch)),
    }
  }
}

pub struct Grid {
  pipes: HashMap<(isize, isize), Pipes>,
  path: HashSet<(isize, isize)>,
}

impl FromStr for Grid {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let pipes = s
      .lines()
      .enumerate()
      .flat_map(|(y, line)|{
        line
          .chars()
          .enumerate()
          .filter(|&(_, ch)| ch != '.')
          .map(move |(x, ch)| match ch.try_into() {
            Ok(pipe) => Ok(((x as isize, y as isize), pipe)),
            Err(e) => Err(e),
          })
      })
      .collect::<Result<_,_>>()?;

    Ok(Self { pipes, path: HashSet::new() })
  }
}

impl Grid {
  pub fn start(&self) -> (isize, isize) {
    *self
      .pipes
      .iter()
      .find(|&(_, &v)| v == Pipes::Start )
      .expect("No start position in map")
      .0
  }

  pub fn set(&mut self, pos: (isize, isize), value: Pipes) {
    self.pipes.insert(pos, value);
  }

  pub fn loop_size(&mut self, start: (isize, isize)) -> usize {
    // let mut seen = HashSet::new();
    // let start = from;
    let mut current = start;
    let mut last = *self.pipes[&start].neighbors(start).first().unwrap();
    let mut count = 0;
    self.path.clear();

    loop {
      self.path.insert(current);
      let pipe = self.pipes[&current];
      let next = pipe.next(current, last);
      last = current;
      current = next;
      count += 1;
      if current == start {
        break count;
      }
    }
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_x = self.pipes.keys().max_by_key(|e| e.0).unwrap().0;
    let max_y = self.pipes.keys().max_by_key(|e| e.1).unwrap().1;

    for y in 0..=max_y {
      for x in 0..=max_x {
        if let Some(p) = self.pipes.get(&(x, y)) {
          if self.path.contains(&(x, y)) {
            write!(f, "{}", format!("{}", p).red())?;
          } else {
            write!(f, "{}", p)?;
          }
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT : &'static str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

  #[test]
  fn part1_example() {
    let mut g: Grid = EXAMPLE_INPUT.parse().expect("Failed to parse grid");
    let s = g.start();
    println!("{}", g);
    g.set(s, Pipes::SouthEast);
    assert_eq!(g.loop_size(s), 16);
  }

  #[test]
  fn part1_solution() {
    let mut g: Grid = INPUT.parse().expect("Failed to parse grid");
    let s = g.start();
    g.set(s, Pipes::Horizontal);
    let ans = g.loop_size(s) / 2;
    println!("{}", g);
    assert_eq!(ans, 7145);
  }

  #[test]
  fn part2_example() {

  }

  #[test]
  fn part2_solution() {

  }
}
