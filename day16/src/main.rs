use std::collections::HashSet;

extern crate anyhow;
use anyhow::Result;

type Direction = (i32, i32);

const NORTH_DIR: Direction = (-1, 0);
const SOUTH_DIR : Direction = (1, 0);
const EAST_DIR: Direction = (0, 1);
const WEST_DIR: Direction = (0, -1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
  position: (i32, i32),
  direction: (i32, i32),
}

fn step(grid: &Vec<Vec<char>>, beams: &mut Vec<Beam>, activations: &mut HashSet<(i32, i32, Direction)>) -> () {
  let mut new_beams = Vec::new();

  beams.retain_mut(|beam| {
    let (i, j) = beam.position;
    let (di, dj) = beam.direction;

    let new_i = i + di;
    let new_j = j + dj;

    if new_i < 0 || new_i >= grid.len() as i32 || new_j < 0 || new_j >= grid[0].len() as i32 {
      return false;
    }

    let new_direction = match grid[new_i as usize][new_j as usize] {
      '/' => {
        match beam.direction {
          NORTH_DIR => EAST_DIR,
          SOUTH_DIR => WEST_DIR,
          EAST_DIR => NORTH_DIR,
          WEST_DIR => SOUTH_DIR,
          _ => beam.direction,
        }
      },
      '\\' => {
        match beam.direction {
          NORTH_DIR => WEST_DIR,
          SOUTH_DIR => EAST_DIR,
          EAST_DIR => SOUTH_DIR,
          WEST_DIR => NORTH_DIR,
          _ => beam.direction
        }
      },
      '|' => {
        match beam.direction {
          EAST_DIR | WEST_DIR => {
            if !activations.contains(&(new_i, new_j, NORTH_DIR)) {
              let beam_up = Beam { position: (new_i, new_j), direction: NORTH_DIR };
              new_beams.push(beam_up);
              activations.insert((new_i, new_j, NORTH_DIR));
            }

            if !activations.contains(&(new_i, new_j, SOUTH_DIR)) {
              let beam_down = Beam { position: (new_i, new_j), direction: SOUTH_DIR };
              new_beams.push(beam_down);
              activations.insert((new_i, new_j, SOUTH_DIR));
            }

            return false;
          },
          _ => beam.direction
        }
      }
      '-' => {
        match beam.direction {
          NORTH_DIR | SOUTH_DIR => {
            if !activations.contains(&(new_i, new_j, EAST_DIR)) {
              let beam_east = Beam { position: (new_i, new_j), direction: EAST_DIR };
              new_beams.push(beam_east);
              activations.insert((new_i, new_j, EAST_DIR));
            }

            if !activations.contains(&(new_i, new_j, WEST_DIR)) {
              let beam_west = Beam { position: (new_i, new_j), direction: WEST_DIR };
              new_beams.push(beam_west);
              activations.insert((new_i, new_j, WEST_DIR));
            }
            return false;
          },
          _ => beam.direction
        }
      },
      _ => beam.direction,
    };

    if new_direction != beam.direction {
      if !activations.contains(&(new_i, new_j, new_direction)) {
        beam.direction = new_direction;
        activations.insert((new_i, new_j, new_direction));
      } else {
        return false;
      }
    }
    beam.position = (new_i, new_j);
    true
  });

  beams.append(&mut new_beams);
}

fn run(grid: &Vec<Vec<char>>, start_position: (i32, i32), start_direction: (i32, i32)) -> usize {
  let mut beams = Vec::from([Beam { position: start_position, direction: start_direction }]);
  let mut energy_map: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
  let mut activations: HashSet<(i32, i32, Direction)> = HashSet::new();

  while beams.len() > 0 {
    step(grid, &mut beams, &mut activations);

    for beam in beams.iter() {
      let (i, j) = beam.position;
      energy_map[i as usize][j as usize] = true;
    }
  }

  energy_map.iter().map(|row| row.iter().filter(|&b| *b).count()).sum()
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

  let energized = run(&grid, (0, -1), EAST_DIR);

  println!("part1: {:?}", energized);

  // Part 2

  let mut max_energized = 0;
  let total_runs = grid.len() * 2 + grid[0].len() * 2;

  for i in 0..grid.len() {
    let energized_west = run(&grid, (i as i32, -1), EAST_DIR);
    let energized_east = run(&grid, (i as i32, grid[0].len() as i32), WEST_DIR);

    max_energized = max_energized.max(energized_west.max(energized_east));

    println!("progress: {:?}%", ((i * 2) as f32 / total_runs as f32) * 100.0);
    println!("max_energized: {:?}", max_energized);
  }

  for j in 0..grid[0].len() {
    let energized_north = run(&grid, (-1, j as i32), SOUTH_DIR);
    let energized_south = run(&grid, (grid.len() as i32, j as i32), NORTH_DIR);

    max_energized = max_energized.max(energized_north.max(energized_south));

    println!("progress: {:?}%", ((j * 2 + grid.len() * 2) as f32 / total_runs as f32) * 100.0);
    println!("max_energized: {:?}", max_energized);
  }

  println!("part2: {:?}", max_energized);

  Ok(())
}