use regex::Regex;

fn contains_abba_code(input:&str) -> bool {
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
      if contains_abba_code(cap.at(1).unwrap()) {
        return false;
      }
    }

    true
  }).filter(|code| {
    contains_abba_code(code)
  }).collect();

  codes.len().to_string()
}

fn part2 (input: String) -> String  {
  let bracket_contents = Regex::new(r"\[([^]]+)\]").unwrap();
  let candidate_contents = Regex::new(r"([^[\]]+)(?:$|\[)").unwrap();

  // Look for the aba
  let contains_aba = |bab: &[u8], candidate: &[u8]| {
    println!("Checking for {:?} in {:?}", bab, candidate);
    for i in 0..candidate.len()-2 {
      if candidate[i] == bab[1] && candidate[i+1] == bab[0] && candidate[i+2] == candidate[i] {
        return true;
      }
    }

    false
  };

  let mut codes:Vec<&str> = input.split("\n").collect();
  codes = codes.into_iter().filter(|code| {

    for cap in bracket_contents.captures_iter(code) {

      // Grab the bab(s) from this cap
      let mut babs:Vec<&[u8]> = vec!();
      let bab_candidates = cap.at(1).unwrap().as_bytes();
      for i in 0..bab_candidates.len()-2 {
        if bab_candidates[i] == bab_candidates[i+2] && bab_candidates[i] != bab_candidates[i+1] {
          babs.push(&bab_candidates[i .. i+3]);
        }
      }

      if babs.len() == 0 {
        continue;
      }

      for candidate_cap in candidate_contents.captures_iter(code) {
        for bab in &babs {
          if contains_aba(bab, candidate_cap.at(1).unwrap().as_bytes()) {
            return true;
          }
        }
      }
    }

    return false;
  }).collect();

  codes.len().to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
    //input: include_str!("sample_input").to_string(),
    //input: include_str!("sample_input2").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "115".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "231".to_string());
}
