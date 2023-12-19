use std::fmt::Display;
use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use crate::{helpers::ParseError, parse_error};

#[derive(Debug, Clone, Copy)]
pub enum Dir {
  Right,
  Left,
  Up,
  Down,
}

#[derive(Debug, Clone)]
struct Instr {
  dir: Dir,
  size: isize,
}

impl Instr {
  fn orientation(&self) -> (isize, isize) {
    match self.dir {
      Dir::Right => (1, 0),
      Dir::Left => (-1, 0),
      Dir::Up => (0, -1),
      Dir::Down => (0, 1),
    }
  }

  pub fn advance(&self, pos: (isize, isize), value: isize) -> (isize, isize) {
    let (x, y) = pos;
    let (sx, sy) = self.orientation();

    (x + sx * value, y + sy * value)
  }
}

#[derive(Debug)]
struct Dig {
  grid: HashSet<(isize, isize)>,
  rows: HashMap<isize, Vec<isize>>,
}

impl TryFrom<&str> for Dir {
  type Error = ParseError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "R" => Ok(Dir::Right),
      "L" => Ok(Dir::Left),
      "U" => Ok(Dir::Up),
      "D" => Ok(Dir::Down),
      chr => Err(parse_error!("Invalid dir: {}", chr)),
    }
  }
}

impl FromStr for Instr {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split_whitespace();

    let dir: Dir = parts
      .next()
      .ok_or(parse_error!("Missing direction"))?
      .try_into()?;
    let size: isize = parts.next().ok_or(parse_error!("Missing size"))?.parse()?;

    Ok(Self { dir, size })
  }
}

impl Instr {
  pub fn load(s: &str) -> Result<Vec<Instr>, anyhow::Error> {
    s.trim().lines().map(|line| line.parse()).collect()
  }

  pub fn decode(s: &str) -> Vec<Instr> {
    s.trim()
      .lines()
      .map(|line| {
        let hex = &line.split_whitespace().last().unwrap()[2..];

        let hex = &hex[..(hex.len() - 1)];
        let v = usize::from_str_radix(&hex, 16).unwrap();
        let dir = match v & 4 {
          0 => Dir::Right,
          1 => Dir::Down,
          2 => Dir::Left,
          3 => Dir::Up,
          _ => unreachable!(),
        };

        Instr {
          size: (v >> 4) as isize,
          dir,
        }
      })
      .collect()
  }
}

impl Dig {
  pub fn new() -> Self {
    Self {
      grid: HashSet::new(),
      rows: HashMap::new(),
    }
  }

  pub fn size(&self) -> usize {
    self.grid.len()
  }

  pub fn excavate(&mut self, instr: &[Instr]) {
    instr.iter().fold((0, 0), |pos, instr| {
      self
        .rows
        .entry(pos.1)
        .and_modify(|v| v.push(pos.0))
        .or_insert_with(|| vec![pos.0]);

      (0..instr.size).for_each(|offset| {
        self.grid.insert(instr.advance(pos, offset));
      });
      instr.advance(pos, instr.size)
    });
  }

  pub fn fill(&mut self, start: (isize, isize)) {
    let mut to_visit = vec![];
    to_visit.push(start);

    while let Some(pos) = to_visit.pop() {
      if !self.grid.contains(&pos) {
        self.grid.insert(pos);
        for xo in -1..=1 {
          for yo in -1..=1 {
            to_visit.push((pos.0 + xo, pos.1 + yo));
          }
        }
      }
    }
  }
}

impl Display for Dig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_h = self.grid.iter().max_by_key(|(_, y)| y).unwrap().1 + 1;
    let max_w = self.grid.iter().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let min_h = self.grid.iter().min_by_key(|(_, y)| y).unwrap().1;
    let min_w = self.grid.iter().min_by_key(|(x, _)| x).unwrap().0;

    for y in min_h..max_h {
      for x in min_w..max_w {
        write!(
          f,
          "{}",
          if self.grid.contains(&(x, y)) {
            '#'
          } else {
            '.'
          }
        )?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

const INPUT: &'static str = include_str!("../inputs/day18.txt");

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

  #[test]
  fn part1_example() {
    let instructions = Instr::load(EXAMPLE_INPUT).expect("Failed to load instructions");
    let mut dig = Dig::new();
    dig.excavate(&instructions);
    //dig.fill((1,1));
    println!("{:?}", dig.rows);
    println!("{}", dig);
    assert_eq!(dig.size(), 62);
  }

  #[test]
  fn part1_solution() {
    // let instructions = Instr::load(INPUT).expect("Failed to load instructions");
    // let mut dig = Dig::new();
    // dig.excavate(&instructions);
    // dig.fill((1,1));
    // assert_eq!(dig.size(), 53844);
  }

  #[test]
  fn part2_example() {
    // let instructions = Instr::decode(EXAMPLE_INPUT);
    // println!("{:?}", instructions);
    // let mut dig = Dig::new();
    // dig.excavate(&instructions);
    // dig.fill((1,1));
    // assert_eq!(dig.size(), 53844);
  }

  #[test]
  fn part2_solution() {}
}
