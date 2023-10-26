#![allow(unused)]

use crate::moves::Move;

mod board;
mod castle_permissions;
mod legal_move_generations;
mod moves;
mod pieces;
mod perft;
mod zobrist_hash;
mod generic_magic;

pub fn main() {

    /*
    - without generics:
        - FEN2: 20.72
        - FEN4: 19.82
    - with generics (IsCapt, IsEP. IsCast, IsProm) in make_move:
        - FEN2: 20.95
        - FEN4: 19.08
    - with generics (IsCapt, IsEP. IsCast, IsProm, IsPS) in make_move:
        - FEN2: 21.74
        - FEN4: 20.09
    - with generics (IsCapt, IsEP. IsCast, IsProm, IsPS) in make_move
        and (IsCapt, IsEP, IsCast, IsProm) in unmake_move:
        - FEN2: 21.81, 38.73, 38.73
        - FEN4: 20.17, 35.30, 32.09
    */

    let mut board_2 = board::Board::from_fen(perft::PERFT_FEN_2);
    let mut board_4 = board::Board::from_fen(perft::PERFT_FEN_4);
    let mut board_5 = board::Board::from_fen(perft::PERFT_FEN_5);
    let mut board_6 = board::Board::from_fen(perft::PERFT_FEN_6);
    perft::perft(&mut board_2, 5, false);
    perft::perft(&mut board_4, 6, false);
    perft::perft(&mut board_5, 5, false);
    perft::perft(&mut board_6, 6, false);

}