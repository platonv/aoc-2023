use std::collections::HashMap;
use std::str::FromStr;

extern crate anyhow;
use anyhow::{Result, anyhow, bail};

extern crate regex;
use regex::Regex;

struct DigPoint {
  position: (i32, i32),
  direction: char,
  distance: u32,
}

fn move_coords(current_i: &i32, current_j: &i32, direction: &char, distance: &i32) -> Result<(i32, i32)> {
  match direction {
    'U' => Ok((*current_i - *distance, *current_j)),
    'D' => Ok((*current_i + *distance, *current_j)),
    'L' => Ok((*current_i, *current_j - *distance )),
    'R' => Ok((*current_i, *current_j + *distance)),
    _ => bail!("invalid direction"),
  }
}

fn create_grid(lines: &Vec<&str>) -> Result<(Vec<Vec<char>>, Vec<DigPoint>)> {
  let mut max_i = 0;
  let mut max_j = 0;
  let mut min_i = 0;
  let mut min_j = 0;
  let mut current_i = 0;
  let mut current_j = 0;
  let mut dig_points = Vec::new();

  for line in lines {
    match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
      [direction, distance, _] => {
        let (new_i, new_j) = move_coords(&current_i, &current_j, &direction.chars().next().unwrap(), &distance.parse::<i32>()?)?;
        dig_points.push(DigPoint { position: (current_i, current_j), direction: direction.chars().next().unwrap(), distance: distance.parse::<u32>()? });
        current_i = new_i;
        current_j = new_j;
        max_i = max_i.max(current_i);
        max_j = max_j.max(current_j);
        min_i = min_i.min(current_i);
        min_j = min_j.min(current_j);
        println!("current_i: {}, current_j: {}", current_i, current_j);
      },
      line => bail!("invalid line: {:?}", line)
    }
  }

  let cols = (max_j - min_j + 1) as usize;
  let rows = (max_i - min_i + 1) as usize;

  let mut grid = vec![vec!['.'; cols]; rows];

  for dig_point in &dig_points {
    let mut c = dig_point.distance.clone();
    let mut i = dig_point.position.0.clone() + (-min_i);
    let mut j = dig_point.position.1.clone() + (-min_j);
    while c > 0 {
      grid[i as usize][j as usize] = '#';
      let (new_i, new_j) = move_coords(&i, &j, &dig_point.direction, &1)?;
      i = new_i;
      j = new_j;
      c -= 1;
    }
  }

  Ok((grid, dig_points))
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j] == '#' {
        return (i, j);
      }
    }
  }
  (0, 0)
}

fn flood_fill(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut visited = grid.clone();
  let (start_i, start_j) = find_start(&grid);
  let mut queue = vec![(start_i + 1, start_j + 1)];
  visited[start_i + 1][start_j + 1] = '#';

  while !queue.is_empty() {
    let current = queue.remove(0);
    for offset in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
      let next = (current.0 as i32 + offset.0, current.1 as i32 + offset.1);
      if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid.len() as i32 {
        continue;
      }
      if visited[next.0 as usize][next.1 as usize] == '#' {
        continue;
      }
      visited[next.0 as usize][next.1 as usize] = '#';
      queue.push((next.0 as usize, next.1 as usize));
    }
  }
  visited

}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let (grid, dig_points) = create_grid(&lines)?;

  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      print!("{}", grid[i][j]);
    }
    println!();
  }
  println!("");

  let filled_grid = flood_fill(&grid);

  for i in 0..filled_grid.len() {
    for j in 0..filled_grid[i].len() {
      print!("{}", filled_grid[i][j]);
    }
    println!();
  }
  println!("");

  let res = filled_grid.iter().map(|row| row.iter().filter(|c| **c == '#').count()).sum::<usize>();

  println!("res: {}", res);

  Ok(())
}