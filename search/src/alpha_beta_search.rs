

pub trait AlphaBetaSearchFunctionality {
    /*
    TODO:
        - remove is_terminal function call and instead check number of legal moves directly
    */
    type Move: Copy + Clone;
    type ZobristHash;

    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
    fn is_terminal(self: &Self) -> bool;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}


pub fn alpha_beta_search<
    T: AlphaBetaSearchFunctionality
>(board: &mut T, max_depth: u8) -> (Option<T::Move>, f32) {
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
        const IS_MAXIMIZER: bool,
    >(board: &mut T, depth: u8, mut alpha: f32, mut beta: f32) -> (Option<T::Move>, f32) {

        // base case
        if depth == 0 {
            return (None, board.evaluate());
        }

        let legal_moves = board.get_legal_moves();

        // is terminal?
        if legal_moves.len() == 0 {
            return (None, board.evaluate());
        }

        if IS_MAXIMIZER {
            let mut max_evaluation: f32 = f32::MIN;
            let mut best_move: Option<T::Move> = None;

            // recursively search children
            for r#move in legal_moves {

                // evaluate recursively
                board.make_move(r#move);
                let (_, child_evaluation) = inner_alpha_beta::<T, false>(board, depth - 1, alpha, beta);
                board.unmake_move();

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

                // evaluate recursively
                board.make_move(r#move);
                let (_, child_evaluation) = inner_alpha_beta::<T, true>(board, depth - 1, alpha, beta);
                board.unmake_move();

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

    return inner_alpha_beta::<T, true>(board, max_depth, f32::MIN, f32::MAX);
}