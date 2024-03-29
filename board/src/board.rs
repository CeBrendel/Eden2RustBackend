
use bitboards::{Bitboard, squares::Square};
use search::traits::AlphaBetaSearchFunctionality;
use generic_magic::{Bool, False, True};

use crate::castle_permissions::CastlePermissions;
use crate::pieces::Piece;
use crate::moves::Move;
use crate::zobrist_hash::ZobristHash;

/*
TODO:
    - joined set masks for captures and castles indexed by a hash of moves?
    - avoid branching (including matches). Keep track which piece is on which square?
    - make get_bitboard depend on const?
    - add "has_white_castling_right", ... to generic-magic
*/

#[derive(PartialEq, Clone)]  // TODO: Unnecessary?
pub struct UnmakeInformation {
    pub r#move: Move,
    pub castle_permissions: CastlePermissions,
    pub en_passant_square: Option<Square>,
    pub fifty_move_counter: u8,
    pub zobrist_hash: ZobristHash
}

#[derive(PartialEq, Clone)]  // TODO: Unnecessary?
pub struct Board {
    // some general information concerning the current position
    pub whites_turn: bool,
    pub castle_permissions: CastlePermissions,
    pub en_passant_square: Option<Square>,

    // various bitboards
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

    pub white_mask: Bitboard,
    pub black_mask: Bitboard,
    pub occupation: Bitboard,  // = white_mask | black_mask

    // easily look up what piece is at which square
    pub square_piece_mapping: [Piece; 64],

    // various information to allow hashing and detection of repetition, 50 move rule, ...
    pub zobrist_hash: ZobristHash,
    pub fifty_move_counter: u8,

    // information to undo made moves
    pub history: Vec<UnmakeInformation>

}


// impls depending only on bools, hopefully to be replaced by const-magic
impl Board {
    #[inline(always)]
    pub fn piece_at(self: &Self, square: Square) -> Piece {
        self.square_piece_mapping[square as usize]
    }

    #[inline(always)]
    pub fn has_en_passant_square(self: &Self) -> bool {
        self.en_passant_square != None
    }

    #[inline(always)]
    pub fn has_short_castling_rights(self: &Self) -> bool {
        if self.whites_turn {
            self.castle_permissions.has_white_short()
        } else {
            self.castle_permissions.has_black_short()
        }
    }

    #[inline(always)]
    pub fn has_long_castling_rights(self: &Self) -> bool {
        if self.whites_turn {
            self.castle_permissions.has_white_long()
        } else {
            self.castle_permissions.has_black_long()
        }
    }

    #[inline(always)]
    pub fn own_pawn<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhitePawn} else {Piece::BlackPawn}
    }

    #[inline(always)]
    pub fn enemy_pawn<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::BlackPawn} else {Piece::WhitePawn}
    }

    #[inline(always)]
    pub fn own_knight<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhiteKnight} else {Piece::BlackKnight}
    }

    #[inline(always)]
    pub fn own_bishop<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhiteBishop} else {Piece::BlackBishop}
    }

    #[inline(always)]
    pub fn own_rook<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhiteRook} else {Piece::BlackRook}
    }

    #[inline(always)]
    pub fn enemy_rook<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::BlackRook} else {Piece::WhiteRook}
    }

    #[inline(always)]
    pub fn own_queen<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhiteQueen} else {Piece::BlackQueen}
    }

    #[inline(always)]
    pub fn own_king<WhitesTurn: Bool>(self: &Self) -> Piece {
        if WhitesTurn::AS_BOOL {Piece::WhiteKing} else {Piece::BlackKing}
    }

    #[inline(always)]
    pub fn own_pawns<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_pawns} else {self.black_pawns}
    }

    #[inline(always)]
    pub fn enemy_pawns<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_pawns} else {self.white_pawns}
    }

    #[inline(always)]
    pub fn own_knights<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_knights} else {self.black_knights}
    }

    #[inline(always)]
    pub fn enemy_knights<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_knights} else {self.white_knights}
    }

    #[inline(always)]
    pub fn own_bishops<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_bishops} else {self.black_bishops}
    }

    #[inline(always)]
    pub fn enemy_bishops<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_bishops} else {self.white_bishops}
    }

    #[inline(always)]
    pub fn own_rooks<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_rooks} else {self.black_rooks}
    }

    #[inline(always)]
    pub fn enemy_rooks<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_rooks} else {self.white_rooks}
    }

    #[inline(always)]
    pub fn own_queens<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_queens} else {self.black_queens}
    }

    #[inline(always)]
    pub fn enemy_queens<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_queens} else {self.white_queens}
    }

    #[inline(always)]
    pub fn own_kings<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_king} else {self.black_king}
    }

    #[inline(always)]
    pub fn enemy_kings<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_king} else {self.white_king}
    }

    #[inline(always)]
    pub fn own_mask<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.white_mask} else {self.black_mask}
    }

    #[inline(always)]
    pub fn own_mask_mut<WhitesTurn: Bool>(self: &mut Self) -> &mut Bitboard {
        if WhitesTurn::AS_BOOL {&mut self.white_mask} else {&mut self.black_mask}
    }

    #[inline(always)]
    pub fn enemy_mask<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        if WhitesTurn::AS_BOOL {self.black_mask} else {self.white_mask}
    }

    #[inline(always)]
    pub fn enemy_mask_mut<WhitesTurn: Bool>(self: &mut Self) -> &mut Bitboard {
        if WhitesTurn::AS_BOOL {&mut self.black_mask} else {&mut self.white_mask}
    }

    pub fn get_bitboard(self: &mut Self, piece: Piece) -> &mut Bitboard {
        // TODO: inefficient, maybe replace by two functions "get_bitboard_own" and "get_bitboard_enemy"
        // Only used in crate::board
        match piece {
            Piece::WhitePawn   => &mut self.white_pawns,
            Piece::WhiteKnight => &mut self.white_knights,
            Piece::WhiteBishop => &mut self.white_bishops,
            Piece::WhiteRook   => &mut self.white_rooks,
            Piece::WhiteQueen  => &mut self.white_queens,
            Piece::WhiteKing   => &mut self.white_king,
            Piece::BlackPawn   => &mut self.black_pawns,
            Piece::BlackKnight => &mut self.black_knights,
            Piece::BlackBishop => &mut self.black_bishops,
            Piece::BlackRook   => &mut self.black_rooks,
            Piece::BlackQueen  => &mut self.black_queens,
            Piece::BlackKing   => &mut self.black_king,
            _ => panic!("Invalid piece!")
        }
    }
}


