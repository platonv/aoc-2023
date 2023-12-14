use std::collections::HashMap;
use std::str::FromStr;
use core::cmp::Ordering;
use std::collections::VecDeque;
use std::cmp;
use std::num;

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

fn travel(directions: String, directions_map: &HashMap<String, Directions>, start: &str, end: &str) -> Result<Vec<String>, anyhow::Error> {
  let mut path = Vec::new();
  let mut current = start.to_string();

  while current != end {
    for c in directions.chars() {
      println!("Current {}", current);
      println!("Map[Current] {:?}", directions_map.get(&current));
      println!("{}", c);
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
      if current == end {
        break;
      }
    }
  }

  Ok(path)
}

fn main() {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

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

  let res = travel(directions, &directions_map, "AAA", "ZZZ").unwrap();
  println!("{:?}", res);
  println!("{}", res.len());
}