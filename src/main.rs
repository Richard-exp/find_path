use::std::collections::VecDeque;

const START: Pos = Pos{column: 0, row: 0};
const FINISH: Pos = Pos{column: 3, row: 3};
const SIZE: usize = 4;                    // square-matrix

#[derive(Clone, PartialEq)]
struct Pos {
  row: i32,         
  column: i32,         
}
#[derive(PartialEq)]
struct Visited {
  node: Pos,
  parent: Pos,
}
struct Neighbours {
  queue: VecDeque<Pos>,
  visited: Vec<Visited>,
}

impl Visited {
  fn new (position: Pos, current: Pos) -> Self {
    Self {
      node: position,
      parent: current,
    }
  }
}

impl Neighbours {
  fn new () -> Self {
    Self {
    queue: VecDeque::new(),
    visited: Vec::new(),
    }
  }

  fn check_clockwise (&mut self, matrix:&[[i32; SIZE];SIZE]) -> Result<(), ()> {

  if let Some(current) = self.queue.pop_front() {

    let iterator_arr: [Pos; 8] = [Pos{row: current.row, column: current.column + 1}, Pos{row: current.row + 1, column: current.column + 1},
    Pos{row: current.row + 1, column: current.column}, Pos{row: current.row + 1, column: current.column - 1}, Pos{row: current.row, column: current.column - 1},
    Pos{row: current.row - 1, column: current.column - 1}, Pos{row: current.row - 1, column: current.column}, Pos{row: current.row - 1, column: current.column + 1}];
    
    for neighbour in iterator_arr.iter() {
      if neighbour.row >= 0 && neighbour.row < SIZE as i32 && neighbour.column >=0 && neighbour.column < SIZE as i32 {
      let row = neighbour.row as usize;
      let column = neighbour.column as usize;

      if matrix[row][column] == 1 {                                         //visits only passable neighbours (1nes)
        let visited_neighbour = Visited::new(neighbour.clone(), current.clone());
        if let Some(_) = self.visited.iter().find(|vis| vis.node == visited_neighbour.node) {
          continue;
        } else {
          self.visited.push(visited_neighbour);
          self.queue.push_back(neighbour.clone());
          println!("{}, {}", neighbour.row, neighbour.column);
        }
      } else {
        continue;
      }
    }
      else {
        continue;
      }
    }
    Ok(())
  } else {
    Err(())
  }
  }

  fn generate_path (&self) -> Vec<Pos> {

  }
}



fn main() {
  let mut matrix:[[i32; SIZE];SIZE] = [[1; SIZE];SIZE];
  matrix[1][1] = 0; matrix[1][2] = 0; matrix[0][3] = 0; matrix[2][1] = 0; matrix[2][3] = 0;
  show_matrix(matrix);
  let mut neighbours = Neighbours::new();
  neighbours.queue.push_back(START);
  neighbours.visited.push(Visited::new(START, START));
  while let Ok(()) = neighbours.check_clockwise(&matrix) {
  }
  neighbours.generate_path
}

fn show_matrix (matrix:[[i32; SIZE];SIZE]) {
  for i in matrix {
    for j in (0..SIZE) {
    print!("{} ",i[j])
    }
    println!("");
  }
}