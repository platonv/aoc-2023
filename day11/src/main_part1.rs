extern crate anyhow;
use std::collections::{ HashSet, HashMap };

use anyhow::{anyhow, bail};

fn expand(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut res = grid.clone();

  let mut inserted = 0;
  for i in 0..grid.len() {
    if grid[i].iter().all(|&c| c == '.') {
      res.insert(i + inserted, vec!['.'; grid[0].len()]);
      inserted += 1;
    }
  }

  inserted = 0;
  for j in 0..grid[0].len() {
    let mut all_clear = true;
    for i in 0..grid.len() {
      if grid[i][j] == '#' {
        all_clear = false;
        break;
      }
    }

    if all_clear {
      for i in 0..res.len() {
        res[i].insert(j + inserted, '.');
      }
      inserted += 1;
    }
  }

  res
}

fn name_nodes(grid: &Vec<Vec<char>>) -> HashMap<usize, (usize, usize)> {
  let mut res = HashMap::new();

  let mut node_id = 0;
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j] == '#' {
        res.insert(node_id, (i, j));
        node_id += 1;
      }
    }
  }

  res
}

fn distances(map: HashMap<usize, (usize, usize)>) -> usize {
  let mut res = Vec::new();

  println!("map: {:?}", map);

  for i in 0..map.len() {
    for j in i + 1..map.len() {
      let node1 = map.get(&i).unwrap();
      let node2 = map.get(&j).unwrap();
      let distance = node1.0.abs_diff(node2.0) + node1.1.abs_diff(node2.1);
      res.push(distance);
    }
  }
  res.iter().sum()
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let expanded_grid = expand(&grid);

  let distance_pairs = name_nodes(&expanded_grid);

  println!("distance_pairs: {:?}", distance_pairs);

  let distances = distances(distance_pairs);

  println!("distances: {:?}", distances);

  // for i in 0..expanded_grid.len() {
  //   for j in 0..expanded_grid[i].len() {
  //     print!("{}", expanded_grid[i][j]);
  //   }
  //   println!();
  // }

  Ok(())
}