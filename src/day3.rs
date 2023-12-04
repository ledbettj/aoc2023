use std::{collections::HashMap, str::FromStr};

use regex::Regex;

const INPUT: &'static str = include_str!("../inputs/day3.txt");

#[derive(Debug, Clone, Copy)]
pub enum PartValue {
  Number(usize),
  Symbol(char),
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
  value: PartValue,
  len: usize,
}

impl Part {
  pub fn is_number(&self) -> bool {
    if let PartValue::Number(_) = self.value {
      true
    } else {
      false
    }
  }

  pub fn is_symbol(&self) -> bool {
    !self.is_number()
  }
}

#[derive(Debug)]
pub struct Engine {
  pub parts: HashMap<(isize, isize), Part>,
}

impl FromStr for Engine {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = HashMap::new();
    let r = Regex::new(r"\d+").expect("Failed to compile regex");

    s.trim().lines().enumerate().for_each(|(y, line)| {
      line
        .chars()
        .enumerate()
        .filter(|(_, ch)| !ch.is_digit(10) && *ch != '.')
        .for_each(|(x, ch)| {
          parts.insert(
            (x as isize, y as isize),
            Part {
              value: PartValue::Symbol(ch),
              len: 1,
            },
          );
        });

      r.captures_iter(line).for_each(|capture| {
        let m = capture.get(0).unwrap();
        let v: usize = m.as_str().parse().unwrap();
        let part = Part {
          value: PartValue::Number(v),
          len: m.len(),
        };
        let x = m.start();
        parts.insert((x as isize, y as isize), part);
      });
    });

    Ok(Engine { parts })
  }
}

impl Engine {
  // pub fn gears(&self) -> Vec<usize> {
  //   self
  //     .parts
  //     .iter()
  //     .filter(|&(pos, p)|{
  //       if let PartValue::Symbol(ch) = p.value {
  //         ch == '*'
  //       } else {
  //         false
  //       }
  //     })

  // }

  pub fn symbol_adjacent_parts(&self) -> Vec<usize> {
    self
      .parts
      .iter()
      .filter(|&(pos, p)| p.is_number() && self.is_symbol_adjacent(*pos, p.len))
      .map(|(_, p)| match p.value {
        PartValue::Number(n) => n,
        _ => unreachable!(),
      })
      .collect()
  }

  fn is_symbol_adjacent(&self, (x, y): (isize, isize), len: usize) -> bool {
    for test_x in (x - 1)..(x + 1 + len as isize) {
      let above = (test_x, y - 1);
      let below = (test_x, y + 1);

      if self.parts.get(&above).is_some_and(|part| part.is_symbol()) {
        return true;
      }
      if self.parts.get(&below).is_some_and(|part| part.is_symbol()) {
        return true;
      }
    }

    let before = (x - 1, y);
    let after = (x + len as isize, y);

    if self.parts.get(&before).is_some_and(|part| part.is_symbol()) {
      return true;
    }
    if self.parts.get(&after).is_some_and(|part| part.is_symbol()) {
      return true;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

  #[test]
  fn part1_example() {
    let e: Engine = EXAMPLE_INPUT.parse().expect("Failed to parse!");
    let s: usize = e.symbol_adjacent_parts().iter().sum();

    assert_eq!(s, 4361);
  }

  #[test]
  fn part1_solution() {
    let e: Engine = INPUT.parse().expect("Failed to parse!");
    let s: usize = e.symbol_adjacent_parts().iter().sum();

    assert_eq!(s, 521515);
  }

  #[test]
  fn part2_example() {}

  #[test]
  fn part2_solution() {}
}
