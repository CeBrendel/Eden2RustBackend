
// adds/removes asserts at compile time
const DO_ASSERTS: bool = false;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
    BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,
    None
}
impl Piece {
    /*const PIECE_CHARS: [char; 13] = [
        '♙','♘','♗','♖','♕','♔',
        '♟','♞','♝','♜','♛','♚',
        '.'
    ];*/
    const PIECE_CHARS: [char; 13] = [
        'P','N','B','R','Q','K',
        'p','n','b','r','q','k',
        '.'
    ];

    pub fn visualize(self: &Self) {
        print!(" {} ", Self::PIECE_CHARS[*self as usize]);
    }

    pub fn from_repr(repr: u8) -> Self {
        if DO_ASSERTS {
            assert!(repr < 12, "Was: {}, in binary: {:b}", repr, repr);
        }
        // disallow to get None from repr.
        return unsafe {std::mem::transmute(repr)};
    }
}