impl Default for Board {
    fn default() -> Self {
        // returns the starting position
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}


impl Board {
    fn empty() -> Self {
        return Self{
            whites_turn: false,
            castle_permissions: CastlePermissions::empty(),
            en_passant_square: None,

            // various bitboards
            white_pawns: Bitboard(0),
            white_knights: Bitboard(0),
            white_bishops: Bitboard(0),
            white_rooks: Bitboard(0),
            white_queens: Bitboard(0),
            white_king: Bitboard(0),

            black_pawns: Bitboard(0),
            black_knights: Bitboard(0),
            black_bishops: Bitboard(0),
            black_rooks: Bitboard(0),
            black_queens: Bitboard(0),
            black_king: Bitboard(0),

            white_mask: Bitboard(0),
            black_mask: Bitboard(0),
            occupation: Bitboard(0),

            square_piece_mapping: [Piece::None; 64],

            zobrist_hash: ZobristHash::empty(),
            fifty_move_counter: 0,

            history: Vec::new()
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        // build a board from the given FEN

        let mut board = Self::empty();

        let blocks: Vec<&str> = fen.split(' ').collect();

        let piece_block = *blocks.get(0).expect("Invalid FEN!");
        let player_block = *blocks.get(1).expect("Invalid FEN!");
        let castling_block = *blocks.get(2).expect("Invalid FEN!");
        let en_passant_block = *blocks.get(3).expect("Invalid FEN!");
        let fifty_counter_block = *blocks.get(4).expect("Invalid FEN!");
        let _move_counter_block = *blocks.get(5).expect("Invalid FEN!");

        // handle pieces
        let rank_strs: Vec<&str> = piece_block.split('/').rev().collect();

        let mut rank: u8 = 0;
        for rank_str in rank_strs {

            let chars: Vec<char> = rank_str.chars().collect();

            let mut file: u8 = 0;
            for char in chars {

                // handle char
                if char.is_digit(10) {
                    // if digit, decrement file and continue to next char
                    let digit: u8 = char.to_digit(10).expect("Invalid FEN!") as u8;
                    file += digit;
                    continue;
                }

                // build current square
                let square = Square::from_file_and_rank(file, rank);

                // match char to Piece
                let (piece, of_white) = match char {
                    'P' => (Piece::WhitePawn, true), 'p' => (Piece::BlackPawn, false),
                    'N' => (Piece::WhiteKnight, true), 'n' => (Piece::BlackKnight, false),
                    'B' => (Piece::WhiteBishop, true), 'b' => (Piece::BlackBishop, false),
                    'R' => (Piece::WhiteRook, true), 'r' => (Piece::BlackRook, false),
                    'Q' => (Piece::WhiteQueen, true), 'q' => (Piece::BlackQueen, false),
                    'K' => (Piece::WhiteKing, true), 'k' => (Piece::BlackKing, false),
                    _ => panic!("Invalid FEN!")
                };

                // put piece onto board and hash it in
                board.get_bitboard(piece).set_bit(square);
                if of_white {
                    board.white_mask.set_bit(square);
                } else {
                    board.black_mask.set_bit(square);
                }
                board.occupation.set_bit(square);
                board.square_piece_mapping[square as usize] = piece;
                board.zobrist_hash.hash_piece(piece, square);

                // decrement file
                file += 1;
            }

            // decrement rank
            rank += 1;
        }

        // handle player
        let player_char = player_block.chars().next().expect("Invalid player in FEN!");
        match player_char {
            'w' => {
                board.whites_turn = true;
                board.zobrist_hash.hash_player();
            },
            'b' => {
                board.whites_turn = false;
            },
            _ => {panic!("Invalid playerin FEN!");}
        }

        // handle castling
        let mut white_short: bool = false;
        let mut white_long: bool = false;
        let mut black_short: bool = false;
        let mut black_long: bool = false;
        for char in castling_block.chars() {
            match char {
                '-' => {break;},
                'K' => {
                    white_short = true;
                    board.zobrist_hash.hash_white_short();
                },
                'Q' => {
                    white_long = true;
                    board.zobrist_hash.hash_white_long();
                },
                'k' => {
                    black_short = true;
                    board.zobrist_hash.hash_black_short();
                },
                'q' => {
                    black_long = true;
                    board.zobrist_hash.hash_black_long();
                },
                _ => {panic!("Invalid castling rights in FEN!");}
            }
        }
        board.castle_permissions = CastlePermissions::new(
            white_short, white_long, black_short, black_long
        );

        // handle en-passant square
        if en_passant_block == "-" {
            board.en_passant_square = None;
        } else {
            let square = Square::from_algebraic(en_passant_block);
            board.en_passant_square = Some(square);
            board.zobrist_hash.hash_en_passant(square);
        }

        // handle 50 counter
        let fifty_count = fifty_counter_block.parse().expect("Invalid 50 move counter in FEN!");
        board.fifty_move_counter = fifty_count;
        board.zobrist_hash.hash_move_count(fifty_count as usize);

        /*// TODO: Add full_move_counter field to struct
        // handle move count
        let move_count = move_counter_block.parse().expect("Invalid move counter in FEN!");*/

        return board;
    }

    fn make_move_generic<
        WhitesTurn: Bool,
        IsCapture: Bool,
        IsEnPassant: Bool,
        IsCastling: Bool,
        IsPromotion: Bool,
        IsPawnStart: Bool
    >(self: &mut Self, r#move: Move) {
        // make the given move on the board

        let from_square = r#move.from_square();
        let to_square = r#move.to_square();
        let moving_piece = r#move.moving_piece();

        // remember current information that is necessary to undo the move (once made)
        self.history.push(
            UnmakeInformation {
                r#move,
                castle_permissions: self.castle_permissions,
                en_passant_square: self.en_passant_square,
                fifty_move_counter: self.fifty_move_counter,
                zobrist_hash: self.zobrist_hash
            }
        );

        // update castling rights
        if moving_piece == self.own_king::<WhitesTurn>() {
            // if we castle we can no longer castle
            self.castle_permissions.remove_rights(self.whites_turn);
            if WhitesTurn::AS_BOOL {
                self.zobrist_hash.hash_white_short();
                self.zobrist_hash.hash_white_long();
            } else {
                self.zobrist_hash.hash_black_short();
                self.zobrist_hash.hash_black_long();
            }
        }

        // if our own rook moves, we can no longer castle in this direction
        // TODO: Kill this check with const-ness
        if moving_piece == self.own_rook::<WhitesTurn>() {
            if WhitesTurn::AS_BOOL {
                if self.castle_permissions.has_white_short() && (from_square == Square::H1) {
                    self.castle_permissions.remove_short_rights(WhitesTurn::AS_BOOL);
                    self.zobrist_hash.hash_white_short();
                } else if self.castle_permissions.has_white_long() && (from_square == Square::A1) {
                    self.castle_permissions.remove_long_rights(WhitesTurn::AS_BOOL);
                    self.zobrist_hash.hash_white_long();
                }
            } else {
                if self.castle_permissions.has_black_short() && (from_square == Square::H8) {
                    self.castle_permissions.remove_short_rights(WhitesTurn::AS_BOOL);
                    self.zobrist_hash.hash_black_short();
                } else if self.castle_permissions.has_black_long() && (from_square == Square::A8) {
                    self.castle_permissions.remove_long_rights(WhitesTurn::AS_BOOL);
                    self.zobrist_hash.hash_black_long();
                }
            }
        }

        // if rook is taken, enemy can't castle on that side anymore
        if IsCapture::AS_BOOL {
            if r#move.captured_piece() == self.enemy_rook::<WhitesTurn>() {
                if WhitesTurn::AS_BOOL {
                    if self.castle_permissions.has_black_short() && (to_square == Square::H8) {
                        self.castle_permissions.remove_short_rights(!WhitesTurn::AS_BOOL);
                        self.zobrist_hash.hash_black_short();
                    } else if self.castle_permissions.has_black_long() && (to_square == Square::A8) {
                        self.castle_permissions.remove_long_rights(!WhitesTurn::AS_BOOL);
                        self.zobrist_hash.hash_black_long();
                    }
                } else {
                    if self.castle_permissions.has_white_short() && (to_square == Square::H1) {
                        self.castle_permissions.remove_short_rights(!WhitesTurn::AS_BOOL);
                        self.zobrist_hash.hash_white_short();
                    } else if self.castle_permissions.has_white_long() && (to_square == Square::A1) {
                        self.castle_permissions.remove_long_rights(!WhitesTurn::AS_BOOL);
                        self.zobrist_hash.hash_white_long();
                    }
                }
            }
        }

        // update en-passant square
        {
            // hash out old en-passant square (if any)
            match self.en_passant_square {
                None => {},
                Some(square) => self.zobrist_hash.hash_en_passant(square)
            }

            // check for new en-passant square
            if IsPawnStart::AS_BOOL {
                // if we have a pawn start, add an en-passant square and hash it in
                let square = r#move.from_square().advance_square(WhitesTurn::AS_BOOL);
                self.en_passant_square = Some(square);
                self.zobrist_hash.hash_en_passant(square)
            } else {
                // if we don't have a pawn start, remove old en-passant square
                self.en_passant_square = None;
            }
        }

        // update bitboards

        // move piece on own bitboard
        {
            let moving_bitboard = self.get_bitboard(moving_piece);
            moving_bitboard.clear_bit(from_square);
            moving_bitboard.set_bit(to_square);
        }

        // move piece on own mask
        self.own_mask_mut::<WhitesTurn>().clear_bit(from_square);
        self.own_mask_mut::<WhitesTurn>().set_bit(to_square);

        // move piece on occupation
        self.occupation.clear_bit(from_square);
        self.occupation.set_bit(to_square);

        // move piece in hash
        self.zobrist_hash.hash_piece(moving_piece, from_square);
        self.zobrist_hash.hash_piece(moving_piece, to_square);

        // move piece in square-piece mapping
        self.square_piece_mapping[from_square as usize] = Piece::None;
        self.square_piece_mapping[to_square as usize] = moving_piece;

        // remove captured piece
        if IsCapture::AS_BOOL {
            let captured_piece = r#move.captured_piece();

            if IsEnPassant::AS_BOOL {
                // is en-passant the taken piece is one behind (from owns perspective) the taken pawn
                let square_of_taken_piece = to_square.advance_square(!WhitesTurn::AS_BOOL);
                self.get_bitboard(captured_piece).clear_bit(square_of_taken_piece);
                self.enemy_mask_mut::<WhitesTurn>().clear_bit(square_of_taken_piece);
                self.occupation.clear_bit(square_of_taken_piece);
                self.zobrist_hash.hash_piece(captured_piece, square_of_taken_piece);
                self.square_piece_mapping[square_of_taken_piece as usize] = Piece::None;
            } else {
                // if not en-passant, the taken square is on the to-square of the move
                // clear piece from own bitboard and enemy mask, but not from occupation or square-piece-mapping!
                self.get_bitboard(captured_piece).clear_bit(to_square);
                self.enemy_mask_mut::<WhitesTurn>().clear_bit(to_square);
                self.zobrist_hash.hash_piece(captured_piece, to_square);
            }
        }

        // handle rook move for castling
        if IsCastling::AS_BOOL {
            let (rook, (rook_from, rook_to)) = {
                if WhitesTurn::AS_BOOL {
                    (
                        Piece::WhiteRook,
                        if to_square == Square::G1 {
                            (Square::H1, Square::F1)
                        } else {
                            (Square::A1, Square::D1)
                        }
                    )
                } else {
                    (
                        Piece::BlackRook,
                        if to_square == Square::G8 {
                            (Square::H8, Square::F8)
                        } else {
                            (Square::A8, Square::D8)
                        }
                    )
                }
            } ;

            let rook_bitboard = self.get_bitboard(rook);

            rook_bitboard.clear_bit(rook_from);
            rook_bitboard.set_bit(rook_to);

            self.own_mask_mut::<WhitesTurn>().clear_bit(rook_from);
            self.own_mask_mut::<WhitesTurn>().set_bit(rook_to);

            self.occupation.clear_bit(rook_from);
            self.occupation.set_bit(rook_to);

            self.zobrist_hash.hash_piece(rook, rook_from);
            self.zobrist_hash.hash_piece(rook, rook_to);

            self.square_piece_mapping[rook_from as usize] = Piece::None;
            self.square_piece_mapping[rook_to as usize] = rook;
        }

        // handle promotion
        if IsPromotion::AS_BOOL {
            // remove pawn from to-square (except for own mask, occupation and square-piece-mapping)
            self.get_bitboard(moving_piece).clear_bit(to_square);
            self.zobrist_hash.hash_piece(moving_piece, to_square);

            // add new piece
            let promoted_to = r#move.promoted_to();
            self.get_bitboard(promoted_to).set_bit(to_square);
            self.zobrist_hash.hash_piece(promoted_to, to_square);
            self.square_piece_mapping[to_square as usize] = promoted_to;
        }

        // update 50/75 move rule counter
        {
            // hash out current count
            self.zobrist_hash.hash_move_count(self.fifty_move_counter as usize);

            // adjust move count
            if IsCapture::AS_BOOL || (moving_piece == self.own_pawn::<WhitesTurn>()) {
                // hash out move count, set to zero
                self.fifty_move_counter = 0;
            } else {
                self.fifty_move_counter += 1;
            }

            // hash in new count
            self.zobrist_hash.hash_move_count(self.fifty_move_counter as usize);
        }

        // swap players
        self.whites_turn ^= true;
        self.zobrist_hash.hash_player()
    }

