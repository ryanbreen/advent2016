use crypto::digest::Digest;
use crypto::md5::Md5;

fn is_open(c: char) -> bool {
  match c {
    'b' | 'c' | 'd' | 'e' | 'f' => true,
    _ => false,
  }
}

fn find_valid_directions(start: &String, x: usize, y: usize) -> Vec<char> {

  let mut rvalue = vec!();

  let mut sh = Md5::new();

  sh.input_str(start);

  let hash = sh.result_str();

  // println!("Hash for {} is {}", start, hash);

  let mut hash_iter = hash.chars();

  let u = hash_iter.next().unwrap();
  if y > 0 && is_open(u) {
    rvalue.push('U');
  }

  let d = hash_iter.next().unwrap();
  if y < 3 && is_open(d) {
    rvalue.push('D');
  }

  let l = hash_iter.next().unwrap();
  if x > 0 && is_open(l) {
    rvalue.push('L');
  }

  let r = hash_iter.next().unwrap();
  if x < 3 && is_open(r) {
    rvalue.push('R');
  }

  rvalue
}

fn search_for_route(start: String) -> String {
  let mut available_nodes:Vec<(usize, usize, String)> = vec!();
  available_nodes.push((0, 0, start.clone()));

  while !available_nodes.is_empty() {

    let iter = available_nodes.clone().into_iter();

    available_nodes.clear();

    for node in iter {

      if node.0 == 3 && node.1 == 3 {
        return (&(node.2)[start.len()..]).to_string();
      }

      let valid_directions = find_valid_directions(&node.2, node.0, node.1);

      for dir in valid_directions {
        let mut new_str = node.2.clone();
        new_str.push(dir);
        match dir {
          'U' => available_nodes.push((node.0, node.1 - 1, new_str)),
          'D' => available_nodes.push((node.0, node.1 + 1, new_str)),
          'L' => available_nodes.push((node.0 - 1, node.1, new_str)),
          'R' => available_nodes.push((node.0 + 1, node.1, new_str)),
          _ => panic!("WTF"),
        };
      }
    }

    // println!("available_nodes: {:?}", available_nodes);
  }

  0.to_string()
}

fn part1(input: String) -> String  {
  search_for_route(input)
}

fn search_for_longest_route(start: String) -> String {
  let mut longest = 0;

  let mut available_nodes:Vec<(usize, usize, String)> = vec!();
  available_nodes.push((0, 0, start.clone()));

  while !available_nodes.is_empty() {

    let iter = available_nodes.clone().into_iter();

    available_nodes.clear();

    for node in iter {

      if node.0 == 3 && node.1 == 3 {
        let candidate = node.2.len() - start.len();
        if candidate > longest {
          longest = candidate;
        }
        continue;
      }

      let valid_directions = find_valid_directions(&node.2, node.0, node.1);

      for dir in valid_directions {
        let mut new_str = node.2.clone();
        new_str.push(dir);
        match dir {
          'U' => available_nodes.push((node.0, node.1 - 1, new_str)),
          'D' => available_nodes.push((node.0, node.1 + 1, new_str)),
          'L' => available_nodes.push((node.0 - 1, node.1, new_str)),
          'R' => available_nodes.push((node.0 + 1, node.1, new_str)),
          _ => panic!("WTF"),
        };
      }
    }

    //println!("available_nodes count {} at depth {}", available_nodes.len(), depth);
  }

  longest.to_string()
}

fn part2 (input: String) -> String  {
  search_for_longest_route(input)
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "qzthpkfp".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "RDDRLDRURD".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "448".to_string());
}
