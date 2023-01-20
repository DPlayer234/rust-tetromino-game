use crate::{Color, Vec2I8};
use crate::pieces::{PieceData, PIECE_DATA_COUNT};

use rand::{rngs::{StdRng}, Rng, SeedableRng};

pub const PLAYFIELD_WIDTH: usize = 10;
pub const PLAYFIELD_HEIGHT: usize = 20;

pub struct Game {
    playfield: Playfield,
    active_piece: ActivePiece,
    next_pieces: Vec<PieceData>,
    held_piece: Option<PieceData>,
    used_hold: bool,
    rng: RandomGenerator,
}

#[derive(Clone)]
pub struct ActivePiece {
    pub piece_data: PieceData,
    pub rotation: usize,
    pub position: Vec2I8
}

pub struct Playfield {
    pub fill_state: [[Color; PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT * 2],
}

pub struct RandomGenerator {
    rng: StdRng,
    pieces: [PieceData; PIECE_DATA_COUNT],
    bag: [usize; PIECE_DATA_COUNT],
    bag_left: usize
}

fn neg_kicks(src: &[Vec2I8; 4]) -> [Vec2I8; 4] {
    [-src[0], -src[1], -src[2], -src[3]]
}

impl Game {
    pub fn new() -> Self {
        let mut slf = Self {
            playfield: Playfield::new(),
            active_piece: ActivePiece::new(PieceData::default(), Vec2I8::new(0, 0)),
            next_pieces: Vec::new(),
            held_piece: None,
            used_hold: false,
            rng: RandomGenerator::new(),
        };

        const NEXT_SIZE: usize = 8;

        let first = slf.rng.next_piece().clone();
        for _ in 0..NEXT_SIZE {
            slf.next_pieces.push(slf.rng.next_piece().clone());
        }

        slf.spawn_new_piece(first);
        slf
    }

    pub fn move_left(&mut self) -> bool {
        self.try_move(|p, _| p.x -= 1)
    }

    pub fn move_right(&mut self) -> bool {
        self.try_move(|p, _| p.x += 1)
    }

    pub fn rotate_left(&mut self) -> bool {
        let cur_rot = self.active_piece.rotation;
        let trg_rot = if cur_rot == 0 { 3 } else { cur_rot - 1 };
        let kicks = neg_kicks(self.active_piece.piece_data.states()[trg_rot].kick_tests());
        self.try_move(|_, r| *r = trg_rot) || self.try_move_kicks(trg_rot, &kicks)
    }

    pub fn rotate_right(&mut self) -> bool {
        let cur_rot = self.active_piece.rotation;
        let trg_rot = if cur_rot == 3 { 0 } else { cur_rot + 1 };
        let kicks = *self.active_piece.piece_data.states()[cur_rot].kick_tests();
        self.try_move(|_, r| *r = trg_rot) || self.try_move_kicks(trg_rot, &kicks)
    }

    pub fn move_down(&mut self) -> bool {
        self.try_move(|p, _| p.y += 1)
    }

    pub fn quick_drop(&mut self) {
        while self.move_down() {}
    }

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
            let next_piece = self.next_pieces.remove(0);
            self.next_pieces.push(self.rng.next_piece().clone());
            self.spawn_new_piece(next_piece);
        }

        self.used_hold = true;
        self.held_piece = to_hold;
        true
    }

    pub fn lock_down_piece(&mut self) -> Option<usize> {
        self.playfield.copy_in_piece(&self.active_piece);

        // Place the next piece in
        let next_piece = self.next_pieces.remove(0);
        self.next_pieces.push(self.rng.next_piece().clone());
        self.used_hold = false;

        let cleared = self.clear_completed_lines();
        if self.spawn_new_piece(next_piece) {
            Some(cleared)
        } else {
            None
        }
    }

    pub fn clear_completed_lines(&mut self) -> usize {
        self.playfield.clear_completed_lines()
    }

    pub fn playfield(&self) -> &Playfield {
        &self.playfield
    }

    pub fn active_piece(&self) -> &ActivePiece {
        &self.active_piece
    }

    pub fn next_pieces(&self) -> &Vec<PieceData> {
        &self.next_pieces
    }

    pub fn held_piece(&self) -> Option<&PieceData> {
        self.held_piece.as_ref()
    }

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

    fn try_move_kicks(&mut self, trg_rot: usize, kick_tests: &[Vec2I8; 4]) -> bool {
        for &t in kick_tests.iter() {
            if self.try_move(|p, r| {
                *r = trg_rot;
                *p += t;
            }) {
                return true;
            }
        }

        false
    }
}

impl ActivePiece {
    pub fn new(piece_data: PieceData, spawn_pos: Vec2I8) -> ActivePiece {
        ActivePiece {
            piece_data,
            position: spawn_pos,
            rotation: 0
        }
    }

    pub fn get_matrix(&self) -> [[bool; 4]; 4] {
        self.piece_data.states()[self.rotation].get_matrix()
    }
}

impl Playfield {
    pub fn new() -> Playfield {
        Playfield {
            fill_state: [[Color::black(); PLAYFIELD_WIDTH]; PLAYFIELD_HEIGHT * 2]
        }
    }

    pub fn has_overlap(&self, piece: &ActivePiece) -> bool {
        let mat = piece.get_matrix();
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

    pub fn get_tile(&self, x: usize, y: usize) -> Color {
        if Playfield::is_in_bounds(x, y) {
            self.fill_state[y][x]
        } else {
            Color::white()
        }
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        if Playfield::is_in_bounds(x, y) {
            Some(&mut self.fill_state[y][x])
        } else {
            None
        }
    }

    pub fn has_tile(&self, x: usize, y: usize) -> bool {
        !self.get_tile(x, y).is_black()
    }

    pub fn is_in_bounds(x: usize, y: usize) -> bool {
        x < PLAYFIELD_WIDTH && y < (PLAYFIELD_HEIGHT * 2)
    }

    pub fn copy_in_piece(&mut self, piece: &ActivePiece) {
        let mat = piece.get_matrix();
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

    pub fn clear_completed_lines(&mut self) -> usize {
        let mut cnt = 0;

        // Move UP in index
        for y in 0..(PLAYFIELD_HEIGHT * 2) {
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

                self.fill_state[0] = [Color::black(); PLAYFIELD_WIDTH];
            }
        }

        cnt
    }
}

impl RandomGenerator {
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            rng: StdRng::from_entropy(),
            pieces: PieceData::make_all_pieces(),
            bag: [0; 7],
            bag_left: 0
        }
    }

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
