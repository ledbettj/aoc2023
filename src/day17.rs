use std::collections::HashSet;
use std::{collections::HashMap, str::FromStr};

use crate::parse_error;
use crate::ParseError;

const INPUT: &'static str = include_str!("../inputs/day17.txt");

struct City {
  grid: HashMap<(isize, isize), usize>,
  w: isize,
  h: isize,
}

impl FromStr for City {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let grid: HashMap<(isize, isize), usize> = s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, ch)| {
          ch.to_digit(10)
            .ok_or(parse_error!("Invalid digit: {}", ch))
            .map(|d| ((x as isize, y as isize), d as usize))
        })
      })
      .collect::<Result<_, _>>()?;

    let w = grid.keys().max_by_key(|k| k.0).unwrap().0 + 1;
    let h = grid.keys().max_by_key(|k| k.1).unwrap().1 + 1;

    Ok(Self { grid, w, h })
  }
}

impl City {
  pub fn min_cost(
    &self,
    cur: usize,
    prev_dir: Option<(isize, isize)>,
    from: (isize, isize),
    to: (isize, isize),
    mut visited: &mut HashSet<(isize, isize)>,
  ) -> usize {
    if from == to {
      println!("at {:?} returning {} + {}", to, cur, self.grid[&to]);
      return 0;
    }

    let mut options = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if let Some(prev) = prev_dir {
      if let Some((index, _)) = options
        .iter()
        .enumerate()
        .find(|(index, &coord)| coord == prev)
      {
        options.remove(index);
      }
    }

    let neighbors: Vec<((isize, isize), (isize, isize))> = (1..=3)
      .flat_map(|n| {
        options
          .iter()
          .map(move |&(a, b)| ((a, b), (from.0 + a * n, from.1 + b * n)))
      })
      .filter(|(_, coord)| !visited.contains(&coord))
      .filter(|(_, coord)| self.grid.contains_key(&coord))
      .collect();
    neighbors
      .iter()
      .map(|&(dir, coord)| {
        visited.insert(coord);
        println!("checking {:?}", coord);
        cur + self.grid[&coord] + self.min_cost(0, Some(dir), coord, to, &mut visited)
      })
      .min()
      .unwrap_or(90000000)
  }

  pub fn p1(&self) -> usize {
    let mut seen = HashSet::new();
    self.min_cost(0, None, (0, 0), (self.w - 1, self.h - 1), &mut seen)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

  #[test]
  fn part1_example() {
    let c: City = EXAMPLE_INPUT.parse().expect("Failed to parse input");
    assert_eq!(c.p1(), 102);
  }

  #[test]
  fn part1_solution() {
    let c: City = INPUT.parse().expect("Failed to parse input");
    assert_eq!(c.p1(), 102);
  }

  #[test]
  fn part2_example() {}

  #[test]
  fn part2_solution() {}
}
