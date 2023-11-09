
use generic_magic::{Bool, False, True};

use crate::optimizer_generics::{Maximizer, Minimizer, Optimizer};
use crate::traits::{AlphaBetaAndQuiescenceSearchFunctionality, sort};
use crate::query_stop;
use crate::search_info::SearchInfo;
use crate::quiescence::quiescence;
use crate::{MAX_QUIESCENCE_DEPTH, MATE_EVALUATION, STOP_CHECKING_PERIOD};


pub fn alpha_beta<
    Board: AlphaBetaAndQuiescenceSearchFunctionality
>(board: &mut Board, max_depth: u8) -> SearchInfo<Board::Move> {

    fn inner_alpha_beta<
        O: Optimizer,
        MaxDepth: Bool,
        Board: AlphaBetaAndQuiescenceSearchFunctionality
    >(
        board: &mut Board,
        mut alpha: f32,
        mut beta: f32,
        depth_left: u8,
        info: &mut SearchInfo<Board::Move>
    ) -> f32 {

        // base case
        if depth_left == 0 {
            return quiescence::<O, Board>(
                board, alpha, beta, MAX_QUIESCENCE_DEPTH, info
            );
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
        let mut legal_moves = board.legal_moves();
        sort(&mut legal_moves);
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

            if MaxDepth::AS_BOOL {
                if O::compare(best_evaluation, child_evaluation) {
                    best_evaluation = child_evaluation;
                    info.evaluation = child_evaluation;
                    info.best_move = Some(r#move);
                }
            } else {
                best_evaluation = O::compare_for_assign(best_evaluation, child_evaluation);
            }


            // update alpha/beta
            if O::IS_MAXIMIZER {
                alpha = O::compare_for_assign(alpha, best_evaluation);
            } else {
                beta = O::compare_for_assign(beta, best_evaluation);
            }

            // cutoff
            if alpha >= beta {

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

                // do cutoff
                return if O::IS_MAXIMIZER {
                    beta  // beta-cutoff
                } else {
                    alpha  // alpha-cutoff
                }
            }
        }

        // count visited nodes
        info.nodes_visited += n_moves;

        // check for terminal state
        if n_moves == 0 {
            return if board.is_check() {
                // checkmate
                if O::IS_MAXIMIZER {-MATE_EVALUATION} else {MATE_EVALUATION}  // TODO: Correct orientation? Add depth offset.
            } else {
                // stalemate
                0.
            }
        }

        return best_evaluation;
    }

    // enter recursion
    let mut info = SearchInfo::default();
    match board.is_whites_turn() {
        false => inner_alpha_beta::<Minimizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
        true  => inner_alpha_beta::<Maximizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
    };
    return info;
}
