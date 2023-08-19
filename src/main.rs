use ggez::{
    event,
    graphics::{self, Color},
    Context, GameError, GameResult,
};

// Here we define the size of our game board in terms of how many grid
// cells it will take up. We choose to make a 30 x 20 game board.
const GRID_SIZE: (i16, i16) = (30, 20);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const MAZE_DIMENTION: (usize, usize) = (40, 25);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct MazeCell {
    x: i16,
    y: i16,
}

impl MazeCell {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for MazeCell {
    fn from(value: (i16, i16)) -> Self {
        MazeCell {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<MazeCell> for graphics::Rect {
    fn from(pos: MazeCell) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

struct Maze {
    width: usize,
    height: usize,
    cells_stack: Vec<MazeCell>,
    visited_cells: i32,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells_stack: vec![
                MazeCell::new(0, 0),
                MazeCell::new(1, 1)
            ],
            visited_cells: 0,
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        for cell in &self.cells_stack {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect((*cell).into())
                    .color(Color::BLUE),
            );
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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

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

    let state = State {
        maze: Maze::new(MAZE_DIMENTION.0, MAZE_DIMENTION.1),
    };

    event::run(ctx, event_loop, state)
}
