
use generic_magic::{False, True};

use crate::{I32_NAN, MAX_QUIESCENCE_DEPTH, query_stop, STOP_CHECKING_PERIOD};
use crate::move_ordering::MoveList;
use crate::optimizer_generics::Optimizer;
use crate::search_info::SearchInfo;
use crate::traits::{AlphaBetaSearchFunctionality, SearchableMove};

pub(crate) fn quiescence<
    O: Optimizer,
    Board: AlphaBetaSearchFunctionality
>(
    board: &mut Board,
    mut alpha: i32,
    mut beta: i32,
    depth_left: u8,
    distance_to_root: i32,
    info: &mut SearchInfo<Board>
) -> i32 {

    // probe transposition table
    let is_hit: bool;
    let is_exact: bool;
    let evaluation: i32;
    let mut maybe_pv_move = None;

    if MAX_QUIESCENCE_DEPTH - depth_left < 4 {  // TODO: Very non-canonical
        (is_hit, is_exact, evaluation, maybe_pv_move) = info.transposition_table.query::<
            True  // CalledInQuiescence: Bool
        >(board, alpha, beta, depth_left, distance_to_root);

        if is_hit {
            info.n_transposition_hits += 1;
            info.n_transposition_hits_in_quiescence += 1;

            if is_exact {
                info.thereof_exact += 1;
                info.thereof_exact_in_quiescence += 1;
            }

            return evaluation;
        }
    }

    // TODO: Case is never reached when doing a full Quiescence search! Remove depth
    /*// base case
    if depth_left == 0 {
        info.leaves_evaluated += 1;
        return board.evaluate();
    }*/

    // standing pat / base case, TODO: Remember cuts? What should happen to the ofm counter?
    let standing_pat = board.evaluate();
    if O::IS_MAXIMIZER {
        if standing_pat >= beta {
            info.leaves_evaluated += 1;
            return beta;
        }
        if alpha < standing_pat {
            alpha = standing_pat
        }
    } else {
        if standing_pat <= alpha {
            info.leaves_evaluated += 1;
            return alpha
        }
        if beta > standing_pat {
            beta = standing_pat
        }
    }

    // get loud moves
    let loud_moves = MoveList::new::<True/*OnlyLoud*/, True/*HasLastMove*/>(
        board.loud_moves(), maybe_pv_move, board.last_move(), &info.history_heuristic
    );

    // recurse children
    let mut n_loud_moves: usize = 0;
    let mut best_evaluation: i32 = if O::IS_MAXIMIZER {i32::MIN} else {i32::MAX};
    let mut best_move: Option<Board::Move> = None;
    for r#move in loud_moves {
        n_loud_moves += 1;

        // find evaluation of child
        board.make_move(r#move);
        let child_evaluation = quiescence::<
            O::Opposite, Board
        >(board, alpha, beta, depth_left-1, distance_to_root+1, info);
        board.unmake_move();

        // check if search should stop
        if info.nodes_visited % STOP_CHECKING_PERIOD == 0 {
            if query_stop() {
                return I32_NAN;
            }
        }

        if O::compare(best_evaluation, child_evaluation) {
            best_evaluation = child_evaluation;
            best_move = Some(r#move);
        }

        // update alpha/beta
        if O::IS_MAXIMIZER {
            alpha = O::compare_for_assign(alpha, best_evaluation);
        } else {
            beta = O::compare_for_assign(beta, child_evaluation);
        }

        // cutoff
        if alpha >= beta {

            // store in transposition table
            info.transposition_table.put::<
                False,  // FromAlphaBeta: Bool
                True  // FromQuiescence: Bool
            >(
                board, depth_left, distance_to_root,
                if O::IS_MAXIMIZER {beta} else {alpha},
                false, !O::IS_MAXIMIZER, O::IS_MAXIMIZER,
                Some(r#move),  // TODO: Don't remember cutoff move?
            );

            // remember cutoff
            if O::IS_MAXIMIZER {
                info.n_beta_cutoffs += 1;
            } else {
                info.n_alpha_cutoffs += 1;
            }

            // is the cutoff on first searched move?
            if n_loud_moves == 1 {
                if O::IS_MAXIMIZER {
                    info.betas_on_first_move += 1;
                } else {
                    info.alphas_on_first_move += 1;
                }
            }

            // update history heuristic
            if r#move.is_loud() {
                if 4 >= MAX_QUIESCENCE_DEPTH - depth_left {  // TODO: Very non-canonical
                    info.history_heuristic
                        [r#move.moving_piece_as_index()]
                        [r#move.to_square_as_index()] += 1;
                }
            }

            return if O::IS_MAXIMIZER {
                beta  // beta-cutoff
            } else {
                alpha  // alpha-cutoff
            };
        }
    }

    // count visited nodes
    info.nodes_visited += n_loud_moves;
    info.thereof_in_quiescence += n_loud_moves;

    // check if we had no loud moves
    if n_loud_moves == 0 {
        info.leaves_evaluated += 1;
        return board.evaluate();  // TODO: Should this detect mates? If check we need to check other legal, non-loud, moves
    }

    // put in transposition table
    info.transposition_table.put::<
        False,  // FromAlphaBeta: Bool
        True  // FromQuiescence: Bool
    >(
        board, depth_left, distance_to_root,
        best_evaluation, true, false, false,
        best_move
    );

    return best_evaluation;

}
