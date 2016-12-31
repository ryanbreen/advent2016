
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

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
    days.push(day3::fill());
    days.push(day4::fill());
    days.push(day5::fill());
    days.push(day6::fill());
    days.push(day7::fill());
    days.push(day8::fill());
    days.push(day9::fill());
    days.push(day10::fill());
    days.push(day11::fill());
    days.push(day12::fill());
    days.push(day13::fill());
    days.push(day14::fill());
    days.push(day15::fill());
    days.push(day16::fill());
    days.push(day17::fill());
    days.push(day18::fill());
    Calendar {
      days: days
    }
  }
}

