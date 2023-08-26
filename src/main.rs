use ggez::{graphics::Color, GameError, GameResult};
use rand::Rng;

const MAZE_CELL_SIZE: (i16, i16) = (10, 10);
const PATH_WIDTH: i16 = 3;
const MAZE_SIZE: (usize, usize) = (40, 25);
const SCREEN_SIZE: (f32, f32) = (
    MAZE_SIZE.0 as f32 * MAZE_CELL_SIZE.0 as f32 * PATH_WIDTH as f32,
    MAZE_SIZE.1 as f32 * MAZE_CELL_SIZE.1 as f32 * PATH_WIDTH as f32,
);

#[derive(Debug, Clone)]
enum Path {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Cell {
    visited: bool,
    paths: Vec<Path>,
}

#[derive(Debug)]
struct Maze {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
    visited_cells_count: usize,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![
            vec![
                Cell {
                    visited: false,
                    paths: Vec::new()
                };
                width
            ];
            height
        ];

        Self {
            cells,
            height,
            width,
            visited_cells_count: 0,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x).cloned())
    }

    fn set_cell(&mut self, x: usize, y: usize, value: Cell) {
        if let Some(row_ref) = self.cells.get_mut(x) {
            if let Some(col_ref) = row_ref.get_mut(y) {
                *col_ref = value;
            }
        }
    }

    fn build_maze(&mut self, x: usize, y: usize, cell: &mut Cell) {
        let total_cells = self.width * self.height;
        if total_cells == self.visited_cells_count {
            println!("VISITED CHECK");
            return;
        }

        let mut rng = rand::thread_rng();
        let paths = [Path::Up, Path::Down, Path::Right, Path::Left];
        let random_idx: usize = rng.gen_range(0..paths.len());

        let random_path = paths.get(random_idx).unwrap();

        match random_path {
            Path::Up => {
                if x == 0 {
                    return self.build_maze(x, y, cell);
                }

                println!("UP");

                let (nx, ny) = (x - 1, y);

                if let Some(mut next_cell) = self.get_cell(nx, ny) {
                    cell.visited = true;
                    cell.paths.push(Path::Up);
                    self.set_cell(x, y, cell.clone());
                    self.visited_cells_count += 1;

                    self.build_maze(nx, ny, &mut next_cell)
                }
            }
            Path::Down => {
                let (nx, ny) = (x + 1, y);
                println!("DOWN");

                if let Some(mut next_cell) = self.get_cell(nx, ny) {
                    cell.visited = true;
                    cell.paths.push(Path::Down);
                    self.set_cell(x, y, cell.clone());
                    self.visited_cells_count += 1;

                    self.build_maze(nx, ny, &mut next_cell)
                }
            }
            Path::Right => {
                let (nx, ny) = (x, y + 1);
                println!("RIGHT");

                if let Some(mut next_cell) = self.get_cell(nx, ny) {
                    cell.visited = true;
                    cell.paths.push(Path::Right);
                    self.set_cell(x, y, cell.clone());
                    self.visited_cells_count += 1;

                    self.build_maze(nx, ny, &mut next_cell)
                }
            }
            Path::Left => {
                if y <= 0 {
                    return self.build_maze(x, y, cell);
                }

                println!("LEFT");

                let (nx, ny) = (x, y - 1);

                if let Some(mut next_cell) = self.get_cell(nx, ny) {
                    cell.visited = true;
                    cell.paths.push(Path::Left);
                    self.set_cell(x, y, cell.clone());
                    self.visited_cells_count += 1;

                    self.build_maze(nx, ny, &mut next_cell)
                }
            }
        }
    }
}

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
        let height = self.maze_renderer.maze.height;
        let width = self.maze_renderer.maze.width;

        for x in 0..height {
            for y in 0..width {
                if let Some(cell) = self.maze_renderer.maze.get_cell(x, y) {
                    let color = if cell.visited {
                        Color::WHITE
                    } else {
                        Color::BLUE
                    };
                    let cell_size = self.maze_renderer.cell_size;

                    for px in 0..PATH_WIDTH {
                        for py in 0..PATH_WIDTH {
                            let rect = ggez::graphics::Rect::new(
                                (x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (px as f32 * cell_size),
                                (y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (py as f32 * cell_size),
                                cell_size,
                                cell_size,
                            );

                            canvas.draw(
                                &ggez::graphics::Quad,
                                ggez::graphics::DrawParam::new()
                                    .dest_rect(rect)
                                    .color(color),
                            );
                        }
                    }

                    for path in cell.paths {
                        match path {
                            Path::Right => {
                                for k in 0..PATH_WIDTH {
                                    let px = PATH_WIDTH;
                                    let py = k;

                                    let rect = ggez::graphics::Rect::new(
                                        (x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (px as f32 * cell_size),
                                        (y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (py as f32 * cell_size),
                                        cell_size,
                                        cell_size,
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
                                for k in 0..PATH_WIDTH {
                                    let px = k;
                                    let py = -1;

                                    let rect = ggez::graphics::Rect::new(
                                        (x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (px as f32 * cell_size),
                                        (y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (py as f32 * cell_size),
                                        cell_size,
                                        cell_size,
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
                                for k in 0..PATH_WIDTH {
                                    let px = -1;
                                    let py = k;

                                    let rect = ggez::graphics::Rect::new(
                                        (x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (px as f32 * cell_size),
                                        (y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (py as f32 * cell_size),
                                        cell_size,
                                        cell_size,
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
                                for k in 0..PATH_WIDTH {
                                    let px = k;
                                    let py = PATH_WIDTH;

                                    let rect = ggez::graphics::Rect::new(
                                        (x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (px as f32 * cell_size),
                                        (y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                            + (py as f32 * cell_size),
                                        cell_size,
                                        cell_size,
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
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }
}

fn main() -> GameResult {
    let mut state = State {
        maze_renderer: MazeRenderer {
            maze: Maze::new(MAZE_SIZE.0, MAZE_SIZE.1),
            cell_size: MAZE_CELL_SIZE.0 as f32,
        },
    };

    let x = 0;
    let y = 0;
    let mut start_cell = state.maze_renderer.maze.get_cell(x, y).unwrap();
    state.maze_renderer.maze.build_maze(x, y, &mut start_cell);

    let window_dimensions =
        ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (ctx, event_loop) = ggez::ContextBuilder::new("maze-rs", "Kevin Carvalho")
        .window_mode(window_dimensions)
        .build()?;

    ggez::event::run(ctx, event_loop, state);
}
