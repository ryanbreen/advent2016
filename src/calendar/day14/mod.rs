use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::HashMap;

fn generate_keys(input: String, stretch_keys: bool) -> String {

  let mut sh = Md5::new();

  let mut potential_keys:HashMap<char, Vec<usize>> = HashMap::new();
  let mut keys:Vec<usize> = vec!();

  let mut run_handler = move |&idx, &c, size| -> usize {
    
    if size == 3 {
      if !potential_keys.contains_key(&c) {
        potential_keys.insert(c, vec!());
      }
      potential_keys.get_mut(&c).unwrap().push(idx);
    }
    if size == 5 {
      if potential_keys.contains_key(&c) {
        let candidates = potential_keys.get(&c).unwrap();
        for candidate in candidates {
          let distance = idx - candidate;
          if distance < 1000 {
            //println!("Candidate {} (char {}) ({} ago) at idx {}", candidate, &c, distance, idx);
            keys.push(idx);
            if keys.len() == 64 {
              return *candidate;
            }
          }
        }
      }
      potential_keys.remove(&c);
    }

    0
  };

  let mut i = 0;
  loop {
    sh.input_str(format!("{}{}", input, i).as_str());
    let mut result_str = sh.result_str();

    if stretch_keys {
      for _ in 0..2016 {
        sh.reset();
        sh.input_str(&result_str);
        result_str = sh.result_str();
      }
    }

    sh.reset();

    // This is gross, but I couldn't figure out how to get regex working.
    let mut last_c = 'z';
    let mut last_last_c = 'z';
    let mut last_last_last_c = 'z';
    let mut last_last_last_last_c = 'z';

    let mut match_three = false;
    let mut match_five = false;

    let mut match_char = 'z';

    for c in result_str.chars() {
      if !match_three && c == last_c && c == last_last_c {
        match_char = c;
        match_three = true;
      }

      if c == last_c && last_c == last_last_c && last_last_c == last_last_last_c && last_last_last_c == last_last_last_last_c {
        //println!("Found five char run {} at {}", c, i);
        match_five = true;
      }

      last_last_last_last_c = last_last_last_c;
      last_last_last_c = last_last_c;
      last_last_c = last_c;
      last_c = c;
    }

    let i_con = i.clone();

    if match_five {
      let rvalue = run_handler(&i_con, &match_char, 5);
      if rvalue != 0 {
        return rvalue.to_string();  
      }
      //println!("Found three char run {} at {}", match_char, i_con);
      run_handler(&i_con, &match_char, 3);
    } else if match_three {
      //println!("Found three char run {} at {}", match_char, i_con);
      run_handler(&i_con, &match_char, 3);
    }

    i += 1;
  }
}

fn part1(input: String) -> String  {
  generate_keys(input, false)
}

fn part2 (input: String) -> String  {
  generate_keys(input, true)
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "jlmsuwbz".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "35186".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "9227737".to_string());
}
