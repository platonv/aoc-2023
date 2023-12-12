use std::collections::HashMap;

extern crate anyhow;
use anyhow::{anyhow, bail};

fn is_number(c: char) -> bool {
  c.is_digit(10)
}

fn is_symbol(c: char) -> bool {
  !is_number(c) && c != '.'
}

#[derive(Debug)]
struct State {
  sum: u32,
  current_number: u32,
  is_part: bool,
  gear_map: HashMap<(u32, u32), u32>,
  matrix: Vec<Vec<char>>,
  part_numbers: Vec<Vec<u32>>,
}

impl State {
  fn add_digit(&mut self, digit: u32) {
    self.current_number = self.current_number * 10 + digit;
  }

  fn mark_as_part(&mut self) {
    self.is_part = true;
  }

  fn check_number(&mut self, i: usize, j: usize) {
    if self.is_part && self.current_number != 0 {
      let mut j1 = j;
      while j1 > 0 && is_number(self.matrix[i][j1 - 1]) {
        self.part_numbers[i][j1 - 1] = self.current_number;
        j1 -= 1;
      }
      self.sum += self.current_number;
      self.is_part = false;
    }
    self.current_number = 0;
  }

  fn is_adjacent_symbol(&mut self, i: i32, j: i32) -> bool {
    let mut found_symbol = false;
    for offset_1 in -1..2 {
      for offset_2 in -1..2 {
        if offset_1 == 0 && offset_2 == 0 {
          continue;
        }
        match self.matrix.get((i + offset_1) as usize).and_then(|row| row.get((j + offset_2) as usize)) {
          Some(c) => {
            if is_symbol(*c) {
              found_symbol = true;
            }
            if *c == '*' {
              self.gear_map.insert(((i + offset_1) as u32, (j + offset_2) as u32), 1);
            }
          }
          None => {}
        }
      }
    }
    found_symbol
  }

  fn find_gears(&mut self) -> Vec<u32> {
    let mut ratios = vec![];
    let mut part_numbers = vec![];

    for i in 0..self.matrix.len() as i32 {
      for j in 0..self.matrix[i as usize].len() as i32 {
        if self.matrix[i as usize][j as usize] == '*' {
          let mut part_number = 0;
          let mut found = false;

          for offset_1 in -1..2 {
            for offset_2 in -1..2 {
              if offset_1 == 0 && offset_2 == 0 {
                if found {
                  part_numbers.push(part_number);
                  part_number = 0;
                  found = false;
                }
                continue;
              }

              match self.part_numbers.get((i + offset_1) as usize).and_then(|row| row.get((j + offset_2) as usize)) {
                Some(n) => {
                  if *n > 0 {
                    part_number = *n;
                    found = true;
                  } else {
                    if found {
                      part_numbers.push(part_number);
                      part_number = 0;
                      found = false;
                    }
                  }
                }
                None => {}
              }

            }

            if found {
              part_numbers.push(part_number);
              part_number = 0;
              found = false;
            }
          }

          if part_numbers.len() == 2 {
            ratios.push(part_numbers[0] * part_numbers[1]);
          }
          part_numbers.clear();
        }
      }
    }
    ratios
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let mut state = State {
    sum: 0,
    current_number: 0,
    is_part: false,
    gear_map: HashMap::new(),
    matrix: lines.iter().map(|line| line.chars().collect()).collect(),
    part_numbers: vec![vec![0; lines.len()]; lines[0].len()],
  };
  for i in 0..state.matrix.len() {
    for j in 0..state.matrix[i].len() {
      if let Some(digit) = state.matrix[i][j].to_digit(10) {
        if state.is_adjacent_symbol(i as i32, j as i32) {
          state.mark_as_part()
        }
        state.add_digit(digit);
      } else {
        state.check_number(i, j);
      }
    }
    state.check_number(i, state.matrix[i].len() - 1);
  }

  println!("Part 1: {}", state.sum);
  println!("Part 2: {:?}", state.find_gears().iter().fold(0, |acc, x| acc + x));
}