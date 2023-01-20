use tetromino_core::{PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT, TRUE_PLAYFIELD_HEIGHT, Game, Color as TtColor};

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::{Transformed, Context, color, rectangle};
use graphics::math::Matrix2d;
use graphics::types::{Color as GlColor, ColorComponent as GlColorComponent};
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::{UpdateEvent, Key, UpdateArgs, RenderArgs, PressEvent, RenderEvent, Button};
use piston::window::WindowSettings;

const EXTRA_LOCK_WAIT: f64 = 0.5;

/// A piston-framework based implementation for the game.
pub struct PistonGame {
    game: Game,
    window: GlutinWindow,
    gl: GlGraphics,
    render_scale: f64,
    difficulty: u8,
    auto_down_left: f64,
    cleared_lines: usize,
    is_game_over: bool,
}

/// Converts a tetromino-core color to a graphics color.
fn tetromino_to_graphics_color(c: TtColor) -> GlColor {
    [
        c.r as GlColorComponent / 255.0,
        c.g as GlColorComponent / 255.0,
        c.b as GlColorComponent / 255.0,
        255.0
    ]
}

impl PistonGame {
    /// Creates a new instance of the game with a specified render scale.
    pub fn new(render_scale: f64) -> PistonGame {
        let opengl_api = OpenGL::V3_2;

        let window = WindowSettings::new("rust-tetromino-game", [render_scale * 26., render_scale * 22.])
            .graphics_api(opengl_api)
            .exit_on_esc(true)
            .build()
            .expect("Failed to create OpenGL window.");

        let gl = GlGraphics::new(opengl_api);

        PistonGame {
            game: Game::new(),
            window,
            gl,
            render_scale,
            difficulty: 1,
            auto_down_left: 2.0,
            cleared_lines: 0,
            is_game_over: false,
        }
    }

    /// Runs the game. This is the only method you will need to call.
    pub fn run(&mut self) {
        let mut event_settings = EventSettings::new();
        event_settings.set_max_fps(60);

        let mut events = Events::new(event_settings);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(ra) = e.render_args() {
                self.render(&ra);
            }

            if !self.is_game_over {
                if let Some(ua) = e.update_args() {
                    self.update(&ua);
                }
    
                if let Some(Button::Keyboard(key)) = e.press_args() {
                    self.on_key_press(&key);
                }
            }
        }
    }

    /// Renders the screen based on the arguments.
    fn render(&mut self, render_args: &RenderArgs) {
        let center = (
            render_args.window_size[0] / (2.0 * self.render_scale),
            render_args.window_size[1] / (2.0 * self.render_scale)
        );

        let top_left = (center.0 - (PLAYFIELD_WIDTH as f64) * 0.5, center.1 - (PLAYFIELD_HEIGHT as f64) * 0.5);
        let render_scale = self.render_scale;
        let active_piece = self.game.active_piece();

        self.gl.draw_begin(render_args.viewport());

        {
            graphics::clear(color::BLACK, &mut self.gl);
            
            let square = rectangle::square(0.0, 0.0, 1.0);
            let c = Context::new_viewport(render_args.viewport());
            let field_trs = c.transform
                .scale(render_scale, render_scale)
                .trans(top_left.0, top_left.1);

            // Render a background
            rectangle(color::grey(0.15), rectangle::rectangle_by_corners(0.0, 0.0, 10.0, 20.0), field_trs, &mut self.gl);

            // Render the playing field
            let playfield = self.game.playfield();
            let full_field_trs = field_trs.trans(0.0, -(PLAYFIELD_HEIGHT as f64));
            for x in 0..PLAYFIELD_WIDTH {
                for y in 0..TRUE_PLAYFIELD_HEIGHT {
                    let tile = playfield.get_tile(x, y);
                    if !tile.is_black() {
                        let color = tetromino_to_graphics_color(tile);
                        let block_trs = full_field_trs.trans(x as f64, y as f64);
                        rectangle(color, square, block_trs, &mut self.gl);
                    }
                }
            }

            // Render the active piece
            draw_piece(
                &mut self.gl,
                field_trs.trans(active_piece.position.x as f64, active_piece.position.y as f64 - 20.0),
                &active_piece.matrix(),
                active_piece.piece_data.color()
            );

            // Render the held piece, if any
            if let Some(held_piece) = self.game.held_piece() {
                draw_piece(
                    &mut self.gl,
                    field_trs.trans(-5.0, 0.0),
                    &held_piece.default_matrix(),
                    held_piece.color()
                );
            }

            // Also draw the list of upcoming pieces
            let next_trs = field_trs.trans(11.0, 0.0).scale(0.5, 0.5);
            for (i, np) in self.game.next_pieces().iter().enumerate() {
                draw_piece(
                    &mut self.gl,
                    next_trs.trans(0.0, (i as f64) * 4.5),
                    &np.default_matrix(),
                    np.color()
                );
            }

            /// Draws a single piece to the screen.
            fn draw_piece(gl: &mut opengl_graphics::GlGraphics, piece_trs: Matrix2d, piece_mtrx: &[[bool; 4]; 4], color: TtColor) {
                let square = rectangle::square(0.0, 0.0, 1.0);
                let color = tetromino_to_graphics_color(color);
                for x in 0..4 {
                    for y in 0..4 {
                        if piece_mtrx[x][y] {
                            let block_trs = piece_trs.trans(x as f64, y as f64);
                            rectangle(color, square, block_trs, gl);
                        }
                    }
                }
            }
        }

        self.gl.draw_end();
    }

    /// Updates the game based on the update step.
    fn update(&mut self, update_args: &UpdateArgs) {
        self.auto_down_left -= update_args.dt;
        
        if self.auto_down_left < 0.0 {
            if !self.game.move_down() {
                if self.auto_down_left < -EXTRA_LOCK_WAIT {
                    self.update_prepare_next_piece();
                }
            } else {
                self.auto_down_left += self.get_auto_down_time();
            }
        }
    }

    /// Prepares dropping the next piece within the update step.
    fn update_prepare_next_piece(&mut self) {
        if let Some(cl) = self.game.finish_piece_turn() {
            self.cleared_lines += cl;
            self.auto_down_left = self.get_auto_down_time();

            let new_diff = 1 + self.cleared_lines / 2;
            self.difficulty = if new_diff <= 9 { new_diff as u8 } else { 9 };
        } else {
            self.is_game_over = true;
        }
    }

    /// Called when a key is pressed. Used for handling input.
    fn on_key_press(&mut self, key: &Key) {
        match key {
            // Move left
            Key::A | Key::Left => {
                self.game.move_left();
            }

            // Move right
            Key::D | Key::Right => {
                self.game.move_right();
            }

            // Move down faster
            Key::S | Key::Down => {
                if self.game.move_down() {
                    self.auto_down_left = self.get_auto_down_time();
                }
            }

            // Rotate-left
            Key::Q => {
                self.game.rotate_left();
            }

            // Rotate-right
            Key::W | Key::Up => {
                self.game.rotate_right();
            }

            // Quick-drop
            Key::Space => {
                self.game.quick_drop();
                self.update_prepare_next_piece();
            }

            // Hold/Swap
            Key::E => {
                self.game.hold_piece();
                self.auto_down_left = self.get_auto_down_time();
            }

            // Don't care about the other keys
            _ => ()
        };
    }

    /// Gets the delay between automatic moves down.
    fn get_auto_down_time(&self) -> f64 {
        2.0 / (self.difficulty as f64 + 0.5)
    }
}
