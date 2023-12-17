use std::{convert::Infallible, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/day13.txt");

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
  Horiz,
  Vert,
}

struct Pattern {
  rows: Vec<String>,
  cols: Vec<String>,
}

impl FromStr for Pattern {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let rows: Vec<String> = s.lines().map(|s| s.into()).collect();

    let cols = (0..rows[0].len())
      .map(|i| {
        (0..rows.len())
          .map(|j| rows[j].chars().nth(i).unwrap())
          .collect()
      })
      .collect::<Vec<_>>();

    Ok(Self { rows, cols })
  }
}

impl Pattern {
  pub fn load_all(s: &str) -> Vec<Pattern> {
    s.trim()
      .split("\n\n")
      .map(|block| block.parse().unwrap())
      .collect()
  }

  fn diff_count(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(ca, cb)| ca != cb).count()
  }

  pub fn split(&self, items: &[String]) -> Option<usize> {
    let mut candidates = vec![];

    for i in 1..items.len() {
      let prev = &items[i - 1];
      let curr = &items[i];

      if prev == curr {
        candidates.push((i - 1, i));
      }
    }

    for (l, r) in candidates {
      let mut offset = 1;
      loop {
        let nl = items.get(l - offset);
        let nr = items.get(r + offset);
        match (nl, nr) {
          (None, _) | (_, None) => return Some(l + 1),
          (Some(a), Some(b)) if a == b => {}
          (Some(_), Some(_)) => break,
        };
        offset += 1;
      }
    }

    None
  }

  pub fn split_smudge(&self, items: &[String], actual: Option<usize>) -> Option<usize> {
    let mut candidates = vec![];

    for i in 1..items.len() {
      let prev = &items[i - 1];
      let curr = &items[i];
      let diff = Pattern::diff_count(prev, curr);
      if diff <= 1 && Some(i) != actual {
        candidates.push((i - 1, i, diff));
      }
    }

    for (l, r, diff) in candidates {
      let mut offset = 1;
      let mut diff_remaining = 1 - diff;

      loop {
        let nl = items.get(l - offset);
        let nr = items.get(r + offset);

        match (nl, nr) {
          (None, _) | (_, None) => return Some(l + 1),
          (Some(a), Some(b)) => {
            let diff = Pattern::diff_count(a, b);
            if diff > diff_remaining {
              break;
            }
            diff_remaining -= diff;
          }
        };
        offset += 1;
      }
    }

    None
  }

  pub fn reflection(&self) -> (usize, Dir) {
    self
      .split(&self.rows)
      .map(|index| (index, Dir::Horiz))
      .unwrap_or_else(|| (self.split(&self.cols).unwrap(), Dir::Vert))
  }

  pub fn smudge_reflection(&self) -> (usize, Dir) {
    let (v, d) = self.reflection();
    let vh = if d == Dir::Horiz { Some(v) } else { None };
    let vv = if d == Dir::Vert { Some(v) } else { None };

    self
      .split_smudge(&self.rows, vh)
      .map(|index| (index, Dir::Horiz))
      .unwrap_or_else(|| (self.split_smudge(&self.cols, vv).unwrap(), Dir::Vert))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

  #[test]
  fn part1_example() {
    let patterns = Pattern::load_all(EXAMPLE_INPUT);
    let ans: usize = patterns
      .iter()
      .map(|p| p.reflection())
      .map(|(v, d)| match d {
        Dir::Vert => v,
        Dir::Horiz => v * 100,
      })
      .sum();
    assert_eq!(ans, 405);
  }

  #[test]
  fn part1_solution() {
    let patterns = Pattern::load_all(INPUT);
    let ans: usize = patterns
      .iter()
      .map(|p| p.reflection())
      .map(|(v, d)| match d {
        Dir::Vert => v,
        Dir::Horiz => v * 100,
      })
      .sum();
    assert_eq!(ans, 33520);
  }

  #[test]
  fn part2_example() {
    let patterns = Pattern::load_all(EXAMPLE_INPUT);
    let ans: usize = patterns
      .iter()
      .map(|p| p.smudge_reflection())
      .map(|(v, d)| match d {
        Dir::Vert => v,
        Dir::Horiz => v * 100,
      })
      .sum();
    assert_eq!(ans, 400);
  }

  #[test]
  fn part2_solution() {
    let patterns = Pattern::load_all(INPUT);
    let ans: usize = patterns
      .iter()
      .map(|p| p.smudge_reflection())
      .map(|(v, d)| match d {
        Dir::Vert => v,
        Dir::Horiz => v * 100,
      })
      .sum();
    assert_eq!(ans, 34824);
  }
}
