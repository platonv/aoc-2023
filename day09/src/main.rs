fn compute_differences(numbers: Vec<i32>) -> Vec<i32> {
  let mut differences: Vec<i32> = Vec::new();

  for i in 1..numbers.len() {
    differences.push(numbers[i] - numbers[i - 1]);
  }

  differences
}

fn compute_pyramid(numbers: Vec<i32>) -> Vec<Vec<i32>> {
  let mut current = numbers.clone();
  let mut res = vec![current.clone()];
  while !current.iter().all(|&x| x == 0) {
    current = compute_differences(current);
    res.push(current.clone());
  }
  res
}

fn compute_next(pyramid: &mut Vec<Vec<i32>>) -> i32 {
  pyramid.last_mut().unwrap().push(0);

  for i in (0..pyramid.len() - 1).rev() {
    let a = *pyramid[i + 1].last().unwrap();
    let b = *pyramid[i].last().unwrap();
    pyramid[i].push(a + b);
  }

  *pyramid[0].last().unwrap()
}

fn compute_previous(pyramid: &mut Vec<Vec<i32>>) -> i32 {
  pyramid.last_mut().unwrap().push(0);

  for i in (0..pyramid.len() - 1).rev() {
    let a = *pyramid[i + 1].first().unwrap();
    let b = *pyramid[i].first().unwrap();
    pyramid[i].insert(0, -a + b);
  }

  *pyramid[0].first().unwrap()
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();
  let mut s1 = 0;
  let mut s2 = 0;

  for line in lines {
    println!("{}", line);

    let mut numbers: Vec<i32> = Vec::new();
    numbers = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut pyramid = compute_pyramid(numbers.clone());
    let next = compute_next(&mut pyramid);
    let previous = compute_previous(&mut pyramid);
    println!("{:?}", pyramid);
    println!("{:?}", next);
    s1 += next;
    s2 += previous;
    println!("__________________________________")
  }

  println!("Part1 {}", s1);
  println!("Part2 {}", s2);
}