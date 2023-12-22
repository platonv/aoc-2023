use std::collections::HashMap;

use itertools::Itertools;

use uuid::{uuid, Uuid};
extern crate anyhow;
extern crate rand;
use rand::Rng;
use anyhow::{Result, Ok};

#[derive(Debug, Clone)]
struct Point3D {
  x: i64,
  y: i64,
  z: i64,
}

#[derive(Debug, Clone)]
struct Brick {
  name: String,
  p0: Point3D,
  p1: Point3D,
}

impl Brick {
  fn occupied_cubes(&self) -> Vec<(i64, i64, i64)> {
    let mut occupied_cubes = vec![];

    for x in self.p0.x..=self.p1.x {
      for y in self.p0.y..=self.p1.y {
        for z in self.p0.z..=self.p1.z {
          occupied_cubes.push((x, y, z));
        }
      }
    }

    occupied_cubes
  }

  // should check intersection between the lines formed by p0 and p1 in each brick on x and y axis
  fn on_top(&self, other: &Brick) -> bool {
    for self_cube in self.occupied_cubes() {
      for other_cube in other.occupied_cubes() {
        if self_cube.0 == other_cube.0 && self_cube.1 == other_cube.1 && self.min_z() > other.max_z() {
          return true;
        }
      }
    }
    false
  }

  fn max_z(&self) -> i64 {
    self.p0.z.max(self.p1.z)
  }

  fn min_z(&self) -> i64 {
    self.p0.z.min(self.p1.z)
  }
}

fn dropped(bricks: Vec<Brick>) -> Vec<Brick> {
  let mut laid_bricks: Vec<Brick> = vec![];
  let mut supports: HashMap<String, Vec<String>> = HashMap::new();
  let mut supports2: HashMap<String, Vec<String>> = HashMap::new();

  let mut sorted_bricks = bricks.clone();
  sorted_bricks.sort_by(|a, b| {
    a.min_z().cmp(&b.min_z())
  });

  for brick in &sorted_bricks {
    let mut support_z = 0;

    for laid_brick in &laid_bricks {
      if brick.on_top(&laid_brick) {
        support_z = support_z.max(laid_brick.max_z());
      }
    }

    let mut new_brick = brick.clone();
    let diff = new_brick.min_z() - support_z - 1;
    new_brick.p0.z -= diff;
    new_brick.p1.z -= diff;
    laid_bricks.push(new_brick);
  }

  for brick in &laid_bricks {
    for laid_brick in &laid_bricks {
      if brick.on_top(&laid_brick) && laid_brick.max_z() == brick.min_z() - 1 {
        supports.entry(brick.name.clone()).or_insert(vec![]).push(laid_brick.name.clone());
      }
    }
  }

  let mut c = 0;
  for brick in &laid_bricks {
    if !supports.values().contains(&vec![brick.name.clone()]) {
      c += 1;
      println!("{} can be destroyed", brick.name);
      continue;
    }
  }

  println!("Part1: {}", c);

  let mut res = 0;

  laid_bricks.sort_by(|a, b| a.name.clone().cmp(&b.name.clone()));

  for brick in &laid_bricks {
    let d = drops(brick.name.clone(), &supports);
    res += d;
  }

  println!("Part2 (slow af): {}", res);

  laid_bricks
}

fn drops(brick_name: String, supports: &HashMap<String, Vec<String>>) -> i64 {
  let mut dropped = supports.clone();
  let mut current_dropping = vec![brick_name.clone()];
  let mut finished = false;
  let mut c = 0;

  println!("BRICK_NAME: {}", brick_name);
  while !finished {
    finished = true;
    for (_, supported_by) in dropped.iter_mut() {
      let s = supported_by.clone();
      let mut support = s.iter().filter(|x| !current_dropping.contains(x)).collect::<Vec<&String>>();

      if support.len() != supported_by.len() {
        finished = false;
        supported_by.clear();
        for s in support {
          supported_by.push(s.clone());
        }
      }
    }

    current_dropping.clear();

    for (brick_name, supported_by) in dropped.clone() {
      if supported_by.len() == 0 {
        current_dropping.push(brick_name.clone());
      }
    }
  }

  for (_, supported_by) in &dropped {
    if supported_by.len() == 0 {
      c += 1;
    }
  }

  println!("c: {}", c);
  c
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let bricks = lines.iter().enumerate().map(|(i, line)| {
    match line.split("~").collect::<Vec<&str>>().as_slice() {
      [a, b] => {
        let a = a.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let b = b.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let brick = Brick { name: i.to_string(), p0: Point3D { x: a[0], y: a[1], z: a[2] }, p1: Point3D { x: b[0], y: b[1], z: b[2] } };
        brick
      },
      _ => panic!("Invalid input")
    }
  }).collect::<Vec<Brick>>();

  dropped(bricks);

  // println!("Part 1: {:?}", bricks);

  Ok(())
}