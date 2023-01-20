//! Defines data for pieces.

use crate::misc::{Color, Vec2I8};

mod pieces_def;

/// Defines the bool matrix for a piece.
pub type PieceBoolMatrix = [[bool; 4]; 4];

/// Defines the matrix for a given piece state.
#[derive(Copy, Clone)]
pub struct PieceMatrix {
    bits: u16,
    size: u8
}

/// Defines a possible state of a piece.
#[derive(Copy, Clone)]
pub struct PieceState {
    matrix: PieceMatrix,

    // NOTE: If rotating left, need to use NEG of target state instead
    kick_tests: [Vec2I8; 4]
}

/// Defines data needed to represent a piece.
#[derive(Clone)]
pub struct PieceData {
    states: [PieceState; 4],
    color: Color
}

/// The amount of unique pieces that exist.
pub(crate) const PIECE_COUNT: usize = 7;

/// Converts the bits of a [PieceMatrix] to an actual 4x4 bool matrix.
const fn bits_to_matrix(bits: u16) -> PieceBoolMatrix {
    [
        [(bits & 0x1) != 0, (bits & 0x2) != 0, (bits & 0x4) != 0, (bits & 0x8) != 0],
        [(bits & 0x10) != 0, (bits & 0x20) != 0, (bits & 0x40) != 0, (bits & 0x80) != 0],
        [(bits & 0x100) != 0, (bits & 0x200) != 0, (bits & 0x400) != 0, (bits & 0x800) != 0],
        [(bits & 0x1000) != 0, (bits & 0x2000) != 0, (bits & 0x4000) != 0, (bits & 0x8000) != 0],
    ]
}

/// Converts a 4x4 bool matrix to the corresponding bits for a [PieceMatrix]. 
const fn matrix_to_bits(mat: &PieceBoolMatrix) -> u16 {
    const fn get_bit(mat: &PieceBoolMatrix, x: u8, y: u8) -> u16 {
        (mat[x as usize][y as usize] as u16) << (x * 4 + y)
    }

    get_bit(&mat, 0, 0) | get_bit(&mat, 0, 1) | get_bit(&mat, 0, 2) | get_bit(&mat, 0, 3) |
    get_bit(&mat, 1, 0) | get_bit(&mat, 1, 1) | get_bit(&mat, 1, 2) | get_bit(&mat, 1, 3) |
    get_bit(&mat, 2, 0) | get_bit(&mat, 2, 1) | get_bit(&mat, 2, 2) | get_bit(&mat, 2, 3) |
    get_bit(&mat, 3, 0) | get_bit(&mat, 3, 1) | get_bit(&mat, 3, 2) | get_bit(&mat, 3, 3)
}

impl PieceData {
    /// Creates a new piece, based on its default rotational matrix,
    /// the kick tests to perform when rotating, and the color to display it as.
    const fn new(base: PieceMatrix, kick_tests: &[[Vec2I8; 4]; 4], color: Color) -> PieceData {
        let mut states = [PieceState::empty(); 4];
        
        // Macro to deduplicate code from loop-unrolling due to const-ness
        macro_rules! apply_to {
            ($i:literal) => { 
                states[$i].matrix = states[$i - 1].matrix.rotate_right();
                states[$i].kick_tests = kick_tests[$i];
            };
        }

        states[0].matrix = base;
        states[0].kick_tests = kick_tests[0];

        apply_to!(1);
        apply_to!(2);
        apply_to!(3);

        PieceData {
            states,
            color
        }
    }

    /// Creates an array of all possible pieces.
    pub const fn create_all_pieces() -> [PieceData; PIECE_COUNT] {
        pieces_def::create_all_pieces()
    }

    /// Gets the state corresponding the index. Needs to be [0..=3].
    pub fn state(&self, index: usize) -> &PieceState {
        &self.states[index]
    }

    /// Get the array of the 4 possible rotational states.
    pub fn states(&self) -> &[PieceState; 4] {
        &self.states
    }

    /// Gets the color of the piece.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Gets the size of the piece.
    pub fn size(&self) -> u8 {
        self.states[0].matrix.size
    }

