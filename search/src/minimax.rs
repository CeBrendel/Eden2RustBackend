
use generic_magic::{Bool, False, True};

use crate::traits::AlphaBetaAndQuiescenceSearchFunctionality;
use crate::optimizer_generics::{Optimizer, Minimizer, Maximizer};
use crate::search_info::SearchInfo;
use crate::quiescence::quiescence;
use crate::MAX_QUIESCENCE_DEPTH;


/*pub fn minimax<
    Board: AlphaBetaAndQuiescenceSearchFunctionality
>(board: &mut Board, max_depth: u8) -> f32 {

    fn inner_minimax<
        O: Optimizer,
        MaxDepth: Bool,
        Board: AlphaBetaAndQuiescenceSearchFunctionality
    >(
        board: &mut Board,
        depth_left: u8,
        info: &mut SearchInfo<Board::Move>
    ) -> f32 {

        // base case for recursion
        if depth_left == 0 {
            // return board.evaluate();
            return quiescence::<O, Board>(
                board, f32::MIN, f32::MAX, MAX_QUIESCENCE_DEPTH, info, trans
            );
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
        for r#move in board.legal_moves() {
            n_moves += 1;

            // find evaluation of child
            board.make_move(r#move);
            let child_evaluation = inner_minimax::<
                O::Opposite,  // switch optimizer
                False,  // don't register moves in recursion
                Board
            >(board, depth_left-1, info);
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
        }

        // count visited nodes
        info.nodes_visited += n_moves;

        // check for terminal state
        if n_moves == 0 {
            // return board.evaluate();  // TODO: This should detect mates
            return quiescence::<O, Board>(
                board, f32::MIN, f32::MAX, MAX_QUIESCENCE_DEPTH, info
            );
        }

        return best_evaluation;
    }

    // enter recursion
    let mut info = SearchInfo::default();
    let result = match board.is_whites_turn() {
        false => inner_minimax::<Minimizer, True, Board>(board, max_depth, &mut info),
        true  => inner_minimax::<Maximizer, True, Board>(board, max_depth, &mut info)
    };
    info.evaluation = result;
    info.visualize();
    return result;
}*/