const MAX_X:usize = 50;
//const MAX_X:usize = 7;
const MAX_Y:usize = 6;
//const MAX_Y:usize = 3;

fn print_screen(pixel_grid:&[[bool; MAX_Y]; MAX_X]) {
  for y in 0..MAX_Y {
    for x in 0..MAX_X {
      if pixel_grid[x][y] {
        print!("#");
      } else {
        print!(".");
      }
    }

    println!("");
  }
}

fn count_set(pixel_grid:&[[bool; MAX_Y]; MAX_X]) -> usize {
  let mut result:usize = 0;
  for y in 0..MAX_Y {
    for x in 0..MAX_X {
      if pixel_grid[x][y] {
        result += 1;
      }
    }
  }

  result
}

// This is necessary because primitve array clone only supports up to 32 els.
fn clone(pixel_grid:&[[bool; MAX_Y]; MAX_X]) -> [[bool; MAX_Y]; MAX_X] {
  let mut orig:[[bool; MAX_Y]; MAX_X] = [[false; MAX_Y]; MAX_X];
  for y in 0..MAX_Y {
    for x in 0..MAX_X {
      orig[x][y] = pixel_grid[x][y];
    }
  }
  orig
}

fn rect(pixel_grid:&mut [[bool; MAX_Y]; MAX_X], x: usize, y: usize) {
  for my_y in 0..y {
    for my_x in 0..x {
      pixel_grid[my_x][my_y] = true;
    }
  }
}

fn get_new_pos(current:usize, distance:usize, max:usize) -> usize {
  (current + distance) % max
}

fn rotate_column(pixel_grid:&mut [[bool; MAX_Y]; MAX_X], index: usize, distance: usize) {
  let orig = clone(&pixel_grid);
  for y in 0..MAX_Y {
    pixel_grid[index][get_new_pos(y, distance, MAX_Y)] = orig[index][y];
  }
}

fn rotate_row(pixel_grid:&mut [[bool; MAX_Y]; MAX_X], index: usize, distance: usize) {
  let orig = clone(&pixel_grid);
  for x in 0..MAX_X {
    pixel_grid[get_new_pos(x, distance, MAX_X)][index] = orig[x][index];
  }
}

fn part1(input: String) -> String  {
  let mut pixel_grid:[[bool; MAX_Y]; MAX_X] = [[false; MAX_Y]; MAX_X];
  println!("{} {}", pixel_grid.len(), pixel_grid[0].len());
  let instructions:Vec<&str> = input.split("\n").collect();
  for instruction in &instructions {
    let parts:Vec<&str> = instruction.split(" ").collect();
    
    match parts[0] {
      "rect" => {
        let coords:Vec<&str> = parts[1].split("x").collect();
        rect(&mut pixel_grid, coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap());
      },
      "rotate" => {
        let index_strs:Vec<&str> = parts[2].split("=").collect();
        let index:usize = index_strs[1].parse::<usize>().unwrap();
        let distance:usize = parts[4].parse::<usize>().unwrap();
        match parts[1] {
          "column" => rotate_column(&mut pixel_grid, index, distance),
          "row" => rotate_row(&mut pixel_grid, index, distance),
          _ => println!("Ragnarok")
        }
      }
      _ => println!("Ragnarok")
    }
  }

  print_screen(&pixel_grid);

  count_set(&pixel_grid).to_string()
}

fn part2 (input: String) -> String  {
  0.to_string()
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
  assert_eq!((day.part1.run)(day.input.to_string()), "106".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "231".to_string());
}
