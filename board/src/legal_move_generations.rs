use bitboards::{
    Bitboard,
    bitloop
};
use generic_magic::{Bool, False, True};
use lookups::{
    X_PEXT_MASK,
    PLUS_PEXT_MASK,
    KNIGHT_MASK,
    BISHOP_MASK,
    ROOK_MASK,
    KING_MASK,
    PATH_WITHOUT_END
};
use crate::moves::Move;
use crate::board::Board;

/*
TODO:
    - loud moves separately
    - move own_<<piece>> functions into crate::piece::Piece
    - handle captures und non-captures in legal move gen separately and remove Move::maybe_capture
    - version of legal move gen, that just counts
*/

impl Board {

    pub fn get_checkmask_and_number_of_checkers<WhitesTurn: Bool>(self: &Self) -> (Bitboard, usize) {
        /*
        if in check: path (well except for knights) from checker to king including checker, excluding king.
        if not: all ones.
        */
        /*
        TODO:
            - directly calculate checkmask (and if we are in check) when making a move (we want to make "is_check" const either way)?
        */

        let king_square = self.own_kings::<WhitesTurn>().tzcnt();
        let x_pext_occupancy = self.occupation.pext(X_PEXT_MASK[king_square]);
        let plus_pext_occupancy = self.occupation.pext(PLUS_PEXT_MASK[king_square]);

        let mut checkmask = Bitboard(0);
        let mut number_of_checkers = 0;

        // pawn gives check to king if it is a pawn move away from(!) the king
        checkmask |= self.enemy_pawns::<WhitesTurn>() & (
            self.own_kings::<WhitesTurn>().shift_left_pawn_attack(WhitesTurn::AS_BOOL) & Bitboard::not_left_file(!WhitesTurn::AS_BOOL)
                | self.own_kings::<WhitesTurn>().shift_right_pawn_attack(WhitesTurn::AS_BOOL) & Bitboard::not_right_file(!WhitesTurn::AS_BOOL)
        );

        // knight gives check to king if it is a knights move away from(!) the king
        checkmask |= KNIGHT_MASK[king_square] & self.enemy_knights::<WhitesTurn>();

        // pawns and knights contribute (exactly one bit) to the checkmask iff they are checkers
        number_of_checkers += checkmask.count_ones();

        /*
        - bishop or queen gives x-check if it is a x-move away from(!) the king
        - rook or plus-queen gives plus-check if it is a plus-move away from(!) the king
        - |-ed together they give a sliding checkers
         */
        let x_checkers = BISHOP_MASK[king_square][x_pext_occupancy] & (self.enemy_bishops::<WhitesTurn>() | self.enemy_queens::<WhitesTurn>());
        let plus_checkers = ROOK_MASK[king_square][plus_pext_occupancy] & (self.enemy_rooks::<WhitesTurn>() | self.enemy_queens::<WhitesTurn>());
        let sliding_checkers = x_checkers | plus_checkers;
        bitloop!(  // for each checking slider add the path from the slider to the king (excluding the king) onto the checkmask
            sliding_checkers, sliding_square => {
                number_of_checkers += 1;
                checkmask |= PATH_WITHOUT_END[sliding_square as usize][king_square];
            }
        );

        if number_of_checkers == 0 {
            checkmask = !Bitboard(0);
        } else if number_of_checkers == 2 {
            checkmask = Bitboard(0);
        }

        return (checkmask, number_of_checkers);
    }

