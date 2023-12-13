extern crate anyhow;
use anyhow::{anyhow, bail};

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut res = vec![vec!['.'; grid.len()]; grid[0].len()];

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      res[j][i] = grid[i][j];
    }
  }

  res
}

fn symmetry(grid: &Vec<Vec<char>>) -> Result<Option<usize>, anyhow::Error> {
  let res: usize = 0;

  for i in 1..grid.len() {
    if grid[i] == grid[i - 1] {
      let mut j: i32 = i as i32;
      let mut k: i32 = (i - 1) as i32;

      while j >= 0 && k < grid.len() as i32 {
        if grid[j as usize] == grid[k as usize] {
          j -= 1;
          k += 1;
        } else {
          break;
        }
      }

      if j < 0 || k >= grid.len() as i32 {
        return Ok(Some(i));
      }
    }
  }

  Ok(None)
}

fn solve(grid: &Vec<Vec<char>>) -> Result<usize, anyhow::Error> {
  let horizontal = symmetry(&grid)?;

  let transpose = transpose(&grid);

  let vertical = symmetry(&transpose)?;

  println!("horizontal: {:?}", horizontal);
  println!("vertical: {:?}", vertical);

  let horizontal_distance = horizontal.map(|h| h.abs_diff(grid.len()));
  let vertical_distance = vertical.map(|h| h.abs_diff(transpose.len()));

  if let (Some(h), Some(v)) = (horizontal_distance, vertical_distance) {
    if h > v {
      Ok(horizontal.unwrap() * 100)
    } else {
      Ok(vertical.unwrap())
    }
  } else {
    horizontal.map(|res| res * 100).or(vertical).ok_or(anyhow!("No solution found"))
  }
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input1_example.txt").lines().collect();

  let results = lines.split(|line| line.is_empty()).map(|group| {
    let grid = group.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    for line in &grid {
      println!("{:?}", line);
    }

    let res = solve(&grid);

    println!("res: {:?}", res);
    res
  }).collect::<Result<Vec<usize>, anyhow::Error>>()?;

  let sum = results.iter().sum::<usize>();

  println!("sum: {:?}", sum);

  Ok(())
}