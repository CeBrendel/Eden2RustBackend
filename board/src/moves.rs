
/*
TODO:
    - score for sorting should be part of move
        -> we have 7 bits left over anyway, so can store 128 different scores
*/

use bitboards::{Bitboard, squares::Square};
use search::traits::SearchableMove;
use generic_magic::Bool;

use crate::board::Board;
use crate::pieces::Piece;
use crate::perft::InformedMove;

// adds/removes asserts at compile time
const DO_ASSERTS: bool = false;

// represent whole move as one number:
// |is capt|cast|ep|ps|prm  |capt |piece|to     |from   |
// |0      |0   |0 |0 |0000 |0000 |0000 |0000 00|00 0000|
//25 bit move:
//-> 6 bits: from                   mask: 0x00000003F
//-> 6 bits: to                     mask: 0x000000FC0
//-> 4 bits: moving piece           mask: 0x00000F000
//-> 4 bits: captured piece         mask: 0x0000F0000
//-> 4 bits: promotion to           mask: 0x000F00000
//-> 1 bit: is pawn start?          mask: 0x002000000
//-> 1 bit: is en passant?          mask: 0x004000000
//-> 1 bit: is castling?            mask: 0x008000000
//-> 1 bit: is capture?             mask: 0x010000000
//-> 1 bit: is promotion?           mask: 0x020000000
#[derive(Clone, Copy)]
#[derive(PartialEq)]  // TODO: Unnecessary
pub struct Move(pub u32);

impl Move {
    const FROM_SHIFT: usize = 0;
    const TO_SHIFT: usize = 6;
    const MOVING_PIECE_SHIFT: usize = 12;
    const CAPTURED_PIECE_SHIFT: usize = 16;
    const PROMOTED_TO_SHIFT: usize = 20;
    const IS_PAWN_START_SHIFT: usize = 24;
    const IS_EN_PASSANT_SHIFT: usize = 25;
    const IS_CASTLING_SHIFT: usize = 26;
    const IS_CAPTURE_SHIFT: usize = 27;
    const IS_PROMOTION_SHIFT: usize = 28;

    const FROM_MASK: u32 = 0x3F << Self::FROM_SHIFT;
    const TO_MASK: u32 = 0x3F << Self::TO_SHIFT;
    const MOVING_PIECE_MASK: u32 = 0xF << Self::MOVING_PIECE_SHIFT;
    const CAPTURED_PIECE_MASK: u32 = 0xF << Self::CAPTURED_PIECE_SHIFT;
    const PROMOTED_TO_MASK: u32 = 0xF << Self::PROMOTED_TO_SHIFT;
    const IS_PAWN_START_MASK: u32 = 0x1 << Self::IS_PAWN_START_SHIFT;
    const IS_EN_PASSANT_MASK: u32 = 0x1 << Self::IS_EN_PASSANT_SHIFT;
    const IS_CASTLING_MASK: u32 = 0x1 << Self::IS_CASTLING_SHIFT;
    const IS_CAPTURE_MASK: u32 = 0x1 << Self::IS_CAPTURE_SHIFT;
    const IS_PROMOTION_MASK: u32 = 0x1 << Self::IS_PROMOTION_SHIFT;

