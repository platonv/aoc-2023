use geo_types::{coord};
use geo::{Line, Coord};
use geo::line_intersection::{line_intersection, LineIntersection};

extern crate anyhow;
extern crate geo;
use std::str::FromStr;

use anyhow::{Result, Ok};

// const LIMIT_1: f64 = 200000000000000.0;
// const LIMIT_2: f64 = 400000000000000.0;
const LIMIT_1: f64 = 7.0;
const LIMIT_2: f64 = 27.0;

#[derive(Debug, PartialEq)]
struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
}


impl Vec3 {
  fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  fn add(&self, other: &Vec3) -> Vec3 {
    Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }

  fn sub(&self, other: &Vec3) -> Vec3 {
    Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }

  fn scale(&self, scalar: f64) -> Vec3 {
    Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
  }
}

#[derive(Debug, PartialEq)]
struct Hailstone {
  position: Vec3,
  velocity: Vec3,
}

impl Hailstone {
  fn new(position: Vec3, velocity: Vec3) -> Hailstone {
    Hailstone { position, velocity }
  }

  fn step(&mut self) {
    self.position = self.position.add(&self.velocity);
  }
}

impl FromStr for Hailstone {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    let (position_str, velocity_str) = s.split_at(s.find('@').unwrap());

    let position = position_str.split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();
    let velocity = velocity_str.replace("@", "").split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();

    Ok(Hailstone::new(
      Vec3::new(position[0], position[1], position[2]),
      Vec3::new(velocity[0], velocity[1], velocity[2]),
    ))
  }
}

fn collision(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
  let p11 = &h1.position;
  let p12 = h1.position.add(&h1.velocity.scale(LIMIT_2));
  let p21 = &h2.position;
  let p22 = h2.position.add(&h2.velocity.scale(LIMIT_2));

  let line1 = Line::new(Coord { x: p11.x, y: p11.y }, Coord { x: p12.x, y: p12.y });
  let line2 = Line::new(Coord { x: p21.x, y: p21.y }, Coord { x: p22.x, y: p22.y });

  match line_intersection(line1, line2) {
    Some(i) => {
      match i {
        LineIntersection::SinglePoint { intersection, is_proper: _ } => {
          println!("Collision: {:?}", intersection);
          Some((intersection.x, intersection.y))
        },
        _ => None
      }
    },
    _ => None,
  }
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1_example.txt").lines().collect();

  let stones = lines.iter().map(|line| line.parse::<Hailstone>().unwrap()).collect::<Vec<Hailstone>>();

  let mut c = 0;

  for stone1 in &stones {
    for stone2 in &stones {
      if stone1 != stone2 {
        if let Some((x, y)) = collision(&stone1, &stone2) {
          if x > LIMIT_1 && x < LIMIT_2 && y > LIMIT_1 && y < LIMIT_2 {
            c += 1;
          }
        }
      }
    }
  }

  println!("c: {:?}", c / 2);

  Ok(())
}