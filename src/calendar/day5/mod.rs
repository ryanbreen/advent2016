use crypto::digest::Digest;
use crypto::md5::Md5;
use std::time::Instant;

fn part1(input: String) -> String  {
  let mut idx:usize = 0;
  let mut password:String = String::new();

  let mut sh = Md5::new();

  let start = Instant::now();
  loop {
    sh.input_str(format!("{}{}", input, idx).as_str());
    if sh.result_str().starts_with("00000") {
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
  "FAILED".to_string()
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
  assert_eq!((day.part2.run)(day.input.to_string()), "482".to_string());
}
