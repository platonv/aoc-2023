extern crate anyhow;
use anyhow::{anyhow, bail};

fn differences(line1: &Vec<char>, line2: &Vec<char>) -> (usize, usize) {
  let mut res: usize = 0;
  let mut last_i = 0;

  for i in 0..line1.len() {
    if line1[i] != line2[i] {
      res += 1;
      last_i = i;
    }
  }

  (res, last_i)
}

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
    let (d, _) = differences(&grid[i as usize], &grid[i - 1 as usize]);
    if d == 0 || d == 1 {
      let mut j: i32 = i as i32 - 2;
      let mut k: i32 = i as i32 + 1;
      let mut total_differences = d;

      while total_differences < 2 && j >= 0 && k < grid.len() as i32 {
        let (differences, last_i) = differences(&grid[j as usize], &grid[k as usize]);
        println!("j: {:?}, k: {:?}", j, k);
        println!("differences: {:?}", differences);
        total_differences += differences;
        if (differences == 1 || differences == 0) {
          j -= 1;
          k += 1;
        } else {
          break;
        }
      }

      if (j < 0 || k >= grid.len() as i32) && total_differences == 1 {
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
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

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