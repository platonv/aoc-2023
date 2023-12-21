use std::ops::DerefMut;
use std::rc::Rc;
use std::{collections::HashMap, thread::current};
use std::collections::VecDeque;
use std::cell::RefCell;

extern crate anyhow;
use anyhow::{Result, anyhow, bail, Ok};

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
  LowPulse,
  HighPulse,
}

#[derive(Debug, Clone)]
enum FlipFlopState {
  On,
  Off
}

#[derive(Debug, Clone)]
enum ModuleKind {
  FlipFlop(FlipFlopState),
  Conjunction(HashMap<String, Pulse>),
  Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
  id: String,
  kind: ModuleKind,
  destinations: Vec<String>,
}

fn parse_destination(s: &str) -> Result<Module> {
  let re = Regex::new(r"(?<kind>[%&]*)(?<name>\w+)\s+->\s+(?<destinations>[\w\s\,]*)")?;

  let captures = match re.captures(s) {
    Some(captures) => captures,
    _ => bail!("Invalid destination: {}", s),
  };

  let kind = match captures.name("kind") {
    Some(m) => match m.as_str() {
      "%" => ModuleKind::FlipFlop(FlipFlopState::Off),
      "&" => ModuleKind::Conjunction(HashMap::new()),
      _ => ModuleKind::Broadcast
    },
    _ => bail!("Invalid kind: {}", s)
  };
  let name = captures.name("name").unwrap().as_str();
  let destinations = captures.name("destinations").unwrap().as_str().split(", ").map(|s| String::from(s)).collect();

  Ok(Module {
    id: String::from(name),
    kind,
    destinations,
  })
}

impl Module {
  fn trigger(&mut self, pulse: &Pulse, from: String) -> Option<Pulse> {
    match &mut self.kind {

      ModuleKind::FlipFlop(state) => {
        match pulse {
          Pulse::LowPulse => {
            match state {
              FlipFlopState::On => {
                self.kind = ModuleKind::FlipFlop(FlipFlopState::Off);
                Some(Pulse::LowPulse)
              },
              FlipFlopState::Off => {
                self.kind = ModuleKind::FlipFlop(FlipFlopState::On);
                Some(Pulse::HighPulse)
              },
            }
          },
          Pulse::HighPulse => None // Nothing happens
        }
      },
      ModuleKind::Conjunction(ref mut remembered_pulses) => {
        remembered_pulses.insert(from.clone(), pulse.clone());
        let res = remembered_pulses.values().all(|pulse| *pulse == Pulse::HighPulse);
        if res {
          Some(Pulse::LowPulse)
        } else {
          Some(Pulse::HighPulse)
        }
      },

      ModuleKind::Broadcast => {
        Some(pulse.clone())
      }
    }
  }
}

fn push_button(modules: &HashMap<String, Rc<RefCell<Module>>>) -> Result<(u64, u64)> {
  let mut queue: VecDeque<(Rc<RefCell<Module>>, Pulse, String)> = VecDeque::new();
  let broadcaster = modules.get("broadcaster").unwrap();

  let mut lows = 1;
  let mut highs = 0;

  queue.push_back((broadcaster.clone(), Pulse::LowPulse, String::new()));
  while queue.len() > 0 {
    let (module, pulse, from) = queue.pop_front().unwrap();

    let rc = module.clone();
    let mut m = rc.try_borrow_mut()?;
    let trigger_result = m.trigger(&pulse, from);


    match trigger_result {
      Some(next_pulse) => 
        for destination in &m.destinations {
          println!("{} -{:?}> {}", m.id, next_pulse, destination);
          highs += match next_pulse {
            Pulse::HighPulse => 1,
            _ => 0,
          };
          lows += match next_pulse {
            Pulse::LowPulse => 1,
            _ => 0,
          };

          let dOpt= modules.get(destination);
          if dOpt.is_none() {
            continue;
          }

          let d = dOpt.unwrap();

          queue.push_back((d.clone(), next_pulse.clone(), m.id.clone()));
        }
      None => {}
    }
  }
  println!("");
  println!("highs: {}, lows: {}", highs, lows);
  println!("");
  Ok((highs, lows))
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input1.txt").lines().collect();

  let modulesMap = lines.iter()
                                                       .map(|s| parse_destination(s).unwrap())
                                                       .map(|m| (m.id.clone(), Rc::new(RefCell::new(m))))
                                                       .collect::<HashMap<String, Rc<RefCell<Module>>>>();

  for (id, module) in &modulesMap {
    for destination in &module.borrow().destinations {
      match modulesMap.get(destination) {
        Some(module) => {
          match module.borrow_mut().kind {
            ModuleKind::Conjunction(ref mut remembered_pulses) => {
              remembered_pulses.insert(id.clone(), Pulse::LowPulse);
            },
            _ => {}

          }
        },
        None => {}
      }
    }
  };

  let mut s1 = 0;
  let mut s2 = 0;
  for _ in 0..1000 {
    let (r1, r2) = push_button(&modulesMap)?;
    s1 += r1;
    s2 += r2;
  }

  let res = s1 * s2;

  println!("res: {:?}", res);

  Ok(())
}