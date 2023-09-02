use crate::{cell::Cell, path::Path};
use ggez::graphics::Color;
use rand::Rng;

#[derive(Debug)]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Cell>,
    pub visited_cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize, cell_path_width: usize, cell_size: f32) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        for x in 0..height {
            for y in 0..width {
                let cell = Cell {
                    visited: false,
                    x,
                    y,
                    paths: Vec::new(),
                    path_width: cell_path_width,
                    size: cell_size,
                };
                cells.push(cell);
            }
        }

        Self {
            cells,
            height,
            width,
            visited_cells: Vec::new(),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let idx = x * self.width + y;
        self.cells.get(idx).cloned()
    }

    fn set_cell(&mut self, x: usize, y: usize, value: Cell) {
        let idx = x * self.width + y;
        if let Some(cell_ref) = self.cells.get_mut(idx) {
            *cell_ref = value;
        }
    }

    pub fn get_random_cell(&self, x: usize, y: usize) -> Option<(Cell, Path)> {
        struct CellPath {
            cell: Option<Cell>,
            path: Path,
        }

        let cell_right = CellPath {
            cell: self.get_cell(x, y + 1),
            path: Path::Right,
        };

        let cell_down = CellPath {
            cell: self.get_cell(x + 1, y),
            path: Path::Down,
        };

        let cell_up = CellPath {
            cell: match x {
                0 => None,
                _ => self.get_cell(x - 1, y),
            },
            path: Path::Up,
        };

        let cell_left = CellPath {
            cell: match y {
                0 => None,
                _ => self.get_cell(x, y - 1),
            },
            path: Path::Left,
        };

        let mut rng = rand::thread_rng();
        let cells = [cell_up, cell_down, cell_left, cell_right];
        let available_cells: Vec<&CellPath> = cells
            .iter()
            .filter(|cell_path| {
                if let Some(c) = cell_path.cell.clone() {
                    if c.visited {
                        return false;
                    } else {
                        return true;
                    }
                }

                return false;
            })
            .collect();

        if available_cells.len() == 0 {
            return None;
        }

        let random_idx: usize = rng.gen_range(0..available_cells.len());
        let random_cell = available_cells[random_idx];

        Some((
            random_cell.cell.clone().unwrap().clone(),
            random_cell.path.clone(),
        ))
    }

    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        for cell in &self.cells {
            let color = if cell.visited {
                Color::WHITE
            } else {
                Color::BLUE
            };
            cell.draw(canvas, color.clone());
        }
    }

    pub fn build_maze(&mut self, x: usize, y: usize, cell: &mut Cell) {
        if self.cells.len() == self.visited_cells.len() {
            return;
        }

        if let Some((random_cell, path)) = self.get_random_cell(x, y) {
            cell.visited = true;
            self.visited_cells.push(cell.clone());
            cell.paths.push(path.clone());
            self.set_cell(x, y, cell.clone());

            return self.build_maze(random_cell.x, random_cell.y, &mut random_cell.clone());
        } else {
            if !cell.visited {
                cell.visited = true;
                self.visited_cells.push(cell.clone());
                self.set_cell(x, y, cell.clone());
                return self.build_maze(cell.x, cell.y, cell);
            }

            self.visited_cells.pop();
            if let Some(mut previous_cell) = self.visited_cells.pop() {
                return self.build_maze(previous_cell.x, previous_cell.y, &mut previous_cell);
            }
        }
    }
}
