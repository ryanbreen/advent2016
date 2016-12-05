use std::cmp::Ordering;

struct Room<'a> {
  code: &'a str,
  sector_id: u32,
  checksum: &'a str
}

impl<'a> Room<'a> {
  fn is_real(&self) -> bool {
    let mut alphabet:[Option<CharCount>; 26] = [None; 26];

    for c in self.code.chars() {
      if c == '-' {
        continue;
      }

      let idx = (c as usize) - 97;

      match alphabet[idx] {
        None => {
          // We haven't seen this character yet
          alphabet[idx] = Some(CharCount {
            c: c,
            count: 1
          });
        },
        Some(mut cc) => {
          cc.count = cc.count + 1;
          alphabet[idx] = Some(cc);
        }
      }
    }

    // Grab only the letters we've seen.
    let mut seen_letters:Vec<CharCount> = alphabet.iter()
      .filter(|l| !l.is_none() )
      .map(|l| l.unwrap() )
      .collect();

    // Sort by count and then by alpha in case of a tie.
    seen_letters.sort_by(|a, b| {
      let mut cmp = b.count.cmp(&a.count);
      if cmp == Ordering::Equal {
        cmp = a.c.cmp(&b.c);
      }
      cmp
    });

    let _ = seen_letters.split_off(5);
    let test_checksum = seen_letters.into_iter().map(|s| s.c).collect::<String>();;

    test_checksum == self.checksum
  }
}

#[derive(Debug, Copy, Clone)]
struct CharCount {
  c: char,
  count: u8
}

fn part1 (input: String) -> String  {

  let mut counter:u32 = 0;

  let parts:Vec<&str> = input.split("\n").collect();
  for line in &parts {

    // sector id: &line[line.len()-10..line.len()-7]
    // checksum: &line[line.len()-6..line.len()-1]
    let checksum = &line[line.len()-6..line.len()-1];
    let code = &line[0..line.len()-11];
    let sector_id = line[line.len()-10..line.len()-7].parse::<u32>().unwrap();
    let room = Room {
      checksum: checksum,
      sector_id: sector_id,
      code: code
    };

    if room.is_real() {
      counter += room.sector_id;
    }
  }

  return counter.to_string();
}

fn part2 (input: String) -> String  {

  let parts:Vec<&str> = input.split("\n").collect();
  for line in &parts {

    // sector id: &line[line.len()-10..line.len()-7]
    // checksum: &line[line.len()-6..line.len()-1]
    let checksum = &line[line.len()-6..line.len()-1];
    let code = &line[0..line.len()-11];
    let sector_id = line[line.len()-10..line.len()-7].parse::<u32>().unwrap();
    let room = Room {
      checksum: checksum,
      sector_id: sector_id,
      code: code
    };

    if room.is_real() {
      // translate code
      let mut translated = String::new();
      for c in room.code.chars() {
        match c {
          '-' => translated.push(' '),
          _ => {
            let mut idx:u32 = (c as u32) - 97;
            idx += room.sector_id as u32;
            let new_char:char = ((idx%26) as u8 + 97) as char;

            translated.push(new_char);
          }
        };
      }

      if translated == "northpole object storage" {
        return room.sector_id.to_string();
      }
    }
  }

  return "FAILED".to_string();
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
