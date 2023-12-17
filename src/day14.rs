use crate::parse_error;
use crate::ParseError;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day14.txt");

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Rock {
  Round,
  Cube,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Dish {
  panels: BTreeMap<(isize, isize), Rock>,
  w: isize,
  h: isize,
}

impl TryFrom<char> for Rock {
  type Error = ParseError;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      'O' => Ok(Rock::Round),
      '#' => Ok(Rock::Cube),
      _ => Err(parse_error!("Invalid rock shape: {}", c)),
    }
  }
}

impl FromStr for Dish {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let panels: BTreeMap<(isize, isize), Rock> = s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, line)| {
        line
          .chars()
          .enumerate()
          .filter_map(move |(x, ch)| match ch {
            '.' => None,
            _ => Some(ch.try_into().map(|rock| ((x as isize, y as isize), rock))),
          })
      })
      .collect::<Result<_, _>>()?;

    let w = panels.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let h = panels.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;

    Ok(Self { panels, w, h })
  }
}

impl Dish {
  pub fn slide_up(&mut self) {
    for y in 1..self.h {
      for x in 0..self.w {
        let coord = (x, y);
        if let Some(Rock::Round) = self.panels.get(&coord) {
          self.panels.remove(&coord);
          let mut move_to = (x, y);
          for offset in 1..=y {
            if let None = self.panels.get(&(x, y - offset)) {
              move_to = (x, y - offset);
            } else {
              break;
            }
          }
          self.panels.insert(move_to, Rock::Round);
        }
      }
    }
  }

  pub fn slide_down(&mut self) {
    for y in (0..self.h).rev() {
      for x in 0..self.w {
        let coord = (x, y);
        if let Some(Rock::Round) = self.panels.get(&coord) {
          self.panels.remove(&coord);
          let mut move_to = (x, y);
          for offset in 1..(self.h - y) {
            if let None = self.panels.get(&(x, y + offset)) {
              move_to = (x, y + offset);
            } else {
              break;
            }
          }
          self.panels.insert(move_to, Rock::Round);
        }
      }
    }
  }

  pub fn slide_left(&mut self) {
    for y in 0..self.h {
      for x in 1..self.w {
        let coord = (x, y);
        if let Some(Rock::Round) = self.panels.get(&coord) {
          self.panels.remove(&coord);
          let mut move_to = (x, y);
          for offset in 1..=x {
            if let None = self.panels.get(&(x - offset, y)) {
              move_to = (x - offset, y);
            } else {
              break;
            }
          }
          self.panels.insert(move_to, Rock::Round);
        }
      }
    }
  }

  pub fn slide_right(&mut self) {
    for y in 0..self.h {
      for x in (0..self.w).rev() {
        let coord = (x, y);
        if let Some(Rock::Round) = self.panels.get(&coord) {
          self.panels.remove(&coord);
          let mut move_to = (x, y);
          for offset in 1..(self.w - x) {
            if let None = self.panels.get(&(x + offset, y)) {
              move_to = (x + offset, y);
            } else {
              break;
            }
          }
          self.panels.insert(move_to, Rock::Round);
        }
      }
    }
  }

  pub fn cycle(&mut self) {
    self.slide_up();
    self.slide_left();
    self.slide_down();
    self.slide_right();
  }

  pub fn cycles(&mut self, count: usize) {
    let mut seen = HashMap::new();
    seen.insert(self.clone(), 0);

    let mut n = 0;
    while n < count {
      self.cycle();
      n += 1;
      if seen.contains_key(&self) {
        let step = n - seen.get(&self).unwrap();
        while n < count - step {
          n += step;
        }
      } else {
        seen.insert(self.clone(), n);
      }
    }
  }

  pub fn load(&self) -> isize {
    let mut value = 0;

    for y in 0..self.h {
      for x in 0..self.w {
        let coord = (x, y);
        if let Some(Rock::Round) = self.panels.get(&coord) {
          value += self.h - y;
        }
      }
    }

    value
  }
}

impl Display for Dish {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.h {
      for x in 0..self.w {
        write!(
          f,
          "{}",
          match self.panels.get(&(x, y)) {
            None => '.',
            Some(Rock::Round) => 'O',
            Some(Rock::Cube) => '#',
          }
        )?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

  #[test]
  fn part1_example() {
    let mut d: Dish = EXAMPLE_INPUT.parse().expect("Failed to parse dish");
    d.slide_up();
    //println!("{}", d);
    assert_eq!(d.load(), 136);
  }

  #[test]
  fn part1_solution() {
    let mut d: Dish = INPUT.parse().expect("Failed to parse dish");
    d.slide_up();
    //println!("{}", d);
    assert_eq!(d.load(), 108889);
  }

  #[test]
  fn part2_example() {
    let mut d: Dish = EXAMPLE_INPUT.parse().expect("Failed to parse dish");
    d.cycles(1000000000);
    //println!("{}", d);
    assert_eq!(d.load(), 64);
  }

  #[test]
  fn part2_solution() {
    let mut d: Dish = INPUT.parse().expect("Failed to parse dish");
    d.cycles(1000000000);
    //println!("{}", d);
    assert_eq!(d.load(), 104671);
  }
}
