use std::collections::HashSet;
use regex::Regex;
use std::fmt;

use std::mem;

// Gross unsafe hack to hoist a string to static
fn string_to_static_str(s: String) -> &'static str {
  unsafe {
    let ret = mem::transmute(&s as &str);
    mem::forget(s);
    ret
  }
}

struct Building<'a> {
  floors: Vec<Floor>,
  elevator_on: usize,
  prior_state: Option<&'a Building<'a>>
}

impl<'a> Clone for Building<'a> {
  fn clone(&self) -> Building<'a> {
    let mut new_building = Building {
      floors: vec!(),
      elevator_on: self.elevator_on,
      prior_state: self.prior_state,
    };

    for floor in &self.floors {
      new_building.floors.push(floor.clone());
    }

    new_building
  }
}

impl<'a> Building<'a> {
  fn is_viable(&self) -> bool {
    for floor in self.floors.iter() {
      if !floor.is_viable() {
        return false;
      }
    }

    true
  }

  fn is_finished(&self) -> bool {
    self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
  }
}

impl<'a> fmt::Debug for Building<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in (0..self.floors.len()).rev() {
      let _ = write!(f, "{} {:?}\n", i, self.floors[i]);
    }

    write!(f, "E is on {}", self.elevator_on)
  }
}

impl<'a> fmt::Display for Building<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in (0..self.floors.len()).rev() {
      let _ = write!(f, "{} {:?}\n", i, self.floors[i]);
    }

    write!(f, "E is on {}", self.elevator_on)
  }
}

struct Floor {
  generators: HashSet<&'static str>,
  chips: HashSet<&'static str>,
}

impl Clone for Floor {
  fn clone(&self) -> Floor {
    let mut new_floor = Floor {
      generators: HashSet::new(),
      chips: HashSet::new(),
    };

    for label in &self.generators {
      new_floor.generators.insert(label);
    }

    for label in &self.chips {
      new_floor.chips.insert(label);
    }

    new_floor
  }
}

impl Floor {
  fn new() -> Self {
    Floor {
      generators: HashSet::new(),
      chips: HashSet::new(),
    }
  }

  fn is_empty(&self) -> bool {
    self.generators.len() == 0 && self.chips.len() == 0
  }

  fn remove_item(&mut self, item:&'static str, is_generator: bool) {
    let mut target = match is_generator {
      true => &mut self.generators,
      false => &mut self.chips,
    };
    target.remove(item);
  }

  fn add_item(&mut self, item:&'static str, is_generator: bool) {
    let mut target = match is_generator {
      true => &mut self.generators,
      false => &mut self.chips,
    };
    target.insert(item);
  }

  fn is_viable(&self) -> bool {

    // println!("G {:?} C {:?}", generators, chips);
    if self.generators.len() == 0 {
      return true;
    }

    self.chips.iter().find(|&chip| !self.generators.contains(chip)).is_none()
  }
}

impl fmt::Debug for Floor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "G{:?} C{:?},", self.generators, self.chips)
  }
}

