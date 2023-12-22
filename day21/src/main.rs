use itertools::Itertools;

extern crate anyhow;
use anyhow::{Result, Ok};

const STEPS: usize = 6;

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1_example.txt").lines().collect();

  let pattern = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let pattern_size_h = pattern[0].len();
  let pattern_size_v = pattern.len();

  let start_i = pattern_size_v / 2;
  let start_j = pattern_size_h / 2;

  println!("pattern_size_h: {}, pattern_size_v: {}", pattern_size_h, pattern_size_v);

  let mut grid = vec![vec!['.'; pattern_size_h]; pattern_size_v];

  let mut count = 0;

  for i in 0..pattern_size_v {
    for j in 0..pattern_size_h {
      if (start_i.abs_diff(i) + start_j.abs_diff(j)) % 2 == STEPS % 2
        && (start_i.abs_diff(i) + start_j.abs_diff(j)) <= STEPS 
          && pattern[i][j] == '.' {
        grid[i][j] = 'O';
        count += 1;
        print!("O");
      } else {
        print!("{}", pattern[i][j]);
      }
    }
    println!("");
  }

  println!("Part 2: {}", count);

  Ok(())
}