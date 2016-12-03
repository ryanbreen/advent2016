
fn part1 (input: String) -> String  {

  let mut counter:u16 = 0;

  let parts:Vec<&str> = input.split("\n").collect();
  for line in &parts {

    let mut number_strings:Vec<&str> = line.split_whitespace().collect();
    let mut numbers:[u16;3] = [0; 3];

    number_strings.sort_by(|a, b| a.parse::<u16>().unwrap().cmp(&b.parse::<u16>().unwrap()));

    for i in 0..3 {
      numbers[i] = number_strings[i].parse::<u16>().unwrap();
    }

    if (numbers[0] + numbers[1]) > numbers[2] {
      counter += 1;
    }
  }

  return counter.to_string();
}

fn part2 (input: String) -> String  {

  let mut keypad:[[char; 5]; 5] = [['-'; 5]; 5];
  keypad[4] = ['-', '-', '1', '-', '-'];
  keypad[3] = ['-', '2', '3', '4', '-'];
  keypad[2] = ['5', '6', '7', '8', '9'];
  keypad[1] = ['-', 'A', 'B', 'C', '-'];
  keypad[0] = ['-', '-', 'D', '-', '-'];

  let mut answer:String = String::new();
  let mut location:(isize, isize) = (1, 1);

  let parts:Vec<&str> = input.split("\n").collect();
  for line in &parts {

    for c in line.chars() {

      let mut proposed_location = location;

      match c {
        'U' => proposed_location.1 += 1,
        'D' => proposed_location.1 -= 1,
        'R' => proposed_location.0 += 1,
        'L' => proposed_location.0 -= 1,
        _ => panic!("Invalid input")
      };

      if proposed_location.0 <= 4 && proposed_location.0 >= 0 &&
         proposed_location.1 <= 4 && proposed_location.1 >= 0 &&
         keypad[proposed_location.1 as usize][proposed_location.0 as usize] != '-' {

        location = proposed_location;
      }
    }

    answer += &keypad[location.1 as usize][location.0 as usize].to_string();
  }

  return answer;

}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "993".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "DD483".to_string());
}
