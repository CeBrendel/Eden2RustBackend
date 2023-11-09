
use crate::traits::SearchableMove;

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
