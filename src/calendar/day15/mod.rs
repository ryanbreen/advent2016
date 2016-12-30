
struct Disc {
  positions: usize,
  start_pos: usize,
}

impl Disc {
  fn position_at_t(&mut self, t: usize) -> usize {
    (self.start_pos + t) % self.positions
  }
}

struct Sculpture {
  discs: Vec<Box<Disc>>,
  capsule_pos: usize,
  t: usize,
}

impl Sculpture {

  fn tick(&mut self) -> bool {
    self.t += 1;
    self.capsule_pos += 1;

    self.discs[self.capsule_pos].position_at_t(self.t) == 0
  }

  fn is_capsule_through(&self) -> bool {
    self.capsule_pos >= self.discs.len() - 1
  }

  fn start_at(&mut self, t: usize) -> bool {
    self.t = t;

    // println!("t == {}, disc count: {}", self.t, self.discs.len());

    while self.tick() {

      if self.is_capsule_through() {
        return true;
      }

      // println!("At time {}, disc at {} is showing {}", self.t, self.capsule_pos, self.discs[self.capsule_pos].position_at_t(self.t));

      if self.capsule_pos >= self.discs.len() {
        break;
      } 
    }

    false
  }

  fn load_discs(&mut self, input: String) {
    // This is bogus, but I'm putting it here to avoid off-by-one bookkeeping
    // because I'm lazy af.
    let disc0 = Disc { positions: 0, start_pos: 0 };
    self.discs.push(box disc0);

    let lines:Vec<&str> = input.split("\n").collect();
    for line in lines {
      let parts:Vec<&str> = line.split(" ").collect();
      let start_pos_str:Vec<&str> = parts[11].split(".").collect();
      let start_pos = start_pos_str[0].parse::<usize>().unwrap();
      let disc = Disc { positions: parts[3].parse::<usize>().unwrap(), start_pos: start_pos };
      self.discs.push(box disc);
    }
  }

  fn reset(&mut self) {
    self.t = 0;
    self.capsule_pos = 0;
  }

}

fn part1(input: String) -> String  {

  let mut sculpture = Sculpture { discs: vec!(), capsule_pos: 0, t: 0 };
  sculpture.load_discs(input);

  let mut t = 0;

  loop {

    t += 1;

    if sculpture.start_at(t) {
      return t.to_string();
    }

    sculpture.reset();
  }
}

fn part2 (input: String) -> String  {

  let mut sculpture = Sculpture { discs: vec!(), capsule_pos: 0, t: 0 };
  sculpture.load_discs(input);
  sculpture.discs.push(box Disc { positions: 11, start_pos: 0 });

  let mut t = 0;

  loop {

    t += 1;

    if sculpture.start_at(t) {
      return t.to_string();
    }

    sculpture.reset();
  }
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
  assert_eq!((day.part1.run)(day.input.to_string()), "121834".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "3208099".to_string());
}
