use genedeck::game::Game;

#[allow(dead_code)]

fn main() {
    let mut game = Game::new();
    game.run(std::io::stdin(), std::io::stdout());
}
