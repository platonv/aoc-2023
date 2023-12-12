extern crate anyhow;
use anyhow::{anyhow, bail};

fn parse_input(input: &str) -> Result<(String, Vec<i32>), anyhow::Error> {
  match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
    [record, condition] => 
        Ok((
          record.to_string(), 
          condition.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>()?
        )),
    _ => bail!("Invalid input: {}", input),
  }
}

fn permutate(input: String, i: usize) -> Vec<String> {
  if i == input.len() {
    return vec![input.clone()];
  }
  if let Some('?') = input.chars().nth(i) {
    let mut p1 = input.clone();
    p1.replace_range(i..i+1, "#");
    let mut p2 = input.clone();
    p2.replace_range(i..i+1, ".");

    let a = permutate(p1, i + 1);
    let b = permutate(p2, i + 1);

    [a, b].concat()
  } else {
    permutate(input, i + 1)
  }
}

fn validate(record: &String, condition: &Vec<i32>) -> bool {
  // count consecutive '#'s in record
  let mut counts = Vec::new();
  let mut current_count = 0;
  for c in record.chars() {
    if c == '#' {
      current_count += 1;
    } else if current_count > 0 {
      counts.push(current_count);
      current_count = 0;
    }
  }
  if current_count > 0 {
    counts.push(current_count);
    current_count = 0;
  }

  counts == *condition
}

fn main() -> Result<(), anyhow::Error> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let parsed_lines = lines.iter().map(|line| parse_input(line)).collect::<Result<Vec<(String, Vec<i32>)>, _>>()?;

  let records = parsed_lines.iter().map(|(record, _)| record).collect::<Vec<&String>>();
  let conditions = parsed_lines.iter().map(|(_, condition)| condition).collect::<Vec<&Vec<i32>>>();

  let mut s = 0;
  // count valid permutations
  records.iter().zip(conditions.iter()).for_each(|(record, condition)| {
    let permutations = permutate(record.to_string(), 0);
    let valid_permutations = permutations.iter().filter(|p| validate(p, condition)).count();
    s += valid_permutations;
    println!("Valid permutations for record {:?}: {:?}", record, valid_permutations);
  });

  println!("Total valid permutations: {:?}", s);

  println!("Records: {:?}", records);
  println!("Conditions: {:?}", conditions);

  Ok(())
}