    pub fn from_algebraic(r#move: &str, board: &Board) -> Self {

        // extract relevant information from str
        let length = r#move.len();
        let chars: Vec<char> = r#move.chars().collect();
        let from_file: u8 = (chars[0] as usize - 'a' as usize).try_into().expect("Invalid file!");
        let from_rank: u8 = (chars[1] as usize - '1' as usize).try_into().expect("Invalid rank!");
        let to_file: u8 = (chars[2] as usize - 'a' as usize).try_into().expect("Invalid file!");
        let to_rank: u8 = (chars[3] as usize - '1' as usize).try_into().expect("Invalid rank!");

        // extract all information needed to construct the move
        let from_square = Square::from_file_and_rank(from_file, from_rank);
        let to_square = Square::from_file_and_rank(to_file, to_rank);
        let moving_piece = board.piece_at(from_square);
        let captured_piece = board.piece_at(to_square);
        let promotion_to = if length == 4 {Piece::None} else {
            match chars[4] {
                'q' => if board.whites_turn {Piece::WhiteQueen} else {Piece::BlackQueen},
                'r' => if board.whites_turn {Piece::WhiteRook} else {Piece::BlackRook},
                'b' => if board.whites_turn {Piece::WhiteBishop} else {Piece::BlackBishop},
                'k' => if board.whites_turn {Piece::WhiteKnight} else {Piece::BlackKnight},
                _ => panic!("Invalid piece!")
            }
        };
        let is_pawn_start = (
            (moving_piece == Piece::WhitePawn) | (moving_piece == Piece::BlackPawn)
        ) & (
            from_rank.abs_diff(to_rank) == 2
        );
        let is_en_passant = (
            (moving_piece == Piece::WhitePawn) | (moving_piece == Piece::BlackPawn)
        ) & (
            board.piece_at(to_square) == Piece::None
        ) & (
            from_file.abs_diff(to_file) == 1
        );
        let is_castling = (
            (moving_piece == Piece::WhiteKing) | (moving_piece == Piece::BlackKing)
        ) & (
            from_file.abs_diff(to_file) == 2
        );
        let is_capture = board.piece_at(to_square) != Piece::None;
        let is_promotion = length != 4;

        // construct and return move
        return Self::from_full_info(
            from_square, to_square, moving_piece, captured_piece, promotion_to,
            is_pawn_start, is_en_passant, is_castling, is_capture, is_promotion
        );
    }

    const fn from_full_info(
        from: Square,
        to: Square,
        moving_piece: Piece,
        captured_piece: Piece,
        promotion_to: Piece,
        is_pawn_start: bool,
        is_en_passant: bool,
        is_castling: bool,
        is_capture: bool,
        is_promotion: bool
    ) -> Self {
        // build a move from all information given
        return Move(
            (from as u32)
                | (to as u32) << Self::TO_SHIFT
                | (moving_piece as u32) << Self::MOVING_PIECE_SHIFT
                | (captured_piece as u32) << Self::CAPTURED_PIECE_SHIFT
                | (promotion_to as u32) << Self::PROMOTED_TO_SHIFT
                | (is_pawn_start as u32) << Self::IS_PAWN_START_SHIFT
                | (is_en_passant as u32) << Self::IS_EN_PASSANT_SHIFT
                | (is_castling as u32) << Self::IS_CASTLING_SHIFT
                | (is_capture as u32) << Self::IS_CAPTURE_SHIFT
                | (is_promotion as u32) << Self::IS_PROMOTION_SHIFT
        );
    }

