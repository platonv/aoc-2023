use std::collections::HashMap;
use std::str::FromStr;

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

impl Recipe {
  fn get_destination(&self, source: u64) -> u64 {
    for mapping in &self.maps {
      if mapping.source_start <= source && source < mapping.source_start + mapping.range_len {
        return mapping.dest_start + (source - mapping.source_start);
      }
    }
    return source;
  }

}

fn main() {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();
  let mut seeds: Vec<u64> = Vec::new();

  let mut next_recipe = true;
  let mut recipes: Vec<Recipe> = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    if i == 0 {
      let seeds_str = line.split(":").last().unwrap();
      seeds_str.split_whitespace().map(|s| s.parse::<u64>().unwrap()).for_each(|s| seeds.push(s));
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

  let mut final_destinations = Vec::new();
  for seed in &seeds {
    let mut dest = *seed;
    for recipe in recipes.iter() {
      dest = recipe.get_destination(dest);
    }
    final_destinations.push(dest);
  }

  println!("Seeds: {:?}", seeds);
  println!("Recipes: {:?}", recipes);
  // minimum destination
  println!("Part 1: {}", final_destinations.iter().min().unwrap());
}