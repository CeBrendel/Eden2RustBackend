#![allow(unused)]

/*
TODO:
    - Factor out searches into separate crate
 */

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
        - FEN2: ???
        - FEN4: ???
    - with generics (IsCapt, IsEP. IsCast, IsProm) in make_move:
        - FEN2: 20.95 MN/s (depth 5)
        - FEN4: 19.08 MN/s (depth 6)
    - with generics (IsCapt, IsEP. IsCast, IsProm, IsPS) in make_move:
        - FEN2: 21.74 MN/s (depth 5)
        - FEN4: 20.09 MN/s (depth 6)
    */

    let mut board_2 = board::Board::from_fen(perft::PERFT_FEN_2);
    let mut board_4 = board::Board::from_fen(perft::PERFT_FEN_4);
    perft::perft(&mut board_2, 5, false);
    perft::perft(&mut board_4, 6, false);

    // board::test_make_unmake(&mut board, 5, 5);  // test whether make/unmake works properly

    /*use search::alpha_beta_search::alpha_beta_search;
    let (maybe_move, value) = alpha_beta_search(&mut board, 8);
    maybe_move.unwrap().visualize();
    println!("\n{}", value);*/

}