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
    x: usize,
    y: usize,
    paths: Vec<Path>,
}

#[derive(Debug)]
struct Maze {
    height: usize,
    width: usize,
    cells: Vec<Cell>,
    visited_cells: Vec<Cell>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
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
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let idx = x * self.width + y;
        self.cells.get(idx).cloned()
    }

    fn set_cell(&mut self, x: usize, y: usize, value: Cell) {
        let idx = x * self.width + y;
        if let Some(cell_ref) = self.cells.get_mut(idx) {
            *cell_ref = value;
        }
    }

    fn get_random_cell(&self, x: usize, y: usize) -> Option<(Cell, Path)> {
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

    fn build_maze(&mut self, x: usize, y: usize, cell: &mut Cell) {
        let total_cells = self.width * self.height;
        if total_cells == self.visited_cells.len() {
            return;
        }

        if let Some((random_cell, path)) = self.get_random_cell(x, y) {
            cell.paths.push(path.clone());
            self.visited_cells.push(cell.clone());
            cell.visited = true;
            self.set_cell(x, y, cell.clone());

            return self.build_maze(random_cell.x, random_cell.y, &mut random_cell.clone());
        } else {
            println!("encountered end at: {x}, {y}");
            return;
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

        for cell in &self.maze_renderer.maze.cells {
            let color = if cell.visited {
                Color::WHITE
            } else {
                Color::BLUE
            };
            let cell_size = self.maze_renderer.cell_size;

            for px in 0..PATH_WIDTH {
                for py in 0..PATH_WIDTH {
                    let rect = ggez::graphics::Rect::new(
                        (cell.x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                            + (px as f32 * cell_size),
                        (cell.y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
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

            for path in &cell.paths {
                match path {
                    Path::Right => {
                        for k in 0..PATH_WIDTH {
                            let px = PATH_WIDTH;
                            let py = k;

                            let rect = ggez::graphics::Rect::new(
                                (cell.x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (px as f32 * cell_size),
                                (cell.y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
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
                                (cell.x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (px as f32 * cell_size),
                                (cell.y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
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
                                (cell.x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (px as f32 * cell_size),
                                (cell.y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
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
                                (cell.x as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
                                    + (px as f32 * cell_size),
                                (cell.y as f32 * cell_size * (PATH_WIDTH as f32 + 1.0))
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
