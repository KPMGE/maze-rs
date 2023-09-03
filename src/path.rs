use ggez::graphics::{Canvas, Color, DrawParam, Quad, Rect};

use crate::cell::Cell;

#[derive(Debug, Clone)]
pub enum Path {
    Left,
    Right,
    Up,
    Down,
}

impl Path {
    fn draw_rect(
        &self,
        canvas: &mut Canvas,
        px: f32,
        py: f32,
        cell: &Cell,
        path_width: usize,
        cell_size: f32,
    ) {
        let rect = Rect::new(
            (cell.y as f32 * cell_size * (path_width as f32 + 1.0)) + (px as f32 * cell_size),
            (cell.x as f32 * cell_size * (path_width as f32 + 1.0)) + (py as f32 * cell_size),
            cell_size,
            cell_size,
        );

        canvas.draw(&Quad, DrawParam::new().dest_rect(rect).color(Color::WHITE));
    }

    pub fn draw(&self, canvas: &mut Canvas, cell: &Cell, path_width: usize, cell_size: f32) {
        match self {
            Path::Right => {
                for k in 0..path_width {
                    let px = path_width;
                    let py = k;
                    self.draw_rect(canvas, px as f32, py as f32, cell, path_width, cell_size);
                }
            }
            Path::Up => {
                for k in 0..path_width {
                    let px = k;
                    let py = -1;
                    self.draw_rect(canvas, px as f32, py as f32, cell, path_width, cell_size);
                }
            }
            Path::Left => {
                for k in 0..path_width {
                    let px = -1;
                    let py = k;
                    self.draw_rect(canvas, px as f32, py as f32, cell, path_width, cell_size);
                }
            }
            Path::Down => {
                for k in 0..path_width {
                    let px = k;
                    let py = path_width;
                    self.draw_rect(canvas, px as f32, py as f32, cell, path_width, cell_size);
                }
            }
        };
    }
}
