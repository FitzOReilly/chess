use crate::bitboard::Bitboard;
use crate::piece::Piece;
use std::fmt;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideToMove {
    White = 0,
    Black = 1,
}

bitflags! {
    pub struct CastlingRights: u8 {
        const WHITE_KINGSIDE = 0b00000001;
        const WHITE_QUEENSIDE = 0b00000010;
        const BLACK_KINGSIDE = 0b00000100;
        const BLACK_QUEENSIDE = 0b00001000;

        const WHITE_BOTH = Self::WHITE_KINGSIDE.bits | Self::WHITE_QUEENSIDE.bits;
        const BLACK_BOTH = Self::BLACK_KINGSIDE.bits | Self::BLACK_QUEENSIDE.bits;
    }
}

pub struct Position {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    en_passant_square: Bitboard,
    side_to_move: SideToMove,
    castling_rights: CastlingRights,
    plies_since_pawn_move_or_capture: u8,
    move_count: u16,
}

impl Position {
    pub fn initial() -> Self {
        Position {
            pawns: Bitboard::RANK_2 | Bitboard::RANK_7,
            knights: Bitboard::B1 | Bitboard::G1 | Bitboard::B8 | Bitboard::G8,
            bishops: Bitboard::C1 | Bitboard::F1 | Bitboard::C8 | Bitboard::F8,
            rooks: Bitboard::A1 | Bitboard::H1 | Bitboard::A8 | Bitboard::H8,
            queens: Bitboard::D1 | Bitboard::D8,
            kings: Bitboard::E1 | Bitboard::E8,
            white_pieces: Bitboard::RANK_1 | Bitboard::RANK_2,
            black_pieces: Bitboard::RANK_7 | Bitboard::RANK_8,
            en_passant_square: Bitboard::EMPTY,
            side_to_move: SideToMove::White,
            castling_rights: CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            plies_since_pawn_move_or_capture: 0,
            move_count: 1,
        }
    }

    pub fn piece_at(&self, square_index: usize) -> Option<Piece> {
        let square = Bitboard(0x1 << square_index);
        if square & self.white_pieces != Bitboard::EMPTY {
            if square & self.pawns != Bitboard::EMPTY {
                Some(Piece::WhitePawn)
            } else if square & self.knights != Bitboard::EMPTY {
                Some(Piece::WhiteKnight)
            } else if square & self.bishops != Bitboard::EMPTY {
                Some(Piece::WhiteBishop)
            } else if square & self.rooks != Bitboard::EMPTY {
                Some(Piece::WhiteRook)
            } else if square & self.queens != Bitboard::EMPTY {
                Some(Piece::WhiteQueen)
            } else {
                debug_assert_ne!(square & self.kings, Bitboard::EMPTY);
                Some(Piece::WhiteKing)
            }
        } else if square & self.black_pieces != Bitboard::EMPTY {
            if square & self.pawns != Bitboard::EMPTY {
                Some(Piece::BlackPawn)
            } else if square & self.knights != Bitboard::EMPTY {
                Some(Piece::BlackKnight)
            } else if square & self.bishops != Bitboard::EMPTY {
                Some(Piece::BlackBishop)
            } else if square & self.rooks != Bitboard::EMPTY {
                Some(Piece::BlackRook)
            } else if square & self.queens != Bitboard::EMPTY {
                Some(Piece::BlackQueen)
            } else {
                debug_assert_ne!(square & self.kings, Bitboard::EMPTY);
                Some(Piece::BlackKing)
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const EMPTY_SQUARE: u8 = b'-';
        const SPACE: u8 = b' ';
        let mut squares_in_rank = [SPACE; 2 * Bitboard::NUM_FILES - 1];
        for rank in (0..Bitboard::NUM_RANKS).rev() {
            for file in 0..Bitboard::NUM_FILES {
                let square = Bitboard::to_square(rank, file);
                squares_in_rank[2 * file] = match self.piece_at(square) {
                    None => EMPTY_SQUARE,
                    Some(piece) => piece.to_ascii(),
                };
            }
            let rank_str = str::from_utf8(&squares_in_rank).unwrap();
            writeln!(f, "{}", rank_str).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position() {
        let pos = Position::initial();
        assert_eq!(Bitboard::EMPTY, pos.en_passant_square);
        assert_eq!(SideToMove::White, pos.side_to_move);
        assert_eq!(
            CastlingRights::WHITE_BOTH | CastlingRights::BLACK_BOTH,
            pos.castling_rights
        );
        assert_eq!(0, pos.plies_since_pawn_move_or_capture);
        assert_eq!(1, pos.move_count);

        assert_eq!(Some(Piece::WhiteRook), pos.piece_at(Bitboard::IDX_A1));
        assert_eq!(Some(Piece::WhiteKnight), pos.piece_at(Bitboard::IDX_B1));
        assert_eq!(Some(Piece::WhiteBishop), pos.piece_at(Bitboard::IDX_C1));
        assert_eq!(Some(Piece::WhiteQueen), pos.piece_at(Bitboard::IDX_D1));
        assert_eq!(Some(Piece::WhiteKing), pos.piece_at(Bitboard::IDX_E1));
        assert_eq!(Some(Piece::WhitePawn), pos.piece_at(Bitboard::IDX_A2));

        assert_eq!(None, pos.piece_at(Bitboard::IDX_A3));
        assert_eq!(None, pos.piece_at(Bitboard::IDX_H6));

        assert_eq!(Some(Piece::BlackRook), pos.piece_at(Bitboard::IDX_A8));
        assert_eq!(Some(Piece::BlackKnight), pos.piece_at(Bitboard::IDX_B8));
        assert_eq!(Some(Piece::BlackBishop), pos.piece_at(Bitboard::IDX_C8));
        assert_eq!(Some(Piece::BlackQueen), pos.piece_at(Bitboard::IDX_D8));
        assert_eq!(Some(Piece::BlackKing), pos.piece_at(Bitboard::IDX_E8));
        assert_eq!(Some(Piece::BlackPawn), pos.piece_at(Bitboard::IDX_A7));
    }

    #[test]
    fn fmt() {
        let expected_str = "\
            r n b q k b n r\n\
            p p p p p p p p\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            - - - - - - - -\n\
            P P P P P P P P\n\
            R N B Q K B N R\n\
        ";
        assert_eq!(expected_str, format!("{}", Position::initial()));
    }
}
