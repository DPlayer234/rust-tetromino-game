use crate::{Color, Vec2I8};

pub type PieceBoolMatrix = [[bool; 4]; 4];

#[derive(Copy, Clone)]
pub struct PieceMatrix {
    bits: u16,
    size: u8
}

#[derive(Copy, Clone)]
pub struct PieceState {
    matrix: PieceMatrix,

    // NOTE: If rotating left, need to use NEG of target state instead
    kick_tests: [Vec2I8; 4]
}

#[derive(Clone)]
pub struct PieceData {
    states: [PieceState; 4],
    color: Color
}

pub(crate) const PIECE_DATA_COUNT: usize = 7;

const fn bits_to_matrix(bits: u16) -> PieceBoolMatrix {
    [
        [(bits & 0x1) != 0, (bits & 0x2) != 0, (bits & 0x4) != 0, (bits & 0x8) != 0],
        [(bits & 0x10) != 0, (bits & 0x20) != 0, (bits & 0x40) != 0, (bits & 0x80) != 0],
        [(bits & 0x100) != 0, (bits & 0x200) != 0, (bits & 0x400) != 0, (bits & 0x800) != 0],
        [(bits & 0x1000) != 0, (bits & 0x2000) != 0, (bits & 0x4000) != 0, (bits & 0x8000) != 0],
    ]
}

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
    fn new(base: PieceMatrix, kick_tests: &[[Vec2I8; 4]; 4], color: Color) -> PieceData {
        let mut states = [PieceState::default(); 4];
        
        states[0].matrix = base;
        states[0].kick_tests = kick_tests[0];

        for i in 1..4 {
            states[i].matrix = states[i - 1].matrix.rotate_right();
            states[i].kick_tests = kick_tests[i];
        }

        PieceData {
            states,
            color
        }
    }

    pub fn make_all_pieces() -> [PieceData; PIECE_DATA_COUNT] {
        let jlstz_kick_tests = [
            [
                Vec2I8::new(-1, 0),
                Vec2I8::new(-1, -1),
                Vec2I8::new(0, 2),
                Vec2I8::new(-1, 2)
            ],
            [
                Vec2I8::new(1, 0),
                Vec2I8::new(1, 1),
                Vec2I8::new(0, -2),
                Vec2I8::new(1, -2)
            ],
            [
                Vec2I8::new(1, 0),
                Vec2I8::new(1, -1),
                Vec2I8::new(0, 2),
                Vec2I8::new(1, 2)
            ],
            [
                Vec2I8::new(-1, 0),
                Vec2I8::new(-1, 1),
                Vec2I8::new(0, -2),
                Vec2I8::new(-1, 2)
            ]
        ];

        let i_kick_tests = [
            [
                Vec2I8::new(-2, 0),
                Vec2I8::new(1, 0),
                Vec2I8::new(-2, 1),
                Vec2I8::new(1, -2)
            ],
            [
                Vec2I8::new(-1, 0),
                Vec2I8::new(2, 0),
                Vec2I8::new(-1, -2),
                Vec2I8::new(2, 1)
            ],
            [
                Vec2I8::new(2, 0),
                Vec2I8::new(-1, 0),
                Vec2I8::new(2, -1),
                Vec2I8::new(-1, 2)
            ],
            [
                Vec2I8::new(1, 0),
                Vec2I8::new(-2, 0),
                Vec2I8::new(1, 1),
                Vec2I8::new(-2, -1)
            ]
        ];

        [
            // I-Piece
            PieceData::new(
                PieceMatrix::new_4(&[[false, true, false, false]; 4]),
                &i_kick_tests,
                Color::new(0x00, 0xf0, 0xf0)
            ),
            // J-Piece
            PieceData::new(
                PieceMatrix::new_3(&[
                    [true, true, false],
                    [false, true, false],
                    [false, true, false]
                ]),
                &jlstz_kick_tests,
                Color::new(0x00, 0x00, 0xf0)
            ),
            // L-Piece
            PieceData::new(
                PieceMatrix::new_3(&[
                    [false, true, false],
                    [false, true, false],
                    [true, true, false]
                ]),
                &jlstz_kick_tests,
                Color::new(0xf0, 0xa0, 0x00)
            ),
            // O-Piece
            PieceData::new(
                PieceMatrix::new_2(&[[true; 2]; 2]),
                &[[Vec2I8::new(0, 0); 4]; 4],
                Color::new(0xf0, 0xf0, 0x00)
            ),
            // S-Piece
            PieceData::new(
                PieceMatrix::new_3(&[
                    [false, true, false],
                    [true, true, false],
                    [true, false, false]
                ]),
                &jlstz_kick_tests,
                Color::new(0x00, 0xf0, 0x00)
            ),
            // T-Piece
            PieceData::new(
                PieceMatrix::new_3(&[
                    [false, true, false],
                    [true, true, false],
                    [false, true, false]
                ]),
                &jlstz_kick_tests,
                Color::new(0xa0, 0x00, 0xf0)
            ),
            // Z-Piece
            PieceData::new(
                PieceMatrix::new_3(&[
                    [true, false, false],
                    [true, true, false],
                    [false, true, false]
                ]),
                &jlstz_kick_tests,
                Color::new(0xf0, 0x00, 0x00)
            )
        ]
    }

    pub fn states(&self) -> &[PieceState; 4] {
        &self.states
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn size(&self) -> u8 {
        self.states[0].matrix.size
    }
}

impl Default for PieceData {
    fn default() -> Self {
        Self { 
            states: [PieceState::default(); 4],
            color: Color::black()
        }
    }
}

impl PieceMatrix {
    fn new_2(bits: &[[bool; 2]; 2]) -> Self {
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

    fn new_3(bits: &[[bool; 3]; 3]) -> Self {
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

    fn new_4(bits: &[[bool; 4]; 4]) -> Self {
        PieceMatrix {
            bits: matrix_to_bits(&bits),
            size: 4
        }
    }

    fn rotate_right(&self) -> Self {
        fn rot2(s: &PieceMatrix) -> PieceMatrix {
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_2(&[
                [b[0][1], b[1][1]],
                [b[0][0], b[0][1]],
            ])
        }

        fn rot3(s: &PieceMatrix) -> PieceMatrix {
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_3(&[
                [b[0][2], b[1][2], b[2][2]],
                [b[0][1], b[1][1], b[2][1]],
                [b[0][0], b[1][0], b[2][0]],
            ])
        }

        fn rot4(s: &PieceMatrix) -> PieceMatrix {
            let b = bits_to_matrix(s.bits);
            PieceMatrix::new_4(&[
                [b[0][3], b[1][3], b[2][3], b[3][3]],
                [b[0][2], b[1][2], b[2][2], b[3][2]],
                [b[0][1], b[1][1], b[2][1], b[3][1]],
                [b[0][0], b[1][0], b[2][0], b[3][0]]
            ])
        }

        match self.size {
            // 2 size can only be O blocks.
            2 => rot2(&self),
            3 => rot3(&self),
            4 => rot4(&self),
            _ => panic!("Invalid block size.")
        }
    }

    pub fn get_matrix(&self) -> PieceBoolMatrix {
        bits_to_matrix(self.bits)
    }
}

impl PieceState {
    pub fn get_matrix(&self) -> [[bool; 4]; 4] {
        self.matrix.get_matrix()
    }

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