    pub fn make_move(self: &mut Self, r#move: Move) {

        // make generics
        let generics = (
            self.whites_turn,
            r#move.is_capture(),
            r#move.is_en_passant(),
            r#move.is_castling(),
            r#move.is_promotion(),
            r#move.is_pawn_start()
        );

        // find correct impl of make_move
        match generics {
            (false, false, false, false, false, false) => self.make_move_generic::<False, False, False, False, False, False>(r#move),
            (false, false, false, false, false, true ) => self.make_move_generic::<False, False, False, False, False, True >(r#move),
            (false, false, false, false, true , false) => self.make_move_generic::<False, False, False, False, True , False>(r#move),
            (false, false, false, false, true , true ) => self.make_move_generic::<False, False, False, False, True , True >(r#move),
            (false, false, false, true , false, false) => self.make_move_generic::<False, False, False, True , False, False>(r#move),
            (false, false, false, true , false, true ) => self.make_move_generic::<False, False, False, True , False, True >(r#move),
            (false, false, false, true , true , false) => self.make_move_generic::<False, False, False, True , True , False>(r#move),
            (false, false, false, true , true , true ) => self.make_move_generic::<False, False, False, True , True , True >(r#move),
            (false, false, true , false, false, false) => self.make_move_generic::<False, False, True , False, False, False>(r#move),
            (false, false, true , false, false, true ) => self.make_move_generic::<False, False, True , False, False, True >(r#move),
            (false, false, true , false, true , false) => self.make_move_generic::<False, False, True , False, True , False>(r#move),
            (false, false, true , false, true , true ) => self.make_move_generic::<False, False, True , False, True , True >(r#move),
            (false, false, true , true , false, false) => self.make_move_generic::<False, False, True , True , False, False>(r#move),
            (false, false, true , true , false, true ) => self.make_move_generic::<False, False, True , True , False, True >(r#move),
            (false, false, true , true , true , false) => self.make_move_generic::<False, False, True , True , True , False>(r#move),
            (false, false, true , true , true , true ) => self.make_move_generic::<False, False, True , True , True , True >(r#move),
            (false, true , false, false, false, false) => self.make_move_generic::<False, True , False, False, False, False>(r#move),
            (false, true , false, false, false, true ) => self.make_move_generic::<False, True , False, False, False, True >(r#move),
            (false, true , false, false, true , false) => self.make_move_generic::<False, True , False, False, True , False>(r#move),
            (false, true , false, false, true , true ) => self.make_move_generic::<False, True , False, False, True , True >(r#move),
            (false, true , false, true , false, false) => self.make_move_generic::<False, True , False, True , False, False>(r#move),
            (false, true , false, true , false, true ) => self.make_move_generic::<False, True , False, True , False, True >(r#move),
            (false, true , false, true , true , false) => self.make_move_generic::<False, True , False, True , True , False>(r#move),
            (false, true , false, true , true , true ) => self.make_move_generic::<False, True , False, True , True , True >(r#move),
            (false, true , true , false, false, false) => self.make_move_generic::<False, True , True , False, False, False>(r#move),
            (false, true , true , false, false, true ) => self.make_move_generic::<False, True , True , False, False, True >(r#move),
            (false, true , true , false, true , false) => self.make_move_generic::<False, True , True , False, True , False>(r#move),
            (false, true , true , false, true , true ) => self.make_move_generic::<False, True , True , False, True , True >(r#move),
            (false, true , true , true , false, false) => self.make_move_generic::<False, True , True , True , False, False>(r#move),
            (false, true , true , true , false, true ) => self.make_move_generic::<False, True , True , True , False, True >(r#move),
            (false, true , true , true , true , false) => self.make_move_generic::<False, True , True , True , True , False>(r#move),
            (false, true , true , true , true , true ) => self.make_move_generic::<False, True , True , True , True , True >(r#move),
            (true , false, false, false, false, false) => self.make_move_generic::<True , False, False, False, False, False>(r#move),
            (true , false, false, false, false, true ) => self.make_move_generic::<True , False, False, False, False, True >(r#move),
            (true , false, false, false, true , false) => self.make_move_generic::<True , False, False, False, True , False>(r#move),
            (true , false, false, false, true , true ) => self.make_move_generic::<True , False, False, False, True , True >(r#move),
            (true , false, false, true , false, false) => self.make_move_generic::<True , False, False, True , False, False>(r#move),
            (true , false, false, true , false, true ) => self.make_move_generic::<True , False, False, True , False, True >(r#move),
            (true , false, false, true , true , false) => self.make_move_generic::<True , False, False, True , True , False>(r#move),
            (true , false, false, true , true , true ) => self.make_move_generic::<True , False, False, True , True , True >(r#move),
            (true , false, true , false, false, false) => self.make_move_generic::<True , False, True , False, False, False>(r#move),
            (true , false, true , false, false, true ) => self.make_move_generic::<True , False, True , False, False, True >(r#move),
            (true , false, true , false, true , false) => self.make_move_generic::<True , False, True , False, True , False>(r#move),
            (true , false, true , false, true , true ) => self.make_move_generic::<True , False, True , False, True , True >(r#move),
            (true , false, true , true , false, false) => self.make_move_generic::<True , False, True , True , False, False>(r#move),
            (true , false, true , true , false, true ) => self.make_move_generic::<True , False, True , True , False, True >(r#move),
            (true , false, true , true , true , false) => self.make_move_generic::<True , False, True , True , True , False>(r#move),
            (true , false, true , true , true , true ) => self.make_move_generic::<True , False, True , True , True , True >(r#move),
            (true , true , false, false, false, false) => self.make_move_generic::<True , True , False, False, False, False>(r#move),
            (true , true , false, false, false, true ) => self.make_move_generic::<True , True , False, False, False, True >(r#move),
            (true , true , false, false, true , false) => self.make_move_generic::<True , True , False, False, True , False>(r#move),
            (true , true , false, false, true , true ) => self.make_move_generic::<True , True , False, False, True , True >(r#move),
            (true , true , false, true , false, false) => self.make_move_generic::<True , True , False, True , False, False>(r#move),
            (true , true , false, true , false, true ) => self.make_move_generic::<True , True , False, True , False, True >(r#move),
            (true , true , false, true , true , false) => self.make_move_generic::<True , True , False, True , True , False>(r#move),
            (true , true , false, true , true , true ) => self.make_move_generic::<True , True , False, True , True , True >(r#move),
            (true , true , true , false, false, false) => self.make_move_generic::<True , True , True , False, False, False>(r#move),
            (true , true , true , false, false, true ) => self.make_move_generic::<True , True , True , False, False, True >(r#move),
            (true , true , true , false, true , false) => self.make_move_generic::<True , True , True , False, True , False>(r#move),
            (true , true , true , false, true , true ) => self.make_move_generic::<True , True , True , False, True , True >(r#move),
            (true , true , true , true , false, false) => self.make_move_generic::<True , True , True , True , False, False>(r#move),
            (true , true , true , true , false, true ) => self.make_move_generic::<True , True , True , True , False, True >(r#move),
            (true , true , true , true , true , false) => self.make_move_generic::<True , True , True , True , True , False>(r#move),
            (true , true , true , true , true , true ) => self.make_move_generic::<True , True , True , True , True , True >(r#move)
        }

    }

    fn unmake_move_generic<
        WhitesTurn: Bool, IsCapture: Bool, IsEnPassant: Bool, IsCastling: Bool, IsPromotion: Bool
    >(self: &mut Self, r#move: Move, info: UnmakeInformation) {
        // unmakes the most recent move on the move stack

        /*
        Strategy:
            - Unpack information from history stack and apply to board
            - Do the steps of make in reverse direction (to avoid issues arising from non-commutative operations)
            - ignore fields for which we the final state in the history stack
        */

        self.castle_permissions = info.castle_permissions;
        self.en_passant_square = info.en_passant_square;
        self.fifty_move_counter = info.fifty_move_counter;
        self.zobrist_hash = info.zobrist_hash;

        let from_square = r#move.from_square();
        let to_square = r#move.to_square();
        let moving_piece = r#move.moving_piece();

        // swap players
        self.whites_turn ^= true;

        // handle promotion
        if IsPromotion::AS_BOOL {
            // add pawn onto to-square (except for own mask and occupation)
            self.get_bitboard(moving_piece).set_bit(to_square);

            // remove new piece
            let promoted_to = r#move.promoted_to();
            self.get_bitboard(promoted_to).clear_bit(to_square);
            self.square_piece_mapping[to_square as usize] = Piece::None;
        }

        // handle rook move for castling
        if IsCastling::AS_BOOL {
            let (rook, (rook_from, rook_to)) = {
                if WhitesTurn::Not::AS_BOOL {
                    (
                        Piece::WhiteRook,
                        if to_square == Square::G1 {
                            (Square::H1, Square::F1)
                        } else {
                            (Square::A1, Square::D1)
                        }
                    )
                } else {
                    (
                        Piece::BlackRook,
                        if to_square == Square::G8 {
                            (Square::H8, Square::F8)
                        } else {
                            (Square::A8, Square::D8)
                        }
                    )
                }
            } ;

            let rook_bitboard = self.get_bitboard(rook);

            rook_bitboard.clear_bit(rook_to);
            rook_bitboard.set_bit(rook_from);

            self.own_mask_mut::<WhitesTurn::Not>().clear_bit(rook_to);
            self.own_mask_mut::<WhitesTurn::Not>().set_bit(rook_from);

            self.occupation.clear_bit(rook_to);
            self.occupation.set_bit(rook_from);

            self.square_piece_mapping[rook_from as usize] = rook;
            self.square_piece_mapping[rook_to as usize] = Piece::None;
        }


        // add captured piece
        if IsCapture::AS_BOOL {
            let captured_piece = r#move.captured_piece();

            if IsEnPassant::AS_BOOL {
                // is en-passant the taken piece is one behind (from owns perspective) the taken pawn
                let square_of_taken_piece = to_square.advance_square(!WhitesTurn::Not::AS_BOOL);
                self.get_bitboard(captured_piece).set_bit(square_of_taken_piece);
                self.enemy_mask_mut::<WhitesTurn::Not>().set_bit(square_of_taken_piece);
                self.occupation.set_bit(square_of_taken_piece);
                self.square_piece_mapping[square_of_taken_piece as usize] = captured_piece;
            } else {
                // if not en-passant, the taken square is on the to-square of the move
                // clear piece from own bitboard and enemy mask, but not from occupation!
                self.get_bitboard(captured_piece).set_bit(to_square);
                self.enemy_mask_mut::<WhitesTurn::Not>().set_bit(to_square);
                self.square_piece_mapping[to_square as usize] = captured_piece;
            }
        }


        // move piece on own bitboard
        {
            let moving_bitboard = self.get_bitboard(moving_piece);
            moving_bitboard.clear_bit(to_square);
            moving_bitboard.set_bit(from_square);
        }

        // move piece on own mask
        self.own_mask_mut::<WhitesTurn::Not>().clear_bit(to_square);
        self.own_mask_mut::<WhitesTurn::Not>().set_bit(from_square);

        // move piece on occupation
        if !IsCapture::AS_BOOL || IsEnPassant::AS_BOOL {
            self.occupation.clear_bit(to_square);
        }
        self.occupation.set_bit(from_square);

        // move piece in square-piece mapping
        self.square_piece_mapping[from_square as usize] = moving_piece;
        if !IsCapture::AS_BOOL || IsEnPassant::AS_BOOL {
            // only kill piece on square if we haven't placed a piece there because it was captured
            self.square_piece_mapping[to_square as usize] = Piece::None;
        }
    }

    pub fn unmake_move(self: &mut Self) {

        // extract last piece of history and corresponding move
        let info = self.history.pop().expect("No more moves to undo!");
        let r#move = info.r#move;

        // find generics
        let generics = (
            self.whites_turn,
            r#move.is_capture(),
            r#move.is_en_passant(),
            r#move.is_castling(),
            r#move.is_promotion()
        );

        // find correct impl of unmake_move
        match generics {
            (false, false, false, false, false) => self.unmake_move_generic::<False, False, False, False, False>(r#move, info),
            (false, false, false, false, true ) => self.unmake_move_generic::<False, False, False, False, True >(r#move, info),
            (false, false, false, true , false) => self.unmake_move_generic::<False, False, False, True , False>(r#move, info),
            (false, false, false, true , true ) => self.unmake_move_generic::<False, False, False, True , True >(r#move, info),
            (false, false, true , false, false) => self.unmake_move_generic::<False, False, True , False, False>(r#move, info),
            (false, false, true , false, true ) => self.unmake_move_generic::<False, False, True , False, True >(r#move, info),
            (false, false, true , true , false) => self.unmake_move_generic::<False, False, True , True , False>(r#move, info),
            (false, false, true , true , true ) => self.unmake_move_generic::<False, False, True , True , True >(r#move, info),
            (false, true , false, false, false) => self.unmake_move_generic::<False, True , False, False, False>(r#move, info),
            (false, true , false, false, true ) => self.unmake_move_generic::<False, True , False, False, True >(r#move, info),
            (false, true , false, true , false) => self.unmake_move_generic::<False, True , False, True , False>(r#move, info),
            (false, true , false, true , true ) => self.unmake_move_generic::<False, True , False, True , True >(r#move, info),
            (false, true , true , false, false) => self.unmake_move_generic::<False, True , True , False, False>(r#move, info),
            (false, true , true , false, true ) => self.unmake_move_generic::<False, True , True , False, True >(r#move, info),
            (false, true , true , true , false) => self.unmake_move_generic::<False, True , True , True , False>(r#move, info),
            (false, true , true , true , true ) => self.unmake_move_generic::<False, True , True , True , True >(r#move, info),
            (true , false, false, false, false) => self.unmake_move_generic::<True , False, False, False, False>(r#move, info),
            (true , false, false, false, true ) => self.unmake_move_generic::<True , False, False, False, True >(r#move, info),
            (true , false, false, true , false) => self.unmake_move_generic::<True , False, False, True , False>(r#move, info),
            (true , false, false, true , true ) => self.unmake_move_generic::<True , False, False, True , True >(r#move, info),
            (true , false, true , false, false) => self.unmake_move_generic::<True , False, True , False, False>(r#move, info),
            (true , false, true , false, true ) => self.unmake_move_generic::<True , False, True , False, True >(r#move, info),
            (true , false, true , true , false) => self.unmake_move_generic::<True , False, True , True , False>(r#move, info),
            (true , false, true , true , true ) => self.unmake_move_generic::<True , False, True , True , True >(r#move, info),
            (true , true , false, false, false) => self.unmake_move_generic::<True , True , False, False, False>(r#move, info),
            (true , true , false, false, true ) => self.unmake_move_generic::<True , True , False, False, True >(r#move, info),
            (true , true , false, true , false) => self.unmake_move_generic::<True , True , False, True , False>(r#move, info),
            (true , true , false, true , true ) => self.unmake_move_generic::<True , True , False, True , True >(r#move, info),
            (true , true , true , false, false) => self.unmake_move_generic::<True , True , True , False, False>(r#move, info),
            (true , true , true , false, true ) => self.unmake_move_generic::<True , True , True , False, True >(r#move, info),
            (true , true , true , true , false) => self.unmake_move_generic::<True , True , True , True , False>(r#move, info),
            (true , true , true , true , true ) => self.unmake_move_generic::<True , True , True , True , True >(r#move, info),
        }
    }

    const PLAYER_CHARS: [char; 2] = ['b', 'w'];
    pub fn visualize(self: &Self) {
        // visualize the given board on console

        println!();

        print!("    ");
        for file in 0..8 {
            print!(" {} ", Square::FILE_CHARS[file])
        }

        print!("\n    ");
        for _ in 0..7 {
            print!("___");
        }
        print!("___");

        println!();

        for rank in (0u8..8).rev() {
            print!("{}  |", Square::RANK_CHARS[rank as usize]);
            for file in 0u8..8 {
                let square = Square::from_file_and_rank(file, rank);
                let piece = self.piece_at(square);
                piece.visualize();  // Piece::None is handled
            }
            print!("|\n");
        }

        println!();

        print!("side to move: {}", Self::PLAYER_CHARS[self.whites_turn as usize]);

        print!("\nen passant on: ");
        match self.en_passant_square {
            None => print!("-"),
            Some(square) => square.visualize()
        }

        print!("\ncastle permissions: ");
        self.castle_permissions.visualize();

        print!("\nboard key: ");
        self.zobrist_hash.visualize();

        /*print!("\npoly key: {:x?}", polykey_from_board(board));*/

        print!("\n");
    }
}


impl AlphaBetaSearchFunctionality for Board {
    type Move = Move;
    type ZobristHash = ZobristHash;

    #[inline(always)]
    fn is_whites_turn(self: &Self) -> bool {
        self.whites_turn
    }

    #[inline(always)]
    fn make_move(self: &mut Self, r#move: Self::Move) {
        self.make_move(r#move)
    }

    #[inline(always)]
    fn unmake_move(self: &mut Self) {
        self.unmake_move()
    }

    #[inline(always)]
    fn evaluate(self: &Self) -> i32 {
        self.evaluate()
    }

    fn is_check(self: &Self) -> bool {
        let (_, n_checkers) = match self.whites_turn {
            false => self.get_checkmask_and_number_of_checkers::<False>(),
            true  => self.get_checkmask_and_number_of_checkers::<True >()
        };
        !(n_checkers == 0)
    }

    #[inline(always)]
    fn zobrist_hash(self: &Self) -> Self::ZobristHash {
        self.zobrist_hash
    }

    #[inline(always)]
    fn legal_moves(self: &Self) -> Vec<Self::Move> {
        self.get_legal_moves()
    }

    #[inline(always)]
    fn loud_moves(self: &mut Self) -> Vec<Self::Move> {
        // TODO: Handle this better (directly in legal move generation)
        self.legal_moves()
            .into_iter()
            .filter(|r#move| r#move.is_capture() || r#move.is_promotion())
            .collect()
    }
    #[inline(always)]
    fn last_move(self: &Self) -> Option<Self::Move> {
        // TODO: own move stack in SearchInfo?
        match self.history.last() {
            None => None,
            Some(info) => Some(info.r#move)
        }
    }
}
