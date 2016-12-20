use regex::Regex;

static mut MATCH:usize = 0;

#[derive(Debug, Copy)]
struct Bot {
  id: usize,
  low: Option<usize>,
  high: Option<usize>,
  low_output_to_bot: bool,
  low_output_target: Option<usize>,
  high_output_to_bot: bool,
  high_output_target: Option<usize>,
}

impl Bot {
  fn new(idx: usize) -> Self {
    Bot {
      id: idx,
      low: None,
      high: None,
      low_output_to_bot: false,
      high_output_to_bot: false,
      low_output_target: None,
      high_output_target: None,
    }
  }

  fn add_value(&mut self, val: usize) {
    //println!("Setting {} on {:?}", val, self);
    if self.low.is_none() {
      self.low = Some(val);
    } else if self.low.is_some() && self.low.unwrap() > val {
      if self.high.is_none() {
        self.high = self.low;
      }

      self.low = Some(val);
    } else if self.high.is_some() && self.high.unwrap() < val {
      self.high = Some(val);
    } else if self.high.is_none() {
      self.high = Some(val);
    }
  }

  fn is_full(&self) -> bool {
    self.low.is_some() && self.high.is_some()
  }

  fn transmit(&mut self, bots:&mut Vec<Bot>, outputs:&mut Vec<usize>) {
    //if bots[i].low == Some(2) && bots[i].high == Some(5) {
    if self.low == Some(17) && self.high == Some(61) {
      unsafe { MATCH = self.id };
    }

    //println!("Bot {} has {} and {}, transmitting", self.id, self.low.unwrap(), self.high.unwrap());

    let low_idx = self.low_output_target.unwrap();
    let high_idx = self.high_output_target.unwrap();

    if self.low_output_to_bot {
      bots[low_idx].add_value(self.low.unwrap());
      self.low = None;
      if bots[low_idx].is_full() {
        let mut new_bot = bots[low_idx].clone();
        bots[low_idx] = new_bot;
        new_bot.transmit(bots, outputs);
      }
    } else {
      outputs[low_idx] = self.low.unwrap();
      self.low = None;
    }

    if self.high_output_to_bot {
      bots[high_idx].add_value(self.high.unwrap());
      self.high = None;
      if bots[high_idx].is_full() {
        let mut new_bot = bots[high_idx].clone();
        bots[high_idx] = new_bot;
        new_bot.transmit(bots, outputs);
      }
    } else {
      outputs[high_idx] = self.high.unwrap();
      self.high = None;
    }
  }
}

impl Clone for Bot {
  fn clone(&self) -> Bot { *self }
}

fn part1(input: String) -> String  {

  // Search input for high bot and output values
  let bot_captures = Regex::new(r"bot ([0-9]*)").unwrap();
  let mut high_bot:usize = 0;
  for cap in bot_captures.captures_iter(&input) {
    let num = cap.at(1).unwrap().parse::<usize>().unwrap() + 1;
    if num > high_bot {
      high_bot = num;
    }
  }

  let mut bots:Vec<Bot> = Vec::with_capacity(high_bot);
  for i in 0..high_bot {
    bots.push(Bot::new(i));
  }

  let output_captures = Regex::new(r"output ([0-9]*)").unwrap();
  let mut high_output:usize = 0;
  for cap in output_captures.captures_iter(&input) {
    let num = cap.at(1).unwrap().parse::<usize>().unwrap() + 1;
    if num > high_output {
      high_output = num;
    }
  }

  let mut outputs:Vec<usize> = Vec::with_capacity(high_output);
  for _ in 0..high_output {
    outputs.push(0);
  }

  let instructions:Vec<&str> = input.split("\n").collect();
  for instruction in &instructions {
    let parts:Vec<&str> = instruction.split(" ").collect();
    match parts[0] {
      "value" => {
        let idx = parts[5].parse::<usize>().unwrap();
        bots[idx].add_value(parts[1].parse::<usize>().unwrap());
        //println!("Value set: {}, bot is now {:?}", instruction, bots[idx]);
      },
      "bot" => {
        let bot_id = parts[1].parse::<usize>().unwrap();
        let low_target_idx = Some(parts[6].parse::<usize>().unwrap());
        let high_target_idx = Some(parts[11].parse::<usize>().unwrap());

        bots[bot_id].low_output_target = low_target_idx;
        match parts[5] {
          "bot" => bots[bot_id].low_output_to_bot = true,
          "output" => bots[bot_id].low_output_to_bot = false,
          _ => println!("Invalid instruction {}", instruction)
        };

        bots[bot_id].high_output_target = high_target_idx;
        match parts[10] {
          "bot" => bots[bot_id].high_output_to_bot = true,
          "output" => bots[bot_id].high_output_to_bot = false,
          _ => println!("Invalid instruction {}", instruction)
        };

        //println!("Linkage set: {}, bot is now {:?}", instruction, bots[bot_id]);
      }
      _ => println!("Invalid instruction {}", instruction),
    };
  }

  for i in 0..bots.len() {
    if bots[i].is_full() {
      let mut new_bot = bots[i].clone();
      bots[i] = new_bot;
      new_bot.transmit(&mut bots, &mut outputs);
      break;
    }
  }

  //println!("{:?}", bots);
  println!("{:?}", outputs);

  unsafe { MATCH.to_string() }
}

