use std::{collections::{HashMap, HashSet}, path};

extern crate anyhow;
use anyhow::{Result, Ok};

fn offsets(c: char) -> Vec<(i32, i32)> {
  match c {
    _ => vec![(-1, 0), (0, -1), (0, 1), (1, 0)],
  }
}

fn junctions(start: &(usize, usize), dest: &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut res = vec![start.clone(), dest.clone()];

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] != '#' {
        let mut count = 0;

        for (di, dj) in offsets(grid[i][j]) {
          let (ni, nj) = (i as i32 + di, j as i32 + dj);

          if ni > 0 &&
            ni < grid.len() as i32 &&
            nj > 0 &&
            nj < grid[0].len() as i32 &&
            grid[ni as usize][nj as usize] != '#' {
              count += 1;
          }
        }

        if count > 2 {
          res.push((i, j));
        }
      }
    }
  }

  res
}

fn graph(grid: &Vec<Vec<char>>, junctions: &Vec<(usize, usize)>) -> HashMap<(usize, usize), Vec<(usize, (usize, usize))>> {
  let mut res: HashMap<(usize, usize), Vec<(usize, (usize, usize))>> = HashMap::new();

  for junction in junctions {
      let mut stack = vec![(0, junction.clone())];
      let mut visited = HashSet::new();

      while let Some((distance, current)) = stack.pop() {

        if distance != 0 && junctions.contains(&current) {
          res.entry(junction.clone()).or_insert(vec![]).push((distance, current.clone()));
          continue;
        }

        for (di, dj) in offsets(grid[current.0][current.1]) {
          let (ni, nj) = (current.0 as i32 + di, current.1 as i32 + dj);

          if ni > 0 &&
            ni < grid.len() as i32 &&
            nj > 0 &&
            nj < grid[0].len() as i32 &&
            grid[ni as usize][nj as usize] != '#' &&
            !visited.contains(&(ni as usize, nj as usize)) {
              stack.push((distance + 1, (ni as usize, nj as usize)));
              visited.insert((ni as usize, nj as usize));
          }
        }
      }
  }

  res
}

fn dfs(
  current: (usize, usize),
  dest: (usize, usize),
  graph: &HashMap<(usize, usize), Vec<(usize, (usize, usize))>>,
  seen: &mut HashSet<(usize, usize)>,
  current_distance: usize,
) {
  if current == dest {
    if current_distance > 6500 {
      println!("Found path: {:?}", current_distance);
    }
  }

  seen.insert(current.clone());
  for (distance, next) in &graph[&current] {
    if !seen.contains(&next) {
      dfs(next.clone(), dest, graph, seen, current_distance + distance);
    }
  }
  seen.remove(&current);
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let start = (0, lines[0].find('.').unwrap());
  let dest = (grid.len() - 1, lines[grid.len() - 1].find('.').unwrap());

  let junctions = junctions(&start, &dest, &grid);

  println!("junctions: {:?}", junctions);

  let graph = graph(&grid, &junctions);

  println!("graph: {:?}", graph);

  dfs(start, dest, &graph, &mut HashSet::new(), 0);

  Ok(())
}