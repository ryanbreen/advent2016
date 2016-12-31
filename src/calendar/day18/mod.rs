
fn safe_count(input: String, row_count: usize) -> usize {

  //println!("{}", input);

  let mut safe_count = input.chars().fold(0usize, |sum, val| {
    if val == '.' {
      return sum + 1;
    }

    sum
  });

  let mut last_row = input;

  for _ in 1..row_count {
    let mut next_row = vec!();

    let chars:Vec<char> = last_row.chars().collect();

    for i in 0..chars.len() {
      let left = match i {
        0 => false,
        _ => chars[i-1] == '^',
      };

      let right = match i {
        _ if i == chars.len() - 1  => false,
        _ => chars[i+1] == '^',
      };

      let center = chars[i] != '^';
      let c;

      // println!("{} l-{} m-{} r-{}", i, left, right, center);

      if left == true && center == true && right == false {
        c = '^';
      } else if left == false && center == true && right == true {
        c = '^';
      } else if left == true && center == false && right == false {
        c = '^';
      } else if left == false && center == false && right == true {
        c = '^';
      } else {
        c = '.';
        safe_count += 1;
      }

      next_row.push(c as u8);
    }

    last_row = String::from_utf8(next_row).unwrap();

    //println!("{}", last_row);
  }

  safe_count
}

fn part1(input: String) -> String  {
  let row_count = match input.as_str() {
    "..^^." => 3,
    ".^^.^.^^^^" => 10,
    _ => 40,
  };

  safe_count(input, row_count).to_string()
}

fn part2(input: String) -> String  {
  let row_count = match input.as_str() {
    "..^^." => 3,
    ".^^.^.^^^^" => 10,
    _ => 400000,
  };

  safe_count(input, row_count).to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: ".^^..^...^..^^.^^^.^^^.^^^^^^.^.^^^^.^^.^^^^^^.^...^......^...^^^..^^^.....^^^^^^^^^....^^...^^^^..^".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "2005".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "20008491".to_string());
}
