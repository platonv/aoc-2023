use std::collections::HashMap;
use std::str::FromStr;
use core::cmp::Ordering;
use std::collections::VecDeque;
use std::cmp;
use std::num;

use regex::Regex;
extern crate regex;
extern crate anyhow;

use anyhow::{anyhow, bail};

const FIVE_KIND: u32 = 7;
const FOUR_KIND: u32= 6;
const FULL_HOUSE: u32= 5;
const THREE_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

fn card_to_u32(c: char) -> u32 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 10,
    'J' => 1,
    _ => c.to_digit(10).unwrap(),
  }
}


#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: u64,
}

impl FromStr for Hand {
  type Err = anyhow::Error;

  fn from_str(str: &str) -> Result<Hand, anyhow::Error> {
    println!("{}", str);
    let (cards, bid) = match str.split_whitespace().collect::<Vec<&str>>().as_slice() {
      [cards, bid] => (
        cards.chars().into_iter().map(|c| card_to_u32(c)).collect::<Vec<u32>>(), 
        bid.parse::<u64>()?
      ),
      _ => bail!("Invalid input"),
    };
    Ok(Hand { cards, bid })
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    let hand_type = self.hand_type();
    let other_hand_type = other.hand_type();
    if hand_type.0 == other_hand_type.0 {
      hand_type.1.cmp(&other_hand_type.1)
    } else {
      hand_type.0.cmp(&other_hand_type.0)
    }
  }
}

impl Hand {
  fn hand_type(&self) -> (u32, Vec<u32>) {
    let mut counts = HashMap::new();
    for c in self.cards.iter() {
      counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    println!("{:?}", self.cards);
    let j_count = counts.get(&1).unwrap_or(&0).clone();
    counts.remove(&1);

    let max_i = counts.iter()
                           .enumerate()
                           .max_by(|(_, (k0, value0)), (_, (k1, value1))| (value0, k0).cmp(&(value1, k1)))
                           .map(|(_, (k, _))| **k).unwrap_or(0) as u32;
    
    counts.insert(&max_i, counts.get(&max_i).unwrap_or(&0) + j_count);

    println!("{:?}", counts);

    let mut cards: Vec<u32> = self.cards.clone();
    if let Some(high) = counts.values().find(|&x| *x == 5) {
      return (FIVE_KIND, cards);
    } else if let Some(high) = counts.values().find(|&x| *x == 4) {
      return (FOUR_KIND, cards);
    } else if let (Some(high), Some(high2)) = (counts.values().find(|&x| *x == 3), counts.values().find(|&x| *x == 2)) {
      return (FULL_HOUSE, cards);
    } else if (counts.values().filter(|&x| *x == 2).count() == 2) {
      return (TWO_PAIR, cards);
    } else if let Some(high) = counts.values().find(|&x| *x == 3) {
      return (THREE_KIND, cards);
    } else if let Some(high) = counts.values().find(|&x| *x == 2) {
      return (ONE_PAIR, cards);
    } else {
      (HIGH_CARD, cards.clone())
    }
  }
}

fn main() {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

  let mut hands = Vec::new();
  for line in lines {
    let hand = Hand::from_str(line).unwrap();
    hands.push(hand);
  }

  hands.sort_by(|a, b| a.cmp(b));

  for (i, hand) in hands.iter().enumerate() {
    println!("{}: {:?}, {:?}", i + 1, hand, hand.hand_type());
  }
  let mut s = 0;
  for (i, hand) in hands.iter().enumerate() {
    s += (i as u32 + 1) * hand.bid as u32; 
  }

  println!("{:?}", s);
}