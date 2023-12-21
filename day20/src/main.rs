use std::ops::DerefMut;
use std::rc::Rc;
use std::{collections::HashMap, thread::current};
use std::collections::VecDeque;
use std::cell::RefCell;

extern crate anyhow;
use anyhow::{Result, anyhow, bail, Ok, Error};

extern crate regex;
use num::Integer;
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

fn solve_part2(modules: &HashMap<String, Rc<RefCell<Module>>>) -> Result<u64> {
  let mut queue: VecDeque<(Rc<RefCell<Module>>, Pulse, String)> = VecDeque::new();
  let broadcaster = modules.get("broadcaster").unwrap();
  let last = modules.values().find(|m| m.borrow().destinations.contains(&"rx".to_string())).unwrap();
  let last_id = last.borrow().id.clone();

  let mut pushes = 0;
  let last_4 = modules.values()
                                   .filter(|m| m.borrow().destinations.contains(&last.borrow().id.clone()))
                                   .map(|m| m.borrow().id.clone())
                                   .collect::<Vec<String>>();

  println!("last_4: {:?}", last_4);

  let mut found = HashMap::new();
  let mut cycles = HashMap::new();

  for id in last_4 {
    found.insert(id.clone(), false);
    cycles.insert(id.clone(), 0);
  }

  loop {
    pushes += 1;
    queue.push_back((broadcaster.clone(), Pulse::LowPulse, String::new()));
    while queue.len() > 0 {
      let (module, pulse, from) = queue.pop_front().unwrap();

      let rc = module.clone();
      let mut m = rc.try_borrow_mut()?;
      let trigger_result = m.trigger(&pulse, from.clone());

      if m.id.clone() == last_id && pulse == Pulse::HighPulse {
        found.insert(from.clone(), true);

        if cycles[&from] == 0 {
          println!("{}: {}", from, pushes);
          cycles.insert(from.clone(), pushes);
        }

        if cycles.values().all(|c| *c > 0) {
          let res = cycles.iter().fold(1, |acc, (_, cycle)| acc.lcm(cycle));
          return Ok(res);
        }
      }

      match trigger_result {
        Some(next_pulse) => {

          for destination in &m.destinations {
            let dOpt= modules.get(destination);
            if dOpt.is_none() {
              continue;
            }

            let d = dOpt.unwrap();

            queue.push_back((d.clone(), next_pulse.clone(), m.id.clone()));
          }
        }
        None => {}
      }
    }
  }
  bail!("Could not find a solution");
}

fn main() -> Result<()> {
  let lines: Vec<&str> = include_str!("input2.txt").lines().collect();

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

  let res = solve_part2(&modulesMap)?;

  println!("res: {:?}", res);

  Ok(())
}