fn part2 (input: String) -> String  {
    // Search input for high bot and output values
  let bot_captures = Regex::new(r"bot ([0-9]*)").unwrap();
  let mut high_bot:usize = 0;
  for cap in bot_captures.captures_iter(&input) {
    let num = cap.at(1).unwrap().parse::<usize>().unwrap() + 1;
    if num > high_bot {
      high_bot = num;
    }
  }

  let mut bots:Vec<Bot> = Vec::with_capacity(high_bot);
  for i in 0..high_bot {
    bots.push(Bot::new(i));
  }

  let output_captures = Regex::new(r"output ([0-9]*)").unwrap();
  let mut high_output:usize = 0;
  for cap in output_captures.captures_iter(&input) {
    let num = cap.at(1).unwrap().parse::<usize>().unwrap() + 1;
    if num > high_output {
      high_output = num;
    }
  }

  let mut outputs:Vec<usize> = Vec::with_capacity(high_output);
  for _ in 0..high_output {
    outputs.push(0);
  }

  let instructions:Vec<&str> = input.split("\n").collect();
  for instruction in &instructions {
    let parts:Vec<&str> = instruction.split(" ").collect();
    match parts[0] {
      "value" => {
        let idx = parts[5].parse::<usize>().unwrap();
        bots[idx].add_value(parts[1].parse::<usize>().unwrap());
        //println!("Value set: {}, bot is now {:?}", instruction, bots[idx]);
      },
      "bot" => {
        let bot_id = parts[1].parse::<usize>().unwrap();
        let low_target_idx = Some(parts[6].parse::<usize>().unwrap());
        let high_target_idx = Some(parts[11].parse::<usize>().unwrap());

        bots[bot_id].low_output_target = low_target_idx;
        match parts[5] {
          "bot" => bots[bot_id].low_output_to_bot = true,
          "output" => bots[bot_id].low_output_to_bot = false,
          _ => println!("Invalid instruction {}", instruction)
        };

        bots[bot_id].high_output_target = high_target_idx;
        match parts[10] {
          "bot" => bots[bot_id].high_output_to_bot = true,
          "output" => bots[bot_id].high_output_to_bot = false,
          _ => println!("Invalid instruction {}", instruction)
        };

        //println!("Linkage set: {}, bot is now {:?}", instruction, bots[bot_id]);
      }
      _ => println!("Invalid instruction {}", instruction),
    };
  }

  for i in 0..bots.len() {
    if bots[i].is_full() {
      let mut new_bot = bots[i].clone();
      bots[i] = new_bot;
      new_bot.transmit(&mut bots, &mut outputs);
      break;
    }
  }

  //println!("{:?}", bots);
  println!("{:?}", outputs);

  (outputs[0] * outputs[1] * outputs[2]).to_string()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "101".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "37789".to_string());
}
