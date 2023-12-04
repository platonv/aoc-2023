use std::collections::HashMap;
use std::str::FromStr;

extern crate anyhow;
use anyhow::{anyhow, bail};

#[derive(Debug)]
struct Card {
  id: u32,
  numbers: Vec<u32>,
  winning_numbers: HashMap<u32, bool>,
}

impl FromStr for Card {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (id_str, all_numbers_str) = match s.split(":").collect::<Vec<&str>>().as_slice() {
      [id, sets] => (id.to_string(), sets.to_string()),
      _ => return Err(anyhow!("Invalid Card"))
    };
    let (numbers_str, winning_numbers_str) = match all_numbers_str.split("|").collect::<Vec<&str>>().as_slice() {
      [numbers, winning_numbers] => (numbers.to_string(), winning_numbers.to_string()),
      _ => return Err(anyhow!("Invalid Card"))
    };

    let numbers = numbers_str.split_whitespace().map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
    let winning_numbers = winning_numbers_str.split_whitespace().map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;

    Ok(Card { id: id_str.split_whitespace().last().unwrap().parse::<u32>()?, numbers: numbers, winning_numbers: winning_numbers.iter().map(|x| (*x, true)).collect::<HashMap<u32, bool>>() })
  }
}

impl Card {
  fn matches(&self) -> u32 {
    self.numbers.iter().filter(|&item| self.winning_numbers.contains_key(item)).count() as u32
  }

  fn score(&self) -> u32 {
    let matches = self.matches();
    println!("Matches: {}", self.matches());
    if matches == 0 {
      return 0;
    }
    let base: u32 = 2;
    (base.pow(self.matches() - 1))
  }
}

fn recurse_copies(cards: &Vec<Card>) -> u32 {
  let mut copies: HashMap<u32, u32> = HashMap::new();
  for card in cards {
    copies.insert(card.id, *copies.get(&card.id).unwrap_or(&0) + 1);
    let available_copies = copies.get(&card.id).unwrap_or(&1).clone();
    let mut matches = card.matches();
    while matches > 0 {
      copies.insert(card.id + matches, copies.get(&(card.id + matches)).unwrap_or(&0) + available_copies);
      matches -= 1;
    }
  }
  copies.values().sum::<u32>()
}


fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let cards: Result<Vec<Card>, anyhow::Error> = lines.iter().map(|line| {
    let card = Card::from_str(*line)?;
    println!("Card: {:?}", card.score());
    Ok(card)
  }).collect::<Result<Vec<Card>, _>>();

  match cards {
    Ok(cards) =>  {
      let score_sum = cards.iter().map(|card| card.score()).sum::<u32>();
      println!("Part 1: {}", score_sum);
      let res = recurse_copies(&cards);
      println!("Part 2: {}", res);
    }
    Err(e) => println!("Error: {}", e)
  }
}