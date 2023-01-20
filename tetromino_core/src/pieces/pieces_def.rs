//! Internal module used to generate the possible pieces.

use crate::misc::{Color, Vec2I8};
use super::{PIECE_COUNT, PieceData, PieceMatrix};

/// Creates the kick tests for the J, L, S, T, and Z pieces.
const fn create_jlstz_kick_tests() -> [[Vec2I8; 4]; 4] {
    [
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
    ]
}

/// Creates the kick tests for the I piece.
const fn create_i_kick_tests() -> [[Vec2I8; 4]; 4] {
    [
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
    ]
}

/// Creates all the canonical pieces with fixed colors and SRS kick tests.
pub(crate) const fn create_all_pieces() -> [PieceData; PIECE_COUNT] {
    let jlstz_kick_tests = create_jlstz_kick_tests();
    let i_kick_tests = create_i_kick_tests();
    // The O-piece does not kick.

    [
        // I-Piece
        PieceData::new(
            PieceMatrix::new_size4(&[[false, true, false, false]; 4]),
            &i_kick_tests,
            Color::new(0x00, 0xf0, 0xf0)
        ),
        // J-Piece
        PieceData::new(
            PieceMatrix::new_size3(&[
                [true, true, false],
                [false, true, false],
                [false, true, false]
            ]),
            &jlstz_kick_tests,
            Color::new(0x00, 0x00, 0xf0)
        ),
        // L-Piece
        PieceData::new(
            PieceMatrix::new_size3(&[
                [false, true, false],
                [false, true, false],
                [true, true, false]
            ]),
            &jlstz_kick_tests,
            Color::new(0xf0, 0xa0, 0x00)
        ),
        // O-Piece
        PieceData::new(
            PieceMatrix::new_size2(&[[true; 2]; 2]),
            &[[Vec2I8::new(0, 0); 4]; 4],
            Color::new(0xf0, 0xf0, 0x00)
        ),
        // S-Piece
        PieceData::new(
            PieceMatrix::new_size3(&[
                [false, true, false],
                [true, true, false],
                [true, false, false]
            ]),
            &jlstz_kick_tests,
            Color::new(0x00, 0xf0, 0x00)
        ),
        // T-Piece
        PieceData::new(
            PieceMatrix::new_size3(&[
                [false, true, false],
                [true, true, false],
                [false, true, false]
            ]),
            &jlstz_kick_tests,
            Color::new(0xa0, 0x00, 0xf0)
        ),
        // Z-Piece
        PieceData::new(
            PieceMatrix::new_size3(&[
                [true, false, false],
                [true, true, false],
                [false, true, false]
            ]),
            &jlstz_kick_tests,
            Color::new(0xf0, 0x00, 0x00)
        )
    ]
}