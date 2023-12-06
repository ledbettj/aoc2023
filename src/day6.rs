const INPUT: &'static str = include_str!("../inputs/day6.txt");

#[derive(Debug, Clone, Copy, Default)]
struct Race {
  record: usize,
  time: usize,
}

impl Race {
  pub fn new(record: usize, time: usize) -> Self {
    Self { record, time }
  }

  pub fn win_counts(&self) -> usize {
    // find smallest+largest values for speed such that
    // speed * (time - speed) > record;
    let test = |n: &usize| n * (self.time - n) > self.record;
    let mut range = 1..self.time;

    if let Some(lo) = range.find(&test) {
      let hi = range.rfind(&test).unwrap();
      hi + 1 - lo
    } else {
      // no way to win
      0
    }
  }

  pub fn load_all(s: &str) -> Vec<Race> {
    let (top, bottom) = s.split_once("\n").expect("Failed to parse race listings");
    let times = top.split_whitespace().skip(1);
    let mut records = bottom.split_whitespace().skip(1);

    times
      .map(|time| {
        let time = time.parse::<usize>().expect("Failed to parse time");
        let record = records
          .next()
          .expect("Missing record")
          .parse::<usize>()
          .expect("Failed to parse distance");

        Race::new(record, time)
      })
      .collect()
  }

  pub fn p1_solution(races: &[Race]) -> usize {
    races
      .iter()
      .map(|r| r.win_counts())
      .reduce(|a, b| a * b)
      .unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200
";

  #[test]
  fn part1_example() {
    let races = Race::load_all(EXAMPLE_INPUT);
    let score = Race::p1_solution(&races);
    assert_eq!(score, 288);
  }

  #[test]
  fn part1_solution() {
    let races = Race::load_all(INPUT);
    let score = Race::p1_solution(&races);
    assert_eq!(score, 3316275);
  }

  #[test]
  fn part2_example() {
    let r = Race::new(940200, 71530);
    assert_eq!(r.win_counts(), 71503);
  }

  #[test]
  fn part2_solution() {
    let r = Race::new(233_1011_1110_1487, 40828492);
    assert_eq!(r.win_counts(), 27102791);
  }
}
