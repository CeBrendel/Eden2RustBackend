
use generic_magic::{Bool, False, True};
use crate::query_stop;
use crate::search_info::SearchInfo;
use crate::traits::AlphaBetaSearchFunctionality;


pub fn alpha_beta_search<
    T: AlphaBetaSearchFunctionality,
    CountNodes: Bool,
    CheckStop: Bool
>(board: &mut T, max_depth: u8, search_info: &mut SearchInfo) -> (Option<T::Move>, f32) {
    // performs a (modified) alpha-beta search on the given board

    /*
    TODO:
        - extensions
        - quiescence search
        - only top layer should deal with best moves
        - principal variation?
    */

    fn inner_alpha_beta<
        T: AlphaBetaSearchFunctionality,
        IsMaximizer: Bool,
        CountNodes: Bool,
        CheckStop: Bool
    >(
        board: &mut T,
        depth: u8, mut alpha: f32,
        mut beta: f32,
        search_info: &mut SearchInfo
    ) -> (Option<T::Move>, f32) {

        // base case
        if depth == 0 {
            return (None, board.evaluate());
        }

        let legal_moves = board.legal_moves();

        // is terminal?
        if legal_moves.len() == 0 {
            return (None, board.evaluate());
        }

        if IsMaximizer::AS_BOOL {
            let mut max_evaluation: f32 = f32::MIN;
            let mut best_move: Option<T::Move> = None;

            // recursively search children
            for r#move in legal_moves {

                // maybe count searched nodes
                if CountNodes::AS_BOOL || CheckStop::AS_BOOL {
                    search_info.nodes_searched += 1;
                }

                // evaluate recursively
                board.make_move(r#move);
                let (_, child_evaluation) = inner_alpha_beta::<
                    T, IsMaximizer::Not, CountNodes, CheckStop
                >(
                    board, depth - 1, alpha, beta, search_info
                );
                board.unmake_move();

                // maybe check for stop signal
                if CheckStop::AS_BOOL {
                    if search_info.nodes_searched % 2048 == 0 {
                        if query_stop() {
                            return (None, 0.)
                        }
                    }
                }

                if child_evaluation > max_evaluation {
                    max_evaluation = child_evaluation;
                    best_move = Some(r#move);
                }

                // check for beta cutoff
                if max_evaluation >= beta {
                    break;
                }

                if max_evaluation > alpha {
                    alpha = max_evaluation
                }
            }

            return (best_move, max_evaluation);

        } else {

            let mut min_evaluation: f32 = f32::MAX;
            let mut best_move: Option<T::Move> = None;

            // recursively search children
            for r#move in legal_moves {

                // maybe count searched nodes
                if CountNodes::AS_BOOL || CheckStop::AS_BOOL {
                    search_info.nodes_searched += 1;
                }

                // evaluate recursively
                board.make_move(r#move);
                let (_, child_evaluation) = inner_alpha_beta::<
                    T, IsMaximizer::Not, CountNodes, CheckStop
                >(
                    board, depth - 1, alpha, beta, search_info
                );
                board.unmake_move();

                // maybe check for stop signal
                if CheckStop::AS_BOOL {
                    if search_info.nodes_searched % 2048 == 0 {
                        if query_stop() {
                            return (None, 0.)
                        }
                    }
                }

                if child_evaluation < min_evaluation {
                    min_evaluation = child_evaluation;
                    best_move = Some(r#move);
                }

                // check for alpha cutoff
                if min_evaluation <= alpha {
                    break;
                }

                if min_evaluation < beta {
                    beta = min_evaluation;
                }
            }

            return (best_move, min_evaluation);
        }
    }

    return match board.is_whites_turn() {
        false => inner_alpha_beta::<T, False, CountNodes, CheckStop>(
            board, max_depth, f32::MIN, f32::MAX, search_info
        ),
        true  => inner_alpha_beta::<T, True , CountNodes, CheckStop>(
            board, max_depth, f32::MIN, f32::MAX, search_info
        )
    }
}