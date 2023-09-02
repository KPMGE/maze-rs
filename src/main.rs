use ggez::{graphics::Color, GameError, GameResult};

mod cell;
mod path;
mod maze;

use maze::Maze;

const MAZE_CELL_SIZE: f32 = 9.0;
const PATH_WIDTH: usize = 3;
const MAZE_SIZE: (usize, usize) = (40, 30);
const SCREEN_SIZE: (f32, f32) = (
    MAZE_SIZE.0 as f32 * MAZE_CELL_SIZE as f32 * PATH_WIDTH as f32 + MAZE_CELL_SIZE as f32,
    MAZE_SIZE.1 as f32 * MAZE_CELL_SIZE as f32 * PATH_WIDTH as f32 + MAZE_CELL_SIZE as f32,
);

#[derive(Debug)]
struct MazeRenderer {
    maze: Maze,
    cell_size: f32,
}

struct State {
    maze_renderer: MazeRenderer,
}

impl ggez::event::EventHandler for State {
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::BLACK);

        for cell in &self.maze_renderer.maze.cells {
            let color = if cell.visited {
                Color::WHITE
            } else {
                Color::BLUE
            };
            cell.draw(&mut canvas, color.clone());
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }
}

fn main() -> GameResult {
    let mut state = State {
        maze_renderer: MazeRenderer {
            maze: Maze::new(MAZE_SIZE.0, MAZE_SIZE.1, PATH_WIDTH, MAZE_CELL_SIZE),
            cell_size: MAZE_CELL_SIZE as f32,
        },
    };

    let x = 0;
    let y = 0;
    let mut start_cell = state.maze_renderer.maze.get_cell(x, y).unwrap();
    // state.maze_renderer.maze.build_maze(x, y, &mut start_cell);

    let window_dimensions =
        ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (ctx, event_loop) = ggez::ContextBuilder::new("maze-rs", "Kevin Carvalho")
        .window_mode(window_dimensions)
        .build()?;

    ggez::event::run(ctx, event_loop, state);
}
