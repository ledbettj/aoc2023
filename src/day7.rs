use std::{cmp::Ordering, str::FromStr};

pub(crate) use crate::parse_error;
use crate::ParseError;

const INPUT: &'static str = include_str!("../inputs/day7.txt");

// PartialOrd/Ord derive here is just the order of the types, which is perfect
// https://doc.rust-lang.org/core/cmp/trait.Ord.html#derivable
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

// we implement PartialOrd below, so Ord uses that + Eq
#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Hand {
  cards: Vec<u32>,
  bid: u32,
  rank: HandRank,
}

impl Hand {
  pub fn load_all(s: &'static str) -> Result<Vec<Hand>, anyhow::Error> {
    s.lines().map(|line| line.parse()).collect()
  }

  pub fn score(hands: &mut [Hand]) -> usize {
    hands.sort();

    hands
      .iter()
      .enumerate()
      .map(|(index, hand)| (index + 1) * hand.bid as usize)
      .sum()
  }

  // for part2: replace Jack(11) with Joker(0) then re-calculate score
  pub fn rescore_with_jokers(&mut self) {
    for n in self.cards.iter_mut() {
      if *n == 11 {
        *n = 0;
      }
    }
    self.rank = Hand::rank(&self.cards)
  }

  pub fn new(cards: Vec<u32>, bid: u32) -> Self {
    let rank = Hand::rank(&cards);
    Self { cards, bid, rank }
  }

  fn rank(cards: &[u32]) -> HandRank {
    let mut buckets = [0u32; 15];
    let mut jokers = 0;

    // count how many instances of each card we have.
    cards.iter().for_each(|n| {
      if *n == 0 {
        // count jokers seperately.
        jokers += 1;
      } else {
        buckets[*n as usize] += 1;
      }
    });

    // the max number of the same card we have determines our hand.
    let max = buckets.iter().max().unwrap();

    match max + jokers {
      5 => HandRank::FiveOfAKind,
      4 => HandRank::FourOfAKind,
      3 => {
        // could be full house or just 3 of a kind.
        let min = buckets.iter().filter(|&&n| n != 0).min().unwrap();
        match min {
          2 => HandRank::FullHouse,
          _ => HandRank::ThreeOfAKind,
        }
      }
      2 => {
        // could be two pair or just one pair
        let pairs = buckets.iter().filter(|&&n| n == 2).count();
        match pairs {
          2 => HandRank::TwoPair,
          _ => HandRank::OnePair,
        }
      }
      1 => HandRank::HighCard,
      _ => unreachable!("More than 5 of a kind of no cards.  what?"),
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self
      .rank
      .partial_cmp(&other.rank) // order by rank
      .map(|ord| match ord {
        // if rank is the same, order by cards
        Ordering::Equal => self.cards.partial_cmp(&other.cards).unwrap(),
        _ => ord,
      })
  }
}

impl FromStr for Hand {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (cards, bid) = s
      .split_once(" ")
      .ok_or(parse_error!("Missing space in hand definition: '{}'", s))?;

    let bid = bid.parse()?;

    let cards: Vec<u32> = cards
      .chars()
      .map(|ch| match ch {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(11),
        'T' => Ok(10),
        n => ch
          .to_digit(10)
          .ok_or_else(|| parse_error!("Invalid character: {}", n)),
      })
      .collect::<Result<_, _>>()?;

    Ok(Hand::new(cards, bid))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

  #[test]
  fn part1_example() {
    let mut hands = Hand::load_all(EXAMPLE_INPUT).expect("Failed to load hands");
    assert_eq!(Hand::score(&mut hands), 6_440);
  }

  #[test]
  fn part1_solution() {
    let mut hands = Hand::load_all(INPUT).expect("Failed to load hands");
    assert_eq!(Hand::score(&mut hands), 250_957_639);
  }

  #[test]
  fn part2_example() {
    let mut hands = Hand::load_all(EXAMPLE_INPUT).expect("Failed to load hands");
    hands.iter_mut().for_each(|h| h.rescore_with_jokers());

    assert_eq!(Hand::score(&mut hands), 5_905);
  }

  #[test]
  fn part2_solution() {
    let mut hands = Hand::load_all(INPUT).expect("Failed to load hands");
    hands.iter_mut().for_each(|h| h.rescore_with_jokers());

    assert_eq!(Hand::score(&mut hands), 251_515_496);
  }
}
