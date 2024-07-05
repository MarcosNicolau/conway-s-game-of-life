use conways_game_of_life::game::Game;
use macroquad::window::Conf;

fn conf() -> Conf {
    Game::conf()
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new();
    game.start().await;
}
