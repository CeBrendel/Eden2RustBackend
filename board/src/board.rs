
use bitboards::{Bitboard, squares::Square};
use search::alpha_beta_search::AlphaBetaSearchFunctionality;
use crate::castle_permissions::CastlePermissions;
use crate::pieces::Piece;
use crate::moves::Move;
use crate::perft::PerftFunctionality;
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
    pub fn own_pawn(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhitePawn} else {Piece::BlackPawn}
    }

    #[inline(always)]
    pub fn enemy_pawn(self: &Self) -> Piece {
        if self.whites_turn {Piece::BlackPawn} else {Piece::WhitePawn}
    }

    #[inline(always)]
    pub fn own_knight(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhiteKnight} else {Piece::BlackKnight}
    }

    #[inline(always)]
    pub fn own_bishop(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhiteBishop} else {Piece::BlackBishop}
    }

    #[inline(always)]
    pub fn own_rook(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhiteRook} else {Piece::BlackRook}
    }

    #[inline(always)]
    pub fn enemy_rook(self: &Self) -> Piece {
        if self.whites_turn {Piece::BlackRook} else {Piece::WhiteRook}
    }

    #[inline(always)]
    pub fn own_queen(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhiteQueen} else {Piece::BlackQueen}
    }

    #[inline(always)]
    pub fn own_king(self: &Self) -> Piece {
        if self.whites_turn {Piece::WhiteKing} else {Piece::BlackKing}
    }

    #[inline(always)]
    pub fn own_pawns(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_pawns} else {self.black_pawns}
    }

    #[inline(always)]
    pub fn enemy_pawns(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_pawns} else {self.white_pawns}
    }

    #[inline(always)]
    pub fn own_knights(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_knights} else {self.black_knights}
    }

    #[inline(always)]
    pub fn enemy_knights(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_knights} else {self.white_knights}
    }

    #[inline(always)]
    pub fn own_bishops(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_bishops} else {self.black_bishops}
    }

    #[inline(always)]
    pub fn enemy_bishops(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_bishops} else {self.white_bishops}
    }

    #[inline(always)]
    pub fn own_rooks(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_rooks} else {self.black_rooks}
    }

    #[inline(always)]
    pub fn enemy_rooks(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_rooks} else {self.white_rooks}
    }

    #[inline(always)]
    pub fn own_queens(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_queens} else {self.black_queens}
    }

    #[inline(always)]
    pub fn enemy_queens(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_queens} else {self.white_queens}
    }

    #[inline(always)]
    pub fn own_kings(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_king} else {self.black_king}
    }

    #[inline(always)]
    pub fn enemy_kings(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_king} else {self.white_king}
    }

    #[inline(always)]
    pub fn own_mask(self: &Self) -> Bitboard {
        if self.whites_turn {self.white_mask} else {self.black_mask}
    }

    #[inline(always)]
    pub fn own_mask_mut(self: &mut Self) -> &mut Bitboard {
        if self.whites_turn {&mut self.white_mask} else {&mut self.black_mask}
    }

    #[inline(always)]
    pub fn enemy_mask(self: &Self) -> Bitboard {
        if self.whites_turn {self.black_mask} else {self.white_mask}
    }

    #[inline(always)]
    pub fn enemy_mask_mut(self: &mut Self) -> &mut Bitboard {
        if self.whites_turn {&mut self.black_mask} else {&mut self.white_mask}
    }

    pub fn get_bitboard(self: &mut Self, piece: Piece) -> &mut Bitboard {
        // TODO: inefficient, maybe replace by two functions "get_bitboard_own" and "get_bitboard_enemy"
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
        let move_counter_block = *blocks.get(5).expect("Invalid FEN!");

        // handle pieces
        let rank_strs: Vec<&str> = piece_block.split('/').rev().collect();

        let mut rank: u8 = 0;
        for rank_str in rank_strs {

            let mut chars: Vec<char> = rank_str.chars().collect();

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

    pub fn make_move(self: &mut Self, r#move: Move) {
        // make the given move on the board

        let from_square = r#move.from_square();
        let to_square = r#move.to_square();
        let moving_piece = r#move.moving_piece();
        let is_capture = r#move.is_capture();
        let is_en_passant = r#move.is_en_passant();
        let is_castling = r#move.is_castling();

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
        if moving_piece == self.own_king() {
            // if we castle we can no longer castle
            self.castle_permissions.remove_rights(self.whites_turn);
            if self.whites_turn {
                self.zobrist_hash.hash_white_short();
                self.zobrist_hash.hash_white_long();
            } else {
                self.zobrist_hash.hash_black_short();
                self.zobrist_hash.hash_black_long();
            }
        }

        // if our own rook moves, we can no longer castle in this direction
        // TODO: Kill this check with const-ness
        if moving_piece == self.own_rook() {
            if self.whites_turn {
                if self.castle_permissions.has_white_short() && (from_square == Square::H1) {
                    self.castle_permissions.remove_short_rights(self.whites_turn);
                    self.zobrist_hash.hash_white_short();
                } else if self.castle_permissions.has_white_long() && (from_square == Square::A1) {
                    self.castle_permissions.remove_long_rights(self.whites_turn);
                    self.zobrist_hash.hash_white_long();
                }
            } else {
                if self.castle_permissions.has_black_short() && (from_square == Square::H8) {
                    self.castle_permissions.remove_short_rights(self.whites_turn);
                    self.zobrist_hash.hash_black_short();
                } else if self.castle_permissions.has_black_long() && (from_square == Square::A8) {
                    self.castle_permissions.remove_long_rights(self.whites_turn);
                    self.zobrist_hash.hash_black_long();
                }
            }
        }

        // if rook is taken, enemy can't castle on that side anymore
        if is_capture {
            if r#move.captured_piece() == self.enemy_rook() {
                if self.whites_turn {
                    if self.castle_permissions.has_black_short() && (to_square == Square::H8) {
                        self.castle_permissions.remove_short_rights(!self.whites_turn);
                        self.zobrist_hash.hash_black_short();
                    } else if self.castle_permissions.has_black_long() && (to_square == Square::A8) {
                        self.castle_permissions.remove_long_rights(!self.whites_turn);
                        self.zobrist_hash.hash_black_long();
                    }
                } else {
                    if self.castle_permissions.has_white_short() && (to_square == Square::H1) {
                        self.castle_permissions.remove_short_rights(!self.whites_turn);
                        self.zobrist_hash.hash_white_short();
                    } else if self.castle_permissions.has_white_long() && (to_square == Square::A1) {
                        self.castle_permissions.remove_long_rights(!self.whites_turn);
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
            if r#move.is_pawn_start() {
                // if we have a pawn start, add an en-passant square and hash it in
                let square = r#move.from_square().advance_square(self.whites_turn);
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
        self.own_mask_mut().clear_bit(from_square);
        self.own_mask_mut().set_bit(to_square);

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
        if is_capture {
            let captured_piece = r#move.captured_piece();

            if is_en_passant {
                // is en-passant the taken piece is one behind (from owns perspective) the taken pawn
                let square_of_taken_piece = to_square.advance_square(!self.whites_turn);
                self.get_bitboard(captured_piece).clear_bit(square_of_taken_piece);
                self.enemy_mask_mut().clear_bit(square_of_taken_piece);
                self.occupation.clear_bit(square_of_taken_piece);
                self.zobrist_hash.hash_piece(captured_piece, square_of_taken_piece);
                self.square_piece_mapping[square_of_taken_piece as usize] = Piece::None;
            } else {
                // if not en-passant, the taken square is on the to-square of the move
                // clear piece from own bitboard and enemy mask, but not from occupation or square-piece-mapping!
                self.get_bitboard(captured_piece).clear_bit(to_square);
                self.enemy_mask_mut().clear_bit(to_square);
                self.zobrist_hash.hash_piece(captured_piece, to_square);
            }
        }

        // handle rook move for castling
        if is_castling {
            let (rook, (rook_from, rook_to)) = {
                if self.whites_turn {
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

            self.own_mask_mut().clear_bit(rook_from);
            self.own_mask_mut().set_bit(rook_to);

            self.occupation.clear_bit(rook_from);
            self.occupation.set_bit(rook_to);

            self.zobrist_hash.hash_piece(rook, rook_from);
            self.zobrist_hash.hash_piece(rook, rook_to);

            self.square_piece_mapping[rook_from as usize] = Piece::None;
            self.square_piece_mapping[rook_to as usize] = rook;
        }

        // handle promotion
        if r#move.is_promotion() {
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
            if is_capture | (moving_piece == self.own_pawn()) {
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

    pub fn unmake_move(self: &mut Self) {
        // unmakes the most recent move on the move stack

        /*
        Strategy:
            - Unpack information from history stack and apply to board
            - Do the steps of make in reverse direction (to avoid issues arising from non-commutative operations)
            - ignore fields for which we the final state in the history stack
        */

        let info = self.history.pop().expect("No more moves to undo!");

        // extract move and apply information
        let r#move = info.r#move;
        self.castle_permissions = info.castle_permissions;
        self.en_passant_square = info.en_passant_square;
        self.fifty_move_counter = info.fifty_move_counter;
        self.zobrist_hash = info.zobrist_hash;

        let from_square = r#move.from_square();
        let to_square = r#move.to_square();
        let moving_piece = r#move.moving_piece();
        let is_capture = r#move.is_capture();
        let is_en_passant = r#move.is_en_passant();
        let is_castling = r#move.is_castling();

        // swap players
        self.whites_turn ^= true;

        // handle promotion
        if r#move.is_promotion() {
            // add pawn onto to-square (except for own mask and occupation)
            self.get_bitboard(moving_piece).set_bit(to_square);

            // remove new piece
            let promoted_to = r#move.promoted_to();
            self.get_bitboard(promoted_to).clear_bit(to_square);
            self.square_piece_mapping[to_square as usize] = Piece::None;
        }

        // handle rook move for castling
        if is_castling {
            let (rook, (rook_from, rook_to)) = {
                if self.whites_turn {
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

            self.own_mask_mut().clear_bit(rook_to);
            self.own_mask_mut().set_bit(rook_from);

            self.occupation.clear_bit(rook_to);
            self.occupation.set_bit(rook_from);

            self.square_piece_mapping[rook_from as usize] = rook;
            self.square_piece_mapping[rook_to as usize] = Piece::None;
        }


        // add captured piece
        if is_capture {
            let captured_piece = r#move.captured_piece();

            if is_en_passant {
                // is en-passant the taken piece is one behind (from owns perspective) the taken pawn
                let square_of_taken_piece = to_square.advance_square(!self.whites_turn);
                self.get_bitboard(captured_piece).set_bit(square_of_taken_piece);
                self.enemy_mask_mut().set_bit(square_of_taken_piece);
                self.occupation.set_bit(square_of_taken_piece);
                self.square_piece_mapping[square_of_taken_piece as usize] = captured_piece;
            } else {
                // if not en-passant, the taken square is on the to-square of the move
                // clear piece from own bitboard and enemy mask, but not from occupation!
                self.get_bitboard(captured_piece).set_bit(to_square);
                self.enemy_mask_mut().set_bit(to_square);
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
        self.own_mask_mut().clear_bit(to_square);
        self.own_mask_mut().set_bit(from_square);

        // move piece on occupation
        if !is_capture || is_en_passant {
            self.occupation.clear_bit(to_square);
        }
        self.occupation.set_bit(from_square);

        // move piece in square-piece mapping
        self.square_piece_mapping[from_square as usize] = moving_piece;
        if !is_capture || is_en_passant {
            // only kill piece on square if we haven't placed a piece there because it was captured
            self.square_piece_mapping[to_square as usize] = Piece::None;
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
            None                 => print!("-"),
            Some(square) => square.visualize()
        }

        print!("\ncastle permissions: ");
        self.castle_permissions.visualize();

        print!("\nboard key: ");
        self.zobrist_hash.visualize();

        /*print!("\npoly key: {:x?}", polykey_from_board(board));*/

        print!("\n");
    }


    pub fn evaluate(self: &Self) -> f32 {
        // TODO: move to quotient space? A 300cp advantage is worth more, the less pieces there are on the board.
        let is_checkmate: bool = false;  // TODO!
        return if is_checkmate {
            f32::MIN
        } else {
            let white_material = (
                self.white_pawns.count_ones()
                + 3*self.white_knights.count_ones()
                + 3*self.white_bishops.count_ones()
                + 5*self.white_rooks.count_ones()
                + 9*self.white_queens.count_ones()
            ) as f32;
            let black_material = (
                self.black_pawns.count_ones()
                + 3*self.black_knights.count_ones()
                + 3*self.black_bishops.count_ones()
                + 5*self.black_rooks.count_ones()
                + 9*self.black_queens.count_ones()
            ) as f32;
            white_material - black_material
        }
    }
}

impl PerftFunctionality for Board {
    type Move = Move;

    fn from_fen(fen: &str) -> Self {
        Self::from_fen(fen)
    }
    fn make_move(self: &mut Self, r#move: Self::Move) {
        self.make_move(r#move)
    }
    fn unmake_move(self: &mut Self) {
        self.unmake_move()
    }
    fn get_legal_moves(self: &Self) -> Vec<Self::Move> {
        self.get_legal_moves()
    }
    fn visualize(self: &Self) {
        self.visualize()
    }
}

pub fn test_make_unmake(board: &mut Board, remaining_depth: usize, max_depth: usize) {
    if remaining_depth == 0 {
        return;
    }

    for r#move in board.get_legal_moves() {
        if remaining_depth == max_depth {print!("At:"); r#move.visualize(); print!("\n")}

        let mut copy = board.clone();

        board.make_move(r#move);
        if board.white_king.tzcnt() >= 64 || board.black_king.tzcnt() >= 64 {
            println!("AHHHH");
            r#move.visualize();

            let mut undone: usize = 0;
            copy.visualize();
            while copy.history.len() > 0 {
                copy.unmake_move();
                undone += 1;
                copy.visualize();
                if undone == 1 {
                    copy.get_legal_moves();
                }
            }
            assert!(false);
        }
        test_make_unmake(board, remaining_depth - 1, max_depth);
        board.unmake_move();

        if *board != copy {
            println!("Discrepancy!");
            r#move.visualize();
            println!();

            if board.whites_turn != copy.whites_turn {println!("p");}
            if board.white_pawns != copy.white_pawns {println!("wp");}
            if board.black_pawns != copy.black_pawns {println!("bp");}
            if board.white_knights != copy.white_knights {println!("wn");}
            if board.black_knights != copy.black_knights {println!("bn");}
            if board.white_bishops != copy.white_bishops {println!("wb");}
            if board.black_bishops != copy.black_bishops {println!("bb");}
            if board.white_rooks != copy.white_rooks {println!("wr");}
            if board.black_rooks != copy.black_rooks {println!("br");}
            if board.white_queens != copy.white_queens {println!("wq");}
            if board.black_queens != copy.black_queens {println!("bq");}
            if board.white_king != copy.white_king {println!("wk");}
            if board.black_king != copy.black_king {println!("bk");}
            if board.white_mask != copy.white_mask {println!("wm");}
            if board.black_mask != copy.black_mask {println!("bm");}
            if board.occupation != copy.occupation {println!("occ"); board.occupation.visualize(); copy.occupation.visualize();}
            if board.square_piece_mapping != copy.square_piece_mapping {println!("spm");}
            if board.castle_permissions != copy.castle_permissions {println!("Castle");}
            if board.en_passant_square != copy.en_passant_square {println!("ep sq");}
            if board.zobrist_hash != copy.zobrist_hash {println!("Hash");}
            if board.fifty_move_counter != copy.fifty_move_counter {println!("50");}
            if board.history != copy.history {println!("History");}

            println!();
            board.visualize();
            println!();
            copy.visualize();
            println!();

            assert!(false);
        }
    }
}

impl AlphaBetaSearchFunctionality for Board {
    type Move = Move;
    type ZobristHash = ZobristHash;

    fn make_move(self: &mut Self, r#move: Self::Move) {
        self.make_move(r#move)
    }
    fn unmake_move(self: &mut Self) {
        self.unmake_move()
    }
    fn evaluate(self: &Self) -> f32 {
        self.evaluate()
    }
    fn is_terminal(self: &Self) -> bool {
        let legal_moves = self.get_legal_moves();
        legal_moves.len() == 0
    }
    fn hash(self: &Self) -> Self::ZobristHash {
        self.zobrist_hash
    }
    fn get_legal_moves(self: &Self) -> Vec<Self::Move> {
        self.get_legal_moves()
    }
}