use std::collections::HashSet;
use std::str::FromStr;

use regex::Regex;

use crate::parse_error;
use crate::ParseError;

const INPUT: &'static str = include_str!("../inputs/day12.txt");
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Spring {
  chars: Vec<char>,
  counts: Vec<usize>,
}

impl FromStr for Spring {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (left, right) = s
      .split_once(" ")
      .ok_or(parse_error!("No space found in line"))?;

    let chars = left.chars().collect();
    let counts = right
      .split(",")
      .map(|n| n.parse())
      .collect::<Result<_, _>>()?;

    Ok(Self { chars, counts })
  }
}

impl Spring {
  pub fn to_string(&self) -> String {
    self.chars.iter().collect()
  }

  pub fn load_all(s: &str) -> Result<Vec<Spring>, anyhow::Error> {
    s.trim().lines().map(|line| line.parse()).collect()
  }

  pub fn arrangements(&self) -> HashSet<Self> {
    let mut set = HashSet::new();
    if !self.is_valid() {
      return set;
    }

    if self.is_final() {
      set.insert(self.clone());
    } else if let Some((index, _)) = self.chars.iter().enumerate().find(|(_, &ch)| ch == '?') {
      for item in self.child(index, '.').arrangements() {
        set.insert(item);
      }
      for item in self.child(index, '#').arrangements() {
        set.insert(item);
      }
    }

    set
  }

  pub fn child(&self, pos: usize, ch: char) -> Spring {
    let mut s = self.clone();
    s.chars[pos] = ch;
    s
  }

  pub fn is_final(&self) -> bool {
    self.chars.iter().all(|&ch| ch != '?')
  }

  pub fn is_valid(&self) -> bool {
    let fmt = self
      .counts
      .iter()
      .map(|&c| format!(r"(#|\?){{{}}}", c))
      .collect::<Vec<String>>()
      .join(r"(\.|\?)+");
    let fmt = format!(r"^[^#]*{}[^#]*$", fmt);
    //println!("fmt: {} | s: {}", fmt, self.to_string());
    let r = Regex::new(&fmt).unwrap();
    r.is_match(&self.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

  #[test]
  fn part1_example() {
    let springs: Vec<Spring> = Spring::load_all(EXAMPLE_INPUT).expect("Failed to load input");
    let ans: Vec<usize> = springs.iter().map(|s| s.arrangements().len()).collect();
    assert_eq!(ans, vec![1, 4, 1, 1, 4, 10]);
    // let ans = springs[5].arrangements();
    // println!("{:?}", springs[5].counts);
    // for a in &ans { println!("{} {}", a.to_string(), a.is_valid()); }
  }

  #[test]
  fn part1_solution() {
    let springs: Vec<Spring> = Spring::load_all(INPUT).expect("Failed to load input");
    let ans: usize = springs.iter().map(|s| s.arrangements().len()).sum();
    assert_eq!(ans, 7694);
  }

  #[test]
  fn part2_example() {}

  #[test]
  fn part2_solution() {}
}
