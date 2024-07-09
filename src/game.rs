use ::rand::prelude::*;
use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets},
};
use miniquad::window::set_window_size;
use std::{ops::Range, process::exit, thread::sleep, time::Duration};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(PartialEq, Debug)]
struct Cell {
    pos: Pos,
    is_dead: bool,
}

pub struct Screen {
    height: u32,
    width: u32,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

type CellMatrix = Vec<Vec<Cell>>;

struct GameState {
    speed_in_ms: f32,
    is_paused: bool,
}

pub struct Game {
    screen: Screen,
    cells: CellMatrix,
    cell_size: u32,
    state: GameState,
}

pub type Seeder = fn(row_idx: i32, col_idx: i32) -> bool;

pub fn get_random_seeder(percentage: u32) -> Box<dyn Fn(u32, u32) -> bool> {
    Box::new(move |_, _| thread_rng().gen_range(0..101) <= percentage)
}

impl Game {
    pub fn new(screen: Screen, seeder: impl Fn(u32, u32) -> bool) -> Self {
        let cell_size = 5;

        Self {
            cell_size,
            cells: Game::generate_cells(&screen, cell_size, seeder),
            screen,
            state: GameState {
                speed_in_ms: 10.,
                is_paused: true,
            },
        }
    }

    fn generate_cells(
        screen: &Screen,
        cell_size: u32,
        seeder: impl Fn(u32, u32) -> bool,
    ) -> CellMatrix {
        let mut matrix: CellMatrix = vec![];
        let num_of_rows = screen.height / cell_size;
        let num_of_cols = screen.width / cell_size;
        for row in 0..num_of_rows {
            let cells: Vec<Cell> = (0..num_of_cols)
                .map(|col_idx| Cell {
                    pos: Pos {
                        x: (col_idx * cell_size) as i32,
                        y: (row * cell_size) as i32,
                    },

                    is_dead: seeder(row, col_idx),
                })
                .collect();
            matrix.push(cells);
        }

        return matrix;
    }

    pub async fn start(&mut self) {
        self.setup();

        loop {
            clear_background(BLACK);
            self.draw_ui();
            self.draw_cells();
            if !self.state.is_paused {
                self.cells = self.get_new_generation();
                sleep(Duration::from_millis(self.state.speed_in_ms as u64));
            }
            next_frame().await;
        }
    }

    fn draw_cells(&self) {
        for cells in self.cells.iter() {
            cells.iter().filter(|cell| !cell.is_dead).for_each(|cell| {
                draw_rectangle(
                    cell.pos.x as f32,
                    cell.pos.y as f32,
                    self.cell_size as f32,
                    self.cell_size as f32,
                    WHITE,
                )
            });
        }
    }

    fn draw_ui(&mut self) {
        widgets::Window::new(
            100,
            vec2(0., self.screen.height as f32 - 120.),
            vec2(self.screen.width as f32, 100.),
        )
        .label("Config")
        .ui(&mut *root_ui(), |ui| {
            let range: Range<f32> = 0.0..100.0;
            ui.slider(hash!(), "speed in ms", range, &mut self.state.speed_in_ms);
            if ui.button(
                vec2(self.screen.width as f32 / 2. - 20., 20.),
                if self.state.is_paused {
                    "Start"
                } else {
                    "Pause"
                },
            ) {
                self.state.is_paused = !self.state.is_paused;
            }

            if ui.button(vec2(self.screen.width as f32 / 2. - 20., 45.), "Exit ") {
                exit(0)
            }
        });
    }

    fn get_new_generation(&self) -> CellMatrix {
        let mut new_gen: CellMatrix = vec![];

        for (row_idx, row) in self.cells.iter().enumerate() {
            let row = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| Cell {
                    is_dead: Self::apply_cell_rules(
                        self.get_neighbors_count(row_idx, col_idx),
                        cell.is_dead,
                    ),
                    ..*cell
                })
                .collect();
            new_gen.push(row);
        }

        new_gen
    }

    fn apply_cell_rules(neighbors_count: i32, is_dead: bool) -> bool {
        match (neighbors_count, is_dead) {
            (3, true) => false,
            (2 | 3, false) => false,
            _ => true,
        }
    }

    fn get_neighbors_count(&self, row_idx: usize, col_idx: usize) -> i32 {
        let cells: &CellMatrix = &self.cells;
        let start_row = if row_idx == 0 { 0 } else { row_idx - 1 };
        let end_row = (row_idx + 2).min(cells.len());

        let mut count = 0;

        for (idx, _) in cells[start_row..end_row].iter().enumerate() {
            let actual_row_idx = start_row + idx;
            if col_idx > 0 {
                count += self.cell_state_to_number(actual_row_idx, col_idx - 1);
            }
            if actual_row_idx != row_idx {
                count += self.cell_state_to_number(actual_row_idx, col_idx);
            }
            count += self.cell_state_to_number(actual_row_idx, col_idx + 1);
        }

        count
    }

    fn cell_state_to_number(&self, row_idx: usize, col_idx: usize) -> i32 {
        self.cells
            .get(row_idx)
            .and_then(|cells| cells.get(col_idx as usize))
            .map_or(0, |cell| if cell.is_dead { 0 } else { 1 })
    }

    pub fn setup(&self) {
        set_window_size(self.screen.width, self.screen.height);
    }

    pub fn conf() -> Conf {
        Conf {
            window_title: "Conway's Game of Life".to_owned(),
            fullscreen: false,
            window_resizable: false,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO finish this test
    #[test]
    fn should_get_next_generation() {}

    #[test]
    fn should_get_neighbors_count() {
        let seeder = |row_idx, col_idx| match row_idx {
            50 if (19..22).contains(&col_idx) => false,
            _ => true,
        };
        let game = Game::new(Screen::default(), seeder);
        assert_eq!(game.get_neighbors_count(50, 20), 2);
    }

    #[test]
    fn should_generate_cells_accordingly() {
        // if the cells size is 600, then there should be 1 col and 1 row
        let cells_matrix = Game::generate_cells(&Screen::default(), 600, |_, _| false);
        assert_eq!(
            cells_matrix,
            vec![vec![Cell {
                pos: Pos { x: 0, y: 0 },
                is_dead: false
            }]]
        )
    }

    #[test]
    fn should_map_cell_state_to_1() {
        let game = Game::new(Screen::default(), |_, _| false);
        assert_eq!(game.cell_state_to_number(2, 2), 1);
    }

    #[test]
    fn should_map_cell_state_to_0() {
        let game = Game::new(Screen::default(), |_, _| true);
        assert_eq!(game.cell_state_to_number(2, 2), 0);
    }
}
