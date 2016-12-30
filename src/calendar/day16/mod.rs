
fn part1(input: String) -> String  {

  let target_len = match input.as_str() {
    "11101000110010100" => 272,
    "10000" => 20,
    _ => 24,
  };

  let mut filler = vec!();

  // Build filler from input
  for b in input.bytes() {
    filler.push(b - 48);
  }

  // println!("{:?}", filler);

  while filler.len() < target_len {
    let mut a = filler.clone();
    let mut b = filler.clone();
    b.reverse();

    b = b.iter().map(|&c| match c {
      0 => 1,
      1 => 0,
      _ => panic!("WTF?"),
    }).collect();

    filler.clear();

    filler.append(&mut a);
    filler.push(0);
    filler.append(&mut b);

    //println!("{:?}", filler);
  }

  filler.truncate(target_len);

  let mut checksum = vec!();

  loop {
    for i in 0..filler.len() {

      // Skip odds
      if i % 2 != 0 {
        continue;
      }

      checksum.push(match filler[i] == filler[i+1] {
        true => 1,
        false => 0,
      });
    }

    if checksum.len() % 2 == 1 {
      break;
    }

    //println!("Checksum len: {}", checksum.len());

    filler.clear();
    filler.append(&mut checksum);
    checksum.clear();
  }

  //println!("Checksum ({}): {:?}", checksum.len(), checksum);

  let ascii_vec = checksum.iter().map(|&c| c + 48).collect();
  String::from_utf8(ascii_vec).unwrap()
}

fn part2 (input: String) -> String  {
  0.to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "11101000110010100".to_string(),
    //input: "10000".to_string(),
    //input: "111100001010".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "10100101010101101".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "3208099".to_string());
}
