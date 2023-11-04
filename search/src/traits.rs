
pub trait Move: Copy + Clone {
    fn to_string(self: &Self) -> String;
}

pub trait QuiescenceSearchFunctionality {
    type Move: Move;
    type ZobristHash;

    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn loud_moves(self: &mut Self) -> Vec<Self::Move>;
}

pub trait AlphaBetaSearchFunctionality {
    /*
    TODO:
        - remove is_terminal function call and instead check number of legal moves directly
    */
    type Move: Move;
    type ZobristHash;

    fn is_whites_turn(self: &Self) -> bool;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> f32;
    fn is_terminal(self: &Self) -> bool;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}

pub trait MCTSFunctionality {
    type Move: Move;
    type ZobristHash;

    fn make_move_and_copy_and_flip(self: &Self, r#move: Self::Move) -> Self;
    fn evaluate(self: &Self) -> f32;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}