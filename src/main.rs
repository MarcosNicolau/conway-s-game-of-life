use macroquad::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

struct Cell {
    pos: Pos,
    is_dead: bool,
}

struct Game {
    cells: Vec<Cell>,
    cell_size: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            cell_size: 10,
            cells: vec![],
        }
    }

    pub async fn start(&mut self) {
        self.setup();

        loop {
            clear_background(BLACK);
            next_frame().await;
        }
    }

    fn setup(&mut self) {}
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

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();
    game.start().await;
}
