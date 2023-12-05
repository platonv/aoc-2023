use std::collections::HashMap;
use std::str::FromStr;
use std::collections::VecDeque;
use std::cmp;

use regex::Regex;
extern crate regex;

extern crate anyhow;
use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq)]
struct Mapping {
  dest_start: u64,
  source_start: u64,
  range_len: u64,
}

impl FromStr for Mapping {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (dest_start, source_start, range_len) = match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
      [dest_start, source_start, range_len] => (dest_start.parse::<u64>()?, source_start.parse::<u64>()?, range_len.parse::<u64>()?),
      _ => return Err(anyhow!("Invalid mapping format"))
    };
    Ok(Mapping { dest_start, source_start, range_len })
  }
}

#[derive(Debug, PartialEq)]
struct Recipe {
  id: String,
  maps: Vec<Mapping>,
}

impl FromStr for Recipe {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let re = Regex::new(r"(?P<name>\w+-to-\w+)\smap:")?;
    let captures = re.captures(s).ok_or(anyhow!("Invalid recipe format"))?;
    let name = captures["name"].parse::<String>()?;
    Ok(Recipe { id: name, maps: Vec::new() })
  }
}

fn intersect(i11: u64, i12: u64, i21: u64, i22: u64) -> Option<(u64, u64)> {
  if i12 <= i21 {
      return None;
  } else if i11 >= i22 {
      return None;
  } else {
      let max_start = cmp::max(i11, i21);
      let min_end = cmp::min(i12, i22);
      return Some((max_start, min_end));
  }
}

impl Recipe {
  fn intersect_ranges(&self, seed: &SeedPair) -> Vec<SeedPair> {
    let mut seeds = VecDeque::from([seed.clone()]);
    let mut dests = Vec::new();

    while !seeds.is_empty() {
      let remaining = seeds.pop_front().unwrap();
      let mut intersected = false;

      for map in &self.maps {
        if let Some((i1, i2)) = intersect(remaining.source, remaining.source + remaining.length, map.source_start, map.source_start + map.range_len) {
          println!("intersected: {:?} {:?} {:?} {:?}", remaining, map, i1, i2);
          if remaining.source < i1 {
            seeds.push_back(SeedPair { source: remaining.source, length: i1 - remaining.source });
          }
          if i2 < remaining.source + remaining.length {
            seeds.push_back(SeedPair { source: i2, length: remaining.source + remaining.length - i2});
          }
          dests.push(
              SeedPair { 
                source: map.dest_start + (i1 - map.source_start),
                length: i2 - i1
              },
          );
          intersected = true;
          break;
        }
      }
      if !intersected {
        dests.push(remaining.clone());
      }
    }
    dests
  }
}

#[derive(Debug, PartialEq, Clone)]
struct SeedPair {
  source: u64,
  length: u64,
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();
  let mut seed_pairs: Vec<SeedPair> = Vec::new();

  let mut next_recipe = true;
  let mut recipes: Vec<Recipe> = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    if i == 0 {
      let seeds_str = line.split(":").last().unwrap();
      seeds_str.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>().chunks(2).for_each(|pair| {
        seed_pairs.push(SeedPair { source: pair[0], length: pair[1] });
      });
    } else {
      if line.is_empty() {
        next_recipe = true;
        continue;
      }
      if next_recipe {
        let recipe = Recipe::from_str(line).unwrap();
        recipes.push(recipe);
      } else {
        let mapping = Mapping::from_str(line).unwrap();
        let recipe = recipes.last_mut().unwrap();
        recipe.maps.push(mapping);
      }
      next_recipe = false;
    }
  }

  let mut current_pairs = seed_pairs;
  for recipe in &recipes {
      let mut new_intervals = Vec::new();
      for seed_interval in current_pairs {
        new_intervals.append(&mut recipe.intersect_ranges(&seed_interval));
      }
      current_pairs = new_intervals;
  }
  let result = current_pairs
      .iter()
      .min_by_key(|i| i.source)
      .unwrap();

  println!("{result:?}");
}