    pub fn silent(from: u8, to: u8, moving_piece: Piece, _board: &Board) -> Move {
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            moving_piece,
            Piece::None,
            Piece::None,
            false,
            false,
            false,
            false,
            false
        )
    }

    pub fn maybe_capture(from: u8, to: u8, moving_piece: Piece, board: &Board, enemy_mask: Bitboard) -> Move {
        // TODO: Split into captures and non-captures in legal move gen!
        if DO_ASSERTS {
            if enemy_mask.has_entry_at(Square::from_repr(to)) {
                if (board.piece_at(Square::from_repr(to)) as usize) >= 12 {
                    assert!(false);
                }
            }
        }
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            moving_piece,
            board.piece_at(Square::from_repr(to)),
            Piece::None,
            false,
            false,
            false,
            enemy_mask.has_entry_at(Square::from_repr(to)),
            false
        )
    }

    pub fn capture(from: u8, to: u8, moving_piece: Piece, board: &Board) -> Move {
        if DO_ASSERTS {
            if (board.piece_at(Square::from_repr(to)) as usize) >= 12 {
                assert!(false);
            };
        }
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            moving_piece,
            board.piece_at(Square::from_repr(to)),
            Piece::None,
            false,
            false,
            false,
            true,
            false
        )
    }

    pub fn pawn_start<WhitesTurn: Bool>(from: u8, to: u8, board: &Board) -> Move {
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            board.own_pawn::<WhitesTurn>(),
            Piece::None,
            Piece::None,
            true,
            false,
            false,
            false,
            false
        )
    }

    pub fn promotion_without_capture<WhitesTurn: Bool>(from: u8, to: u8, promoted_to: Piece, board: &Board) -> Move {
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            board.own_pawn::<WhitesTurn>(),
            Piece::None,
            promoted_to,
            false,
            false,
            false,
            false,
            true
        )
    }

    pub fn promotion_with_capture<WhitesTurn: Bool>(from: u8, to: u8, promoted_to: Piece, board: &Board) -> Self {
        if DO_ASSERTS {
            if (board.piece_at(Square::from_repr(to)) as usize) >= 12 {
                assert!(false);
            }
        }
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            board.own_pawn::<WhitesTurn>(),
            board.piece_at(Square::from_repr(to)),
            promoted_to,
            false,
            false,
            false,
            true,
            true
        )
    }

    pub fn en_passant<WhitesTurn: Bool>(from: u8, to: u8, board: &Board) -> Move {
        Self::from_full_info(
            Square::from_repr(from),
            Square::from_repr(to),
            board.own_pawn::<WhitesTurn>(),
            board.enemy_pawn::<WhitesTurn>(),
            Piece::None,
            false,
            true,
            false,
            true,
            false
        )
    }

    pub const WHITE_SHORT_CASTLE: Self = Self::from_full_info(
        Square::E1,
        Square::G1,
        Piece::WhiteKing,
        Piece::None,
        Piece::None,
        false,
        false,
        true,
        false,
        false
    );

    pub const BLACK_SHORT_CASTLE: Self = Self::from_full_info(
        Square::E8,
        Square::G8,
        Piece::BlackKing,
        Piece::None,
        Piece::None,
        false,
        false,
        true,
        false,
        false
    );

    pub const WHITE_LONG_CASTLE: Self = Self::from_full_info(
        Square::E1,
        Square::C1,
        Piece::WhiteKing,
        Piece::None,
        Piece::None,
        false,
        false,
        true,
        false,
        false
    );

    pub const BLACK_LONG_CASTLE: Self = Self::from_full_info(
        Square::E8,
        Square::C8,
        Piece::BlackKing,
        Piece::None,
        Piece::None,
        false,
        false,
        true,
        false,
        false
    );

    #[inline(always)]
    pub fn from_square(self: &Self) -> Square {
        let repr = (self.0 & Self::FROM_MASK) >> Self::FROM_SHIFT;
        return Square::from_repr(repr.try_into().unwrap());
    }

    #[inline(always)]
    pub fn to_square(self: &Self) -> Square {
        let repr = (self.0 & Self::TO_MASK) >> Self::TO_SHIFT;
        return Square::from_repr(repr.try_into().unwrap());
    }

    #[inline(always)]
    pub fn moving_piece(self: &Self) -> Piece {
        let repr = (self.0 & Self::MOVING_PIECE_MASK) >> Self::MOVING_PIECE_SHIFT;
        return Piece::from_repr(repr.try_into().unwrap());
    }

    #[inline(always)]
    pub fn captured_piece(self: &Self) -> Piece {
        if DO_ASSERTS {assert!(self.is_capture());}
        if DO_ASSERTS {
            if ((self.0 & Self::CAPTURED_PIECE_MASK) >> Self::CAPTURED_PIECE_SHIFT) >= 12 {
                println!("{:b}", self.0);
                assert!(false, "{}", self.to_string());
            }
        }
        let repr = (self.0 & Self::CAPTURED_PIECE_MASK) >> Self::CAPTURED_PIECE_SHIFT;
        return Piece::from_repr(repr.try_into().unwrap());
    }

    #[inline(always)]
    pub fn promoted_to(self: &Self) -> Piece {
        if DO_ASSERTS {assert!(self.is_promotion());}
        let repr = (self.0 & Self::PROMOTED_TO_MASK) >> Self::PROMOTED_TO_SHIFT;
        return Piece::from_repr(repr.try_into().unwrap());
    }

    #[inline(always)]
    pub fn is_capture(self: &Self) -> bool {
        return (self.0 & Self::IS_CAPTURE_MASK) != 0;
    }

    #[inline(always)]
    pub fn is_pawn_start(self: &Self) -> bool {
        return (self.0 & Self::IS_PAWN_START_MASK) != 0;
    }

    #[inline(always)]
    pub fn is_en_passant(self: &Self) -> bool {
        return (self.0 & Self::IS_EN_PASSANT_MASK) != 0;
    }

    #[inline(always)]
    pub fn is_castling(self: &Self) -> bool {
        return (self.0 & Self::IS_CASTLING_MASK) != 0;
    }

    #[inline(always)]
    pub fn is_promotion(self: &Self) -> bool {
        return (self.0 & Self::IS_PROMOTION_MASK) != 0;
    }

    pub fn visualize(self: &Self) {
        self.moving_piece().visualize();
        self.from_square().visualize();
        self.to_square().visualize();
        if self.is_promotion() {
            print!("{}", match self.promoted_to() {
                Piece::WhiteKnight | Piece::BlackKnight => 'k',
                Piece::WhiteBishop | Piece::BlackBishop => 'b',
                Piece::WhiteRook   | Piece::BlackRook   => 'r',
                Piece::WhiteQueen  | Piece::BlackQueen  => 'q',
                _ => panic!("Invalid promotion!")
            });
        }
    }

    pub fn to_string(self: &Self) -> String {
        return format!(
            "{}{}", self.from_square().to_string(), self.to_square().to_string()
        );
    }
}

