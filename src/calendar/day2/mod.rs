
fn part1 (input: String) -> String  {

  let mut keypad:[[u8; 3]; 3] = [[0; 3]; 3];
  keypad[2] = [1, 2, 3];
  keypad[1] = [4, 5, 6];
  keypad[0] = [7, 8, 9];

  let mut answer:String = String::new();
  let mut location:(usize, usize) = (1, 1);

  let parts:Vec<&str> = input.split("\n").collect();
  for line in &parts {

    for c in line.chars() {

      match c {
        'U' => {
          if location.1 < 2 {
            location.1 += 1;
          }
        },
        'D' =>  {
          if location.1 > 0 {
            location.1 -= 1;
          }
        },
        'R' => {
          if location.0 < 2 {
            location.0 += 1;
          }
        },
        'L' =>  {
          if location.0 > 0 {
            location.0 -= 1;
          }
        },
        _ => panic!("Invalid input")
      };
    }

    answer += &keypad[location.1][location.0].to_string();
  }

  return answer;
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
    //input: include_str!("sample_input").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "99332".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1783".to_string());
}
