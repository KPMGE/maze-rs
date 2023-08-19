use std::cell::RefCell;

use ggez::{
    event,
    graphics::{self, Color},
    Context, GameError, GameResult,
};

const MAZE_CELL_SIZE: (i16, i16) = (10, 10);
const MAZE_SIZE: (usize, usize) = (95, 75);
const SCREEN_SIZE: (f32, f32) = (
    MAZE_SIZE.0 as f32 * MAZE_CELL_SIZE.0 as f32,
    MAZE_SIZE.1 as f32 * MAZE_CELL_SIZE.1 as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct MazeCell {
    x: i16,
    y: i16,
    visited: bool,
}

impl MazeCell {
    fn new(x: i16, y: i16, visited: bool) -> Self {
        Self { x, y, visited }
    }
}

impl From<(i16, i16)> for MazeCell {
    fn from(value: (i16, i16)) -> Self {
        MazeCell {
            x: value.0,
            y: value.1,
            visited: false,
        }
    }
}

impl From<MazeCell> for graphics::Rect {
    fn from(pos: MazeCell) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * MAZE_CELL_SIZE.0 as i32,
            pos.y as i32 * MAZE_CELL_SIZE.1 as i32,
            MAZE_CELL_SIZE.0 as i32,
            MAZE_CELL_SIZE.1 as i32,
        )
    }
}

struct Maze {
    width: usize,
    height: usize,
    cells_stack: Vec<MazeCell>,
    visited_cells: i32,
    path_width: i16,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let mut cells_stack = Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                cells_stack.push(MazeCell::new(x as i16, y as i16, false));
            }
        }

        Self {
            width,
            height,
            cells_stack,
            visited_cells: 0,
            path_width: 3,
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        for cell in &self.cells_stack {

            for px in 0..self.path_width {
                for py in 0..self.path_width {
                    let mut new_cell = cell.clone();
                    new_cell.x *= self.path_width + 1;
                    new_cell.y *= self.path_width + 1;
                    new_cell.x += px;
                    new_cell.y += py;

                    if cell.visited {
                        canvas.draw(
                            &graphics::Quad,
                            graphics::DrawParam::new()
                                .dest_rect(new_cell.into())
                                .color(Color::WHITE),
                        );
                    } else {
                        canvas.draw(
                            &graphics::Quad,
                            graphics::DrawParam::new()
                                .dest_rect(new_cell.into())
                                .color(Color::BLUE),
                        );
                    }
                }
            }
        }
    }
}

struct State {
    maze: Maze,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.maze.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let window_dimensions =
        ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);

    let (ctx, event_loop) = ggez::ContextBuilder::new("maze", "Kevin Carvalho")
        .window_mode(window_dimensions)
        .build()?;

    let mut maze = Maze::new(MAZE_SIZE.0, MAZE_SIZE.1);
    maze.cells_stack[0].visited = true;
    maze.visited_cells = 1;

    let state = State { maze };

    event::run(ctx, event_loop, state)
}