    fn get_pinmasks<WhitesTurn: Bool>(self: &Self) -> (Bitboard, Bitboard) {
        /*
        for each (horizontal or vertical) pin the path from king to pinning piece. Path as if the
        pinned piece is removed all while excluding king, and including pinning piece
        */


        let king_square = self.own_kings::<WhitesTurn>().tzcnt();

        /*
        Calculate pinning pieces via fancy XORs. See here:
        https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)#ModifyingOccupancy
        */
        let x_pext_mask = X_PEXT_MASK[king_square];
        let plus_pext_mask = PLUS_PEXT_MASK[king_square];

        let x_attacks = BISHOP_MASK[king_square][self.occupation.pext(x_pext_mask)];
        let plus_attacks = ROOK_MASK[king_square][self.occupation.pext(plus_pext_mask)];

        let blockers = self.own_mask::<WhitesTurn>() & (x_attacks | plus_attacks);
        let non_blockers = self.occupation ^ blockers;  // remove "first level blockers" from occupation

        let x_pinners_with_semi_paths = x_attacks ^ BISHOP_MASK[king_square][non_blockers.pext(x_pext_mask)];
        let plus_pinners_with_semi_paths = plus_attacks ^ ROOK_MASK[king_square][non_blockers.pext(plus_pext_mask)];

        let x_pinners = (self.enemy_bishops::<WhitesTurn>() | self.enemy_queens::<WhitesTurn>()) & x_pinners_with_semi_paths;
        let plus_pinners = (self.enemy_rooks::<WhitesTurn>() | self.enemy_queens::<WhitesTurn>()) & plus_pinners_with_semi_paths;

        // build pinmasks from pinners
        let mut x_pinmask = Bitboard(0);
        bitloop!(
            x_pinners, pinner_square => {
                x_pinmask |= PATH_WITHOUT_END[pinner_square as usize][king_square];
            }
        );
        let mut plus_pinmask = Bitboard(0);
        bitloop!(
            plus_pinners, pinner_square => {
                plus_pinmask |= PATH_WITHOUT_END[pinner_square as usize][king_square];
            }
        );

        return (x_pinmask, plus_pinmask);
    }

    fn get_seen_squares<WhitesTurn: Bool>(self: &Self) -> Bitboard {
        /*
        The bitboard of all squares seen by the enemy. Ignores the king when calculating squares
        seen by sliders, to prevent king from escaping slider-check by stepping one square away in
        check direction.

        TODO:
            - Solution is very crude. Can we do this more efficiently?
            - directly calculate when making/unmaking a move: Allows to detect check!
        */

        let mut seen = Bitboard(0);

        // ignore king for squares seen by sliders. Prevents king from evading slider-check by stepping one square away from slider along check direction
        let occupation = self.occupation ^ self.own_kings::<WhitesTurn>();

        // pawns (left and right)
        seen |= (self.enemy_pawns::<WhitesTurn>() & Bitboard::not_left_file(!WhitesTurn::AS_BOOL)).shift_left_pawn_attack(!WhitesTurn::AS_BOOL);
        seen |= (self.enemy_pawns::<WhitesTurn>() & Bitboard::not_right_file(!WhitesTurn::AS_BOOL)).shift_right_pawn_attack(!WhitesTurn::AS_BOOL);

        // knights
        bitloop!(
            self.enemy_knights::<WhitesTurn>(), square => {
                seen |= KNIGHT_MASK[square as usize];
            }
        );

        // bishops
        bitloop!(
            self.enemy_bishops::<WhitesTurn>(), square => {
                seen |= BISHOP_MASK[square as usize][occupation.pext(X_PEXT_MASK[square as usize])];
            }
        );

        // rooks
        bitloop!(
            self.enemy_rooks::<WhitesTurn>(), square => {
                seen |= ROOK_MASK[square as usize][occupation.pext(PLUS_PEXT_MASK[square as usize])];
            }
        );

        // queens
        bitloop!(
            self.enemy_queens::<WhitesTurn>(), square => {
                seen |= BISHOP_MASK[square as usize][occupation.pext(X_PEXT_MASK[square as usize])];
                seen |= ROOK_MASK[square as usize][occupation.pext(PLUS_PEXT_MASK[square as usize])];
            }
        );

        // kings
        bitloop!(
            self.enemy_kings::<WhitesTurn>(), square => {
                seen |= KING_MASK[square as usize];
            }
        );

        return seen;
    }

