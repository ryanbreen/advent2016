
mod day1;
mod day2;

pub struct Puzzle {
  pub run: fn(String) -> String
}

impl Puzzle {
}

pub struct Day {
  pub input: String,
  pub part1: Puzzle,
  pub part2: Puzzle
}

pub struct Calendar {
  pub days: Vec<Day>
}

impl Calendar {
  pub fn new() -> Calendar
  {
    let mut days = Vec::new();
    days.push(day1::fill());
    days.push(day2::fill());
    Calendar {
      days: days
    }
  }
}

