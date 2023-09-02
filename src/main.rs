use ggez::{graphics::Color, GameError, GameResult};

mod cell;
mod maze;
mod path;

use maze::Maze;

const FPS: u32 = 60;
const MAZE_CELL_SIZE: f32 = 9.0;
const PATH_WIDTH: usize = 3;
const MAZE_SIZE: (usize, usize) = (40, 25);
const SCREEN_SIZE: (f32, f32) = (
    MAZE_SIZE.0 as f32 * MAZE_CELL_SIZE as f32 * (PATH_WIDTH + 1) as f32 - MAZE_CELL_SIZE as f32,
    MAZE_SIZE.1 as f32 * MAZE_CELL_SIZE as f32 * (PATH_WIDTH + 1) as f32 - MAZE_CELL_SIZE as f32,
);

struct State {
    maze: Maze,
}

impl ggez::event::EventHandler for State {
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.maze.draw(&mut canvas);
        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(FPS) {
            if self.maze.cells.len() == self.maze.visited_cells.len() {
                return Ok(());
            }

            let mut cell = self
                .maze
                .visited_cells
                .pop()
                .expect("the visited_cells stack must have at least 1 cell");

            if let Some((random_cell, path)) = self.maze.get_random_cell(cell.x, cell.y) {
                cell.visited = true;
                cell.paths.push(path);
                self.maze.set_cell(cell.x, cell.y, cell.clone());

                self.maze.visited_cells.push(cell.clone());
                self.maze.visited_cells.push(random_cell.clone());
            } else {
                self.maze.visited_cells.pop();
            }
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let mut state = State {
        maze: Maze::new(MAZE_SIZE.0, MAZE_SIZE.1, PATH_WIDTH, MAZE_CELL_SIZE),
    };

    let x = 0;
    let y = 0;
    let start_cell = state.maze.get_cell(x, y).unwrap();
    state.maze.visited_cells.push(start_cell);

    let window_dimensions =
        ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (ctx, event_loop) = ggez::ContextBuilder::new("maze-rs", "Kevin Carvalho")
        .window_mode(window_dimensions)
        .build()?;

    ggez::event::run(ctx, event_loop, state);
}
