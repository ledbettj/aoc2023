use std::ops::Range;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day5.txt");

#[derive(Debug, Clone, Copy, Default)]
pub struct ConversionRange {
  src_start: usize,
  dest_start: usize,
  len: usize,
}

impl FromStr for ConversionRange {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split_whitespace();

    let dest_start = parts
      .next()
      .ok_or("Missing dest start")?
      .parse()
      .map_err(|_| "Failed to parse number")?;
    let src_start = parts
      .next()
      .ok_or("Missing src start")?
      .parse()
      .map_err(|_| "Failed to parse number")?;
    let len = parts
      .next()
      .ok_or("Missing len")?
      .parse()
      .map_err(|_| "Failed to parse number")?;

    Ok(ConversionRange {
      dest_start,
      src_start,
      len,
    })
  }
}

// for part2
#[derive(Debug, Clone, Copy, Default)]
pub struct SeedRange {
  start: usize,
  len: usize,
}

impl ConversionRange {
  pub fn includes(&self, n: usize) -> bool {
    n >= self.src_start && n < self.src_start + self.len
  }

  pub fn convert(&self, n: usize) -> usize {
    self.dest_start + (n - self.src_start)
  }
}

#[derive(Debug)]
pub struct Almanac {
  seeds: Vec<usize>,
  pub conversions: Vec<Vec<ConversionRange>>,
}

impl Almanac {
  pub fn part2_seed_ranges(&self) -> Vec<Range<usize>> {
    let mut i = self.seeds.iter();
    let mut results = vec![];

    while let Some(&start) = i.next() {
      let len = i.next().unwrap();
      let r = Range {
        start,
        end: start + len,
      };
      results.push(r)
    }

    results
  }

  pub fn seed_to_location(&self, seed: usize) -> usize {
    (0..self.conversions.len()).fold(seed, |value, index| self.convert(value, index))
  }

  pub fn convert(&self, value: usize, index: usize) -> usize {
    let ranges = &self.conversions[index];

    if let Some(range) = ranges.iter().find(|r| r.includes(value)) {
      range.convert(value)
    } else {
      value
    }
  }
}

impl FromStr for Almanac {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chunks = s.split("\n\n");

    let (_, seed_list) = chunks
      .next()
      .ok_or("Failed to find seeds")?
      .split_once(": ")
      .ok_or("Failed to parse seed list")?;

    let seeds = seed_list
      .split_whitespace()
      .map(|e| e.parse().expect("Failed to parse number"))
      .collect();

    let mut conversions = vec![];

    while let Some(chunk) = chunks.next() {
      let lines = chunk.lines().skip(1);
      let ranges = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;

      conversions.push(ranges);
    }

    Ok(Self { seeds, conversions })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

  #[test]
  fn part1_example() {
    let a: Almanac = EXAMPLE_INPUT.parse().expect("Failed to parse input");

    let ans = a
      .seeds
      .iter()
      .map(|seed| a.seed_to_location(*seed))
      .min()
      .unwrap();

    assert_eq!(ans, 35);
  }

  #[test]
  fn part1_solution() {
    let a: Almanac = INPUT.parse().expect("Failed to parse input");

    let ans = a
      .seeds
      .iter()
      .map(|seed| a.seed_to_location(*seed))
      .min()
      .unwrap();

    assert_eq!(ans, 346433842);
  }

  #[test]
  fn part2_example() {
    let a: Almanac = EXAMPLE_INPUT.parse().expect("Failed to parse input");

    let ans = a
      .part2_seed_ranges()
      .iter()
      .flat_map(|range| range.clone().into_iter())
      .map(|seed| a.seed_to_location(seed))
      .min()
      .unwrap();

    assert_eq!(ans, 46);
  }

  #[test]
  fn part2_solution() {
    let a: Almanac = INPUT.parse().expect("Failed to parse input");

    let ans = a
      .part2_seed_ranges()
      .iter()
      .flat_map(|range| range.clone().into_iter())
      .map(|seed| a.seed_to_location(seed))
      .min()
      .unwrap();

    assert_eq!(ans, 60294664);
  }
}