impl InformedMove for Move {
    fn is_capture(self: &Self) -> bool {
        self.is_capture()
    }
    fn is_en_passant(self: &Self) -> bool {
        self.is_en_passant()
    }
    fn is_castling(self: &Self) -> bool {
        self.is_castling()
    }
    fn is_promotion(self: &Self) -> bool {
        self.is_promotion()
    }
    fn visualize(self: &Self) {
        self.visualize()
    }
}

static MVV_LVA_SCORES: [[i32; 12]; 12] = {
    // for each pair (victim, attacker) of pieces the MVV-LVA score given by arr[victim][attacker]
    let mut scores = [[0; 12]; 12];

    let capture_bonus: i32 = 10;  // positive: captures before non-captures, negative: non-captures before captures of score 0
    let piece_values: [i32; 6] = [100, 300, 325, 500, 900, i32::MAX/3];

    let mut victim: usize = 0;
    while victim < 12 {

        let mut attacker: usize = 0;
        while attacker < 12 {

            let victim_value = piece_values[victim % 6];
            let attacker_value = piece_values[victim % 6];
            scores[victim][attacker] = victim_value - attacker_value + capture_bonus;

            attacker += 1;
        }

        victim += 1;
    }

    scores
};

impl SearchableMove for Move {
    fn to_string(self: &Self) -> String {
        self.to_string()
    }
    fn score(self: &Self) -> i32 {
        return if self.is_capture() {
            MVV_LVA_SCORES[self.captured_piece() as usize][self.moving_piece() as usize]
        } else {
            0
        }
    }
    #[inline(always)]
    fn to_square_as_index(self: &Self) -> usize {
        self.to_square() as usize
    }
    #[inline(always)]
    fn moving_piece_as_index(self: &Self) -> usize {
        self.moving_piece() as usize
    }
}