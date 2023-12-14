use std::collections::HashMap;
use std::str::FromStr;
use std::collections::VecDeque;
use std::cmp;
use std::num;

use regex::Regex;
extern crate regex;
extern crate anyhow;

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
  fn solve(&self) -> u64 {
    let time = self.time;
    let record = self.record;

    let b = -1.0 * (time as f64);
    let c = record as f64;
    let a = 1 as f64;

    let x1 = (-b - f64::sqrt(b * b - 4.0 * a * c)) / 2.0 * a;
    let x2 = (-b + f64::sqrt(b * b - 4.0 * a * c)) / 2.0 * a;

    (x2 - x1) as u64
  }
}

fn merge_races(races: &Vec<Race>) -> Race {
    let mut time_str = "".to_string();
    let mut record_str = "".to_string();

    for race in races {
        time_str = time_str + &race.time.to_string();
        record_str = record_str + &race.record.to_string();
    }

    Race { 
        time: time_str.parse::<u64>().unwrap(), 
        record: record_str.parse::<u64>().unwrap(),
    }
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let time_line_str = lines.first().unwrap().split(":").last().unwrap();
  let distance_line_str = lines[1].split(":").last().unwrap();

  let times = time_line_str.split_whitespace().map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
  let durations = distance_line_str.split_whitespace().map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();

  let mut races = Vec::new();
  for i in 0..times.len() {
    let race = Race { time: times[i], record: durations[i] };
    races.push(race);
  }

  let race = merge_races(&races);
  println!("{:?}", race);
  let answer = race.solve();
  println!("Answer: {}", answer);
}