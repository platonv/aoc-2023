use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

extern crate anyhow;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Block {
  position: (usize, usize),
  cost: u32,
  current_direction: (i32, i32),
  direction_count: u32,
}

impl Ord for Block {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for Block {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

fn offsets(grid: &Vec<Vec<u32>>, position: (usize, usize)) -> Vec<(i32, i32)> {
  let mut offsets = Vec::new();

  if position.0 > 0 { offsets.push((-1, 0)); }
  if position.0 < grid.len() - 1 { offsets.push((1, 0)); }
  if position.1 > 0 { offsets.push((0, -1)); }
  if position.1 < grid[0].len() - 1 { offsets.push((0, 1)); }

  offsets
}

fn djikstra(grid: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<(u32, HashMap<(usize, usize), Block>)> {
  let mut seen: HashSet<((usize, usize), (i32, i32), u32)> = HashSet::new();
  let mut directions: HashMap<(usize, usize), Block> = HashMap::new();
  let mut heap = BinaryHeap::new();

  heap.push(Block { position: start, cost: 0, current_direction: (0, 0), direction_count: 1 });

  while let Some(Block { position, cost , current_direction, direction_count }) = heap.pop() {
    if position == end && direction_count >= 4 { return Some((cost, directions)); }

    if seen.contains(&(position, current_direction, direction_count)) { continue; }
    seen.insert((position, current_direction, direction_count));

    if direction_count < 10 && current_direction != (0, 0) {
      let new_i = position.0 as i32 + current_direction.0;
      let new_j = position.1 as i32 + current_direction.1;
      if new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 {
        let new_block = Block { position: (new_i as usize, new_j as usize), cost: cost + grid[new_i as usize][new_j as usize], current_direction: current_direction, direction_count: direction_count + 1 };
        directions.insert((new_i as usize, new_j as usize), new_block.clone());
        heap.push(new_block);
      }
    }

    if direction_count >= 4 || current_direction == (0, 0) {
      for offset in offsets(grid, position) {
        if offset.0 == -current_direction.0 && offset.1 == -current_direction.1 { continue; }
        if offset == current_direction { continue; }

        let new_i = (position.0 as i32 + offset.0) as usize;
        let new_j = (position.1 as i32+ offset.1) as usize;

        let new_cost = cost + grid[new_i][new_j];

        println!("2: {} {} {} {}", position.0, position.1, new_i, new_j);

        let new_block = Block { position: (new_i, new_j), cost: new_cost, current_direction: offset, direction_count: 1 };
        directions.insert((new_i, new_j), new_block.clone());
        heap.push(new_block);
      }
    }
  }
  None
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let grid: Vec<Vec<u32>> = lines.iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      print!("{}", grid[i][j]);
    }
    println!("");
  }
  println!("");

  let (res, _) = djikstra(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1)).unwrap();

  println!("part1: {:?}", res);

  Ok(())
}