#[derive(Debug, Copy, Clone)]
struct CharCount {
  c: char,
  count: u8
}

fn part1(input: String) -> String  {
  let mut parts:Vec<u8> = vec!(0,0,0,0,0,0,0,0);
  for i in 0..8 {
    let mut alphabet:Vec<CharCount> = vec![CharCount { c: '-', count: 0 }; 26];
    let lines:Vec<&str> = input.split("\n").collect();
    for line in &lines {
      let c = line.as_bytes()[i];
      let idx = (c as usize) - 97;
      alphabet[idx].c = c as char;
      alphabet[idx].count += 1;
    }

    // Sort by count and then by alpha in case of a tie.
    alphabet.sort_by(|a, b| b.count.cmp(&a.count));
    parts[i] = alphabet[0].c as u8;
  }

  String::from_utf8(parts).unwrap()
}

fn part2 (input: String) -> String  {
  let mut parts:Vec<u8> = vec!(0,0,0,0,0,0,0,0);
  for i in 0..8 {
    let mut alphabet:Vec<CharCount> = vec![CharCount { c: '-', count: 0 }; 26];
    let lines:Vec<&str> = input.split("\n").collect();
    for line in &lines {
      let c = line.as_bytes()[i];
      let idx = (c as usize) - 97;
      alphabet[idx].c = c as char;
      alphabet[idx].count += 1;
    }

    // Sort by count and then by alpha in case of a tie.
    alphabet = alphabet.iter()
      .filter(|cc| cc.c != '-')
      .map(|cc| *cc)
      .collect();
    alphabet.sort_by(|a, b| a.count.cmp(&b.count));
    parts[i] = alphabet[0].c as u8;
  }

  String::from_utf8(parts).unwrap()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "361724".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "482".to_string());
}
