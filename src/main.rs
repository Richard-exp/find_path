use std::collections::VecDeque;

const START: Pos = Pos { row: 0, column: 0 };
const FINISH: Pos = Pos { row: 7, column: 5 };
const SIZE: usize = 10; // square-matrix

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
    fn new(position: Pos, current: Pos) -> Self {
        Self {
            node: position,
            parent: current,
        }
    }
}

impl Neighbours {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            visited: Vec::new(),
        }
    }

    fn check_clockwise(&mut self, matrix: &[[i32; SIZE]; SIZE]) -> Result<(), ()> {
        if let Some(current) = self.queue.pop_front() {
            let iterator_arr: [Pos; 8] = [
                Pos {
                    row: current.row,
                    column: current.column + 1,
                },
                Pos {
                    row: current.row + 1,
                    column: current.column + 1,
                },
                Pos {
                    row: current.row + 1,
                    column: current.column,
                },
                Pos {
                    row: current.row + 1,
                    column: current.column - 1,
                },
                Pos {
                    row: current.row,
                    column: current.column - 1,
                },
                Pos {
                    row: current.row - 1,
                    column: current.column - 1,
                },
                Pos {
                    row: current.row - 1,
                    column: current.column,
                },
                Pos {
                    row: current.row - 1,
                    column: current.column + 1,
                },
            ];

            for neighbour in iterator_arr.iter() {
                if !(neighbour.row >= 0
                    && neighbour.row < SIZE as i32
                    && neighbour.column >= 0
                    && neighbour.column < SIZE as i32)
                {
                    continue;
                }
                let row = neighbour.row as usize;
                let column = neighbour.column as usize;

                if matrix[row][column] != 1 {
                    //visits only passable neighbours (1nes)
                    continue;
                }
                let visited_neighbour = Visited::new(neighbour.clone(), current.clone());
                if let Some(_) = self
                    .visited
                    .iter()
                    .find(|vis| vis.node == visited_neighbour.node)
                {
                    continue;
                } else {
                    self.visited.push(visited_neighbour);
                    self.queue.push_back(neighbour.clone());
                    //println!("{}, {}", neighbour.row, neighbour.column);
                }
            }
            // if current == FINISH {
            //     return Err(());
            // }
            Ok(())
        } else {
            Err(())
        }
    }

    fn generate_path(&self) -> Vec<Pos> {
        let mut path: Vec<Pos> = Vec::new();
        let mut parent = FINISH;
        while let Some(visited_node) = self.visited.iter().find(|vis| vis.node == parent) {
            path.push(visited_node.node.clone());
            if visited_node.node == START {
                break;
            }
            parent = visited_node.parent.clone();
        }
        path.iter().rev().map(|pos| pos.clone()).collect()
    }
}

fn main() {
    let matrix: [[i32; SIZE]; SIZE] = [[1; SIZE]; SIZE];
    let matrix = square_obstacle(
        matrix,
        Pos {
            row: (7),
            column: (2),
        },
        Pos {
            row: (3),
            column: (7),
        },
    );
    show_matrix(matrix);
    let mut neighbours = Neighbours::new();
    neighbours.queue.push_back(START);
    neighbours.visited.push(Visited::new(START, START));
    while let Ok(()) = neighbours.check_clockwise(&matrix) {}
    let path: Vec<Pos> = neighbours.generate_path();
    println!();
    path.iter()
        .for_each(|pos| print!(" -> [{},{}]", pos.row, pos.column));
}

fn show_matrix(matrix: [[i32; SIZE]; SIZE]) {
    for i in matrix {
        for j in (0..SIZE) {
            print!("{} ", i[j])
        }
        println!();
    }
}

fn square_obstacle(
    mut matrix: [[i32; SIZE]; SIZE],
    bottom_corner: Pos,
    top_corner: Pos,
) -> [[i32; SIZE]; SIZE] {
    for row in (top_corner.row..bottom_corner.row) {
        for column in (bottom_corner.column..top_corner.column) {
            matrix[row as usize][column as usize] = 0;
        }
    }
    matrix
}
