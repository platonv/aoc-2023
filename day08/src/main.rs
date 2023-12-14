use std::collections::HashMap;
use std::str::FromStr;
use core::cmp::Ordering;
use std::collections::VecDeque;
use std::cmp;
use num;

use regex::Regex;
extern crate regex;
extern crate anyhow;

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Directions {
  left: String,
  right: String,
}

fn parse_directions(str: &str) -> Result<(String, Directions), anyhow::Error> {
  match str.split(" = ").collect::<Vec<&str>>().as_slice() {
    [direction_name, outputs] => {
      match outputs.replace("(", "").replace(")", "").split(", ").collect::<Vec<&str>>().as_slice() {
        [left, right] => return Ok((direction_name.to_string(), Directions { left: left.to_string(), right: right.to_string() })),
        _ => bail!("Invalid input")
      }
    },
    _ => bail!("Invalid input"),
  };
}

fn is_end(node: &String) -> bool {
  node.chars().last().unwrap() == 'Z'
}

fn is_start(node: &String) -> bool {
  node.chars().last().unwrap() == 'A'
}

fn travel(start: String, directions: String, directions_map: &HashMap<String, Directions>) -> Result<Vec<String>, anyhow::Error> {
  let mut path = Vec::new();
  let mut current = start.clone();

  while !is_end(&current) {
    for c in directions.chars() {
      println!("Current {}", &current);
      if c == 'L' {
        let direction = directions_map.get(&current).unwrap();
        current = direction.left.clone();
        path.push(current.clone());
      } else if c == 'R' {
        let direction = directions_map.get(&current).unwrap();
        current = direction.right.clone();
        path.push(current.clone());
      } else {
        bail!("Invalid input");
      }
      if is_end(&current) {
        break;
      }
    }
  }

  Ok(path)
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let mut directions_map = HashMap::new();
  let mut directions: String = "".to_string();
  for (i, line) in lines.iter().enumerate() {
    if i == 0 {
      directions = line.to_string();
    } else if (i == 1) {
      continue;
    } else {
      let (name, directions) = parse_directions(line).unwrap();
      directions_map.insert(name, directions);
    }
  }

  println!("{:?}", directions_map);

  let res = directions_map.keys().map(|k|
    if is_start(k) {
      Some(travel(k.to_string(), directions.clone(), &directions_map).unwrap().len())
    } else {
      None
    }
  ).flatten().collect::<Vec<usize>>();

  let result = res.iter().fold(1, |acc: i64, x| num::integer::lcm(acc, *x as i64));

  println!("{:?}", res);
  println!("{}", result);
}