
fn highest_set_bit(n: usize) -> usize {
  let mut highest_set_bit = 0;

  let binary = format!("{:b}", n);
  let bits:Vec<char> = binary.chars().collect();
  let reversed_bits:Vec<char> = bits.into_iter().rev().collect();

  for i in 0..reversed_bits.len() {
    if reversed_bits[i] == '1' {
      highest_set_bit = i;
    }
  }

  1 << highest_set_bit
}

fn part1(input: String) -> String  {
  let n:usize = input.parse::<usize>().unwrap();
  let l = n - highest_set_bit(n);
  (2 * l + 1).to_string()
}

fn part2(input: String) -> String  {
  0.to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "3018458".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "2005".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "20008491".to_string());
}
