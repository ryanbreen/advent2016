use core::str;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::time::Instant;
use pbr::ProgressBar;

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

  let mut pb = ProgressBar::new(8);
  pb.format("╢▌▌░╟");
  pb.message(&String::from_utf8(password.clone()).unwrap());

  loop {
    sh.input_str(format!("{}{}", input, idx).as_str());
    idx += 1;
    let tmp = sh.result_str();
    sh.reset();

    if tmp.starts_with(TEST) {
      let mut pos = tmp.as_bytes()[5] as usize;
      if pos < 48 || pos > 55 {
        continue;
      }

      pos -= 48;
  
      if password[pos as usize] != 'x' as u8 {
        continue;
      }

      pb.inc();

      password[pos] = tmp.as_bytes()[6];
      pb.message(&String::from_utf8(password.clone()).unwrap());

      len += 1;
      if len == 8 {
        pb.finish_print(&format!("That took {}s\n", start.elapsed().as_secs()));
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
