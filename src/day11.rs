use std::{collections::HashSet, convert::Infallible, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/day11.txt");

struct Universe {
  galaxies: Vec<(isize, isize)>,
  rows: HashSet<isize>,
  cols: HashSet<isize>,
  pub cost: usize,
}

impl FromStr for Universe {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let galaxies = s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, line)| {
        line
          .chars()
          .enumerate()
          .filter(|&(_, ch)| ch == '#')
          .map(move |(x, _)| (x as isize, y as isize))
      })
      .collect::<Vec<(isize, isize)>>();

    for &(x, y) in galaxies.iter() {
      rows.insert(y);
      cols.insert(x);
    }

    Ok(Self {
      galaxies,
      rows,
      cols,
      cost: 2,
    })
  }
}

impl Universe {
  fn min_dist(&self, (ax, ay): (isize, isize), (bx, by): (isize, isize)) -> usize {
    let sx = ax.min(bx);
    let sy = ay.min(by);

    let ex = ax.max(bx);
    let ey = ay.max(by);

    let xsum: usize = (sx..ex)
      .map(|x| if self.cols.contains(&x) { 1 } else { self.cost })
      .sum();

    let ysum: usize = (sy..ey)
      .map(|y| if self.rows.contains(&y) { 1 } else { self.cost })
      .sum();

    xsum + ysum
  }

  fn p1_solution(&self) -> usize {
    self
      .galaxies
      .iter()
      .map(|&a|{
        self
          .galaxies
          .iter()
          .filter_map(|&b| if a != b { Some(self.min_dist(a, b)) } else { None })
          .sum::<usize>()
      }).sum::<usize>() / 2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

  #[test]
  fn part1_example() {
    let u: Universe = EXAMPLE_INPUT.parse().unwrap();
    assert_eq!(u.p1_solution(), 374);
  }

  #[test]
  fn part1_solution() {
    let u: Universe = INPUT.parse().unwrap();
    assert_eq!(u.p1_solution(), 9521550);
  }

  #[test]
  fn part2_example() {
    let mut u: Universe = EXAMPLE_INPUT.parse().unwrap();
    u.cost = 1000000;
    assert_eq!(u.p1_solution(), 82000210);
  }

  #[test]
  fn part2_solution() {
    let mut u: Universe = INPUT.parse().unwrap();
    u.cost = 1000000;
    assert_eq!(u.p1_solution(), 298932923702);
  }
}
