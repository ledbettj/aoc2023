use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

use crate::parse_error;
use crate::ParseError;

const INPUT: &'static str = include_str!("../inputs/day19.txt");

#[derive(Debug, Clone, Copy)]
struct Part {
  x: isize,
  m: isize,
  a: isize,
  s: isize,
}

impl Part {
  pub fn rating(&self) -> isize {
    self.x + self.m + self.a + self.s
  }

  pub fn get(&self, s: &str) -> isize {
    match s {
      "x" => self.x,
      "m" => self.m,
      "a" => self.a,
      "s" => self.s,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Clone)]
enum Instruction {
  CallGT(String, isize, String),
  CallLT(String, isize, String),
  Call(String),
  Accept,
  Reject,
}

#[derive(Debug)]
struct Workflow {
  name: String,
  rules: Vec<Instruction>,
}

impl Workflow {
  pub fn accept() -> Self {
    Self {
      name: "A".into(),
      rules: vec![Instruction::Accept],
    }
  }

  pub fn reject() -> Self {
    Self {
      name: "R".into(),
      rules: vec![Instruction::Reject],
    }
  }
}

#[derive(Debug)]
struct Workflows {
  parts: Vec<Part>,
  wf: HashMap<String, Workflow>,
}

impl Workflows {
  pub fn eval(&self) -> isize {
    self
      .parts
      .iter()
      .filter(|p| self.is_accepted(p))
      .map(|p| p.rating())
      .sum()
  }

  pub fn is_accepted(&self, p: &Part) -> bool {
    let mut step = "in".to_string();

    loop {
      let workflow = &self.wf[&step];
      step = match workflow.eval(p) {
        Instruction::Call(s) => s,
        Instruction::Accept => return true,
        Instruction::Reject => return false,
        _ => unreachable!(),
      }
    }
  }
}

impl Workflow {
  pub fn eval(&self, p: &Part) -> Instruction {
    for i in &self.rules {
      match i {
        Instruction::Call(_) | Instruction::Accept | Instruction::Reject => return i.clone(),
        Instruction::CallGT(field, value, dest) => {
          if p.get(field) > *value {
            return Instruction::Call(dest.clone());
          }
        }
        Instruction::CallLT(field, value, dest) => {
          if p.get(field) < *value {
            return Instruction::Call(dest.clone());
          }
        }
      }
    }

    unreachable!()
  }
}

impl FromStr for Workflows {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (rules, parts) = s
      .trim()
      .split_once("\n\n")
      .ok_or(parse_error!("No blank line"))?;

    let parts = parts
      .lines()
      .map(|line| line.parse())
      .collect::<Result<_, _>>()?;

    let mut wf: HashMap<String, Workflow> = rules
      .lines()
      .map(|line| line.parse().expect("Failed to parse workflow"))
      .map(|wf: Workflow| (wf.name.clone(), wf))
      .collect();

    wf.insert("R".into(), Workflow::reject());
    wf.insert("A".into(), Workflow::accept());

    Ok(Self { parts, wf })
  }
}

impl FromStr for Workflow {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (name, rest) = s.split_once("{").ok_or(parse_error!("No {{ found"))?;
    let rules = rest[..rest.len() - 1]
      .split(",")
      .map(|rule| rule.parse())
      .collect::<Result<_, _>>()?;

    Ok(Self {
      name: name.into(),
      rules,
    })
  }
}

impl FromStr for Instruction {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let r = Regex::new(r"([amsx])([><])(\d+):([A-Za-z]+)").expect("Failed to compile regex");
    if let Some(cap) = r.captures(s) {
      let (_, [reg, op, value, dest]) = cap.extract();
      Ok(match op {
        ">" => Instruction::CallGT(reg.into(), value.parse()?, dest.into()),
        "<" => Instruction::CallLT(reg.into(), value.parse()?, dest.into()),
        _ => unreachable!(),
      })
    } else {
      Ok(Instruction::Call(s.into()))
    }
  }
}
impl FromStr for Part {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let r = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").expect("Failed to compile regex");
    let (_, [x, m, a, s]) = r
      .captures(s)
      .ok_or(parse_error!("Regex didnt match"))?
      .extract();

    Ok(Self {
      x: x.parse()?,
      m: m.parse()?,
      a: a.parse()?,
      s: s.parse()?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_INPUT: &'static str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

  #[test]
  fn part1_example() {
    let wfs: Workflows = EXAMPLE_INPUT.parse().expect("Failed to parse input");
    assert_eq!(wfs.eval(), 19114);
  }

  #[test]
  fn part1_solution() {
    let wfs: Workflows = INPUT.parse().expect("Failed to parse input");
    assert_eq!(wfs.eval(), 19114);
  }

  #[test]
  fn part2_example() {}

  #[test]
  fn part2_solution() {}
}
