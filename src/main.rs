use conways_game_of_life::game::get_random_seeder;
use conways_game_of_life::game::Game;
use conways_game_of_life::game::Screen;
use macroquad::window::Conf;

fn conf() -> Conf {
    Game::conf()
}

#[macroquad::main(conf)]
async fn main() {
    let mut game = Game::new(Screen::default(), get_random_seeder(99));
    game.start().await;
}
