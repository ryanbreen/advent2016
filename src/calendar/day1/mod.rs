
fn part1 (input: String) -> String  {

  // 0 is north, 1 is east, 2 is south, 3 is west
  let mut dir:i8 = 0;

  let mut location:(i16, i16) = (0, 0);

  let parts:Vec<&str> = input.split(", ").collect();
  for instruction in &parts {

    match instruction.chars().next() {
      Some('L') => dir -= 1,
      Some('R') => dir += 1,
      _ => panic!("Invalid input")
    };

    if dir < 0 {
      dir = 3;
    }

    if dir > 3 {
      dir = 0;
    }

    let distance_str:String = String::from(*instruction).chars().skip(1).collect();
    let distance:i16 = distance_str.parse::<i16>().unwrap();

    match dir {
      0 => { location.1 += distance },
      1 => { location.0 += distance },
      2 => { location.1 -= distance },
      3 => { location.0 -= distance },
      _ => { println!("Invalid direction") }
    }

    println!("{}, heading {}", instruction, dir);
  }

  return (location.0.abs() + location.1.abs()).to_string();
}

fn part2 (input: String) -> String  {
  return input;
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
  assert_eq!((day.part1.run)(day.input.to_string()), "232".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1783".to_string());
}
