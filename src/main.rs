pub mod board;
pub mod constraints;
pub mod cursor;
pub mod game;
pub mod parsing;

use std::env;
use std::path::Path;

use game::Game;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Missing Argument: Path to puzzle file".into());
    } else if args.len() > 2 {
        return Err("Too many arguments".into());
    }

    let mut game = Game::from_file(Path::new(&args[1]))?;

    game.init()?;
    game.run()?;
    game.quit()?;

    Ok(())
}
