use std::collections::HashMap;
extern crate anyhow;
use anyhow::{anyhow, bail};

fn compute_mount_points(grid: &Vec<Vec<char>>) -> HashMap<usize, Vec<usize>> {
  let mut mount_points: HashMap<usize, Vec<usize>> = HashMap::new();

  for column in 0..grid[0].len() {
    let mut mount_points_for_row = mount_points.entry(column).or_insert(Vec::new());
    mount_points_for_row.push(0);
    for row in 0..grid.len() {
      if grid[row][column] == '#' {
        mount_points_for_row.push(row + 1);
      }
    }
  }

  mount_points
}

fn mounted_map(grid: &Vec<Vec<char>>, mount_points: &HashMap<usize, Vec<usize>>) -> HashMap<(usize, usize), usize> {
  let mut tilted_map: HashMap<(usize, usize), usize> = HashMap::new();
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' {
        let mut previous_hang_point = 0;
        for hang_point in mount_points[&j].iter() {
          if *hang_point >= i + 1 {
            break;
          }
          previous_hang_point = *hang_point;
        }
        tilted_map.insert((previous_hang_point, j), tilted_map.get(&(previous_hang_point, j)).unwrap_or(&0) + 1);
      }
    }
  }
  tilted_map 
}

fn weights(total_rows: usize, tilted: HashMap<(usize, usize), usize>) -> usize {
  let mut s = 0;
  for ((row, _), count) in tilted.iter() {
    let weight = total_rows - row;
    for i in ((weight - *count + 1)..weight + 1).rev() {
      s += i;
    }
  }
  s
}

fn tilted(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut res = grid.clone();

  for j in 0..grid[0].len() {
    let mut i = 0;
    while i < grid.len() {
      while i < grid.len() && grid[i][j] == '#' {
        i += 1;
      }

      let mut count = 0;
      let mut mount = i;

      while i < grid.len() && grid[i][j] != '#' {
        if grid[i][j] == 'O' {
          count += 1;
        }
        i += 1;
      }


      for i in mount..mount + count {
        res[i][j] = 'O';
      }
      for i in (mount + count)..i {
        res[i][j] = '.';
      }
    }
  }
  res
}

fn spin(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut res = Vec::new();
  let mut row = Vec::new();
  row.resize(grid.len(), '.');
  res.resize(grid[0].len(), row.clone());


  for i in 0..grid[0].len() {
    for j in 0..grid.len() {
      res[i][j] = grid[j][i];
    }
  }
  for i in 0..grid.len() {
    res[i].reverse();
  }
  res
}

fn cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let initial_tilt = tilted(&grid);
  let spin1 = tilted(&spin(&initial_tilt));
  let spin2 = tilted(&spin(&spin1));
  let spin3 = tilted(&spin(&spin2));
  let spin4 = spin(&spin3);

  print_grid(&spin4);
  spin4
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
  let mut res = 0;
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == 'O' {
        res += grid.len() - i;
      }
    }
  }
  res
}

fn part_2(grid: &Vec<Vec<char>>) -> usize {
  let mut patterns: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
  let mut index: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

  let mut res = 0;
  let mut cycled = grid.clone();
  for i in 0..1000000000 {
    println!("i: {}", i);
    cycled = cycle(&cycled);


    // let mount_points = compute_mount_points(&cycled);
    // let map = mounted_map(&cycled, &mount_points);
    // res = weights(cycled.len(), map);
    res = total_load(&cycled);

    println!("res: {}", res);

    if index.contains_key(&cycled) {
      let cycle_interval = i - index[&cycled];
      let good_cycle = (1000000000 - 1 - index[&cycled]) % cycle_interval + index[&cycled];
      let c = patterns[&good_cycle].clone();

      res = total_load(&c);

      break;
    }

    index.insert(cycled.clone(), i);
    patterns.insert(i, cycled.clone());
  }
  res
}

fn print_grid(grid: &Vec<Vec<char>>) {
  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      print!("{}", grid[i][j]);
    }
    println!("");
  }
  println!("");
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let mount_points = compute_mount_points(&grid);

  let mounted_map = mounted_map(&grid, &mount_points);

  for i in 0..grid.len() {
    for j in 0..grid[0].len() {
      if grid[i][j] == '#' {
        print!("#");
      } else if mounted_map.contains_key(&(i, j)) {
        print!("{}", mounted_map[&(i, j)]);
      } else {
        print!(".");
      }
    }
    println!("");
  }

  println!("---------------------");

  println!("weights: {}", weights(grid.len(), mounted_map));

  cycle(&grid);

  let part_2_res = part_2(&grid);

  println!("part 2: {}", part_2_res);

  Ok(())
}