use std::collections::HashSet;
use std::fmt;

const MAP_HEIGHT:usize = 50;
const MAP_WIDTH:usize = 50;

struct Map {
  grid: [[bool; MAP_WIDTH]; MAP_HEIGHT],
  traversed: [[bool; MAP_WIDTH]; MAP_HEIGHT],
  seed: usize,
}

impl Map {

  fn fill_grid(&mut self) {
    for x in 0..MAP_WIDTH {
      for y in 0..MAP_HEIGHT {
        let start:usize = (x*x + 3*x + 2*x*y + y + y*y) + self.seed;
        let binary_string:String = format!("{:b}", start);
        let mut ones_count = 0;
        for c in binary_string.chars() {
          if c == '1' {
            ones_count += 1;
          }
        }

        self.grid[y][x] = ones_count % 2 == 0;
        self.traversed[y][x] = false;
      }
    }
  }

  fn search_for_route(&mut self, x_start:usize, y_start:usize, x_goal:usize, y_goal:usize) -> usize {
    let mut depth:usize = 0;
    let mut seen_nodes:HashSet<(usize, usize)> = HashSet::new();

    let mut available_nodes:HashSet<(usize, usize)> = HashSet::new();
    available_nodes.insert((x_start, y_start));

    while !available_nodes.is_empty() {

      let iter = available_nodes.into_iter();

      available_nodes = HashSet::new();

      for node in iter {

        self.traversed[node.1][node.0] = true;

        if node.0 == x_goal && node.1 == y_goal {
          println!("Found path\n{:?}", self);
          return depth;
        }

        available_nodes.remove(&node);
        seen_nodes.insert((node.0, node.1));

        // Find neighboring elements
        for x in -1..2 {

          let my_x:isize = node.0 as isize + x;
          if my_x < 0 || my_x >= self.grid[0].len() as isize {
            continue;
          }

          for y in -1..2 {

            // Cut off diagonals.
            if (x == -1 && y == -1) || (x == 1 && y == 1) || (x == 1 && y == -1) || (x == -1 && y == 1) {
              continue;
            }

            let my_y:isize = node.1 as isize + y;
            if my_y < 0 || my_y >= self.grid.len() as isize {
              continue;
            }

            if seen_nodes.contains(&(my_x as usize, my_y as usize)) {
              continue;
            }

            // If this part of the map is open, traverse to it.
            if self.grid[my_y as usize][my_x as usize] {
              available_nodes.insert((my_x as usize, my_y as usize));
            }
          }
        }
      }

      depth += 1;
    }

    0
  }

  fn visit_depth_n(&mut self, x_start:usize, y_start:usize, max_depth:usize) -> usize {
    let mut depth:usize = 0;
    let mut seen_nodes:HashSet<(usize, usize)> = HashSet::new();

    let mut available_nodes:HashSet<(usize, usize)> = HashSet::new();
    available_nodes.insert((x_start, y_start));

    while depth <= max_depth {

      let iter = available_nodes.into_iter();

      available_nodes = HashSet::new();

      for node in iter {

        self.traversed[node.1][node.0] = true;

        available_nodes.remove(&node);
        seen_nodes.insert((node.0, node.1));

        // Find neighboring elements
        for x in -1..2 {

          let my_x:isize = node.0 as isize + x;
          if my_x < 0 || my_x >= self.grid[0].len() as isize {
            continue;
          }

          for y in -1..2 {

            // Cut off diagonals.
            if (x == -1 && y == -1) || (x == 1 && y == 1) || (x == 1 && y == -1) || (x == -1 && y == 1) {
              continue;
            }

            let my_y:isize = node.1 as isize + y;
            if my_y < 0 || my_y >= self.grid.len() as isize {
              continue;
            }

            if seen_nodes.contains(&(my_x as usize, my_y as usize)) {
              continue;
            }

            // If this part of the map is open, traverse to it.
            if self.grid[my_y as usize][my_x as usize] {
              available_nodes.insert((my_x as usize, my_y as usize));
            }
          }
        }
      }

      depth += 1;
    }

    println!("{:?}", self);

    seen_nodes.len()
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..self.grid.len() {
      for x in 0..self.grid[y].len() {
        match self.traversed[y][x] {
          true => { let _ = write!(f, "O"); },
          _ => {
            match self.grid[y][x] {
              true => { let _ = write!(f, "."); },
              false => { let _ = write!(f, "#"); },
            };
          }
        };
      }
      let _ = write!(f, "\n");
    }

    Ok(())
  }
}


fn part1(input: String) -> String  {
  let mut map = Map {
    grid: [[false; MAP_WIDTH]; MAP_HEIGHT],
    traversed: [[false; MAP_WIDTH]; MAP_HEIGHT],
    seed: input.parse::<usize>().unwrap(),
  };

  map.fill_grid();

  map.search_for_route(1, 1, 31, 39).to_string()
}

fn part2 (input: String) -> String  {
  let mut map = Map {
    grid: [[false; MAP_WIDTH]; MAP_HEIGHT],
    traversed: [[false; MAP_WIDTH]; MAP_HEIGHT],
    seed: input.parse::<usize>().unwrap(),
  };

  map.fill_grid();

  map.visit_depth_n(1, 1, 50).to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "1352".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "90".to_string());
}

// Leaving disabled until I can make this fast enough.
#[allow(dead_code)]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "9227737".to_string());
}
