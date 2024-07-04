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
                    is_dead: false,
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