    /// Gets the matrix for the default state.
    pub fn default_matrix(&self) -> PieceBoolMatrix {
        self.states[0].matrix()
    }
}

impl Default for PieceData {
    fn default() -> Self {
        Self { 
            states: [PieceState::default(); 4],
            color: Color::BLACK
        }
    }
}

impl PieceMatrix {
    /// Creates an empty matrix, deemed to be of size 2.
    const fn empty() -> Self {
        PieceMatrix {
            bits: 0,
            size: 2
        }
    }

    /// Creates a new size 2 matrix from the given filled blocks.
    const fn new_size2(bits: &[[bool; 2]; 2]) -> Self {
        PieceMatrix {
            bits: matrix_to_bits(&[
                [bits[0][0], bits[0][1], false, false],
                [bits[1][0], bits[1][1], false, false],
                [false; 4],
                [false; 4],
            ]),
            size: 2
        }
    }

    /// Creates a new size 3 matrix from the given filled blocks.
    const fn new_size3(bits: &[[bool; 3]; 3]) -> Self {
        PieceMatrix {
            bits: matrix_to_bits(&[
                [bits[0][0], bits[0][1], bits[0][2], false],
                [bits[1][0], bits[1][1], bits[1][2], false],
                [bits[2][0], bits[2][1], bits[2][2], false],
                [false; 4],
            ]),
            size: 3
        }
    }

    /// Creates a new size 4 matrix from the given filled blocks.
    const fn new_size4(bits: &[[bool; 4]; 4]) -> Self {
        PieceMatrix {
            bits: matrix_to_bits(&bits),
            size: 4
        }
    }

    /// Creates a new matrix that is the same as this one, but
    /// rotated right by 90°.
    const fn rotate_right(&self) -> Self {
        const fn rot2(s: &PieceMatrix) -> PieceMatrix {
            // Technically, this is redundant as size 2 can only be O-blocks
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_size2(&[
                [b[0][1], b[1][1]],
                [b[0][0], b[0][1]],
            ])
        }

        const fn rot3(s: &PieceMatrix) -> PieceMatrix {
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_size3(&[
                [b[0][2], b[1][2], b[2][2]],
                [b[0][1], b[1][1], b[2][1]],
                [b[0][0], b[1][0], b[2][0]],
            ])
        }

        const fn rot4(s: &PieceMatrix) -> PieceMatrix {
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_size4(&[
                [b[0][3], b[1][3], b[2][3], b[3][3]],
                [b[0][2], b[1][2], b[2][2], b[3][2]],
                [b[0][1], b[1][1], b[2][1], b[3][1]],
                [b[0][0], b[1][0], b[2][0], b[3][0]]
            ])
        }

        match self.size {
            2 => rot2(&self),
            3 => rot3(&self),
            4 => rot4(&self),
            _ => *self
        }
    }

    /// Gets the 4x4 bool matrix that represents this piece.
    pub fn matrix(&self) -> PieceBoolMatrix {
        bits_to_matrix(self.bits)
    }
}

impl PieceState {
    /// Creates an empty piece with no useful kick data.
    pub const fn empty() -> Self {
        PieceState {
            matrix: PieceMatrix::empty(),
            kick_tests: [Vec2I8::new(0, 0); 4]
        }
    }

    /// Gets the 4x4 bool matrix that represents this state.
    pub fn matrix(&self) -> PieceBoolMatrix {
        self.matrix.matrix()
    }

    /// Gets the kick tests to check. The `(0, 0)` check is implied.
    pub fn kick_tests(&self) -> &[Vec2I8; 4] {
        &self.kick_tests
    }
}

impl Default for PieceMatrix {
    fn default() -> Self {
        PieceMatrix {
            bits: u16::MAX,
            size: 4
        }
    }
}

impl Default for PieceState {
    fn default() -> Self {
        PieceState {
            matrix: PieceMatrix::default(),
            kick_tests: [Vec2I8::new(0, 0); 4]
        }
    }
}
