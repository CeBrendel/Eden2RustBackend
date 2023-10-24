
/*
TODO:
    - Handle flip via a new data structure, that wraps two boards? Less expensive than true flipping
*/


pub trait MCTSFunctionality {
    type Move: Copy + Clone;
    type ZobristHash;

    fn make_move_and_copy_and_flip(self: &Self, r#move: Self::Move) -> Self;
    fn evaluate(self: &Self) -> f32;
    fn hash(self: &Self) -> Self::ZobristHash;
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
}