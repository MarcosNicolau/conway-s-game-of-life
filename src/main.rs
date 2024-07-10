use conways_game_of_life::game::Game;
use macroquad::window::Conf;

pub fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new(None);
    game.start().await;
}
