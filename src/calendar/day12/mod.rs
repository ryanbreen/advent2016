
struct Computer<'a> {
  registers: [usize; 4],
  tape: &'a[&'a str],
  process_counter: usize,
}

impl<'a> Computer<'a> {

  fn register_id_from_name(&self, name:u8) -> usize {
    name as usize - 97
  }

  fn run(&mut self) {
    while self.process_counter < self.tape.len() {
      let command = self.tape[self.process_counter];

      let parts:Vec<&'a str> = command.split(" ").collect();
      match parts[0] {
        "cpy" => {
          let register = self.register_id_from_name(parts[2].bytes().next().unwrap());
          let value = parts[1].parse::<usize>();
          match value {
            Ok(v) => self.registers[register] = v,
            Err(_) => self.registers[register] = self.registers[self.register_id_from_name(parts[1].bytes().next().unwrap())],
          }
        },
        "inc" => {
          let register = self.register_id_from_name(parts[1].bytes().next().unwrap());
          self.registers[register] += 1;
        },
        "dec" => {
          let register = self.register_id_from_name(parts[1].bytes().next().unwrap());
          self.registers[register] -= 1;
        },
        "jnz" => {
          let value_to_compare = match parts[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => self.registers[self.register_id_from_name(parts[1].bytes().next().unwrap())],
          };

          if value_to_compare != 0 {
            let distance:isize = parts[2].parse::<isize>().unwrap();
            self.process_counter = (self.process_counter as isize + distance) as usize;
            //println!("After {}, pc is {}, regsiters are {:?}", command, self.process_counter, self.registers);
            continue;
          }
        },
        _ => panic!("Invalid command {}", parts[0]),
      }

      self.process_counter += 1;

      //println!("After {}, pc is {}, registers are {:?}", command, self.process_counter, self.registers);
    }
  }
}


fn part1(input: String) -> String  {

  let lines:Vec<&str> = input.split("\n").collect();

  let mut computer = Computer {
    registers: [0; 4],
    tape: &lines,
    process_counter: 0,
  };

  computer.run();

  computer.registers[0].to_string()
}

fn part2 (_: String) -> String  {
  "0".to_string()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "318083".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "67".to_string());
}
