use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day5.txt");

#[derive(Debug, Clone)]
pub struct Entry {
  category: String,
  value: usize,
}

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
  seeds: Vec<Entry>,
  pub conversions: HashMap<String, Vec<ConversionRange>>,
}

impl Entry {
  pub fn new(category: &str, value: usize) -> Self {
    Self {
      category: category.into(),
      value,
    }
  }

  pub fn convert(&self, to: &str, a: &Almanac) -> Option<Entry> {
    let conversion_type = format!("{}-to-{}", self.category, to);
    a.conversions.get(&conversion_type).map(|ranges| {
      let v = if let Some(range) = ranges.iter().find(|r| r.includes(self.value)) {
        range.convert(self.value)
      } else {
        self.value
      };
      Entry {
        category: to.into(),
        value: v,
      }
    })
  }
}

impl Almanac {
  pub fn part2_seed_ranges(&self) -> Vec<Range<usize>> {
    let mut i = self.seeds.iter();
    let mut results = vec![];

    while let Some(start) = i.next() {
      let len = i.next().unwrap();
      let r = Range {
        start: start.value,
        end: start.value + len.value,
      };
      results.push(r)
    }

    results
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
    println!("seed list {:?}", seed_list);
    let seeds = seed_list
      .split_whitespace()
      .map(|e| Entry::new("seed", e.parse().expect("Failed to parse number")))
      .collect();

    let mut conversions = HashMap::new();

    while let Some(chunk) = chunks.next() {
      let mut lines = chunk.lines();
      let (conversion_type, _) = lines
        .next()
        .ok_or("Failed to get conversion map type")?
        .split_once(" ")
        .ok_or("Failed to parse conversion map")?;

      let ranges = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;

      conversions.insert(conversion_type.to_string(), ranges);
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
    let path = [
      "soil",
      "fertilizer",
      "water",
      "light",
      "temperature",
      "humidity",
      "location",
    ];

    let ans = a
      .seeds
      .iter()
      .map(|seed| {
        path
          .iter()
          .fold(seed.clone(), |e, &next| {
            e.convert(next, &a).expect("Failed to convert!")
          })
          .value
      })
      .min()
      .unwrap();

    assert_eq!(ans, 35);
  }

  #[test]
  fn part1_solution() {
    let a: Almanac = INPUT.parse().expect("Failed to parse input");
    let path = [
      "soil",
      "fertilizer",
      "water",
      "light",
      "temperature",
      "humidity",
      "location",
    ];

    let ans = a
      .seeds
      .iter()
      .map(|seed| {
        path
          .iter()
          .fold(seed.clone(), |e, &next| {
            e.convert(next, &a).expect("Failed to convert!")
          })
          .value
      })
      .min()
      .unwrap();

    assert_eq!(ans, 346433842);
  }

  #[test]
  fn part2_example() {
    let a: Almanac = EXAMPLE_INPUT.parse().expect("Failed to parse input");
    let path = [
      "soil",
      "fertilizer",
      "water",
      "light",
      "temperature",
      "humidity",
      "location",
    ];

    let ans = a
      .part2_seed_ranges()
      .iter()
      .flat_map(|range| range.clone().map(|n| Entry::new("seed", n)))
      .map(|seed| {
        path
          .iter()
          .fold(seed.clone(), |e, &next| {
            e.convert(next, &a).expect("Failed to convert!")
          })
          .value
      })
      .min()
      .unwrap();

    assert_eq!(ans, 46);
  }

  #[test]
  fn part2_solution() {
    let a: Almanac = INPUT.parse().expect("Failed to parse input");
    let path = [
      "soil",
      "fertilizer",
      "water",
      "light",
      "temperature",
      "humidity",
      "location",
    ];

    let ans = a
      .part2_seed_ranges()
      .iter()
      .flat_map(|range| range.clone().map(|n| Entry::new("seed", n)))
      .map(|seed| {
        path
          .iter()
          .fold(seed.clone(), |e, &next| {
            e.convert(next, &a).expect("Failed to convert!")
          })
          .value
      })
      .min()
      .unwrap();

    assert_eq!(ans, 60294664);
  }
}
