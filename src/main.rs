mod scrunt;

use scrunt::Game;

fn main() -> Result<(), serde_json::Error> {
    let mut game = Game::new("DEFAULT");
    game.add_random();

    let (x, y) = game.get_sentence();
    println!("{}, {}", x, y);

    Ok(())
}