fn permute_items_on_floor(chip_map: &HashSet<&'static str>, generator_map: &HashSet<&'static str>) -> Vec<Vec<(bool, &'static str)>> {

  let generators:Vec<&'static str> = generator_map.clone().into_iter().collect();
  let chips:Vec<&'static str> = chip_map.clone().into_iter().collect();

  let mut rvalue:Vec<Vec<(bool, &'static str)>> = vec!();

  for i in 0..chips.len() {
    for j in 0..generators.len() {
      let mut tmp = vec!();
      tmp.push((true, generators[j]));
      tmp.push((false, chips[i]));
      rvalue.push(tmp);

      let mut tmp = vec!();
      tmp.push((true, generators[j]));
      rvalue.push(tmp);

      for k in j+1..generators.len() {
        let mut tmp = vec!();
        tmp.push((true, generators[j]));
        tmp.push((true, generators[k]));
        rvalue.push(tmp);
      }
    }

    for j in i+1..chips.len() {
      let mut tmp = vec!();
      tmp.push((false, chips[j]));
      tmp.push((false, chips[i]));
      rvalue.push(tmp);      
    }

    let mut tmp = vec!();
    tmp.push((false, chips[i]));
    rvalue.push(tmp);
  }

  // Handle the case where only the elevator moves
  rvalue.push(vec!());

  rvalue
}

fn generate_potential_states<'a>(starting_states: &Vec<Building>, seen_states:& mut HashSet<String>) -> Vec<Building<'a>> {
  let mut rvalue = vec!();

  for starting_state in starting_states {

    let mut target_floors = vec!();
    if starting_state.elevator_on == 0 {
      target_floors.push(1);
    } else if starting_state.elevator_on == 3 {
      target_floors.push(2);
    } else {
      target_floors.push(starting_state.elevator_on - 1);
      target_floors.push(starting_state.elevator_on + 1);
    }
    
    // println!("Target floors: {:?}", target_floors);
    
    let permutations =
      permute_items_on_floor(
        &starting_state.floors[starting_state.elevator_on].chips,
        &starting_state.floors[starting_state.elevator_on].generators
      );

    for target_floor in target_floors {

      for data in &permutations {

        let mut new_building = starting_state.clone();
        new_building.prior_state = Some(starting_state);

        // Remove items from current floor and add to next floor
        for &(is_generator, name) in data {
          new_building.floors[starting_state.elevator_on].remove_item(name, is_generator);
          new_building.floors[target_floor].add_item(name, is_generator);
        }

        new_building.elevator_on = target_floor;
        let new_building_id = new_building.to_string();

        if new_building.is_finished() {
          // YAY!
          rvalue = vec!();
          rvalue.push(new_building);
          return rvalue;
        }

        // println!("Building {} is viable? {}", new_building_id, new_building.is_viable());

        if new_building.is_viable() && !seen_states.contains(&new_building_id) {
          seen_states.insert(new_building_id);
          rvalue.push(new_building);
        }
      }
    }
  }

  rvalue
}

fn dump_state<'a>(state: Option<&'a Building>) {
  let mut current = state;
  while current.is_some() {
    println!("{:?}", current);
    current = current.unwrap().prior_state;
  }
}

fn permute<'a>(building: Building) -> usize {
  // Possible permutations: elevator moves 1 floor with 0 to 2 items

  let mut depth = 1;
  let mut starting_states:Vec<Building> = vec!();
  starting_states.push(building);

  let mut seen_states:HashSet<String> = HashSet::new();

  loop {
    depth += 1;

    println!("There are {} starting states at depth {}", starting_states.len(), depth);
    // println!("Starting states:\n{:?}", starting_states);

    // Generate all possible states at this depth
    let mut potential_states:Vec<Building> = vec!();

    {
      potential_states = generate_potential_states(&starting_states, &mut seen_states);
      if potential_states.len() == 1 {
        //println!("Found match:\n{:?}", potential_states[0]);

        dump_state(Some(&potential_states[0]));

        return depth+1;
      }
    }

    //println!("Potential states:\n{:?}", potential_states);

    assert!(potential_states.len() != 0);
    
    println!("Got {} potential states", potential_states.len());
    // println!("{:?}", potential_states);

    starting_states = potential_states;
  }
}

fn part1(input: String) -> String  {

  let chip_types = Regex::new(r"([^-^\s]*)-compatible").unwrap();
  let generator_types = Regex::new(r"([^\s]*) generator").unwrap();

  let mut building = Building {
    floors: vec!(),
    elevator_on: 0,
    prior_state: None,
  };

  building.floors.push(Floor::new());
  building.floors.push(Floor::new());
  building.floors.push(Floor::new());
  building.floors.push(Floor::new());

  let lines:Vec<&str> = input.split("\n").collect();
  let mut floor_n = 0;

  for line in &lines {

    // Search input for high bot and output values
    for cap in chip_types.captures_iter(&line) {
      building.floors[floor_n].add_item(string_to_static_str(cap.at(1).unwrap().to_string()), false);
    }

    for cap in generator_types.captures_iter(&line) {
      building.floors[floor_n].add_item(string_to_static_str(cap.at(1).unwrap().to_string()), true);
    }

    floor_n += 1;
  }

  println!("Building:\n{:?}", building);
  permute(building).to_string()
}

fn part2 (input: String) -> String  {
  "0".to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    //input: include_str!("input").to_string(),
    input: include_str!("sample_input").to_string(),
    part1: super::Puzzle {
      run: part1,
    },
    part2: super::Puzzle {
      run: part2,
    }
  };
}

#[test]
fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "101".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "37789".to_string());
}
