
use crate::{query_stop, STOP_CHECKING_PERIOD};
use crate::optimizer_generics::Optimizer;
use crate::search_info::SearchInfo;
use crate::traits::{AlphaBetaAndQuiescenceSearchFunctionality, sort};

pub(crate) fn quiescence<
    O: Optimizer,
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
        return board.evaluate();
    }

    // recurse children
    let mut n_loud_moves: usize = 0;
    let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
    let mut loud_moves = board.loud_moves();
    sort(&mut loud_moves);
    for r#move in loud_moves {
        n_loud_moves += 1;

        // find evaluation of child
        board.make_move(r#move);
        let child_evaluation = quiescence::<
            O::Opposite, Board
        >(board, alpha, beta,depth_left-1, info);
        board.unmake_move();

        // check if search should stop
        if info.nodes_visited % STOP_CHECKING_PERIOD == 0 {
            if query_stop() {
                // println!("Quiescence: Stopped at remaining depth {}", depth_left);
                return f32::NAN;
            }
        }

        best_evaluation = O::compare_for_assign(best_evaluation, child_evaluation);

        // update alpha/beta
        if O::IS_MAXIMIZER {
            alpha = O::compare_for_assign(alpha, best_evaluation);
        } else {
            beta = O::compare_for_assign(beta, child_evaluation);
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
            if n_loud_moves == 1 {
                if O::IS_MAXIMIZER {
                    info.betas_on_first_move += 1;
                } else {
                    info.alphas_on_first_move += 1;
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
        return board.evaluate();  // TODO: Should this detect mates? If check we need to check other legal, non-loud, moves
    }

    return best_evaluation;

}
