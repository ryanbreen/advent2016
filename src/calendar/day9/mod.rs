
fn process_parts(input: String) -> String {
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

  new.iter().cloned().collect::<String>()
}

fn part1(input: String) -> String  {
  process_parts(input).len().to_string()
}

/**
 * Return the size of a subset of the final string.  If any data regions are contained in this
 * subset, we recursively traverse them.
 */
fn size_of_substring(input: String) -> usize {
  let original:Vec<u8> = input.bytes().collect();

  let mut my_size = 0;

  let mut i = 0;

  while i < original.len() {
    match original[i] {
      40 /* ( */ => {
        i += 1;
        let mut marker:String = String::new();
        while original[i] != 41 {
          marker.push(original[i] as char);
          i += 1;
        }

        i += 1; /* eat the final paren of this marker */

        let marker_parts = marker.split("x").collect::<Vec<&str>>();
        let data_block_len = marker_parts[0].parse::<usize>().unwrap();
        let repeats = marker_parts[1].parse::<usize>().unwrap();

        let marker_range:&str = &input[i .. i+data_block_len];
        let marker_size = size_of_substring(marker_range.to_string());

        my_size += marker_size * repeats;

        i += data_block_len;
      },
      _ => {
        my_size += 1;
        i += 1;
      }
    };
  }

  //println!("Size of {} is {}", input, my_size);

  my_size
}

fn part2 (input: String) -> String  {
  size_of_substring(input).to_string()
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
  assert_eq!((day.part2.run)(day.input.to_string()), "11052855125".to_string());
}
