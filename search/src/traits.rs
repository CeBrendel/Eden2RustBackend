
pub trait SearchableMove: Copy + Clone {
    fn to_string(self: &Self) -> String;
}

}

pub trait AlphaBetaSearchFunctionality {
    type Move: SearchableMove;
    type ZobristHash;

    fn is_whites_turn(self: &Self) -> bool;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
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