    pub fn get_legal_moves(self: &Self) -> Vec<Move> {
        /*
        Get all legal moves in the current position.
        Logic heavily inspired by D. Inführs "Gigantua", see
            https://github.com/Gigantua/Gigantua
        and
            https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator
        */


        let mut moves: Vec<Move> = Vec::with_capacity(32);  // TODO: With capacity? Different type? Passed Move buffer?

        // get checkmask and number of checkers
        let (checkmask, n_checkers) = match self.whites_turn {
            false => self.get_checkmask_and_number_of_checkers::<False>(),
            true  => self.get_checkmask_and_number_of_checkers::<True>()
        };

        // call inner_get_legal_moves
        match (
            self.whites_turn,  // WhitesTurn
            n_checkers != 0,  // IsCheck
            n_checkers == 2,  // IsDoubleCheck
            self.has_en_passant_square()  // HasEnPassant
        ) {
            (false, false, false, false) => self.inner_get_legal_moves::<False, False, False, False>(checkmask, &mut moves),
            (false, false, false, true ) => self.inner_get_legal_moves::<False, False, False, True >(checkmask, &mut moves),
            (false, false, true , false) => self.inner_get_legal_moves::<False, False, True , False>(checkmask, &mut moves),
            (false, false, true , true ) => self.inner_get_legal_moves::<False, False, True , True >(checkmask, &mut moves),
            (false, true , false, false) => self.inner_get_legal_moves::<False, True , False, False>(checkmask, &mut moves),
            (false, true , false, true ) => self.inner_get_legal_moves::<False, True , False, True >(checkmask, &mut moves),
            (false, true , true , false) => self.inner_get_legal_moves::<False, True , True , False>(checkmask, &mut moves),
            (false, true , true , true ) => self.inner_get_legal_moves::<False, True , True , True >(checkmask, &mut moves),

            (true , false, false, false) => self.inner_get_legal_moves::<True , False, False, False>(checkmask, &mut moves),
            (true , false, false, true ) => self.inner_get_legal_moves::<True , False, False, True >(checkmask, &mut moves),
            (true , false, true , false) => self.inner_get_legal_moves::<True , False, True , False>(checkmask, &mut moves),
            (true , false, true , true ) => self.inner_get_legal_moves::<True , False, True , True >(checkmask, &mut moves),
            (true , true , false, false) => self.inner_get_legal_moves::<True , True , False, False>(checkmask, &mut moves),
            (true , true , false, true ) => self.inner_get_legal_moves::<True , True , False, True >(checkmask, &mut moves),
            (true , true , true , false) => self.inner_get_legal_moves::<True , True , True , False>(checkmask, &mut moves),
            (true , true , true , true ) => self.inner_get_legal_moves::<True , True , True , True >(checkmask, &mut moves),
        };

        return moves;
    }

