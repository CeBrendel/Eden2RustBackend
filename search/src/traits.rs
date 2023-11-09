use std::cmp::Reverse;

pub trait SearchableMove: Copy + Clone {
    fn to_string(self: &Self) -> String;
    fn score(self: &Self) -> i32;
}

pub fn sort<Move: SearchableMove>(moves: &mut Vec<Move>) {
    moves.sort_unstable_by_key(|m| Reverse(Move::score(m)))
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
    type ZobristHash;

    fn is_whites_turn(self: &Self) -> bool;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
    fn is_check(self: &Self) -> bool;
    fn hash(self: &Self) -> Self::ZobristHash;
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