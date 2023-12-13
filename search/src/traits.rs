
/*
TODO:
    - Should all functions of SearchableMove take Self instead of &Self. Moves should probably be copied and not passed by reference.
*/


pub trait SearchableMove: Copy + Clone + PartialEq {
    // for visualization
    fn to_string(self: &Self) -> String;

    // for history heuristic:
    fn is_capture(self: &Self) -> bool;
    fn is_loud(self: &Self) -> bool;
    fn to_square_as_index(self: &Self) -> usize;
    fn moving_piece_as_index(self: &Self) -> usize;
    fn captured_piece_as_index(self: &Self) -> usize;
}


pub trait AlphaBetaSearchFunctionality {
    type Move: SearchableMove;
    type ZobristHash: Eq + Copy;

    fn is_whites_turn(self: &Self) -> bool;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn evaluate(self: &Self) -> i32;
    fn is_check(self: &Self) -> bool;
    fn zobrist_hash(self: &Self) -> Self::ZobristHash;
    fn legal_moves(self: &Self) -> Vec<Self::Move>;

    fn loud_moves(self: &mut Self) -> Vec<Self::Move>;
    fn last_move(self: &Self) -> Option<Self::Move>;
}

/*pub trait MCTSFunctionality {
    type Move: SearchableMove;
    type ZobristHash;

    fn make_move_and_copy_and_flip(self: &Self, r#move: Self::Move) -> Self;
    fn evaluate(self: &Self) -> f32;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}*/