    fn inner_get_legal_moves<
        WhitesTurn: Bool,
        IsCheck: Bool,
        IsDoubleCheck: Bool,
        HasEnPassant: Bool  // TODO: HasXPins, HasPlusPins
    >(self: &Self, checkmask: Bitboard, moves: &mut Vec<Move>) {

        let (x_pinmask, plus_pinmask) = self.get_pinmasks::<WhitesTurn>();

        let enemy = self.enemy_mask::<WhitesTurn>();
        let enemy_or_empty = !self.own_mask::<WhitesTurn>();
        let viable_squares = if IsCheck::AS_BOOL {enemy_or_empty & checkmask} else {enemy_or_empty};

        // handle pawns
        if !IsDoubleCheck::AS_BOOL {
            /*
            1. horizontally pinned pawns can't move
            2. vertically pinned pawns can move forward along the pin but cannot take sideways
            3. diagonally pinned pawns can take diagonally along the pin and cannot move forward
            4. both in case 3 & 4, the pawn can promote
            5. handle en-passant and check for special case of pinned en-passant take (which removes TWO pieces from the same rank)
            */

            // split into pawns that can move forward or sideways
            let forward_pawns = self.own_pawns::<WhitesTurn>() & !x_pinmask;
            let sideways_pawns = self.own_pawns::<WhitesTurn>() & !plus_pinmask;

            // handle occupation and checkmask for pawns that can move either 1 or 2 squares forward (joined to optimize number of CPU instructions)
            let mut single_pawns = forward_pawns & (!self.occupation).shift_backwards(WhitesTurn::AS_BOOL);  // space 1 in front is not blocked
            let mut double_pawns = single_pawns & Bitboard::home_rank(WhitesTurn::AS_BOOL) & (if IsCheck::AS_BOOL {!self.occupation & checkmask} else {!self.occupation}).shift_backwards_twice(WhitesTurn::AS_BOOL);  // is on home-rank, spaces 1 and 2 in front are not blocked, and space 2 in front is valid for blocking if in check
            if IsCheck::AS_BOOL {
                single_pawns &= checkmask.shift_backwards(WhitesTurn::AS_BOOL);  // square 1 in front is valid for blocking if in check
            }

            // handle occupation and checkmask for paws that can take to their left or right
            let viable_enemy: Bitboard = if IsCheck::AS_BOOL {enemy & checkmask} else {enemy};
            let mut left_pawns = sideways_pawns & Bitboard::not_left_file(WhitesTurn::AS_BOOL) & viable_enemy.shift_left_pawn_attack(!WhitesTurn::AS_BOOL);
            let mut right_pawns = sideways_pawns & Bitboard::not_right_file(WhitesTurn::AS_BOOL) & viable_enemy.shift_right_pawn_attack(!WhitesTurn::AS_BOOL);

            // handle pinning
            single_pawns = {
                let pinned = single_pawns & plus_pinmask.shift_backwards(WhitesTurn::AS_BOOL);  // filter pawns for which target square is on the pin
                let unpinned = single_pawns & !plus_pinmask;
                pinned | unpinned
            };
            double_pawns = {
                let pinned = double_pawns & plus_pinmask.shift_backwards_twice(WhitesTurn::AS_BOOL);  // filter pawns for which target square is on the pin
                let unpinned = double_pawns & !plus_pinmask;
                pinned | unpinned
            };
            left_pawns = {
                let pinned = left_pawns & x_pinmask.shift_left_pawn_attack(!WhitesTurn::AS_BOOL);  // filter pawns for which target square is on the pin
                let unpinned = left_pawns & !x_pinmask;
                pinned | unpinned
            };
            right_pawns = {
                let pinned = right_pawns & x_pinmask.shift_right_pawn_attack(!WhitesTurn::AS_BOOL);  // filter pawns for which target square is on the pin
                let unpinned = right_pawns & !x_pinmask;
                pinned | unpinned
            };

            // handle promotion
            if ((single_pawns | left_pawns | right_pawns) & Bitboard::home_rank(!WhitesTurn::AS_BOOL)).has_bits() {
                // we have pawns that can promote!

                // pawns that can promote
                let single_promoters = single_pawns & Bitboard::home_rank(!WhitesTurn::AS_BOOL);
                let left_promoters = left_pawns & Bitboard::home_rank(!WhitesTurn::AS_BOOL);
                let right_promoters = right_pawns & Bitboard::home_rank(!WhitesTurn::AS_BOOL);

                // register all promotions
                bitloop!(  // single promoters
                    single_promoters, square => {
                        let to_square = if WhitesTurn::AS_BOOL {square + 8} else {square - 8};
                        moves.push(Move::promotion_without_capture::<WhitesTurn>(square, to_square, self.own_queen::<WhitesTurn>(), self));
                        moves.push(Move::promotion_without_capture::<WhitesTurn>(square, to_square, self.own_knight::<WhitesTurn>(), self));
                        moves.push(Move::promotion_without_capture::<WhitesTurn>(square, to_square, self.own_rook::<WhitesTurn>(), self));
                        moves.push(Move::promotion_without_capture::<WhitesTurn>(square, to_square, self.own_bishop::<WhitesTurn>(), self));
                    }
                );
                bitloop!(  // left promoters
                    left_promoters, square => {
                        let to_square = if WhitesTurn::AS_BOOL {square + 7} else {square - 7};
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_queen::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_knight::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_rook::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_bishop::<WhitesTurn>(), self));
                    }
                );
                bitloop!(  // right promoters
                    right_promoters, square => {
                        let to_square = if WhitesTurn::AS_BOOL {square + 9} else {square - 9};
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_queen::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_knight::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_rook::<WhitesTurn>(), self));
                        moves.push(Move::promotion_with_capture::<WhitesTurn>(square, to_square, self.own_bishop::<WhitesTurn>(), self));
                    }
                );

                // filter pawns that cannot promote
                single_pawns = single_pawns & !Bitboard::home_rank(!WhitesTurn::AS_BOOL);
                left_pawns = left_pawns & !Bitboard::home_rank(!WhitesTurn::AS_BOOL);
                right_pawns = right_pawns & !Bitboard::home_rank(!WhitesTurn::AS_BOOL);
            }

            // register pawn moves (possibly after removing the promotions)
            bitloop!(  // single
                single_pawns, square => {
                    let to_square = if WhitesTurn::AS_BOOL {square + 8} else {square - 8};
                    moves.push(Move::silent(square, to_square, self.own_pawn::<WhitesTurn>(), self));
                }
            );
            /*println!("register single done");*/
            bitloop!(  // left
                left_pawns, square => {
                    let to_square = if WhitesTurn::AS_BOOL {square + 7} else {square - 7};
                    moves.push(Move::capture(square, to_square, self.own_pawn::<WhitesTurn>(), self));
                }
            );
            /*println!("register left done");*/
            bitloop!(  // right
                right_pawns, square => {
                    let to_square = if WhitesTurn::AS_BOOL {square + 9} else {square - 9};
                    moves.push(Move::capture(square, to_square, self.own_pawn::<WhitesTurn>(), self));
                }
            );
            /*println!("register right done");*/
            bitloop!(  // double
                double_pawns, square => {
                    let to_square = if WhitesTurn::AS_BOOL {square + 16} else {square - 16};
                    moves.push(Move::pawn_start::<WhitesTurn>(square, to_square, self));
                }
            );

            // handle en-passant
            if HasEnPassant::AS_BOOL {
                /*
                1. is en-passant square on checkmask? No! A pawn push from the enemy can never discover a sliding check that passes through the en-passant square (it was and isn't blocked)
                2. is the pushed pawn on the checkmask?
                3. is own pawn pinned? If it is plus-pinned it can't take en-passant, if it is x-pinned it can only take en-passant along the pin. If it is x-pinned only along one diagonal (as we don't have two kings per side)
                4. is the pushed pawn "pinned" to our king by an enemy slider? Either x or plus?
                    - x cannot happen: If I remove the pushed pawn and this leaves own king in check, then own king was in check even before the push.
                    - plus can happen: If there is an enemy plus-slider and own king on the same rank only disconnected by both pawns, own pawn cannot take en-passant as this would remove both blockers and leave own king in check <- TODO
                */

                // extract en-passant square and represent by a bitboard
                let en_passant_square = self.en_passant_square.unwrap();
                let en_passant_bitboard = Bitboard(1 << en_passant_square as usize);


                // find those pawns which can take en-passant by either taking left or right
                let mut left_and_ep = if IsCheck::AS_BOOL {
                    sideways_pawns  // pawns take can take as they are not plus-pinned (see 3.)
                        & checkmask.shift_left(WhitesTurn::AS_BOOL) // pushed pawn on checkmask? (see 2.)
                        & (
                            en_passant_bitboard & Bitboard::not_right_file(WhitesTurn::AS_BOOL)  // en-passant square that is not on the right
                        ).shift_left_pawn_attack(!WhitesTurn::AS_BOOL)
                } else {
                    sideways_pawns  // pawns take can take as they are not plus-pinned (see 3.)
                        & (
                            en_passant_bitboard & Bitboard::not_right_file(WhitesTurn::AS_BOOL)  // en-passant square that is not on the right
                        ).shift_left_pawn_attack(!WhitesTurn::AS_BOOL)
                };
                let mut right_and_ep = if IsCheck::AS_BOOL {
                    sideways_pawns  // pawns take can take as they are not plus-pinned (see 3.)
                        & checkmask.shift_right(WhitesTurn::AS_BOOL) // pushed pawn on checkmask? (see 2.)
                        & (
                            en_passant_bitboard & Bitboard::not_left_file(WhitesTurn::AS_BOOL)  // en-passant square that is not on the right
                        ).shift_right_pawn_attack(!WhitesTurn::AS_BOOL)
                } else {
                    sideways_pawns  // pawns take can take as they are not plus-pinned (see 3.)
                        & (
                            en_passant_bitboard & Bitboard::not_left_file(WhitesTurn::AS_BOOL)  // en-passant square that is not on the right
                        ).shift_right_pawn_attack(!WhitesTurn::AS_BOOL)
                };

                // handle pinning (see 3.)
                left_and_ep = {
                    let pinned = left_and_ep & x_pinmask.shift_left_pawn_attack(!WhitesTurn::AS_BOOL);  // if pinned, is to-square on pin as well?
                    let unpinned = left_and_ep & !x_pinmask;
                    pinned | unpinned
                };
                right_and_ep = {
                    let pinned = right_and_ep & x_pinmask.shift_right_pawn_attack(!WhitesTurn::AS_BOOL);  // if pinned, is to-square on pin as well?
                    let unpinned = right_and_ep & !x_pinmask;
                    pinned | unpinned
                };


                let en_passant_rank = if WhitesTurn::AS_BOOL {Bitboard(0xFF000000).flip()} else {Bitboard(0xFF000000)};
                if (en_passant_rank & self.own_kings::<WhitesTurn>()).has_bits() && (en_passant_rank & (self.enemy_rooks::<WhitesTurn>() | self.enemy_queens::<WhitesTurn>())).has_bits() {
                    // TODO: check if en-passant removes two pieces from file and leaves king in check (see 4.)
                }

                // register moves
                bitloop!(
                    left_and_ep, square => {
                        let to_square = if WhitesTurn::AS_BOOL {square + 7} else {square - 7};
                        moves.push(Move::en_passant::<WhitesTurn>(square, to_square, self));
                    }
                );
                bitloop!(
                    right_and_ep, square => {
                        let to_square = if WhitesTurn::AS_BOOL {square + 9} else {square - 9};
                        moves.push(Move::en_passant::<WhitesTurn>(square, to_square, self));
                    }
                );
            }
        }

        // handle knight moves
        if !IsDoubleCheck::AS_BOOL {
            // a knight can only move if it isn't pinned, hence remove them from consideration
            let knights = self.own_knights::<WhitesTurn>() & !(x_pinmask | plus_pinmask);
            bitloop!(
                knights, square => {
                    // find legal moves
                    let pseudo_legal_moves: Bitboard = KNIGHT_MASK[square as usize];
                    let legal_moves = pseudo_legal_moves & viable_squares;

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_knight::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );
        }

        // handle bishops
        if !IsDoubleCheck::AS_BOOL {
            // plus pinned bishops cannot move at all, hence remove them from consideration
            let bishops = self.own_bishops::<WhitesTurn>() & !plus_pinmask;

            // handle bishops that are pinned along a diagonal
            let x_pinned_bishops = bishops & x_pinmask;
            bitloop!(
                x_pinned_bishops, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = X_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = BISHOP_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares & x_pinmask;  // x-pinned bishop can only move along the pin

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_bishop::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );

            // handle bishops that are not pinned
            let unpinned_bishops = bishops & !x_pinmask;
            bitloop!(
                unpinned_bishops, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = X_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = BISHOP_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares;  // unpinned bishops are free to roam

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_bishop::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );
        }

        // handle rooks
        if !IsDoubleCheck::AS_BOOL {
            // x pinned rooks cannot move at all, hence remove them from consideration
            let rooks = self.own_rooks::<WhitesTurn>() & !x_pinmask;

            // handle pinned rooks
            let plus_pinned_rooks = rooks & plus_pinmask;
            bitloop!(
                plus_pinned_rooks, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = PLUS_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = ROOK_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares & plus_pinmask;  // plus-pinned rooks can only move along the pin

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_rook::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );

            // handle unpinned rooks
            let unpinned_rooks = rooks & !plus_pinmask;
            bitloop!(
                unpinned_rooks, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = PLUS_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = ROOK_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares;  // unpinned rooks are free to roam

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_rook::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );
        }

        // handle queens
        if !IsDoubleCheck::AS_BOOL {
            // handle x-pinned queens like bishops
            let x_pinned_queens = self.own_queens::<WhitesTurn>() & x_pinmask;
            bitloop!(
                x_pinned_queens, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = X_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = BISHOP_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares & x_pinmask;  // x-pinned queens can only move along that pin

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_queen::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );

            // handle plus-pinned queens like rooks
            let plus_pinned_queens = self.own_queens::<WhitesTurn>() & plus_pinmask;
            bitloop!(
                plus_pinned_queens, square => {
                    // find legal moves for bishop on <<square>>
                    let pext_mask: Bitboard = PLUS_PEXT_MASK[square as usize];
                    let pext_occupancy: usize = self.occupation.pext(pext_mask);
                    let pseudo_legal_moves: Bitboard = ROOK_MASK[square as usize][pext_occupancy];
                    let legal_moves: Bitboard = pseudo_legal_moves & viable_squares & plus_pinmask;  // plus-pinned queens can only move along that pin

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_queen::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );

            let unpinned_queens = self.own_queens::<WhitesTurn>() & !(x_pinmask | plus_pinmask);
            bitloop!(
                unpinned_queens, square => {

                    // find legal moves for queen as bishop on <<square>>
                    let x_pext_mask: Bitboard = X_PEXT_MASK[square as usize];
                    let x_pext_occupancy: usize = self.occupation.pext(x_pext_mask);
                    let x_pseudo_legal_moves: Bitboard = BISHOP_MASK[square as usize][x_pext_occupancy];

                    // find legal moves for queen as rook on <<square>>
                    let plus_pext_mask: Bitboard = PLUS_PEXT_MASK[square as usize];
                    let plus_pext_occupancy: usize = self.occupation.pext(plus_pext_mask);
                    let plus_pseudo_legal_moves: Bitboard = ROOK_MASK[square as usize][plus_pext_occupancy];

                    // union the x- and plus-moves
                    let legal_moves: Bitboard = (x_pseudo_legal_moves | plus_pseudo_legal_moves) & viable_squares;  // unpinned queens are free to roam in every direction

                    // register moves
                    bitloop!(
                        legal_moves, to_square => {
                            moves.push(Move::maybe_capture(square, to_square, self.own_queen::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>()));
                        }
                    );
                }
            );
        }

        // Handle king moves
        /*
        TODO:
            - only handle king moves if king isn't blocked in by own pieces? Maybe via a occupation & lookup[king_sq]
        */
        {
            // squares where the king cannot move to or move over (when castling)
            let seen_squares = self.get_seen_squares::<WhitesTurn>();

            // normal moves (possibly evading check)
            let king_square = self.own_kings::<WhitesTurn>().tzcnt();
            let legal_moves = KING_MASK[king_square] & !seen_squares & enemy_or_empty;
            bitloop!(
                legal_moves, square => {
                    moves.push(
                        Move::maybe_capture(king_square as u8, square, self.own_king::<WhitesTurn>(), self, self.enemy_mask::<WhitesTurn>())
                    );
                }
            );

            // castling short
            if self.has_short_castling_rights() {
                let must_be_empty = if WhitesTurn::AS_BOOL {Bitboard(0b01100000)} else {Bitboard(0b01100000).flip()};
                let must_not_be_seen = if WhitesTurn::AS_BOOL {Bitboard(0b01110000)} else {Bitboard(0b01110000).flip()};

                let is_empty = !(self.occupation & must_be_empty).has_bits();
                let is_not_seen = !(seen_squares & must_not_be_seen).has_bits();

                if is_empty && is_not_seen {
                    moves.push(
                        if WhitesTurn::AS_BOOL {Move::WHITE_SHORT_CASTLE} else {Move::BLACK_SHORT_CASTLE}
                    );
                }
            }

            // castling long
            if self.has_long_castling_rights() {
                let must_be_empty = if WhitesTurn::AS_BOOL {Bitboard(0b00001110)} else {Bitboard(0b00001110).flip()};
                let must_not_be_seen = if WhitesTurn::AS_BOOL {Bitboard(0b00011100)} else {Bitboard(0b00011100).flip()};

                let is_empty = !(self.occupation & must_be_empty).has_bits();
                let is_not_seen = !(seen_squares & must_not_be_seen).has_bits();

                if is_empty && is_not_seen {
                    moves.push(
                        if WhitesTurn::AS_BOOL {Move::WHITE_LONG_CASTLE} else {Move::BLACK_LONG_CASTLE}
                    );
                }
            }
        }
    }
}