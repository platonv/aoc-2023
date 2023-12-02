use std::collections::HashMap;
use std::str::FromStr;

extern crate anyhow;
use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq)]
struct GameSet {
  cubes: Vec<(String, u32)>
}

impl FromStr for GameSet {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cubes: Vec<(String, u32)> = s.split(",").collect::<Vec<&str>>().iter().map(|cube_str| {
      Ok(match cube_str.trim().split(" ").collect::<Vec<&str>>().as_slice() {
        [number, color] => (color.to_string(), number.parse::<u32>()?),
        _ => bail!("Invalid cube format")
      })
    }).collect::<Result<Vec<(String, u32)>, anyhow::Error>>()?;
    Ok(GameSet { cubes })
  }
}

#[derive(Debug, PartialEq)]
struct Game {
  id: u32,
  sets: Vec<GameSet>,
}

impl Game {
  fn is_valid(&self) -> bool {
    let maximums: HashMap<&str, u32> = HashMap::from([
      ("red", 12),
      ("green", 13),
      ("blue", 14),
    ]);
    self.sets.iter().all(|set| {
      set.cubes.iter().all(|(color, number)| {
        if number > maximums.get(&*color.as_str()).unwrap() {
          return false;
        } else {
          return true;}
      })
    })
  }

  fn power(&self) -> u32 {
    let mut maximums: HashMap<&str, u32> = HashMap::from([
      ("red", 0),
      ("green", 0),
      ("blue", 0),
    ]);
    for (color, number) in self.sets.iter().flat_map(|set| set.cubes.iter()) {
      let maximum = maximums.get(&*color.as_str()).unwrap();
      if number > maximum {
        maximums.insert(&*color.as_str(), *number);
      }
    }
    println!("Game: {:?}", self.id);
    println!("Maximums: {:?}", maximums);
    maximums.values().fold(1, |acc, x| acc * x)
  }
}

impl FromStr for Game {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (id_str, sets_str) = match s.split(":").collect::<Vec<&str>>().as_slice() {
      [id, sets] => (id.to_string(), sets.to_string()),
      _ => return Err(anyhow!("Invalid game format"))
    };
    match id_str.split(" ").collect::<Vec<&str>>().as_slice() {
      ["Game", id] => Ok(Game { id: id.parse::<u32>()?, sets: sets_str.split(";").map(|set_str| {
        set_str.parse::<GameSet>()
      }).collect::<Result<Vec<GameSet>, anyhow::Error>>()? }),
      _ => Err(anyhow!("Invalid game format"))
    }
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let result: Vec<Result<u32, anyhow::Error>> = lines.iter().map(|line| {
    let game = Game::from_str(*line)?;
    // Uncomment for part 1
    // if game.is_valid() {
      Ok(game.power())
    // } else {
      // Ok(0)
    // }
  }).collect::<Vec<Result<u32, anyhow::Error>>>();

  let sum = result.iter().fold(0, |acc, x| {
    match x {
      Ok(id) => acc + id,
      Err(_) => acc
    }
  });

  println!("Result: {:?}", result);
  println!("Sum: {:?}", sum);
  println!("Done!")
}