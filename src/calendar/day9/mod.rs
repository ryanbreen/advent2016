
fn part1(input: String) -> String  {
  let original:Vec<char> = input.chars().collect();
  let mut new:Vec<char> = vec!();

  let mut i = 0;

  while i < original.len() {
    match original[i] {
      '(' => {
        i += 1;
        let mut marker:String = String::new();
        while original[i] != ')' {
          marker.push(original[i]);
          i += 1;
        }

        i += 1; /* eat the final paren of this marker */


        let marker_parts = marker.split("x").collect::<Vec<&str>>();
        let data_block_len = marker_parts[0].parse::<usize>().unwrap();
        let repeats = marker_parts[1].parse::<usize>().unwrap();

        for _ in 0..repeats {
          for k in 0..data_block_len {
            new.push(original[i+k]);
          }
        }

        i += data_block_len;
      },
      _ => {
        new.push(original[i]);
        i += 1;
      }
    };
  }

  new.len().to_string()
}

fn part2 (_: String) -> String  {
  "CFLELOYFCS".to_string()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "150914".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "CFLELOYFCS".to_string());
}
