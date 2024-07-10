use crate::cell::*;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use std::process::exit;
use std::{ops::Range, thread::sleep, time::Duration};

#[derive(Clone, PartialEq, Debug)]
struct GameState {
    is_paused: bool,
    game_has_started: bool,
    speed_in_ms: f32,
    gen_number: i32,
    alive_cells_number: i32,
    alive_percentage_factor: f32,
}

pub struct Game {
    cells: CellMatrix,
    cell_size: u32,
    state: GameState,
}

impl Game {
    pub fn new(seeder: Option<Seeder>) -> Self {
        let cell_size = 10;

        let mut game = Self {
            cell_size,
            cells: vec![],
            state: GameState {
                game_has_started: false,
                alive_cells_number: 0,
                gen_number: 0,
                speed_in_ms: 10.,
                is_paused: true,
                alive_percentage_factor: 80.,
            },
        };

        game.generate_cells(seeder);

        game
    }

    fn generate_cells(&mut self, seeder: Option<Seeder>) {
        let mut matrix: CellMatrix = vec![];
        let num_of_rows = screen_height() as u32 / self.cell_size;
        let num_of_cols = screen_width() as u32 / self.cell_size;

        for row in 0..num_of_rows {
            let cells: Vec<Cell> = (0..num_of_cols)
                .map(|col_idx| match &seeder {
                    Some(fun) => Cell::create(fun(row, col_idx)),
                    _ => Cell::create(true),
                })
                .collect();
            matrix.push(cells);
        }

        self.cells = matrix;
    }

    pub async fn start(&mut self) {
        loop {
            clear_background(BLACK);

            if !self.state.game_has_started {
                self.draw_paint_cells();
            }

            self.draw_config_bar();
            self.draw_grid();
            self.draw_cells();
            self.draw_gen_state();

            if !self.state.is_paused {
                self.cells = self.get_new_generation();
                sleep(Duration::from_millis(self.state.speed_in_ms as u64));
            }

            next_frame().await;
        }
    }

    fn restart(&mut self, seeder: Option<Seeder>) {
        self.state.game_has_started = false;
        self.state.alive_cells_number = 0;
        self.state.is_paused = true;
        self.state.gen_number = 0;
        self.generate_cells(seeder);
    }

    fn draw_paint_cells(&mut self) {
        self.draw_text_centered("Paint the cells", 0., 0., 50.0);
        self.draw_text_centered("and press start", 0., 20., 30.0);

        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let mouse_pos = mouse_position();

        let cell_x = (mouse_pos.0 / self.cell_size as f32) as usize;
        let cell_y = (mouse_pos.1 / self.cell_size as f32) as usize;

        if let Some(cell) = self
            .cells
            .get_mut(cell_y)
            .and_then(|row| row.get_mut(cell_x))
        {
            cell.swap_state();
        }
    }

    fn draw_cells(&self) {
        for (row_idx, cells) in self.cells.iter().enumerate() {
            cells
                .iter()
                .enumerate()
                .filter(|(_, cell)| cell.is_dead())
                .for_each(|(col_idx, _)| {
                    draw_rectangle(
                        (col_idx as u32 * self.cell_size) as f32,
                        (row_idx as u32 * self.cell_size) as f32,
                        self.cell_size as f32,
                        self.cell_size as f32,
                        WHITE,
                    )
                });
        }
    }

    fn draw_grid(&self) {
        let num_of_rows = screen_height() as u32 / self.cell_size;
        let num_of_cols = screen_width() as u32 / self.cell_size;

        for idx in 0..=num_of_rows {
            let y = (idx * self.cell_size) as f32;
            draw_line(0.0, y, screen_width(), y, 1.0, GRAY);
        }

        for idx in 0..=num_of_cols {
            let x = (idx * self.cell_size) as f32;
            draw_line(x, 0.0, x, screen_height(), 1.0, GRAY);
        }
    }

    fn draw_config_bar(&mut self) {
        widgets::Window::new(100, vec2(screen_width() + 140., 0.), vec2(500., 180.))
            .movable(true)
            .label("Config")
            .ui(&mut root_ui(), |ui| {
                let range: Range<f32> = 0.0..100.0;
                ui.slider(1, "speed in ms", range, &mut self.state.speed_in_ms);

                if ui.button(
                    vec2(0., 50.),
                    if self.state.is_paused {
                        "Start"
                    } else {
                        "Pause"
                    },
                ) {
                    self.state.is_paused = !self.state.is_paused;
                    self.state.game_has_started = true;
                }

                if !self.state.game_has_started {
                    let range: Range<f32> = 0.0..100.0;
                    ui.slider(
                        3,
                        "random factor",
                        range,
                        &mut self.state.alive_percentage_factor,
                    );
                    if ui.button(vec2(0., 80.), "Randomize") {
                        self.restart(Some(get_random_seeder(
                            self.state.alive_percentage_factor as u32,
                        )));
                    }
                }

                if ui.button(vec2(0., 110.), "Restart") {
                    self.restart(None);
                }

                if ui.button(vec2(0., 140.), "Exit ") {
                    exit(0);
                }
            });
    }

    fn draw_gen_state(&self) {
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

    fn draw_text_centered(&self, text: &str, offset_x: f32, offset_y: f32, font_size: f32) {
        let text_width = measure_text(text, None, font_size as u16, 1.).width;
        draw_text(
            text,
            screen_width() / 2.0 - text_width / 2.0 + offset_x,
            screen_height() / 2.0 - font_size / 2.0 + offset_y,
            font_size,
            WHITE,
        );
    }

    fn get_new_generation(&mut self) -> CellMatrix {
        let mut new_gen: CellMatrix = vec![];
        let mut alive_cells_count = 0;

        for (row_idx, row) in self.cells.iter().enumerate() {
            let new_row: Vec<Cell> = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| {
                    let is_dead = apply_cell_rules(
                        self.get_neighbors_count(row_idx, col_idx),
                        cell.is_dead(),
                    );
                    if !is_dead {
                        alive_cells_count += 1;
                    }
                    Cell::create(is_dead)
                })
                .collect();

            new_gen.push(new_row);
        }

        self.state.alive_cells_number = alive_cells_count;
        self.state.gen_number += 1;

        new_gen
    }

    fn get_neighbors_count(&self, row_idx: usize, col_idx: usize) -> i32 {
        let mut count = 0;
        let cells = &self.cells;

        let start_row = if row_idx == 0 { 0 } else { row_idx - 1 };
        let end_row = (row_idx + 2).min(cells.len());

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
            .and_then(|cells| cells.get(col_idx))
            .map_or(0, |cell| if cell.is_dead() { 0 } else { 1 })
    }
}
