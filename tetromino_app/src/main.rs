// Disable the console window
#![windows_subsystem = "windows"]

use tetromino_piston::PistonGame;

fn main() {
    let mut g = PistonGame::new(32.0);
    g.run();
}
