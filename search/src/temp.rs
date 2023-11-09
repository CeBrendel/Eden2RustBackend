
use crate::traits::{SearchableMove, AlphaBetaAndQuiescenceSearchFunctionality, sort};
use generic_magic::{Bool, True, False};
use crate::query_stop;

const STOP_CHECKING_PERIOD: usize = 4096;
const MAX_QUIESCENCE_DEPTH: u8 = 4;
const MATE_EVALUATION: f32 = 30_000.;


trait Optimizer {
    const IS_MAXIMIZER: bool;
    type Opposite: Optimizer;
    fn compare(old: f32, new: f32) -> bool;
    fn compare_for_assign(old: f32, new: f32) -> f32;
}


struct Maximizer;
struct Minimizer;

impl Optimizer for Maximizer {
    const IS_MAXIMIZER: bool = true;
    type Opposite = Minimizer;
    #[inline(always)]
    fn compare(old: f32, new: f32) -> bool {
        new > old
    }
    #[inline(always)]
    fn compare_for_assign(old: f32, new: f32) -> f32 {
        old.max(new)
    }
}

impl Optimizer for Minimizer {
    const IS_MAXIMIZER: bool = false;
    type Opposite = Maximizer;
    #[inline(always)]
    fn compare(old: f32, new: f32) -> bool {
        new < old
    }
    #[inline(always)]
    fn compare_for_assign(old: f32, new: f32) -> f32 {
        old.min(new)
    }
}


pub struct SearchInfo<Move>{
    pub evaluation: f32,
    pub best_move: Option<Move>,
    pub nodes_visited: usize,
    pub thereof_in_quiescence: usize,
    pub n_alpha_cutoffs: usize,
    pub alphas_on_first_move: usize,
    pub n_beta_cutoffs: usize,
    pub betas_on_first_move: usize
}

impl<Move> Default for SearchInfo<Move> {
    fn default() -> Self {
        Self{
            evaluation: f32::NAN,
            best_move: None,
            nodes_visited: 0,
            thereof_in_quiescence: 0,
            n_alpha_cutoffs: 0,
            alphas_on_first_move: 0,
            n_beta_cutoffs: 0,
            betas_on_first_move: 0
        }
    }
}

impl<Move: SearchableMove> SearchInfo<Move> {
    pub fn visualize(self: &Self) {
        print!(
            "\n\
            Evaluation: {}, bestmove {}\n\
            Nodes searched: {}, thereof in quiescence: {}\n\
            Cutoffs:\n\
            \t(alpha) {}, ofm {}, quot {:.4}\n\
            \t(beta)  {}, ofm {}, quot {:.4}\n",
            self.evaluation, self.best_move.unwrap().to_string(),
            self.nodes_visited, self.thereof_in_quiescence,
            self.n_alpha_cutoffs, self.alphas_on_first_move, self.alphas_on_first_move as f32 / self.n_alpha_cutoffs as f32,
            self.n_beta_cutoffs, self.betas_on_first_move, self.betas_on_first_move as f32 / self.n_beta_cutoffs as f32
        );
    }
}


pub fn minimax<
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
                board, f32::MIN, f32::MAX, MAX_QUIESCENCE_DEPTH, info
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
}

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


fn quiescence<
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