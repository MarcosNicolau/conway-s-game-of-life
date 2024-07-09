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
    is_paused: bool,
    game_has_started: bool,
    speed_in_ms: f32,
    gen_number: i32,
    alive_cells_number: i32,
}

pub struct Game {
    screen: Screen,
    cells: CellMatrix,
    cell_size: u32,
    state: GameState,
}

pub type Seeder = fn(row_idx: u32, col_idx: u32) -> bool;

impl Game {
    pub fn new(screen: Screen, seeder: Option<Seeder>) -> Self {
        let cell_size = 5;

        Self {
            cell_size,
            cells: Game::generate_cells(&screen, cell_size, seeder),
            screen,
            state: GameState {
                game_has_started: false,
                alive_cells_number: 0,
                gen_number: 0,
                speed_in_ms: 10.,
                is_paused: true,
            },
        }
    }

    fn generate_cells(screen: &Screen, cell_size: u32, seeder: Option<Seeder>) -> CellMatrix {
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

                    is_dead: seeder.unwrap_or(|_, _| true)(row, col_idx),
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
            if !self.state.game_has_started {
                draw_text(
                    "Paint the cells",
                    self.screen.width as f32 / 2.0
                        - measure_text("Paint the cells", None, 50, 1.).width / 2.0,
                    self.screen.height as f32 / 2.0 - 50.0 / 2.0,
                    50.0,
                    WHITE,
                );
                draw_text(
                    "and press start",
                    self.screen.width as f32 / 2.0
                        - measure_text("and press start", None, 30, 1.).width / 2.0,
                    self.screen.height as f32 / 2.0 - 30.0 / 2.0 + 20.,
                    30.0,
                    WHITE,
                );
                self.show_paint_cells();
            }
            self.draw_ui();
            self.draw_grid();
            self.draw_cells();
            if !self.state.is_paused {
                self.cells = self.get_new_generation();
                sleep(Duration::from_millis(self.state.speed_in_ms as u64));
            }
            next_frame().await;
        }
    }

    fn show_paint_cells(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();

            let cell_x = (mouse_pos.0 / self.cell_size as f32) as usize;
            let cell_y = (mouse_pos.1 / self.cell_size as f32) as usize;

            if let Some(cell) = self
                .cells
                .get_mut(cell_y)
                .and_then(|row| row.get_mut(cell_x))
            {
                cell.is_dead = !cell.is_dead;
            }
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

    fn draw_grid(&self) {
        let num_of_rows = self.screen.height / self.cell_size;
        let num_of_cols = self.screen.width / self.cell_size;

        for idx in 0..=num_of_rows {
            let y = (idx * self.cell_size) as f32;
            draw_line(0.0, y, self.screen.width as f32, y, 1.0, GRAY);
        }

        for idx in 0..=num_of_cols {
            let x = (idx * self.cell_size) as f32;
            draw_line(x, 0.0, x, self.screen.height as f32, 1.0, GRAY);
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
                self.state.game_has_started = true;
            }

            if ui.button(vec2(self.screen.width as f32 / 2. - 20., 45.), "Exit ") {
                exit(0)
            }
        });
        draw_text(
            &format!("Alive cells: {}", self.state.alive_cells_number),
            20.,
            20.,
            30.,
            WHITE,
        );
        draw_text(
            &format!("Gen number: {}", self.state.gen_number),
            20.,
            45.,
            30.,
            WHITE,
        );
    }

    fn get_new_generation(&mut self) -> CellMatrix {
        let mut new_gen: CellMatrix = vec![];
        let mut alive_cells_count = 0;

        for (row_idx, row) in self.cells.iter().enumerate() {
            let row = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| {
                    let is_dead = Self::apply_cell_rules(
                        self.get_neighbors_count(row_idx, col_idx),
                        cell.is_dead,
                    );
                    if !is_dead {
                        alive_cells_count += 1;
                    }
                    Cell { is_dead, ..*cell }
                })
                .collect();

            new_gen.push(row);
        }
        self.state.alive_cells_number = alive_cells_count;
        self.state.gen_number += 1;
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
        let game = Game::new(Screen::default(), Some(seeder));
        assert_eq!(game.get_neighbors_count(50, 20), 2);
    }

    #[test]
    fn should_generate_cells_accordingly() {
        // if the cells size is 600, then there should be 1 col and 1 row
        let cells_matrix = Game::generate_cells(&Screen::default(), 600, Some(|_, _| false));
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
        let game = Game::new(Screen::default(), Some(|_, _| false));
        assert_eq!(game.cell_state_to_number(2, 2), 1);
    }

    #[test]
    fn should_map_cell_state_to_0() {
        let game = Game::new(Screen::default(), Some(|_, _| true));
        assert_eq!(game.cell_state_to_number(2, 2), 0);
    }
}
