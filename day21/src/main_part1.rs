use itertools::Itertools;

extern crate anyhow;
use anyhow::{Result, Ok};

fn plots_at_steps(grid: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> Vec<(usize, usize)> {
  let mut current = vec![start];
  let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

  for _ in 0..steps {
    let mut new_current = vec![];
    for c in current.iter() {
      for d in directions.iter() {
        let new = (c.0 as i64 + d.0, c.1 as i64 + d.1);
        if new.0 >= 0 && new.0 < grid.len() as i64 && new.1 >= 0 && new.1 < grid[0].len() as i64
          && (grid[new.0 as usize][new.1 as usize] == '.' || grid[new.0 as usize][new.1 as usize] == 'S') {
          let new = (new.0 as usize, new.1 as usize);
          new_current.push(new);
        }
      }
    }
    current = new_current.iter().unique().map(|x| x.clone()).collect::<Vec<(usize, usize)>>();
  }

  current
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let mut start = (0, 0);
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j] == 'S' {
        start = (i, j);
      }
    }
  }

  let res = plots_at_steps(&grid, start, 64);

  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if res.contains(&(i, j)) {
        print!("O");
      } else {
        print!("{}", grid[i][j]);
      }
    }
    println!("");
  }

  println!("Part 1: {:?}", res.len());

  Ok(())
}