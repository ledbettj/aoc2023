use regex::Regex;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day2.txt");

pub struct Game {
  id: usize,
  plays: Vec<Play>,
}

impl Game {
  /// Given a list of game descriptors, return a list of Game objects
  ///
  /// # Example
  /// ```
  /// use aoc::day2::Game;
  ///
  /// let input = "Game 1: 3 blue\nGame 2: 3 red";
  /// let games = Game::load_all(input);
  ///
  /// assert!(games.is_ok());
  /// assert_eq!(games.unwrap().len(), 2);
  /// ```
  pub fn load_all(s: &'static str) -> Result<Vec<Game>, &'static str> {
    s.lines().map(|line| line.parse()).collect::<Result<_, _>>()
  }

  /// Determine if a game is possible with the given block constraints.
  ///
  /// # Example
  /// ```
  /// use aoc::day2::Game;
  ///
  /// let game : Game = "Game 1: 3 blue; 1 red; 2 green;".parse().expect("Failed to parse game");
  ///
  /// assert!(game.is_possible(1, 2, 3));
  /// assert!(!game.is_possible(0, 1, 5));
  /// ```
  pub fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
    self.plays.iter().all(|play| play.is_possible(r, g, b))
  }

  /// Return the minimum number of each block color required for this game.
  ///
  /// # Example
  /// ```
  /// use aoc::day2::Game;
  ///
  /// let game : Game = "Game 1: 1 red, 2 blue; 2 red; 2 green;".parse().expect("Failed to parse game");
  ///
  /// assert_eq!(game.min_blocks(), (2, 2, 2));
  /// ```
  pub fn min_blocks(&self) -> (usize, usize, usize) {
    self.plays.iter().fold((0, 0, 0), |(ar, ag, ab), p| {
      let r = if p.r > ar { p.r } else { ar };
      let g = if p.g > ag { p.g } else { ag };
      let b = if p.b > ab { p.b } else { ab };

      (r, g, b)
    })
  }

  /// Return the power of the game, which is the number of required blocks multiplied together.
  pub fn power(&self) -> usize {
    let (r, g, b) = self.min_blocks();
    r * g * b
  }
}

impl FromStr for Game {
  type Err = &'static str;

  /// Parse a game from a single line
  ///
  /// The line should be in the format:
  /// "Game 100: 4 red, 5 blue, 3 green; 1 blue, 2 green; 3 red"
  ///
  /// # Example:
  /// ```
  /// use aoc::day2::Game;
  ///
  /// let g = "Game 100: 4 red, 5 blue, 3 green; 1 blue, 2 green; 3 red".parse::<Game>();
  /// assert!(g.is_ok());
  /// ```
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (left, right) = s.split_once(": ").ok_or("no colon found")?;
    let (_, id) = left.split_once(" ").ok_or("No space in Game id")?;

    let id: usize = id.parse().map_err(|_| "Failed to parse ID")?;

    let plays: Vec<Play> = right
      .split("; ")
      .map(|segment| segment.parse())
      .collect::<Result<_, _>>()?;

    Ok(Game { id, plays })
  }
}

#[derive(Default)]
pub struct Play {
  pub r: usize,
  pub g: usize,
  pub b: usize,
}

  /// Determine if a game is possible with the given block constraints.
  ///
  /// # Example
  /// ```
  /// use aoc::day2::Play;
  ///
  /// let play = Play { r: 1, g: 1, b: 1 };
  ///
  /// assert!(play.is_possible(1, 1, 1));
  /// assert!(!play.is_possible(0, 1, 1));
  /// ```
impl Play {
  pub fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
    self.r <= r && self.g <= g && self.b <= b
  }
}

impl FromStr for Play {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // "3 blue, 4 red"
    // "1 red, 2 green, 6 blue"
    let r = Regex::new(r#"(\d+)\s(\w)\w+"#).expect("Failed to compile regex");
    let mut play = Play::default();

    for cap in r.captures_iter(s) {
      let ch = cap.get(2).unwrap().as_str();
      let count: usize = cap.get(1).unwrap().as_str().parse().unwrap();

      match ch {
        "r" => play.r = count,
        "g" => play.g = count,
        "b" => play.b = count,
        _ => unreachable!(),
      };
    }

    Ok(play)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

  #[test]
  fn part1_example() {
    let games = Game::load_all(EXAMPLE_INPUT).expect("Failed to parse games");

    let sum: usize = games
      .iter()
      .filter(|g| g.is_possible(12, 13, 14))
      .map(|g| g.id)
      .sum();

    assert_eq!(sum, 8);
  }

  #[test]
  fn part1_solution() {
    let games = Game::load_all(INPUT).expect("Failed to parse games");

    let sum: usize = games
      .iter()
      .filter(|g| g.is_possible(12, 13, 14))
      .map(|g| g.id)
      .sum();

    assert_eq!(sum, 2617);
  }

  #[test]
  fn part2_example() {
    let games = Game::load_all(EXAMPLE_INPUT).expect("Failed to parse games");

    let sum: usize = games.iter().map(|g| g.power()).sum();

    assert_eq!(sum, 2286);
  }

  #[test]
  fn part2_solution() {
    let games = Game::load_all(INPUT).expect("Failed to parse games");

    let sum: usize = games.iter().map(|g| g.power()).sum();

    assert_eq!(sum, 59795);
  }
}
