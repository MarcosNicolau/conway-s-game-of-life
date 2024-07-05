use ::rand::prelude::*;
use macroquad::prelude::*;
use miniquad::window::set_window_size;

// Structs definitions here

#[derive(Copy, Clone, PartialEq, Debug)]
struct Pos {
    x: f32,
    y: f32,
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

pub struct Game {
    screen: Screen,
    cells: CellMatrix,
    cell_size: f32,
}

pub type Seeder = fn(row_idx: i32, col_idx: i32) -> bool;

pub fn get_random_seeder(percentage: u32) -> Box<dyn Fn(i32, i32) -> bool> {
    Box::new(move |_, _| thread_rng().gen_range(0..101) <= percentage)
}

impl Game {
    pub fn new(screen: Screen, seeder: impl Fn(i32, i32) -> bool) -> Self {
        let cell_size = 5.0;

        Self {
            cell_size,
            cells: Game::generate_cells(&screen, cell_size, seeder),
            screen,
        }
    }

    fn generate_cells(
        screen: &Screen,
        cell_size: f32,
        seeder: impl Fn(i32, i32) -> bool,
    ) -> CellMatrix {
        let mut matrix: CellMatrix = vec![];
        let num_of_rows: i32 = (screen.height as f32 / cell_size) as i32;
        let num_of_cols = (screen.width as f32 / cell_size) as i32;
        (0..num_of_rows).into_iter().for_each(|row_idx| {
            let cells: Vec<Cell> = (0..num_of_cols)
                .into_iter()
                .map(|col_idx| Cell {
                    pos: Pos {
                        x: col_idx as f32 * cell_size,
                        y: row_idx as f32 * cell_size,
                    },

                    is_dead: seeder(row_idx, col_idx),
                })
                .collect();
            matrix.push(cells);
        });

        return matrix;
    }

    pub async fn start(&mut self) {
        self.setup();
        loop {
            clear_background(BLACK);
            self.draw_cells();
            self.cells = self.get_new_generation();
            next_frame().await;
        }
    }

    fn draw_cells(&self) {
        for cells in self.cells.iter() {
            cells.iter().for_each(|cell| {
                draw_rectangle(
                    cell.pos.x,
                    cell.pos.y,
                    self.cell_size,
                    self.cell_size,
                    if cell.is_dead { BLACK } else { WHITE },
                )
            });
        }
    }

    fn get_new_generation(&self) -> CellMatrix {
        self.cells
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, col)| {
                        let count = self.get_neighbors_count(row_idx, col_idx);
                        let is_dead = match count {
                            3 => false,
                            2..=3 => false,
                            _ => true,
                        };
                        Cell { is_dead, ..*col }
                    })
                    .collect()
            })
            .collect()
    }

    fn get_neighbors_count(&self, row_idx: usize, col_idx: usize) -> i32 {
        let cells: &CellMatrix = &self.cells;
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
        let cells_matrix = Game::generate_cells(&Screen::default(), 600.0, |_, _| false);
        assert_eq!(
            cells_matrix,
            vec![vec![Cell {
                pos: Pos { x: 0.0, y: 0.0 },
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
