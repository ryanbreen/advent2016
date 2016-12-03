#![feature(box_syntax, const_fn)]
mod calendar;

extern crate core;

#[cfg(not(test))]
extern crate getopts;

#[cfg(not(test))]
use getopts::Options;

#[cfg(not(test))]
use std::env;

#[cfg(not(test))]
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

#[cfg(not(test))]
fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.reqopt("d", "", "The day to test (0-24).", "DAY");
  opts.reqopt("p", "", "The puzzle to test (0-1).", "PUZZLE");
  opts.optopt("i", "", "Alternate input text.", "INPUT");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!(f.to_string()) }
  };
  if matches.opt_present("h") {
      print_usage(&program, opts);
      return;
  }
  let day = matches.opt_str("d").expect("Missing value").parse::<usize>().unwrap();
  let first_puzzle:bool = matches.opt_str("p").expect("Missing value") == "0";

  let calendar = calendar::Calendar::new();

  if day > calendar.days.len()-1 {
    panic!("Invalid day {}", day);
  }

  let input= if matches.opt_present("i") {
    matches.opt_str("i").expect("Missing value")
  } else {
    calendar.days[day].input.to_string()
  };

  if first_puzzle {
    println!("{}", (&(calendar.days[day]).part1.run)(input));
  } else {
    println!("{}", (&(calendar.days[day]).part2.run)(input));
  }
}

#[test]
fn calendar_test() {
  let calendar = calendar::Calendar::new();
  assert_eq!(calendar.days.len(), 3);
}