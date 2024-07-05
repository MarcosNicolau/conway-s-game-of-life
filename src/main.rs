use ::rand::prelude::*;
use macroquad::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

struct Cell {
    pos: Pos,
    is_dead: bool,
}

type CellMatrix = Vec<Vec<Cell>>;

struct Game {
    cells: CellMatrix,
    cell_size: f32,
}

impl Game {
    pub fn new() -> Self {
        let cell_size = 5.0;
        Self {
            cell_size,
            cells: Game::generate_cells(cell_size),
        }
    }

    fn generate_cells(cell_size: f32) -> CellMatrix {
        let mut matrix: CellMatrix = vec![];
        let num_of_rows = screen_height() / cell_size;
        let num_of_cols = screen_width() / cell_size;
        (0..num_of_rows as i32).into_iter().for_each(|row_idx| {
            let cells: Vec<Cell> = (0..num_of_cols as i32)
                .into_iter()
                .map(|col_idx| Cell {
                    pos: Pos {
                        x: col_idx as f32 * cell_size,
                        y: row_idx as f32 * cell_size,
                    },
                    // 1 percent chance of cell being alive
                    // or 99 percent chances of cell being dead
                    is_dead: thread_rng().gen_range(0..101) <= 99,
                })
                .collect();
            matrix.push(cells);
        });

        return matrix;
    }

    pub async fn start(&mut self) {
        loop {
            clear_background(BLACK);
            self.cells.iter().for_each(|el| {
                el.iter().for_each(|el| {
                    draw_rectangle(
                        el.pos.x,
                        el.pos.y,
                        self.cell_size,
                        self.cell_size,
                        if el.is_dead { BLACK } else { WHITE },
                    )
                })
            });
            next_frame().await;
        }
    }

    fn get_neighbors_count(&self, row_idx: usize, col_idx: usize) -> i32 {
        let cells: &Vec<Vec<Cell>> = &self.cells;
        let start_row = if row_idx == 0 { 0 } else { row_idx - 1 };
        let end_row = if row_idx >= cells.len() - 1 {
            cells.len()
        } else {
            row_idx + 2
        };

        cells[start_row..end_row]
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let actual_row_idx = start_row + idx;
                let mut count = 0;

                count += self.cell_state_to_number(actual_row_idx, col_idx + 1);
                count += match col_idx {
                    0 => 0,
                    _ => self.cell_state_to_number(actual_row_idx, col_idx - 1),
                };
                count += match actual_row_idx {
                    idx if idx == row_idx => 0,
                    _ => self.cell_state_to_number(actual_row_idx, col_idx),
                };

                count
            })
            .sum()
    }

    fn cell_state_to_number(&self, row_idx: usize, col_idx: usize) -> i32 {
        self.cells
            .get(row_idx)
            .and_then(|cells| cells.get(col_idx))
            .map_or(0, |cell| if cell.is_dead { 0 } else { 1 })
    }

    fn conf() -> Conf {
        Conf {
            window_title: "Conway's Game of Life".to_owned(),
            fullscreen: false,
            window_resizable: false,
            window_height: 600,
            window_width: 800,
            ..Default::default()
        }
    }
}

fn conf() -> Conf {
    Game::conf()
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();
    game.start().await;
}
