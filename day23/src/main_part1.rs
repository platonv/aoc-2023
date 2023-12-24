use std::{collections::{HashMap, HashSet}, path};

extern crate anyhow;
use anyhow::{Result, Ok};

fn offsets(c: char) -> Vec<(i32, i32)> {
  match c {
    '^' => vec![(-1, 0)],
    'v' => vec![(1, 0)],
    '<' => vec![(0, -1)],
    '>' => vec![(0, 1)],
    '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
    _ => vec![],
  }
}

fn dfs(
  grid: &Vec<Vec<char>>, 
  current: &(usize, usize), 
  path: &mut Vec<(usize, usize)>,
  paths: &mut Vec<Vec<(usize, usize)>>,
) {
  path.push(current.clone());

  let (i, j) = current;


  if *i == grid.len() - 1 {
    paths.push(path.clone());
  } else {
    for (di, dj) in offsets(grid[*i][*j]) {
      let (ni, nj) = (*i as i32 + di, *j as i32 + dj);

      if ni > 0 &&
        ni < grid.len() as i32 &&
        nj > 0 &&
        nj < grid[0].len() as i32 &&
        grid[ni as usize][nj as usize] != '#' && 
        !path.contains(&(ni as usize, nj as usize)) {
          dfs(grid, &(ni as usize, nj as usize), path, paths);
      }
    }
  }

  path.pop();
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let start = (0, lines[0].find('.').unwrap());

  println!("start: {:?}", start);

  let mut current_path = vec![];
  current_path.push(start);
  let mut paths = vec![];
  dfs(&grid, &start, &mut current_path, &mut paths);

  // for path in &paths {
  //   for i in 0..grid.len() {
  //     for j in 0..grid[0].len() {
  //       if path.contains(&(i, j)) {
  //         print!("O");
  //       } else {
  //         print!("{}", grid[i][j]);
  //       }
  //     }
  //     println!("");
  //   }

  //   println!("{}", path.len());
  //   println!("-----------------------------------");
  // }

  let res = &paths.iter().map(|path| path.len()).max().unwrap() - 2;

  println!("Part 1: {}", res);

  Ok(())
}