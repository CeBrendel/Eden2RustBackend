
use generic_magic::{Bool, False, True};

use crate::optimizer_generics::{Maximizer, Minimizer, Optimizer};
use crate::traits::{AlphaBetaAndQuiescenceSearchFunctionality, SearchableMove, sort};
use crate::query_stop;
use crate::search_info::SearchInfo;
use crate::quiescence::quiescence;
use crate::{MAX_QUIESCENCE_DEPTH, MATE_EVALUATION, STOP_CHECKING_PERIOD};
use crate::transposition_table::TranspositionTable;


pub fn alpha_beta<
    'a, Board: AlphaBetaAndQuiescenceSearchFunctionality
>(
    board: &mut Board,
    max_depth: u8,
    transposition_table: &'a mut TranspositionTable<Board>
) -> SearchInfo<'a, Board> {

    fn inner_alpha_beta<
        'a,
        O: Optimizer,
        MaxDepth: Bool,
        Board: AlphaBetaAndQuiescenceSearchFunctionality
    >(
        board: &mut Board,
        mut alpha: f32,
        mut beta: f32,
        depth_left: u8,
        info: &mut SearchInfo<'a, Board>
    ) -> f32 {

        // query transposition table
        let (
            is_hit,
            is_exact,
            evaluation,
            maybe_pv_move
        ) = info.transposition_table.query::<
            True,  // CalledInAlphaBeta: Bool
            False  // CalledInQuiescence: Bool
        >(board, alpha, beta, depth_left);

        if is_hit {
            if MaxDepth::AS_BOOL {
                if is_exact {

                    info.n_transposition_hits += 1;
                    info.thereof_exact += 1;

                    info.evaluation = evaluation;
                    info.best_move = maybe_pv_move;

                    return evaluation;
                }
            }

            info.n_transposition_hits += 1;

            if is_exact {
                info.thereof_exact += 1;
            }

            return evaluation;
        }

        // base case
        if depth_left == 0 {
            return quiescence::<O, Board>(
                board, alpha, beta, MAX_QUIESCENCE_DEPTH, info
            );
        }

        // get legal moves
        let mut legal_moves = board.legal_moves();
        sort(&mut legal_moves, info);

        // handle pv move if any
        match maybe_pv_move {
            None => (),
            Some(r#move) => {
                // find position of pv move
                let index: usize = legal_moves
                    .iter()
                    .position(|&r| r == r#move)
                    .unwrap();
                // remove
                legal_moves.remove(index);
                // put at beginning
                legal_moves.insert(0, r#move);
            }
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
        let mut best_move: Option<Board::Move> = None;
        for r#move in legal_moves {
            n_moves += 1;

            // find evaluation of child
            board.make_move(r#move);
            let child_evaluation = inner_alpha_beta::<
                O::Opposite, False, Board
            >(board, alpha, beta,depth_left-1, info);
            board.unmake_move();

            // check if search should stop
            if info.nodes_visited % STOP_CHECKING_PERIOD == 0 {
                if query_stop() {
                    return f32::NAN;
                }
            }

            if O::compare(best_evaluation, child_evaluation) {
                best_evaluation = child_evaluation;
                best_move = Some(r#move);
                if MaxDepth::AS_BOOL {
                    info.evaluation = child_evaluation;
                    info.best_move = Some(r#move);
                }
            }


            // update alpha/beta
            if O::IS_MAXIMIZER {
                alpha = O::compare_for_assign(alpha, best_evaluation);
            } else {
                beta = O::compare_for_assign(beta, best_evaluation);
            }

            // cutoff
            if alpha >= beta {

                // store in transposition table
                info.transposition_table.put::<
                    True,  // FromAlphaBeta: Bool
                    False  // FromQuiescence: Bool
                >(
                    board, depth_left, best_evaluation,
                    false, !O::IS_MAXIMIZER, O::IS_MAXIMIZER,
                    None,  // TODO: Remember cutoff move?
                );

                // remember cutoff
                if O::IS_MAXIMIZER {
                    info.n_beta_cutoffs += 1;
                } else {
                    info.n_alpha_cutoffs += 1;
                }

                // is the cutoff on first searched move?
                if n_moves == 1 {
                    if O::IS_MAXIMIZER {
                        info.betas_on_first_move += 1;
                    } else {
                        info.alphas_on_first_move += 1;
                    }
                }

                // update history heuristic
                info.history_heuristic
                    [r#move.moving_piece_as_index()]
                    [r#move.to_square_as_index()] += 2 << depth_left;

                // do cutoff, TODO: should alpha/beta be flipped?
                return if O::IS_MAXIMIZER {
                    beta  // beta-cutoff
                } else {
                    alpha  // alpha-cutoff
                }
            }
        }

        // count visited nodes
        info.nodes_visited += n_moves;

        // check for terminal state, TODO: This should be stored in TT as well (same for quiescence)
        if n_moves == 0 {
            return if board.is_check() {
                // checkmate
                if O::IS_MAXIMIZER {-MATE_EVALUATION} else {MATE_EVALUATION}  // TODO: Correct orientation? Add depth offset.
            } else {
                // stalemate
                0.
            }
        }

        // put in transposition table
        info.transposition_table.put::<
            True,  // FromAlphaBeta: Bool
            False  // FromQuiescence: Bool
        >(
            board, depth_left, best_evaluation,
            true, false, false,
            best_move
        );

        return best_evaluation;
    }

    // enter recursion and time
    let mut info = SearchInfo::default_from_transposition_table(transposition_table);
    let now = std::time::Instant::now();
    match board.is_whites_turn() {
        false => inner_alpha_beta::<Minimizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
        true  => inner_alpha_beta::<Maximizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
    };
    info.time_spent_searching = now.elapsed().as_millis();

    return info;
}
