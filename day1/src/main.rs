// Get the first and last digit from a string
fn get_first_and_last_digit(s: &str) -> Option<(u32, u32)> {
  let mut first: Option<u32> = None;
  let mut last: Option<u32> = None;
  for c in s.chars() {
    if first.is_none() {
      c.to_digit(10).map(|v| first = Some(v));
    }
    c.to_digit(10).map(|v| last = Some(v));
  }
  if let (Some(first), Some(last)) = (first, last) {
    Some((first, last))
  } else {
    None
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input.txt").lines().collect();

  let result = lines.iter().map(|line| {
    get_first_and_last_digit(line)
  }).filter(|v| v.is_some()).map(|v| v.unwrap()).map(|(first, last)| {
    println!("{} {}", first, last);
    first * 10 + last
  }).sum::<u32>();

  println!("Result: {}", result);
}