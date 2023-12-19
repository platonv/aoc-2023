use std::{collections::HashMap, thread::current};
use std::str::FromStr;

extern crate anyhow;
use anyhow::{Result, anyhow, bail, Ok};

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
enum Condition {
  GreaterThan(i64),
  LessThan(i64),
  Any,
}

#[derive(Debug, Clone)]
struct Rule {
  property: String,
  condition: Condition,
  target: String,
}

#[derive(Debug)]
struct Workflow {
  name: String,
  rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part {
  properties: HashMap<String, i64>,
}

impl Part {
  fn rating(&self) -> i64 {
    self.properties.iter().fold(0, |acc, (_, value)| acc + value)
  }
}

impl FromStr for Rule {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let re = Regex::new(r"(?P<property>[a-z]*)*(?P<condition>[<>=])*(?P<condition_target>\d*)*:(?P<target>\w+)")?;

    let captures = match re.captures(s) {
      Some(captures) => captures,
      _ => return Ok(Rule {
        property: String::from(""),
        condition: Condition::Any,
        target: String::from(s),
      })
    };

    let property = captures.name("property").map(|m| m.as_str()).unwrap_or("");
    let condition = captures.name("condition").map(|m| m.as_str());
    let condition_target = captures.name("condition_target").map(|m| m.as_str());
    let target = captures.name("target").unwrap().as_str();

    let condition = match condition {
      Some(">") => Condition::GreaterThan(i64::from_str(condition_target.unwrap())?),
      Some("<") => Condition::LessThan(i64::from_str(condition_target.unwrap())?),
      _ => Condition::Any,
    };

    Ok(Rule {
      property: String::from(property),
      condition,
      target: String::from(target),
    })
  }
}

impl FromStr for Workflow {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    let re = Regex::new(r"(?P<name>\w+)\{(?P<rules>.*)\}")?;

    let captures = re.captures(s).ok_or(anyhow!("Invalid Workflow"))?;
    let name = captures.name("name").unwrap().as_str();
    let rules_str = captures.name("rules").unwrap().as_str();

    let rules = rules_str.split(",").map(|rule| Rule::from_str(rule)).collect::<Result<Vec<Rule>>>()?;

    Ok(Workflow {
      name: String::from(name),
      rules,
    })
  }
}

fn match_rule(rule: &Rule, part: &Part) -> Option<String> {
  match part.properties.get(&rule.property) {
    Some(property) => 
      match rule.condition {
        Condition::GreaterThan(value) => if (property > &value) { Some(rule.target.clone()) } else { None },
        Condition::LessThan(value) => if (property < &value) { Some(rule.target.clone()) } else { None},
        Condition::Any => Some(rule.target.clone()),
      }
    _ => Some(rule.target.clone()),
  }
}

fn check_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
  let mut current_workflow = &workflows["in"];


  loop {
    for rule in &current_workflow.rules {
      match match_rule(rule, part) {
        Some(target) => {
          if target == "A" {
            return true;
          } else if target == "R" {
            return false;
          }
          current_workflow = &workflows[&target];
          break;
        },
        _ => continue,
      }
    }
  }
}

fn solve_part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> i64 {
  let accepted_parts: Vec<&Part> = parts.iter().filter(|part| check_part(part, workflows)).collect();

  accepted_parts.iter().fold(0, |acc, part| acc + part.rating())
}

fn interval(rules: &Vec<&Rule>) -> (i64, i64) {
  let mut max_min = 0;
  let mut min_max = 4001;
  for rule in rules {
    match rule.condition {
      Condition::GreaterThan(value) => {
        if value > max_min {
          max_min = value;
        }
      },
      Condition::LessThan(value) => {
        if value < min_max {
          min_max = value;
        }
      },
      _ => continue,
    }
  }

  println!("{:?} {:?}", rules, (max_min + 1, min_max - 1));
  (max_min + 1, min_max - 1)
}

