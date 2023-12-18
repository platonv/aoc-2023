use std::collections::HashMap;
use std::str::FromStr;
use std::i64;
use polygonical::polygon::Polygon;
use polygonical::point::Point;

extern crate anyhow;
use anyhow::{Result, anyhow, bail, Ok};

extern crate regex;
use regex::Regex;

fn move_coords(current_i: &i64, current_j: &i64, direction: &char, distance: &i64) -> Result<(i64, i64)> {
  match direction {
    'U' => Ok((*current_i - *distance as i64, *current_j)),
    'D' => Ok((*current_i + *distance as i64, *current_j)),
    'L' => Ok((*current_i, *current_j - *distance as i64)),
    'R' => Ok((*current_i, *current_j + *distance as i64)),
    _ => bail!("invalid direction"),
  }
}

fn from_color(color: &str) -> Result<(char, i64)> {
  let re = Regex::new(r"\(#(?<distance>\w{5})(?<direction>\w)\)")?;
  let captures = re.captures(color).ok_or(anyhow!("Invalid recipe format"))?;
  let distance_str = captures.name("distance").unwrap().as_str();
  let direction_str = captures.name("direction").unwrap().as_str();

  let distance = i64::from_str_radix(distance_str, 16)? as i64;
  let direction = match direction_str.chars().next().unwrap() {
    '0' => 'R',
    '1' => 'D',
    '2' => 'L',
    '3' => 'U',
    _ => bail!("invalid direction")
  };

  Ok((direction, distance))
}

fn parse_points(lines: &Vec<&str>) -> Result<(Polygon, u64)> {
  let mut points = Vec::new();
  let mut current_i = 0;
  let mut current_j = 0;

  let mut perimeter: u64 = 0;

  for line in lines {
    match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
      // [_, _, color] => {
      [direction, distance, color] => {
        // Part1
        // let (new_i, new_j) = move_coords(&current_i, &current_j, &direction.chars().next().unwrap(), &distance.parse::<i64>()?)?;
        // Part2
        let (direction, distance) = from_color(color)?;
        let (new_i, new_j) = move_coords(&current_i, &current_j, &direction, &distance)?;

        points.push(Point::new(current_i as f64, current_j as f64));

        perimeter += (new_i - current_i).abs() as u64 + (new_j - current_j).abs() as u64 ;
        current_i = new_i;
        current_j = new_j;
      },
      line => bail!("invalid line: {:?}", line)
    }
  }

  println!("perimeter: {}", perimeter);
  Ok((Polygon::new(points), perimeter))
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let (poly, perimeter) = parse_points(&lines)?;

  let area = poly.area();
  let i = area - perimeter as f64 / 2.0 + 1.0;

  let res = i + perimeter as f64;

  println!("area: {} {}", area, perimeter);
  println!("result: {}", res);

  Ok(())
}