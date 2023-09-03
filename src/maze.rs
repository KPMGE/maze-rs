use crate::{cell::Cell, path::Path};
use ggez::graphics::Color;
use rand::Rng;

#[derive(Debug)]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub cell_size: f32,
    pub cell_path_width: usize,
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
                };
                cells.push(cell);
            }
        }

        Self {
            cells,
            height,
            width,
            visited_cells: Vec::new(),
            cell_size,
            cell_path_width
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let idx = x * self.width + y;
        self.cells.get(idx).cloned()
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: Cell) {
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
                    return !c.visited;
                }

                false
            })
            .collect();

        if available_cells.is_empty() {
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
        let current_cell = self
            .visited_cells
            .last()
            .expect("the visited_cells stack must have at least 1 cell");

        for cell in &self.cells {
            let color = if cell == current_cell {
                Color::GREEN
            } else if cell.visited {
                Color::WHITE
            } else {
                Color::BLUE
            };

            cell.draw(canvas, color, self.cell_size, self.cell_path_width);
        }
    }
}
