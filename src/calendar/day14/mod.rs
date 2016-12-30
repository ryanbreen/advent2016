use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::HashMap;

fn part1(input: String) -> String  {

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
            keys.push(idx);
            if keys.len() == 64 {
              return idx;
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

    // This is gross, but I couldn't figure out how to get regex working.
    let mut last_c = 'z';
    let mut last_last_c = 'z';
    let mut last_last_last_c = 'z';
    let mut last_last_last_last_c = 'z';

    let mut matches:Vec<(char, usize)> = vec!();

    for c in sh.result_str().chars() {
      if c == last_c && c == last_last_c {
        matches.push((c, 3));
      }

      if c == last_c && last_c == last_last_c && last_last_c == last_last_last_c && last_last_last_c == last_last_last_last_c {
        matches.push((c, 5));
      }

      last_last_last_last_c = last_last_last_c;
      last_last_last_c = last_last_c;
      last_last_c = last_c;
      last_c = c;
    }

    let i_con = i.clone();

    for idx in 0..matches.len() {
      if matches[idx].1 == 5 {
        let rvalue = run_handler(&i_con, &matches[idx].0, 5);
        if rvalue != 0 {
          return rvalue.to_string();
        }  
      }
    }

    for idx in 0..matches.len() {
      if matches[idx].1 == 3 {
        run_handler(&i_con, &matches[idx].0, 3);
      }
    }

    i += 1;

    sh.reset();
  }
}

fn part2 (_: String) -> String  {
  "0".to_string()
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
