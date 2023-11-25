use bitboards::bitloop;
use crate::board::Board;

const fn flip_vertical(table: [i32; 64]) -> [i32; 64] {
    let mut new_table: [i32; 64] = [0; 64];

    let mut index: usize = 0;
    while index < 64 {

        let (rank, mut file) = (index % 8, index / 8);
        file = 7 - file;
        let flipped_index = rank + 8*file;

        new_table[index] = -table[flipped_index];

        index += 1;
    }

    return new_table;
}

static WHITE_PAWN_PIECE_SQUARE_TABLE: [i32; 64] = {

    let mut inc = [
        0	,	0	,	0	,	0	,	0	,	0	,	0	,	0	,
        10	,	10	,	0	,	-10	,	-10	,	0	,	10	,	10	,
        5	,	0	,	0	,	5	,	5	,	0	,	0	,	5	,
        0	,	0	,	10	,	20	,	20	,	10	,	0	,	0	,
        5	,	5	,	5	,	10	,	10	,	5	,	5	,	5	,
        10	,	10	,	10	,	20	,	20	,	10	,	10	,	10	,
        20	,	20	,	20	,	30	,	30	,	20	,	20	,	20	,
        0	,	0	,	0	,	0	,	0	,	0	,	0	,	0
    ];

    let mut index: usize = 0;
    while index < 64 {
        inc[index] += 100;
        index += 1;
    }

    inc
};
static WHITE_KNIGHT_PIECE_SQUARE_TABLE: [i32; 64] = {

    let mut inc = [
        -15	,	-10	,	0	,	0	,	0	,	0	,	-10	,  -15	,
        -10	,	0	,	0	,	5	,	5	,	0	,	0	,  -10	,
        -10 	,	0	,	10	,	10	,	10	,	10	,	0	,  -10	,
        -5	,	0	,	10	,	20	,	20	,	10	,	5	,  -5	,
        -5	,	10	,	15	,	20	,	20	,	15	,	10	,  -5	,
        -5	,	10	,	10	,	20	,	20	,	10	,	10	,  -5	,
        -5	,	0	,	5	,	10	,	10	,	5	,	0	,  -5	,
        -10	,	0	,	0	,	0	,	0	,	0	,	0	,  -10
    ];

    let mut index: usize = 0;
    while index < 64 {
        inc[index] += 300;
        index += 1;
    }

    inc
};
static WHITE_BISHOP_PIECE_SQUARE_TABLE: [i32; 64] = {

    let mut inc = [
        0	,	0	,	-10	,	0	,	0	,	-10	,	0	,	0	,
        0	,	10	,	0	,	10	,	10	,	0	,	10	,	0	,
        0	,	0	,	10	,	15	,	15	,	10	,	0	,	0	,
        0	,	10	,	15	,	20	,	20	,	15	,	10	,	0	,
        0	,	10	,	15	,	20	,	20	,	15	,	10	,	0	,
        0	,	0	,	10	,	15	,	15	,	10	,	0	,	0	,
        0	,	0	,	0	,	10	,	10	,	0	,	0	,	0	,
        0	,	0	,	0	,	0	,	0	,	0	,	0	,	0
    ];

    let mut index: usize = 0;
    while index < 64 {
        inc[index] += 320;
        index += 1;
    }

    inc
};
static WHITE_ROOK_PIECE_SQUARE_TABLE: [i32; 64] = {

    let mut inc = [
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0	,
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0	,
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0	,
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0	,
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0	,
        10	,	10	,	15	,	10	,	10	,	15	,	10	,	10	,
        25	,	25	,	25	,	25	,	25	,	25	,	25	,	25	,
        0	,	0	,	5	,	10	,	10	,	5	,	0	,	0
    ];

    let mut index: usize = 0;
    while index < 64 {
        inc[index] += 500;
        index += 1;
    }

    inc
};
static WHITE_QUEEN_PIECE_SQUARE_TABLE: [i32; 64] = {

    let mut inc = [
        -10  ,   0	,  -10	,  -5	,  -5	,  -5	,  -10	,  -20	,
        -20  ,  -10	,  -10	,  -5	,  -5	,  -10	,  -10	,  -20	,
        -10  ,   0	,   0	,   0	,   0	,   0	,   0	,  -10	,
        -10  ,   0	,   5	,  -5	,  -5	,   5	,   0	,  -10	,
        -5   ,   0	,   5 	,  -5	,  -5	,   5	,   0	,  -5	,
        0   ,   0	,   5 	,  -5	,  -5	,   5	,   0	,  -5	,
        -10  ,   5	,   5	,  -5	,  -5	,   5	,   0	,  -10	,
        -10  ,   0 	,   5	,   0	,   0	,   0	,   0	,  -10	,
    ];

    let mut index: usize = 0;
    while index < 64 {
        inc[index] += 900;
        index += 1;
    }

    inc
};
static WHITE_KING_PIECE_SQUARE_TABLE: [i32; 64] = [
    25 ,   35  ,   15  ,   0   ,   0   ,   10  ,   30  ,   20  ,
    20 ,   20  ,   0   ,   0   ,   0   ,   0   ,   20  ,   20  ,
    -10 ,  -20  ,  -20  ,  -20  ,  -20  ,  -20  ,  -20  ,  -10  ,
    -20 ,  -30  ,  -30  ,  -40  ,  -40  ,  -30  ,  -30  ,  -20  ,
    -30 ,  -40  ,  -40  ,  -50  ,  -50  ,  -40  ,  -40  ,  -30  ,
    -30 ,  -40  ,  -40  ,  -50  ,  -50  ,  -40  ,  -40  ,  -30  ,
    -30 ,  -40  ,  -40  ,  -50  ,  -50  ,  -40  ,  -40  ,  -30  ,
    -30 ,  -40  ,  -40  ,  -50  ,  -50  ,  -40  ,  -40  ,  -30  ,
];


