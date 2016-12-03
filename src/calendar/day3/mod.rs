
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

  let mut original_parts:Vec<&str> = input.split("\n").collect();

  let mut parts:Vec<Vec<&str>> = Vec::new();

  for i in 0..(original_parts.len() / 3) {
    let one:Vec<&str> = original_parts.pop().unwrap().split_whitespace().collect();
    let two:Vec<&str> = original_parts.pop().unwrap().split_whitespace().collect();
    let three:Vec<&str> = original_parts.pop().unwrap().split_whitespace().collect();

    let mut new_one:Vec<&str> = Vec::new();
    new_one.push(one[0]);
    new_one.push(two[0]);
    new_one.push(three[0]);
    let mut new_two:Vec<&str> = Vec::new();
    new_two.push(one[1]);
    new_two.push(two[1]);
    new_two.push(three[1]);
    let mut new_three:Vec<&str> = Vec::new();
    new_three.push(one[2]);
    new_three.push(two[2]);
    new_three.push(three[2]);

    new_one.sort_by(|a, b| a.parse::<u16>().unwrap().cmp(&b.parse::<u16>().unwrap()));
    new_two.sort_by(|a, b| a.parse::<u16>().unwrap().cmp(&b.parse::<u16>().unwrap()));
    new_three.sort_by(|a, b| a.parse::<u16>().unwrap().cmp(&b.parse::<u16>().unwrap()));

    parts.push(new_one);
    parts.push(new_two);
    parts.push(new_three);
  }

  let mut counter:u16 = 0;

  for number_strings in &parts {

    let mut numbers:[u16;3] = [0; 3];

    for i in 0..3 {
      numbers[i] = number_strings[i].parse::<u16>().unwrap();
    }

    if (numbers[0] + numbers[1]) > numbers[2] {
      counter += 1;
    }
  }

  return counter.to_string();
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
  assert_eq!((day.part1.run)(day.input.to_string()), "993".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "DD483".to_string());
}
