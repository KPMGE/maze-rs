use ggez::graphics::{self, Color};

use crate::path::Path;

#[derive(Debug, Clone)]
pub struct Cell {
    pub visited: bool,
    pub x: usize,
    pub y: usize,
    pub paths: Vec<Path>,
    pub path_width: usize,
    pub size: f32,
}

impl Cell {
    pub fn draw(&self, canvas: &mut graphics::Canvas, color: Color) {
        for px in 0..self.path_width {
            for py in 0..self.path_width {
                let rect = ggez::graphics::Rect::new(
                    (self.y as f32 * self.size * (self.path_width as f32 + 1.0))
                        + (px as f32 * self.size),
                    (self.x as f32 * self.size * (self.path_width as f32 + 1.0))
                        + (py as f32 * self.size),
                    self.size,
                    self.size,
                );

                canvas.draw(
                    &ggez::graphics::Quad,
                    ggez::graphics::DrawParam::new()
                        .dest_rect(rect)
                        .color(color),
                );
            }
        }

        for path in &self.paths {
            match path {
                Path::Right => {
                    for k in 0..self.path_width {
                        let px = self.path_width;
                        let py = k;

                        let rect = ggez::graphics::Rect::new(
                            (self.y as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (px as f32 * self.size),
                            (self.x as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (py as f32 * self.size),
                            self.size,
                            self.size,
                        );

                        canvas.draw(
                            &ggez::graphics::Quad,
                            ggez::graphics::DrawParam::new()
                                .dest_rect(rect)
                                .color(Color::WHITE),
                        );
                    }
                }
                Path::Up => {
                    for k in 0..self.path_width {
                        let px = k;
                        let py = -1;

                        let rect = ggez::graphics::Rect::new(
                            (self.y as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (px as f32 * self.size),
                            (self.x as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (py as f32 * self.size),
                            self.size,
                            self.size,
                        );

                        canvas.draw(
                            &ggez::graphics::Quad,
                            ggez::graphics::DrawParam::new()
                                .dest_rect(rect)
                                .color(Color::WHITE),
                        );
                    }
                }
                Path::Left => {
                    for k in 0..self.path_width {
                        let px = -1;
                        let py = k;

                        let rect = ggez::graphics::Rect::new(
                            (self.y as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (px as f32 * self.size),
                            (self.x as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (py as f32 * self.size),
                            self.size,
                            self.size,
                        );

                        canvas.draw(
                            &ggez::graphics::Quad,
                            ggez::graphics::DrawParam::new()
                                .dest_rect(rect)
                                .color(Color::WHITE),
                        );
                    }
                }
                Path::Down => {
                    for k in 0..self.path_width {
                        let px = k;
                        let py = self.path_width;

                        let rect = ggez::graphics::Rect::new(
                            (self.y as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (px as f32 * self.size),
                            (self.x as f32 * self.size * (self.path_width as f32 + 1.0))
                                + (py as f32 * self.size),
                            self.size,
                            self.size,
                        );

                        canvas.draw(
                            &ggez::graphics::Quad,
                            ggez::graphics::DrawParam::new()
                                .dest_rect(rect)
                                .color(Color::WHITE),
                        );
                    }
                }
            };
        }
    }
}