fn combinations(rule_chain: &Vec<Rule>) -> i64 {
  println!("{:?}", &rule_chain.iter().map(|rule| rule.target.clone()).collect::<Vec<String>>());

  let x_rules = rule_chain.iter().filter(|rule| rule.property == "x").map(|x| x).collect::<Vec<&Rule>>();
  let x_interval = interval(&x_rules);

  let m_rules = rule_chain.iter().filter(|rule| rule.property == "m").map(|x| x).collect::<Vec<&Rule>>();
  let m_interval = interval(&m_rules);

  let a_rules = rule_chain.iter().filter(|rule| rule.property == "a").map(|x| x).collect::<Vec<&Rule>>();
  let a_interval = interval(&a_rules);

  let s_rules = rule_chain.iter().filter(|rule| rule.property == "s").map(|x| x).collect::<Vec<&Rule>>();
  let s_interval = interval(&s_rules);

  let c1 = x_interval.1 - x_interval.0 + 1;
  let c2 = m_interval.1 - m_interval.0 + 1;
  let c3 = a_interval.1 - a_interval.0 + 1;
  let c4 = s_interval.1 - s_interval.0 + 1;

  println!("{} {} {} {}", c1, c2, c3, c4);
  let res = c1 * c2 * c3 * c4;
  res
}

fn opposite_rule(rule: &Rule) -> Rule {
  match rule.condition {
    Condition::GreaterThan(value) => Rule {
      property: rule.property.clone(),
      condition: Condition::LessThan(value + 1),
      target: rule.target.clone(),
    },
    Condition::LessThan(value) => Rule {
      property: rule.property.clone(),
      condition: Condition::GreaterThan(value - 1),
      target: rule.target.clone(),
    },
    _ => Rule { // doesn't matter
      property: rule.property.clone(),
      condition: Condition::Any,
      target: rule.target.clone(),
    },
  }
}

fn solve_part2(node: String, workflows: &HashMap<String, Workflow>, rule_chain: &Vec<Rule>) -> i64 {
  // println!("{:?}", node);
  let start_workflow = &workflows[node.as_str()];
  // println!("{:?}", start_workflow);
  let mut res = 0;
  let mut remaining_chain: Vec<Rule> = rule_chain.iter().map(|x| x.to_owned()).collect();

  for rule in start_workflow.rules.iter() {
    // println!("{:?}", rule);
    match rule {
      Rule { property: _, condition: _, target } => {
        let mut new_chain = remaining_chain.clone();
        new_chain.push(rule.clone());
        if target == "A" {
          res += combinations(&new_chain);
        } else if target == "R" {
          res += 0;
        } else {
          res += solve_part2(target.clone(), workflows, &new_chain);
        }
        remaining_chain.push(opposite_rule(rule));
      }
    }
  }

  res
}

fn main() -> Result<()> {
  let lines: Vec<String> = include_str!("input2.txt").lines().collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();

  match lines.split(|s| *s == "")
  .collect::<Vec<&[String]>>().as_slice() {
    [workflow_str, part_str] => {
      let mut workflows = HashMap::new();
      for workflow_s in *workflow_str {
        let workflow = Workflow::from_str(workflow_s)?;
        workflows.insert(workflow.name.clone(), workflow);
      }
      let mut parts = Vec::new();
      for part_s in *part_str {
        let mut part_str_inner = part_s.clone();
        part_str_inner.remove(part_s.find("{").unwrap());
        part_str_inner = part_str_inner.replace("}", "");
        let part_str_inner = part_str_inner.split(",").collect::<Vec<&str>>();
        let mut properties = HashMap::new();
        for property in part_str_inner {
          let property = property.split("=").collect::<Vec<&str>>();
          properties.insert(String::from(property[0]), i64::from_str(property[1])?);
        }
        let part = Part {
          properties,
        };
        parts.push(part);
      }

      // let res = solve_part1(&workflows, &parts);

      // println!("part1: {}", res);
      let res_part2 = solve_part2("in".to_string(), &workflows, &Vec::new());

      println!("part2: {}", res_part2);
    },
    _ => bail!("invalid input")
  }

  Ok(())
}