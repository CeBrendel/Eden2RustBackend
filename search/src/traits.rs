
use std::cmp::Reverse;
use crate::search_info::SearchInfo;

pub trait SearchableMove: Copy + Clone + PartialEq {
    // for visualization
    fn to_string(self: &Self) -> String;

    // for move ordering
    fn score(self: &Self) -> i32;

    // for history heuristic:
    fn to_square_as_index(self: &Self) -> usize;
    fn moving_piece_as_index(self: &Self) -> usize;
}

pub(crate) fn sort<
    Board: AlphaBetaAndQuiescenceSearchFunctionality
>(moves: &mut Vec<Board::Move>, info: &SearchInfo<Board>) {
    // TODO: For history heuristic: Pass SearchInfo, maybe pass to Move::score
    moves.sort_unstable_by_key(
        |m| Reverse({
            Board::Move::score(m)
                + 1024 * info.history_heuristic[m.moving_piece_as_index()][m.to_square_as_index()]
        })
    );
}


/*pub(crate) struct LazySorted<Move: SearchableMove>{
    moves: Vec<Move>
}


impl<Move: SearchableMove> LazySorted<Move> {
    pub(crate) fn from_vec(v: Vec<Move>) -> Self {
        Self {
            moves: v,
        }
    }
}

impl<Move: SearchableMove> Iterator for LazySorted<Move> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {

        let n = self.moves.len();
        if n == 0 {
            return self.moves.pop();
        }

        // find best move
        let mut best_idx: usize = usize::MAX;
        let mut best_score: i32 = i32::MIN;
        for idx in 0..n {
            let score = self.moves[idx].score();
            if score > best_score {
                best_idx = idx;
                best_score = score
            }
        }

        // swap and pop
        self.moves.swap(best_idx, n-1);
        return self.moves.pop()
    }
}*/

pub trait AlphaBetaSearchFunctionality {
    type Move: SearchableMove;
    type ZobristHash: Eq;

    fn is_whites_turn(self: &Self) -> bool;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
    fn is_check(self: &Self) -> bool;
    fn zobrist_hash(self: &Self) -> Self::ZobristHash;
    fn legal_moves(self: &Self) -> Vec<Self::Move>;
}

pub trait AlphaBetaAndQuiescenceSearchFunctionality: AlphaBetaSearchFunctionality {
    fn loud_moves(self: &mut Self) -> Vec<Self::Move>;
}

/*pub trait MCTSFunctionality {
    type Move: SearchableMove;
    type ZobristHash;

    fn make_move_and_copy_and_flip(self: &Self, r#move: Self::Move) -> Self;
    fn evaluate(self: &Self) -> f32;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}*/