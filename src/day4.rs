use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

const INPUT: &'static str = include_str!("../inputs/day4.txt");

pub struct Card {
  winners: Vec<usize>,
  numbers: HashSet<usize>,
}

impl FromStr for Card {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(&[':', '|']).skip(1);

    let winners = parts
      .next()
      .ok_or("No winning numbers found")?
      .split_whitespace()
      .map(|num| {
        num
          .parse::<usize>()
          .map_err(|_| "Failed to parse winning number")
      })
      .collect::<Result<_, _>>()?;

    let numbers = parts
      .next()
      .ok_or("No card numbers found")?
      .split_whitespace()
      .map(|num| {
        num
          .parse::<usize>()
          .map_err(|_| "Failed to parse card number")
      })
      .collect::<Result<_, _>>()?;

    Ok(Self { winners, numbers })
  }
}

impl Card {
  pub fn load_all(s: &'static str) -> Result<Vec<Card>, &'static str> {
    s.lines().map(|line| line.parse()).collect()
  }

  pub fn run_p2(cards: Vec<Card>) -> usize {
    let r = 0..(cards.len());
    let mut total = 0;

    // map of card number => number of instances of that card.
    // initially we have 1 of all the supplied cards
    let mut counts: HashMap<usize, usize> = r.map(|index| (index, 1)).collect();

    while !counts.is_empty() {
      for index in 0..(cards.len()) {
        let card = cards.get(index).unwrap();
        let count = *counts.get(&index).unwrap_or(&0);

        counts.remove(&index);
        total += count;

        // if we don't have any of this card, ignore
        if count == 0 {
          continue;
        }

        // tally these cards and remove them from processing
        let wins = card.wins();

        if wins != 0 {
          // add new cards for each instance of the card that won
          let range = (index + 1)..(index + 1 + wins);
          range.for_each(|i| {
            counts.entry(i).and_modify(|v| *v += count).or_insert(count);
          });
        }
      }
    }

    total
  }

  pub fn wins(&self) -> usize {
    self
      .winners
      .iter()
      .filter(|winner| self.numbers.contains(winner))
      .count()
  }

  pub fn score(&self) -> usize {
    match self.wins() {
      0 => 0,
      n => 2_usize.pow(n as u32 - 1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

  #[test]
  fn part1_example() {
    let cards = Card::load_all(EXAMPLE_INPUT).expect("Failed to load cards");
    let score: usize = cards.iter().map(|c| c.score()).sum();
    assert_eq!(score, 13);
  }

  #[test]
  fn part1_solution() {
    let cards = Card::load_all(INPUT).expect("Failed to load cards");
    let score: usize = cards.iter().map(|c| c.score()).sum();
    assert_eq!(score, 21821);
  }

  #[test]
  fn part2_example() {
    let cards = Card::load_all(EXAMPLE_INPUT).expect("Failed to load cards");
    let ans = Card::run_p2(cards);
    assert_eq!(ans, 30);
  }

  #[test]
  fn part2_solution() {
    let cards = Card::load_all(INPUT).expect("Failed to load cards");
    let ans = Card::run_p2(cards);
    assert_eq!(ans, 5539496);
  }
}