static BLACK_PAWN_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_PAWN_PIECE_SQUARE_TABLE);
static BLACK_KNIGHT_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_KNIGHT_PIECE_SQUARE_TABLE);
static BLACK_BISHOP_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_BISHOP_PIECE_SQUARE_TABLE);
static BLACK_ROOK_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_ROOK_PIECE_SQUARE_TABLE);
static BLACK_QUEEN_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_QUEEN_PIECE_SQUARE_TABLE);
static BLACK_KING_PIECE_SQUARE_TABLE: [i32; 64] = flip_vertical(WHITE_KING_PIECE_SQUARE_TABLE);



impl Board {
    /*pub fn evaluate(self: &Self) -> i32 {
        // TODO: move to quotient space? A 300cp advantage is worth more, the less pieces there are on the board.
        return {
            let white_material = (
                self.white_pawns.count_ones()
                    + 3*self.white_knights.count_ones()
                    + 3*self.white_bishops.count_ones()
                    + 5*self.white_rooks.count_ones()
                    + 9*self.white_queens.count_ones()
            ) as i32;
            let black_material = (
                self.black_pawns.count_ones()
                    + 3*self.black_knights.count_ones()
                    + 3*self.black_bishops.count_ones()
                    + 5*self.black_rooks.count_ones()
                    + 9*self.black_queens.count_ones()
            ) as i32;
            white_material - black_material
        }
    }*/

    pub fn evaluate(self: &Self) -> i32 {

        let mut evaluation: i32 = 0;

        // pawns
        bitloop!(self.white_pawns, square => {evaluation += WHITE_PAWN_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_pawns, square => {evaluation += BLACK_PAWN_PIECE_SQUARE_TABLE[square as usize];});

        // knights
        bitloop!(self.white_knights, square => {evaluation += WHITE_KNIGHT_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_knights, square => {evaluation += BLACK_KNIGHT_PIECE_SQUARE_TABLE[square as usize];});

        // bishops
        bitloop!(self.white_bishops, square => {evaluation += WHITE_BISHOP_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_bishops, square => {evaluation += BLACK_BISHOP_PIECE_SQUARE_TABLE[square as usize];});

        // rooks
        bitloop!(self.white_rooks, square => {evaluation += WHITE_ROOK_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_rooks, square => {evaluation += BLACK_ROOK_PIECE_SQUARE_TABLE[square as usize];});

        // queens
        bitloop!(self.white_queens, square => {evaluation += WHITE_QUEEN_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_queens, square => {evaluation += BLACK_QUEEN_PIECE_SQUARE_TABLE[square as usize];});

        // king
        bitloop!(self.white_king, square => {evaluation += WHITE_KING_PIECE_SQUARE_TABLE[square as usize];});
        bitloop!(self.black_king, square => {evaluation += BLACK_KING_PIECE_SQUARE_TABLE[square as usize];});

        return evaluation;
    }
}