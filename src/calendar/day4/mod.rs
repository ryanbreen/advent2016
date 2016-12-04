use std::cmp::Ordering;

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

    let mut alphabet:[Option<CharCount>; 26] = [None; 26];

    for c in code.chars() {
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

    // Grab only the letters we've seen.  I'm sure there's a cleaner way to do this.
    let mut seen_letters:Vec<CharCount> = vec!();
    for cco in &alphabet {
      match *cco {
        None => {},
        Some(cc) => {
          seen_letters.push(cc);
        }
      }
    }

    // Sort by count and then alpha
    seen_letters.sort_by(|a, b| {
      let mut cmp = b.count.cmp(&a.count);
      if cmp == Ordering::Equal {
        cmp = a.c.cmp(&b.c);
      }
      cmp
    });

    let _ = seen_letters.split_off(5);
    let mut test_checksum = String::new();

    for cc in seen_letters {
      test_checksum.push(cc.c);
    }

    if test_checksum == checksum {
      counter += line[line.len()-10..line.len()-7].parse::<u32>().unwrap();
    }
  }

  return counter.to_string();
}

fn part2 (input: String) -> String  {
  return input.to_string();
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
  assert_eq!((day.part2.run)(day.input.to_string()), "1849".to_string());
}
