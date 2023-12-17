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
    if position == end { return Some((cost, directions)); }

    if seen.contains(&(position, current_direction, direction_count)) { continue; }

    seen.insert((position, current_direction, direction_count));

    for offset in offsets(grid, position) {
      if offset.0 == -current_direction.0 && offset.1 == -current_direction.1 { continue; }

      let new_direction_count = if offset == current_direction { direction_count + 1 } else { 1 };
      if new_direction_count > 3 { continue; }

      let new_i = (position.0 as i32 + offset.0) as usize;
      let new_j = (position.1 as i32+ offset.1) as usize;

      let new_cost = cost + grid[new_i][new_j];

      let new_block = Block { position: (new_i, new_j), cost: new_cost, current_direction: offset, direction_count: new_direction_count };
      directions.insert((new_i, new_j), new_block.clone());
      heap.push(new_block);
    }
  }
  None
}

fn offset_to_char(offset: (i32, i32)) -> char {
  match offset {
    (-1, 0) => '^',
    (1, 0) => 'v',
    (0, -1) => '<',
    (0, 1) => '>',
    _ => ' ',
  }
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let grid: Vec<Vec<u32>> = lines.iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      print!("{}", grid[i][j]);
    }
    println!("");
  }
  println!("");

  let (res, directions) = djikstra(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1)).unwrap();

  println!("part1: {:?}", res);

  Ok(())
}