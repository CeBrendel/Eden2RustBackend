
use crate::traits::AlphaBetaSearchFunctionality;
use generic_magic::{Bool, True, False};

trait Optimizer {
    const IS_MAXIMIZER: bool;
    type Opposite: Optimizer;
    fn criterion(old: f32, new: f32) -> bool;
}

struct Maximizer;
struct Minimizer;

impl Optimizer for Maximizer {
    const IS_MAXIMIZER: bool = true;
    type Opposite = Minimizer;
    fn criterion(old: f32, new: f32) -> bool {
        new > old
    }
}

impl Optimizer for Minimizer {
    const IS_MAXIMIZER: bool = false;
    type Opposite = Maximizer;
    fn criterion(old: f32, new: f32) -> bool {
        new < old
    }
}


#[derive(Default)]
struct SearchInfo{
    pub nodes_visited: usize,
    pub n_alpha_cutoffs: usize,
    pub n_beta_cutoffs: usize
}

impl SearchInfo {
    fn visualize(self: &Self) {
        print!(
            "\nNodes searched: {}\nCutoffs:\n\t(alpha) {},\n\t(beta)  {}\n",
            self.nodes_visited, self.n_alpha_cutoffs, self.n_beta_cutoffs
        );
    }
}


pub fn minimax<
    Board: AlphaBetaSearchFunctionality
>(board: &mut Board, max_depth: u8) -> (Option<Board::Move>, f32) {

    fn inner_minimax<
        O: Optimizer,
        ReturnMove: Bool,
        Board: AlphaBetaSearchFunctionality
    >(board: &mut Board, depth_left: u8, info: &mut SearchInfo) -> (Option<Board::Move>, f32) {

        // base case for recursion
        if depth_left == 0 {
            return (None, board.evaluate());
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_move: Option<Board::Move> = None;
        let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
        for r#move in board.get_legal_moves() {
            n_moves += 1;

            // find evaluation of child
            board.make_move(r#move);
            let (_, child_evaluation) = inner_minimax::<
                O::Opposite,  // switch optimizer
                False,  // don't return moves in recursion
                Board
            >(board, depth_left-1, info);
            board.unmake_move();

            // update belief
            if O::criterion(best_evaluation, child_evaluation) {
                best_evaluation = child_evaluation;
                if ReturnMove::AS_BOOL {
                    best_move = Some(r#move);
                }
            }
        }

        // count visited nodes
        info.nodes_visited += n_moves;

        // check for terminal state
        if n_moves == 0 {
            return (None, board.evaluate());  // TODO: This should detect mates
        }

        return (best_move, best_evaluation);
    }

    // enter recursion
    let mut info = SearchInfo::default();
    let result = match board.is_whites_turn() {
        false => inner_minimax::<Minimizer, True, Board>(board, max_depth, &mut info),
        true  => inner_minimax::<Maximizer, True, Board>(board, max_depth, &mut info)
    };
    info.visualize();
    return result;
}

pub fn alpha_beta<
    Board: AlphaBetaSearchFunctionality
>(board: &mut Board, max_depth: u8) -> (Option<Board::Move>, f32) {

    fn inner_alpha_beta<
        O: Optimizer,
        ReturnMove: Bool,
        Board: AlphaBetaSearchFunctionality
    >(
        board: &mut Board,
        mut alpha: f32,
        mut beta: f32,
        depth_left: u8,
        info: &mut SearchInfo
    ) -> (Option<Board::Move>, f32) {

        // base case
        if depth_left == 0 {
            return (None, board.evaluate());
        }

        // recurse children
        let mut n_moves: usize = 0;
        let mut best_move: Option<Board::Move> = None;
        let mut best_evaluation: f32 = if O::IS_MAXIMIZER {f32::MIN} else {f32::MAX};
        for r#move in board.get_legal_moves() {
            n_moves += 1;

            // find evaluation of child
            board.make_move(r#move);
            let (_, child_evaluation) = inner_alpha_beta::<
                O::Opposite, False, Board
            >(board, alpha, beta,depth_left-1, info);
            board.unmake_move();

            if O::criterion(best_evaluation, child_evaluation) {
                best_evaluation = child_evaluation;
                if ReturnMove::AS_BOOL {
                    best_move = Some(r#move)
                }
            }

            // update alpha/beta
            if O::IS_MAXIMIZER {
                if O::criterion(alpha, best_evaluation) {
                    alpha = best_evaluation;
                }
            } else {
                if O::criterion(beta, best_evaluation) {
                    beta = best_evaluation
                }
            }

            // cutoff
            if alpha >= beta {
                return if O::IS_MAXIMIZER {
                    info.n_beta_cutoffs += 1;
                    (None, beta)  // beta-cutoff
                } else {
                    info.n_alpha_cutoffs += 1;
                    (None, alpha)  // alpha-cutoff
                }
            }
        }

        // count visited nodes
        info.nodes_visited += n_moves;

        // check for terminal state
        if n_moves == 0 {
            return (None, board.evaluate());  // TODO: This should detect mates
        }

        return (best_move, best_evaluation);
    }

    // enter recursion
    let mut info = SearchInfo::default();
    let result = match board.is_whites_turn() {
        false => inner_alpha_beta::<Minimizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
        true  => inner_alpha_beta::<Maximizer, True, Board>(board, f32::MIN, f32::MAX, max_depth, &mut info),
    };
    info.visualize();
    return result;
}