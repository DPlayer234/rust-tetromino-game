//! Defines the core game logic.

use crate::misc::{Color, Vec2I8};
use crate::pieces::{PieceData, PieceBoolMatrix, PIECE_COUNT};

use std::collections::VecDeque;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// The width of the playfield in blocks.
pub const PLAYFIELD_WIDTH: usize = 10;

/// The height of the playfield in blocks.
/// The true height is double this value and only the "high" half is visible.
pub const PLAYFIELD_HEIGHT: usize = 20;

/// The "true" possible height of the playfield in blocks including the non-visible parts.
pub const TRUE_PLAYFIELD_HEIGHT: usize = PLAYFIELD_HEIGHT * 2;

/// Represents an active tetromino game.
pub struct Game {
    playfield: Playfield,
    active_piece: ActivePiece,
    next_pieces: VecDeque<PieceData>,
    held_piece: Option<PieceData>,
    used_hold: bool,
    rng: RandomGenerator,
}

/// Represents an active, falling piece in the game.
///
/// This is mostly a transparent struct and its methods are only helpers.
#[derive(Clone)]
pub struct ActivePiece {
    pub piece_data: PieceData,
    pub rotation: usize,
    pub position: Vec2I8
}

/// Represents an active playfield.
pub struct Playfield {
    fill_state: [[Color; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT * 2],
}

/// A random generator supplying the game with pieces.
///
/// It starts by filling a bag with all known pieces in random order, then draining that bag in order.
/// Once the bag is empty, it restarts.
pub struct RandomGenerator {
    rng: StdRng,
    pieces: [PieceData; PIECE_COUNT],
    bag: [usize; PIECE_COUNT],
    bag_left: usize
}

/// Helper function to negate kick test values.
fn neg_kicks(src: &[Vec2I8; 4]) -> [Vec2I8; 4] {
    [-src[0], -src[1], -src[2], -src[3]]
}

impl Game {
    /// Creates a new empty game state.
    /// An active piece has already been placed on the field.
    pub fn new() -> Self {
        let mut slf = Self {
            playfield: Playfield::new(),
            active_piece: ActivePiece::new(PieceData::default(), Vec2I8::new(0, 0)),
            next_pieces: VecDeque::new(),
            held_piece: None,
            used_hold: false,
            rng: RandomGenerator::new(),
        };

        const NEXT_SIZE: usize = 8;

        let first = slf.rng.next_piece().clone();
        for _ in 0..NEXT_SIZE {
            slf.next_pieces.push_back(slf.rng.next_piece().clone());
        }

        slf.spawn_new_piece(first);
        slf
    }

    /// Tries to move the active piece left.
    ///
    /// Returns whether it succeeded.
    pub fn move_left(&mut self) -> bool {
        self.try_move(|p, _| p.x -= 1)
    }

    /// Tries to move the active piece right.
    ///
    /// Returns whether it succeeded.
    pub fn move_right(&mut self) -> bool {
        self.try_move(|p, _| p.x += 1)
    }

    /// Tries to rotate the piece left.
    ///
    /// This attempts to make use of the SRS kick tests.
    /// Returns whether any rotation succeeded.
    pub fn rotate_left(&mut self) -> bool {
        let cur_rot = self.active_piece.rotation;
        let trg_rot = if cur_rot == 0 { 3 } else { cur_rot - 1 };
        let kicks = neg_kicks(self.active_piece.piece_data.state(trg_rot).kick_tests());
        self.try_move(|_, r| *r = trg_rot) || self.try_move_kicks(trg_rot, &kicks)
    }

    /// Tries to rotate the piece right.
    ///
    /// This attempts to make use of the SRS kick tests.
    /// Returns whether any rotation succeeded.
    pub fn rotate_right(&mut self) -> bool {
        let cur_rot = self.active_piece.rotation;
        let trg_rot = if cur_rot == 3 { 0 } else { cur_rot + 1 };
        let kicks = *self.active_piece.piece_data.state(cur_rot).kick_tests();
        self.try_move(|_, r| *r = trg_rot) || self.try_move_kicks(trg_rot, &kicks)
    }

    /// Tries to move the piece down.
    ///
    /// Returns whether it succeeded.
    /// If it fails, this indicates the piece has hit the bottom.
    pub fn move_down(&mut self) -> bool {
        self.try_move(|p, _| p.y += 1)
    }

    /// Drops the piece to the bottom in a single move.
    pub fn quick_drop(&mut self) {
        while self.move_down() {}
    }

