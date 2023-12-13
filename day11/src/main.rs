extern crate anyhow;
use std::collections::{ HashSet, HashMap };

use anyhow::{anyhow, bail};

const SCALE_FACTOR: usize = 1000000;

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

fn expand(grid: &Vec<Vec<char>>, nodes: HashMap<usize, (usize, usize)>) -> HashMap<usize, (usize, usize)> {
  let mut res = nodes.clone();
  let mut inserted = 1;
  for line_i in 0..grid.len() {
    if grid[line_i].iter().all(|&c| c == '.') {
      for (node_id, (i, j)) in nodes.iter() {
        if *i > line_i {
          let current_i = res.get(node_id).unwrap().0;
          let current_j = res.get(node_id).unwrap().1;
          res.insert(*node_id, (current_i + SCALE_FACTOR - 1, current_j));
        }
      }
      inserted += 1;
    }
  }

  inserted = 1;
  for j in 0..grid[0].len() {
    let mut all_clear = true;
    for i in 0..grid.len() {
      if grid[i][j] == '#' {
        all_clear = false;
        break;
      }
    }

    if all_clear {
      for (node_id, (i, j_column)) in nodes.iter() {
        if *j_column > j {
          let current_i = res.get(node_id).unwrap().0;
          let current_j = res.get(node_id).unwrap().1;
          println!("Pushing right node {:?} to {:?}", node_id + 1, (current_i, current_j + SCALE_FACTOR - 1));
          res.insert(*node_id, (current_i, current_j + SCALE_FACTOR - 1));
        }
      }
      inserted += 1;
    }
  }
  res
}

fn distances(map: HashMap<usize, (usize, usize)>) -> usize {
  let mut res = Vec::new();

  for i in 0..map.len() {
    for j in i + 1..map.len() {
      let node1 = map.get(&i).unwrap();
      let node2 = map.get(&j).unwrap();
      let distance = node1.0.abs_diff(node2.0) + node1.1.abs_diff(node2.1);
      println!("node1: {:?}, node2: {:?}, distance: {:?}", i + 1, j + 1, distance);
      res.push(distance);
    }
  }
  res.iter().sum()
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let grid = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let nodes = name_nodes(&grid);

  let expanded_nodes = expand(&grid, nodes);

  println!("expanded_nodes: {:?}", expanded_nodes);

  let distance_pairs = distances(expanded_nodes);

  println!("distance_pairs: {:?}", distance_pairs);

  Ok(())
}