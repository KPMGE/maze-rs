use ggez::{graphics::Color, GameError, GameResult};

mod cell;
mod path;
mod maze;

use maze::Maze;

const MAZE_CELL_SIZE: f32 = 9.0;
const PATH_WIDTH: usize = 3;
    const MAZE_SIZE: (usize, usize) = (40, 25);
const SCREEN_SIZE: (f32, f32) = (
    MAZE_SIZE.0 as f32 * MAZE_CELL_SIZE as f32 * (PATH_WIDTH + 1) as f32 - MAZE_CELL_SIZE as f32,
    MAZE_SIZE.1 as f32 * MAZE_CELL_SIZE as f32 * (PATH_WIDTH + 1) as f32 - MAZE_CELL_SIZE as f32,
);

struct State {
    maze: Maze,
    counter: i32
}

impl ggez::event::EventHandler for State {
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.maze.draw(&mut canvas);
        canvas.finish(ctx)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        if self.counter >= self.maze.width as i32 {
            return Ok(());
        }

        let mut cell = self.maze.get_cell(0 as usize, self.counter as usize).unwrap();
        cell.visited = true;
        self.maze.set_cell(cell.x, cell.y, cell.clone());
        self.counter += 1;

        Ok(())
    }
}

fn main() -> GameResult {
    let mut state = State {
        maze: Maze::new(
            MAZE_SIZE.0,
            MAZE_SIZE.1,
            PATH_WIDTH, 
            MAZE_CELL_SIZE
        ),
        counter: 0
    };

    let x = 0;
    let y = 0;
    let mut start_cell = state.maze.get_cell(x, y).unwrap();
    // state.maze.build_maze(x, y, &mut start_cell);

    let window_dimensions =
        ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (ctx, event_loop) = ggez::ContextBuilder::new("maze-rs", "Kevin Carvalho")
        .window_mode(window_dimensions)
        .build()?;

    ggez::event::run(ctx, event_loop, state);
}