    /// Hold the currently active piece in the "hold" slot and swap in the held piece if there was one.
    /// If no piece was held yet, puts in a new piece from the sequence.
    ///
    /// Returns whether it was successful. This fails if it had been used already without placing a piece down.
    pub fn hold_piece(&mut self) -> bool {
        if self.used_hold {
            return false;
        }

        // Copy out the current piece
        let to_hold = Some(self.active_piece.piece_data.clone());

        if let Some(held) = self.held_piece.take() {
            // If we already held a piece, swap it in
            self.spawn_new_piece(held);
        } else {
            // Otherwise take one from the queue
            let next_piece = self.pop_next_piece();
            self.spawn_new_piece(next_piece);
        }

        self.used_hold = true;
        self.held_piece = to_hold;
        true
    }

    /// Locks down the piece by copying it into the playfield and spawning a new one.
    /// Additionally, full lines are cleared.
    ///
    /// If returning [`Some`], its value indicates the amount of cleared lines.
    /// If returning [`None`], putting in the new piece failed, and the game is over.
    pub fn finish_piece_turn(&mut self) -> Option<usize> {
        self.lock_down_piece();

        // Place the next piece in
        let next_piece = self.pop_next_piece();
        self.used_hold = false;

        let cleared = self.clear_completed_lines();
        if self.spawn_new_piece(next_piece) {
            Some(cleared)
        } else {
            None
        }
    }

    /// Copies the active piece into the playfield with no further actions.
    ///
    /// This is done automatically by [`Game::finish_piece_turn()`].
    pub fn lock_down_piece(&mut self) {
        self.playfield.copy_in_piece(&self.active_piece)
    }

    /// Clears all completed lines and returns how many were cleared.
    ///
    /// This is done automatically by [`Game::finish_piece_turn()`].
    pub fn clear_completed_lines(&mut self) -> usize {
        self.playfield.clear_completed_lines()
    }

    /// Gets the playfield.
    pub fn playfield(&self) -> &Playfield {
        &self.playfield
    }

    /// Gets the active piece.
    pub fn active_piece(&self) -> &ActivePiece {
        &self.active_piece
    }

    /// Gets the queue of upcoming pieces.
    pub fn next_pieces(&self) -> &VecDeque<PieceData> {
        &self.next_pieces
    }

    /// Gets the held piece.
    pub fn held_piece(&self) -> Option<&PieceData> {
        self.held_piece.as_ref()
    }

    /// Pops the next piece of the upcoming pieces.
    fn pop_next_piece(&mut self) -> PieceData {
        let next_piece = self.next_pieces.pop_front().expect("next_pieces queue cannot be empty");
        self.next_pieces.push_back(self.rng.next_piece().clone());
        next_piece
    }

    /// Spawns a new active piece onto the field, replacing the old one.
    fn spawn_new_piece(&mut self, new_piece: PieceData) -> bool {
        // Pick a central position above the playfield
        let new_piece_size = new_piece.size();
        let spawn_pos = match new_piece_size {
            2 => Vec2I8::new(4, (PLAYFIELD_HEIGHT - 1) as i8),
            3 | 4 => Vec2I8::new(3, (PLAYFIELD_HEIGHT - 1) as i8),
            _ => panic!("Invalid piece size")
        };

        self.active_piece = ActivePiece::new(new_piece, spawn_pos);

        if new_piece_size < 4 {
            // If not I piece (only 4-size), try to move down 1 tile
            self.move_down();
        }

        !self.playfield.has_overlap(&self.active_piece)
    }

    /// Attempts to perform a movement action through the specified function.
    fn try_move(&mut self, change: impl FnOnce(&mut Vec2I8, &mut usize)) -> bool {
        // Keep a backup in case moving fails
        let old_pos = self.active_piece.position;
        let old_rot = self.active_piece.rotation;

        // Apply the changes from the fn to the active state
        change(&mut self.active_piece.position, &mut self.active_piece.rotation);

        // If it overlaps, we need to reset.
        if self.playfield.has_overlap(&self.active_piece) {
            self.active_piece.position = old_pos;
            self.active_piece.rotation = old_rot;
            false
        } else {
            true
        }
    }

