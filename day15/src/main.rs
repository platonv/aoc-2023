use std::collections::HashMap;
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate anyhow;
use anyhow::{anyhow, bail};

fn hash(s: &str) -> u8 {
  s.chars().fold(0, |acc, c| {
    ((acc as u64 + (c as u8) as u64) * 17 as u64 % 256 as u64) as u8
  })
}

#[derive(Debug, Clone)]
struct Step {
  label: String,
  operation: char,
  value: Option<u64>,
  source: String,
}

impl FromStr for Step {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let regex = Regex::new(r"(?<label>[a-z]*)(?<operation>[=-])(?<value>\d?)").unwrap();

    let captures = regex.captures(s).ok_or(anyhow!("invalid step"))?;
    let label = captures.name("label").unwrap().as_str().to_string();
    let operation = captures.name("operation").unwrap().as_str().chars().next().unwrap();
    let value = captures.name("value").unwrap().as_str().parse::<u64>().ok();

    Ok(Step { label, operation, value, source: s.to_string() })
  }
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let string = lines.iter().fold(String::new(), |acc, line| acc + line);

  let steps = string.split(",").collect::<Vec<&str>>();

  let hashes = steps.iter().map(|step| hash(step)).collect::<Vec<u8>>();

  let sum = hashes.iter().fold(0, |acc, hash| acc + (*hash as u64));

  let mut boxes: HashMap<u8, Vec<Step>> = HashMap::new();

  println!("part1: {}", sum);

  steps.iter().for_each(|step| {
    let step = Step::from_str(step).unwrap();

    if step.operation == '=' {
      let entries = boxes.entry(hash(&step.label)).or_insert(Vec::new());
      let opt = entries.iter()
                               .enumerate()
                               .find(|(i, s)| s.label == step.label);

      match opt {
        Some((i, _)) => {
          entries.remove(i);
          boxes.entry(hash(&step.label)).or_insert(Vec::new()).insert(i, step.clone())
        },
        None => 
          boxes.entry(hash(&step.label)).or_insert(Vec::new()).push(step.clone())
      }

    } else {
      let entries = boxes.entry(hash(&step.label)).or_insert(Vec::new());
      let opt = entries.iter()
                               .enumerate()
                               .find(|(i, s)| s.label == step.label);

      match opt {
        Some((i, _)) => {
          entries.remove(i);
        },
        None => ()
      }
    }

    // println!("After {:?}", &step.source);
    // for (box_id, steps) in &boxes {
    //   println!("{}: {:?}", box_id, steps.iter().map(|step| step.source.clone()).collect::<Vec<String>>());
    // }
    // println!("");
  });

  let mut s = 0;
  for (box_id, steps) in &boxes {
    for (i, step) in steps.iter().enumerate() {
      s += (*box_id as u64 + 1) as u64 * (i as u64 + 1) as u64 * step.value.unwrap() as u64;
    }
    println!("{}", s);
  }
  println!("part2: {}", s);

  Ok(())
}