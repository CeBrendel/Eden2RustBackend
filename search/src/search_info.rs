use crate::I32_NAN;
use crate::traits::{AlphaBetaAndQuiescenceSearchFunctionality, SearchableMove};
use crate::transposition_table::TranspositionTable;


pub struct SearchInfo<'a, Board: AlphaBetaAndQuiescenceSearchFunctionality>{
    pub evaluation: i32,
    pub best_move: Option<Board::Move>,

    pub transposition_table: &'a mut TranspositionTable<Board>,

    pub time_spent_searching: u128,

    pub leaves_evaluated: usize,  // TODO: visualize

    pub nodes_visited: usize,
    pub thereof_in_quiescence: usize,
    pub n_alpha_cutoffs: usize,
    pub alphas_on_first_move: usize,
    pub n_beta_cutoffs: usize,
    pub betas_on_first_move: usize,
    pub n_transposition_hits: usize,
    pub n_transposition_hits_in_quiescence: usize,
    pub thereof_exact: usize,
    pub thereof_exact_in_quiescence: usize,

    pub history_heuristic: [[i32; 64]; 12]

}

impl<'a, Board: AlphaBetaAndQuiescenceSearchFunctionality> SearchInfo<'a, Board> {
    pub fn default_from_transposition_table(
        transposition_table: &'a mut TranspositionTable<Board>
    ) -> Self {
        Self{
            evaluation: I32_NAN,
            best_move: None,

            transposition_table,

            time_spent_searching: 0,

            leaves_evaluated: 0,

            nodes_visited: 0,
            thereof_in_quiescence: 0,

            n_alpha_cutoffs: 0,
            alphas_on_first_move: 0,
            n_beta_cutoffs: 0,
            betas_on_first_move: 0,

            n_transposition_hits: 0,
            n_transposition_hits_in_quiescence: 0,

            thereof_exact: 0,
            thereof_exact_in_quiescence: 0,

            history_heuristic: [[0; 64]; 12]
        }
    }

    pub fn visualize(self: &Self) where Board::Move: SearchableMove {
        print!(
            "\n\
            Evaluation: {}, bestmove {}\n\
            Time spent: {}ms,\n\
            Nodes searched: {}, thereof in quiescence: {}\n\
            Cutoffs:\n\
            \t(alpha) {}, ofm {}, quot {:.4}\n\
            \t(beta)  {}, ofm {}, quot {:.4}\n\
            Transposition hits:\n\
            Total:         {}, thereof exact: {}\n\
            In Quiescence: {}, thereof exact: {}\n\
            \n",
            self.evaluation, self.best_move.unwrap().to_string(),
            self.time_spent_searching,
            self.nodes_visited, self.thereof_in_quiescence,
            self.n_alpha_cutoffs, self.alphas_on_first_move, self.alphas_on_first_move as f32 / self.n_alpha_cutoffs as f32,
            self.n_beta_cutoffs, self.betas_on_first_move, self.betas_on_first_move as f32 / self.n_beta_cutoffs as f32,
            self.n_transposition_hits, self.thereof_exact,
            self.n_transposition_hits_in_quiescence, self.thereof_exact_in_quiescence
        );
    }
}
