use std::collections::BTreeSet;
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

struct Building {
  floors: Vec<Floor>,
  elevator_on: usize,
  state: usize,
  prior_state: usize
}

impl Clone for Building {
  fn clone(&self) -> Building {
    let mut new_building = Building {
      floors: vec!(),
      elevator_on: self.elevator_on,
      state: self.state,
      prior_state: self.prior_state,
    };

    for floor in &self.floors {
      new_building.floors.push(floor.clone());
    }

    new_building
  }
}

impl Building {
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

  fn uuid(&self) -> String {
    format!("{:?}{:?}{:?}{:?}{}", self.floors[0], self.floors[1], self.floors[2], self.floors[3], self.elevator_on)
  }
}

impl fmt::Debug for Building {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in (0..self.floors.len()).rev() {
      let _ = write!(f, "{} {:?}\n", i, self.floors[i]);
    }

    write!(f, "E is on {}, prior_state is {}", self.elevator_on, self.prior_state)
  }
}

struct Floor {
  generators: BTreeSet<&'static str>,
  chips: BTreeSet<&'static str>,
}

impl Clone for Floor {
  fn clone(&self) -> Floor {
    let mut new_floor = Floor {
      generators: BTreeSet::new(),
      chips: BTreeSet::new(),
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
      generators: BTreeSet::new(),
      chips: BTreeSet::new(),
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

fn permute_items_on_floor(chip_map: &BTreeSet<&'static str>, generator_map: &BTreeSet<&'static str>) -> Vec<Vec<(bool, &'static str)>> {

  let generators:Vec<&'static str> = generator_map.clone().into_iter().collect();
  let chips:Vec<&'static str> = chip_map.clone().into_iter().collect();

  // println!("G: {:?}, C: {:?}", generators, chips);

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

  rvalue
}

fn enumerate_potential_states_from_depth(depth: u8,
                                         starting_states: Vec<Building>,
                                         mut seen_states: &mut BTreeSet<String>,
                                         mut state_detail: &mut Vec<String>) -> u8 {

  let mut potential_states = vec!();

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
    
    let permutations =
      permute_items_on_floor(
        &starting_state.floors[starting_state.elevator_on].chips,
        &starting_state.floors[starting_state.elevator_on].generators
      );

    for target_floor in target_floors {

      for data in &permutations {

        let mut new_building = starting_state.clone();

        // Remove items from current floor and add to next floor
        for &(is_generator, name) in data {
          new_building.floors[starting_state.elevator_on].remove_item(name, is_generator);
          new_building.floors[target_floor].add_item(name, is_generator);
        }

        new_building.elevator_on = target_floor;
        let new_building_id = new_building.uuid();

        if new_building.is_finished() {
          // YAY!
          new_building.prior_state = starting_state.state;
          new_building.state = state_detail.len();
          state_detail.push(format!("{:?}", new_building));
          dump_state(new_building.state, state_detail);
          return depth;
        }

        // println!("Building {} is viable? {}", new_building_id, new_building.is_viable());

        if new_building.is_viable() && !seen_states.contains(&new_building_id) {
          new_building.prior_state = starting_state.state;
          new_building.state = state_detail.len();
          // println!("Building id {} from {}", new_building.state, new_building.prior_state);
          seen_states.insert(new_building_id);
          state_detail.push(format!("{:?}", new_building));
          potential_states.push(new_building);
        }
      }
    }
  }

  println!("Generated {} potential_states at depth {}", potential_states.len(), depth);

  return enumerate_potential_states_from_depth(depth + 1, potential_states, &mut seen_states, &mut state_detail);
}

fn dump_state(state_id: usize, state_detail: &Vec<String>) {

  println!("***id {}***\n{}\n", state_id, state_detail[state_id]);

  if state_id == 0 {
    return;
  }

  let re = Regex::new(r"prior_state is ([0-9]*)").unwrap();
  let cap = re.captures(&state_detail[state_id]).unwrap();

  dump_state(cap.at(1).unwrap().parse::<usize>().unwrap(), state_detail);
}

fn permute(building: Building) -> u8 {
  // Possible permutations: elevator moves 1 floor with 0 to 2 items

  let depth:u8 = 1;

  let starting_states:Vec<Building> = vec!(building.clone());
  let mut seen_states:BTreeSet<String> = BTreeSet::new();
  let mut state_detail:Vec<String> = vec!(format!("{:?}", building));

  return enumerate_potential_states_from_depth(depth, starting_states, &mut seen_states, &mut state_detail);
}

fn part1(input: String) -> String  {

  let chip_types = Regex::new(r"([^-^\s]*)-compatible").unwrap();
  let generator_types = Regex::new(r"([^\s]*) generator").unwrap();

  let mut building = Building {
    floors: vec!(),
    elevator_on: 0,
    state: 0,
    prior_state: 0,
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

  permute(building).to_string()
}

fn part2 (_: String) -> String  {
  let input = include_str!("input2").to_string();

  let chip_types = Regex::new(r"([^-^\s]*)-compatible").unwrap();
  let generator_types = Regex::new(r"([^\s]*) generator").unwrap();

  let mut building = Building {
    floors: vec!(),
    elevator_on: 0,
    state: 0,
    prior_state: 0,
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

  permute(building).to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
    //input: include_str!("sample_input").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "37".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "67".to_string());
}
