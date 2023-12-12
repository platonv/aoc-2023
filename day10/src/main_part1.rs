use std::collections::HashMap;

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
      if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid.len() as i32 {
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

fn flood_fill(start: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
  let start_i = start.0 as i32;
  let start_j = start.1 as i32;
  let mut queue = vec![(start_i, start_j)];
  let mut visited = grid.clone().iter().map(|row| row.iter().map(|_| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
  visited[start.0 as usize][start.1 as usize] = true;
  let mut corners = HashMap::new();
  while !queue.is_empty() {
    let current = queue.remove(0);
    for offset in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
      let next = (current.0 + offset.0, current.1 + offset.1);
      if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid.len() as i32 {
        continue;
      }
      if grid[next.0 as usize][next.1 as usize] == 'L' || 
        grid[next.0 as usize][next.1 as usize] == 'J' || 
        grid[next.0 as usize][next.1 as usize] == '7' || 
        grid[next.0 as usize][next.1 as usize] == 'F' {
          corners.insert(grid[next.0 as usize][next.1 as usize], corners.get(&grid[next.0 as usize][next.1 as usize]).unwrap_or(&0) + 1);
      }
      if grid[next.0 as usize][next.1 as usize] != '.' || visited[next.0 as usize][next.1 as usize] {
        continue;
      }
      visited[next.0 as usize][next.1 as usize] = true;
      queue.push(next);
    }
  }
  for i in 0..grid.len() {
    for j in 0..grid.len() {
      if i == start.0 && j == start.1 {
        print!("0");
      } else if visited[i][j] {
        print!("X");
      } else {
        print!("{}", grid[i][j])
      }
    }
    println!("");
  }
  println!("");
  println!("{:?}", corners);
  visited
}

fn count_true(grid: &Vec<Vec<bool>>) -> usize {
  grid.iter().map(|row| row.iter().filter(|&x| *x == true).count()).sum()
}

fn inside_areas(grid: &Vec<Vec<char>>) -> usize {
  let mut areas = grid.iter().map(|row| row.iter().map(|_| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
  let mut s = 0;
  for i in 0..grid.len() {
    for j in 0..grid.len() {
      let area = match grid[i][j] {
        'L' => count_true(&flood_fill((i - 1, j + 1), grid)),
        'J' => count_true(&flood_fill((i - 1, j - 1), grid)),
        'F' => count_true(&flood_fill((i + 1, j + 1), grid)),
        '7' => count_true(&flood_fill((i + 1, j - 1), grid)),
        _ => 0
      };
      s += area;
    }
  }
  s
}

fn main() {
  let lines: Vec<&str> = include_str!("input2_example.txt").lines().collect();

  let mut start = (0, 0);
  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  for i in 0..grid.len() {
    for j in 0..grid.len() {
      print!("{}", grid[i][j]);
      if grid[i][j] == 'S' {
        start = (i as i32, j as i32);
      }
    }
    println!("");
  }

  let visited = bfs(&grid, start);

  let mut max = 0;
  for i in 0..visited.len() {
    for j in 0..visited.len() {
      if visited[i][j].is_some() && visited[i][j].unwrap_or(0) > max {
        max = visited[i][j].unwrap();
      }
    }
  }

  let areas = inside_areas(&grid);

  println!("");
  println!("Part1: {}", max);
  println!("Part2: {}", areas);
}