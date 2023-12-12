use std::{collections::HashMap, os::unix::fs::MetadataExt};

extern crate regex;
use regex::Regex;

extern crate anyhow;
use anyhow::{anyhow, bail};

fn offsets(c: char) -> Vec<(i32, i32)> {
  match c {
    '|' => vec![(1, 0), (-1, 0)],
    '-' => vec![(0, 1), (0, -1)],
    'L' => vec![(-1, 0), (0, 1)],
    'J' => vec![(-1, 0), (0, -1)],
    '7' => vec![(0, -1), (1, 0)],
    'F' => vec![(0, 1), (1, 0)],
    'S' => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
    _ => vec![]
  }
}

fn bfs(grid: &Vec<Vec<char>>, start: (i32, i32)) -> Vec<Vec<Option<i32>>> {
  let mut queue = vec![start];
  let mut visited = grid.clone().iter().map(|row| row.iter().map(|_| None).collect::<Vec<Option<i32>>>()).collect::<Vec<Vec<Option<i32>>>>();
  visited[start.0 as usize][start.1 as usize] = Some(0);
  while !queue.is_empty() {
    let current = queue.remove(0);
    for offset in offsets(grid[current.0 as usize][current.1 as usize]) {
      let next = (current.0 + offset.0, current.1 + offset.1);
      if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid[0].len() as i32 {
        continue;
      }
      if grid[next.0 as usize][next.1 as usize] == '.' || visited[next.0 as usize][next.1 as usize].is_some() {
        continue;
      }
      if !offsets(grid[next.0 as usize][next.1 as usize]).iter().any(|offset| offset.0 + next.0  == current.0 && offset.1 + next.1 == current.1) {
        continue;
      }
      visited[next.0 as usize][next.1 as usize] = Some(visited[current.0 as usize][current.1 as usize].unwrap() + 1);
      queue.push(next);
    }
  }

  visited
}

fn count(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &Vec<Vec<Option<i32>>>) -> usize {
  let mut c = 0;
  for k in 0..j {
    if visited[i][k].is_none() {
      continue;
    }
    let char = grid[i][k];
    if char == 'L' || char == 'J' || char == '|' {
      c += 1;
    }
  }
  c
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let mut start = (0, 0);
  let mut grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'S' {
        start = (i as i32, j as i32);
      }
    }
  }

  let visited = bfs(&grid, start);

  let mut res = 0;
  for (i, row) in grid.iter().enumerate() {
    for (j, col) in grid[i].iter().enumerate() {
      if visited[i][j].is_none() {
        let c = count(i, j, &grid, &visited);
        if c % 2 == 1 {
          print!("I");
          res += 1
        } else {
          print!(".");
        }
      } else {
        print!("*");
      }
    }
      println!("");
  }

  println!("Part2: {}", res);
}