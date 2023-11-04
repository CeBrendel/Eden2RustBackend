
use crate::traits::QuiescenceSearchFunctionality;


pub fn quiescence_search<T: QuiescenceSearchFunctionality>(board: &mut T) -> f32 {
    /*
    TODO:
        - what about checks & promotions? Limit depth to avoid repeated checks (return value 0?).
        - standing pat theoretically NOT sound in Zugzwang (like check) i. e. I am not confident in the implementation!
    */

    fn inner<
        T: QuiescenceSearchFunctionality
    >(board: &mut T, mut alpha: f32, beta: f32, node_count: &mut usize) -> f32 {

        *node_count += 1;


        let standing_pat = board.evaluate();
        if standing_pat >= beta {
            return beta;
        } else if standing_pat > alpha {
            alpha = standing_pat;
        }

        for r#move in board.loud_moves() {
            board.make_move(r#move);
            let score = - inner(board, -beta, -alpha, node_count);
            board.unmake_move();

            if score > alpha {
                if score >= beta {
                    return beta
                }
                alpha = score
            }
        }

        return alpha;
    }

    let node_count = &mut 0usize;

    let score = inner(board, f32::MIN, f32::MAX, node_count);

    println!("Nodes visited: {}", node_count);

    return score;
}