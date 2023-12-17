use std::{collections::HashMap, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/day15.txt");

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Lense {
  pub label: String,
  pub focal_length: usize,
}

#[derive(Debug)]
pub struct Boxes {
  pub boxes: Vec<Vec<Lense>>,
}

impl Boxes {
  pub fn new() -> Self {
    Self {
      boxes: vec![vec![]; 256],
    }
  }
  pub fn power(&self) -> usize {
    self
      .boxes
      .iter()
      .enumerate()
      .map(|(box_no, lenses)| {
        lenses
          .iter()
          .enumerate()
          .map(move |(slot_no, lense)| (box_no + 1) * (slot_no + 1) * lense.focal_length)
          .sum::<usize>()
      })
      .sum::<usize>()
  }

  pub fn eval(&mut self, instr: &str) {
    instr.trim().split(",").for_each(|text| {
      match text.split_once(&['-', '=']) {
        None => panic!("Failed to split text {}", text),
        Some((label, "")) => {
          // -
          let h = hash(label);
          if let Some((i, _)) = self.boxes[h]
            .iter()
            .enumerate()
            .find(|(index, lense)| lense.label == label)
          {
            self.boxes[h].remove(i);
          }
        }
        Some((label, focal_length)) => {
          // =
          let focal_length: usize = focal_length.parse().unwrap();
          let h = hash(label);
          if let Some((i, _)) = self.boxes[h]
            .iter()
            .enumerate()
            .find(|(index, lense)| lense.label == label)
          {
            self.boxes[h][i].focal_length = focal_length;
          } else {
            self.boxes[h].push(Lense {
              label: label.into(),
              focal_length,
            });
          }
        }
      }
    })
  }
}

pub fn hash(s: &str) -> usize {
  s.trim().chars().fold(0, |accum, ch| {
    let v = (ch as u32) as usize;
    ((accum + v) * 17) % 256
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

  #[test]
  fn part1_example() {
    assert_eq!(hash("HASH"), 52);

    let ans: usize = EXAMPLE_INPUT.split(",").map(|s| hash(s)).sum();

    assert_eq!(ans, 1320);
  }

  #[test]
  fn part1_solution() {
    let ans: usize = INPUT.split(",").map(|s| hash(s)).sum();

    assert_eq!(ans, 513158);
  }

  #[test]
  fn part2_example() {
    let mut b = Boxes::new();
    b.eval(EXAMPLE_INPUT);
    assert_eq!(b.power(), 145);
  }

  #[test]
  fn part2_solution() {
    let mut b = Boxes::new();
    b.eval(INPUT);
    assert_eq!(b.power(), 200277);
  }
}
