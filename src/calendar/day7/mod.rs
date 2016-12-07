use regex::Regex;

fn contains_code(input:&str) -> bool {
  let bytes = input.as_bytes();
  for i in 0..bytes.len()-3 {
    if bytes[i] == bytes[i+3] && bytes[i+1] == bytes[i+2] && bytes[i] != bytes[i+1] {
      return true;
    }
  }

  false
}

fn part1(input: String) -> String  {
  let bracket_contents = Regex::new(r"\[([^\]]*)").unwrap();

  let mut codes:Vec<&str> = input.split("\n").collect();
  codes = codes.into_iter().filter(|code| {
    for cap in bracket_contents.captures_iter(code) {
      if contains_code(cap.at(1).unwrap()) {
        return false;
      }
    }

    true
  }).filter(|code| {
    contains_code(code)
  }).collect();

  codes.len().to_string()
}

fn part2 (input: String) -> String  {
  let mut counter:u32 = 0;

  counter.to_string()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "asvcbhvg".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "odqnikqv".to_string());
}
