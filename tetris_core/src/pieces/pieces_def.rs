use crate::misc::{Color, Vec2I8};
use super::{PIECE_COUNT, PieceData, PieceMatrix};

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

pub(crate) const fn create_all_pieces() -> [PieceData; PIECE_COUNT] {
    let jlstz_kick_tests = create_jlstz_kick_tests();
    let i_kick_tests = create_i_kick_tests();

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