    /// Attempts all SRS kick options until one succeeds or all were tried.
    fn try_move_kicks(&mut self, trg_rot: usize, kick_tests: &[Vec2I8; 4]) -> bool {
        for &t in kick_tests.iter() {
            let c = |p: &mut Vec2I8, r: &mut usize| {
                *r = trg_rot;
                *p += t;
            };

            if self.try_move(c) {
                return true;
            }
        }

        false
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl ActivePiece {
    /// Creates a new active piece with the given piece data and a spawn position.
    pub fn new(piece_data: PieceData, spawn_pos: Vec2I8) -> ActivePiece {
        ActivePiece {
            piece_data,
            position: spawn_pos,
            rotation: 0
        }
    }

    /// Gets the matrix that is currently in use based on its rotation.
    pub fn matrix(&self) -> PieceBoolMatrix {
        self.piece_data.state(self.rotation).matrix()
    }
}

impl Playfield {
    /// Creates a new empty playfield. This means all its tiles are black.
    pub fn new() -> Playfield {
        Playfield {
            fill_state: [[Color::BLACK; PLAYFIELD_WIDTH]; TRUE_PLAYFIELD_HEIGHT]
        }
    }

    /// Determines whether the filled playfield tiles overlap with the active piece.
    pub fn has_overlap(&self, piece: &ActivePiece) -> bool {
        let mat = piece.matrix();
        let x_base = piece.position.x as usize;
        let y_base = piece.position.y as usize;

        // mat dimensions are 4x4
        for x in 0..4 {
            for y in 0..4 {
                // Use wrapping_add to avoid overflow (negative numbers cast to unsigned)
                if mat[x][y] && self.has_tile(x.wrapping_add(x_base), y.wrapping_add(y_base)) {
                    return true;
                }
            }
        }

        false
    }

    /// Gets the color of a tile. If not in range, it is [`Color::WHITE`].
    pub fn get_tile(&self, x: usize, y: usize) -> Color {
        if Playfield::is_in_bounds(x, y) {
            self.fill_state[y][x]
        } else {
            Color::WHITE
        }
    }

    /// Gets a mutable reference to the color of a tile.
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        if Playfield::is_in_bounds(x, y) {
            Some(&mut self.fill_state[y][x])
        } else {
            None
        }
    }

    /// Determines if a tile is set. That is to say, it is not black and in range.
    pub fn has_tile(&self, x: usize, y: usize) -> bool {
        !self.get_tile(x, y).is_black()
    }

    /// Determines if a specified coordinate is in bpunds.
    pub fn is_in_bounds(x: usize, y: usize) -> bool {
        x < PLAYFIELD_WIDTH && y < TRUE_PLAYFIELD_HEIGHT
    }

    /// Copies an active piece into the playfield matrix.
    pub fn copy_in_piece(&mut self, piece: &ActivePiece) {
        let mat = piece.matrix();
        let x_base = piece.position.x as usize;
        let y_base = piece.position.y as usize;

        // mat dimensions are 4x4
        for x in 0..4 {
            for y in 0..4 {
                // If the matrix has a tile, override the playfield's tile's color
                if mat[x][y] {
                    // Use wrapping_add to avoid overflow (negative numbers cast to unsigned)
                    if let Some(m) = self.get_tile_mut(x.wrapping_add(x_base), y.wrapping_add(y_base)) {
                        *m = piece.piece_data.color();
                    }
                }
            }
        }
    }

    /// Clears all completed lines, returning the amount of lines that were cleared.
    pub fn clear_completed_lines(&mut self) -> usize {
        let mut cnt = 0;

        // Move UP in index
        for y in 0..TRUE_PLAYFIELD_HEIGHT {
            let mut row_full = true;
            for x in 0..PLAYFIELD_WIDTH {
                if !self.has_tile(x, y) {
                    row_full = false;
                    break;
                }
            }

            if row_full {
                cnt += 1;

                for yc in (1..=y).rev() {
                    self.fill_state[yc] = self.fill_state[yc - 1];
                }

                self.fill_state[0] = [Color::BLACK; PLAYFIELD_WIDTH];
            }
        }

        cnt
    }
}

impl Default for Playfield {
    fn default() -> Self {
        Playfield::new()
    }
}

impl RandomGenerator {
    /// Creates a new random generator with an empty bag.
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            rng: StdRng::from_entropy(),
            pieces: PieceData::create_all_pieces(),
            bag: [0; 7],
            bag_left: 0
        }
    }

    /// Gets the next piece from the bag. The bag is automatically refilled when needed.
    pub fn next_piece(&mut self) -> &PieceData {
        if self.bag_left > 0 {
            self.bag_left -= 1;
            &self.pieces[self.bag[self.bag_left]]
        } else {
            let mut new_bag = vec![0, 1, 2, 3, 4, 5, 6];
            for i in 0..new_bag.len() {
                self.bag[i] = new_bag.remove(self.rng.gen_range(0..new_bag.len()));
            }

            self.bag_left = 6;
            &self.pieces[self.bag[6]]
        }
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        RandomGenerator::new()
    }
}
