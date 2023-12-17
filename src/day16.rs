use std::fmt::Display;
use std::{
  collections::{HashMap, HashSet},
  convert::Infallible,
  str::FromStr,
};

const INPUT: &'static str = include_str!("../inputs/day16.txt");

struct Mirror {
  grid: HashMap<(isize, isize), char>,
  energy: HashSet<(isize, isize)>,
  w: isize,
  h: isize,
}

impl FromStr for Mirror {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let grid: HashMap<(isize, isize), char> = s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, line)| {
        line
          .chars()
          .enumerate()
          .filter_map(move |(x, ch)| match ch {
            '.' => None,
            _ => Some(((x as isize, y as isize), ch)),
          })
      })
      .collect();

    let w = grid.keys().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let h = grid.keys().max_by_key(|(_, y)| y).unwrap().1 + 1;

    Ok(Self {
      grid,
      energy: HashSet::new(),
      w,
      h,
    })
  }
}

impl Mirror {
  pub fn start(&mut self) -> usize {
    self.start_from((0, 0), (1, 0))
  }

  pub fn max_energy(&mut self) -> usize {
    let mut max = 0;

    for y in 0..self.h {
      max = max.max(self.start_from((0, y), (1, 0)));
      max = max.max(self.start_from((self.w - 1, y), (-1, 0)));
    }
    for x in 0..self.w {
      max = max.max(self.start_from((x, 0), (0, 1)));
      max = max.max(self.start_from((x, self.h - 1), (0, -1)));
    }

    max
  }

  pub fn start_from(&mut self, pos: (isize, isize), dir: (isize, isize)) -> usize {
    let mut seen = HashSet::new();
    self.energy.clear();
    self.energize(pos, dir, &mut seen);
    self.count()
  }

  pub fn count(&self) -> usize {
    self.energy.len()
  }

  pub fn energize(
    &mut self,
    mut pos: (isize, isize),
    mut dir: (isize, isize),
    seen: &mut HashSet<((isize, isize), (isize, isize))>,
  ) {
    while pos.0 >= 0 && pos.0 < self.w && pos.1 >= 0 && pos.1 < self.h {
      if seen.contains(&(pos, dir)) {
        break;
      }
      seen.insert((pos, dir));
      self.energy.insert(pos);
      dir = match self.grid.get(&pos) {
        None => dir,
        Some('|') => match dir {
          (0, n) => (0, n),
          (_, 0) => {
            self.energize((pos.0, pos.1 - 1), (0, -1), seen);
            self.energize((pos.0, pos.1 + 1), (0, 1), seen);
            return;
          }
          (_, _) => unreachable!(),
        },
        Some('-') => match dir {
          (n, 0) => (n, 0),
          (0, _) => {
            self.energize((pos.0 - 1, pos.1), (-1, 0), seen);
            self.energize((pos.0 + 1, pos.1), (1, 0), seen);
            return;
          }
          (_, _) => unreachable!(),
        },
        Some('\\') => (dir.1, dir.0),
        Some('/') => (-dir.1, -dir.0),
        Some(_) => unreachable!(),
      };
      pos.0 += dir.0;
      pos.1 += dir.1;
    }
  }
}

impl Display for Mirror {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.h {
      for x in 0..self.w {
        write!(
          f,
          "{}",
          if self.energy.contains(&(x, y)) {
            '#'
          } else {
            match self.grid.get(&(x, y)) {
              None => '.',
              Some(&ch) => ch,
            }
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

  const EXAMPLE_INPUT: &'static str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

  #[test]
  fn part1_example() {
    let mut m: Mirror = EXAMPLE_INPUT.parse().expect("Failed to parse Mirror");
    assert_eq!(m.start(), 46);
  }

  #[test]
  fn part1_solution() {
    let mut m: Mirror = INPUT.parse().expect("Failed to parse Mirror");
    assert_eq!(m.start(), 6978);
  }

  #[test]
  fn part2_example() {
    let mut m: Mirror = EXAMPLE_INPUT.parse().expect("Failed to parse Mirror");
    assert_eq!(m.max_energy(), 51);
  }

  #[test]
  fn part2_solution() {
    let mut m: Mirror = INPUT.parse().expect("Failed to parse Mirror");
    assert_eq!(m.max_energy(), 7315);
  }
}
