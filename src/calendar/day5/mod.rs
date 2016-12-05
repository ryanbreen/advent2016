use crypto::digest::Digest;
use crypto::md5::Md5;
use std::time::Instant;

const TEST:&'static str = "00000";

fn part1(input: String) -> String  {
  let mut idx:usize = 0;
  let mut password:String = String::new();

  let mut sh = Md5::new();

  let start = Instant::now();

  loop {
    sh.input_str(format!("{}{}", input, idx).as_str());
    if sh.result_str().starts_with(TEST) {
      password.push((sh.result_str().as_bytes()[5]) as char);
      if password.len() == 8 {
        println!("That took {}s", start.elapsed().as_secs());
        return password;
      }
    }

    sh.reset();
    idx += 1;
  };
}

fn part2 (input: String) -> String  {
  let mut idx:usize = 0;
  let mut len = 0;

  let mut password:Vec<u8> = vec!(120, 120, 120, 120, 120, 120, 120, 120);
  let mut sh = Md5::new();

  let start = Instant::now();

  let mut current_placeholder = 0;
  let placeholders:[u8;4] = ['-' as u8, '\\' as u8, '|' as u8, '/' as u8];

  loop {
    sh.input_str(format!("{}{}", input, idx).as_str());
    idx += 1;
    let tmp = sh.result_str();
    sh.reset();

    // Only run this on a fraction of indices for speed.
    if idx % 1000 == 0 {
      print!("\r {}", String::from_utf8(
        (password.iter().map(|c| {
          match c {
            &120 => placeholders[current_placeholder],
            _ => *c
          }
        }).collect::<Vec<u8>>())).unwrap()
      );

      current_placeholder += 1;
      if current_placeholder > 3 {
        current_placeholder = 0;
      }
    }

    if tmp.starts_with(TEST) {
      let mut pos = tmp.as_bytes()[5] as usize;
      if pos < 48 || pos > 55 {
        continue;
      }

      pos -= 48;
  
      if password[pos as usize] != 'x' as u8 {
        continue;
      }

      password[pos] = tmp.as_bytes()[6];

      len += 1;
      if len == 8 {
        println!("That took {}s", start.elapsed().as_secs());
        return String::from_utf8(password).unwrap();
      }
    }
  };
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "reyedfim".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "f97c354d".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "863dde27".to_string());
}
