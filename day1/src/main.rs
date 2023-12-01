use std::collections::HashMap;

// Get the first and last digit from a string
fn get_first_and_last_digit(s: &str) -> Option<(u32, u32)> {
  let mut mapped_digits = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
  ]);

  let search_terms = mapped_digits.keys().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut first_string: Option<&str> = None;
  let mut last_string: Option<&str> = None;
  let mut min_index = s.len() + 1;
  let mut max_index = 0;
  for search_str in search_terms {
    if let Some(start_index) = s.find(&search_str) {
      if start_index <= min_index {
        first_string = Some(&s[start_index..(start_index + search_str.len())]);
        min_index = start_index;
      }
    }

    if let Some(end_index) = s.rfind(&search_str) {
      if end_index >= max_index {
        last_string = Some(&s[end_index..(end_index + search_str.len())]);
        max_index = end_index;
      }
    }
  }

  if let (Some(first), Some(last)) = (first_string, last_string) {
    println!("{:?}", s);
    println!("{} {}", first, last);
    let a = mapped_digits.get(first)?;
    let b = mapped_digits.get(last)?;
    Some((*a, *b))
  } else {
      None
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let result = lines.iter().map(|line| {
    get_first_and_last_digit(line)
  }).filter(|v| v.is_some()).map(|v| v.unwrap()).map(|(first, last)| {
    first * 10 + last
  }).sum::<u32>();

  println!("Result: {}", result);
}