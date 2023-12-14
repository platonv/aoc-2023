use std::collections::HashMap;
use std::str::FromStr;
use std::collections::VecDeque;
use std::cmp;

use regex::Regex;
extern crate regex;
extern crate anyhow;

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
  fn solve(&self) -> u32 {
    let time = self.time;
    let record = self.record;

    let mut count = 0;
    for choice in 0..(time + 1) {
        let distance = (time - choice) * choice;
        if  distance > record {
            count += 1;
        }
        println!("{}, {}", choice, distance);
    }
    println!("----------------------------------");
    count
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let time_line_str = lines.first().unwrap().split(":").last().unwrap();
  let distance_line_str = lines[1].split(":").last().unwrap();

  let times = time_line_str.split_whitespace().map(|str| str.parse::<u32>().unwrap()).collect::<Vec<u32>>();
  let durations = distance_line_str.split_whitespace().map(|str| str.parse::<u32>().unwrap()).collect::<Vec<u32>>();

  let mut results = Vec::new();
  for i in 0..times.len() {
    let race = Race { time: times[i], record: durations[i] };
    println!("Race: {:?}", race);
    let result = race.solve();
    results.push(result);

    println!("Result: {}", result);
  }

  let answer = results.iter().fold(1, |acc, r| acc * r);
  println!("Answer: {}", answer);
}