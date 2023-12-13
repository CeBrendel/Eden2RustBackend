
use generic_magic::{Bool, False, True};

use crate::traits::AlphaBetaSearchFunctionality;
use crate::optimizer_generics::{Optimizer, Minimizer, Maximizer};
use crate::search_info::SearchInfo;
use crate::quiescence::quiescence;
use crate::{MAX_QUIESCENCE_DEPTH, MATE_EVALUATION};
use crate::transposition_table::TranspositionTable;


pub fn minimax<
    Board: AlphaBetaSearchFunctionality
>(board: &mut Board, max_depth: u8, transposition_table: &mut TranspositionTable<Board>) -> i32 {

    fn inner_minimax<
        O: Optimizer,
        MaxDepth: Bool,
        Board: AlphaBetaSearchFunctionality
    >(
        board: &mut Board,
        mut alpha: i32,
        mut beta: i32,
        depth_left: u8,
        info: &mut SearchInfo<Board>
    ) -> i32 {

        // base case for recursion
        if depth_left == 0 {
            return quiescence::<O, Board>(
                board, alpha, beta, MAX_QUIESCENCE_DEPTH, info
            );
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_evaluation: i32 = if O::IS_MAXIMIZER {i32::MIN} else {i32::MAX};
        for r#move in board.legal_moves() {
            n_moves += 1;

            // find evaluation of child
            board.make_move(r#move);
            let child_evaluation = inner_minimax::<
                O::Opposite,  // switch optimizer
                False,  // don't register moves in recursion
                Board
            >(board, alpha, beta, depth_left-1, info);
            board.unmake_move();

            // update belief
            if MaxDepth::AS_BOOL {
                if O::compare(best_evaluation, child_evaluation) {
                    best_evaluation = child_evaluation;
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
                0
            }
        }

        return best_evaluation;
    }

    // enter recursion
    let mut info = SearchInfo::default_from_transposition_table(transposition_table);
    let result = match board.is_whites_turn() {
        false => inner_minimax::<Minimizer, True, Board>(board, i32::MIN, i32::MAX, max_depth, &mut info),
        true  => inner_minimax::<Maximizer, True, Board>(board, i32::MIN, i32::MAX, max_depth, &mut info)
    };
    info.evaluation = result;
    info.visualize();
    return result;
}