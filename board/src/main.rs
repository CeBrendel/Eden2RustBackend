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

pub fn main() {

    let mut board = board::Board::from_fen("r3r1k1/ppp3pp/4N3/2bP4/2n1p3/2P5/PP3PPP/R1B1K2R b KQ - 0 16");

    board.visualize();

    // board::test_make_unmake(&mut board, 5, 5);  // test whether make/unmake works properly
    // perft::perft(&mut board, 6, false);

    /*use search::alpha_beta_search::alpha_beta_search;
    let (maybe_move, value) = alpha_beta_search(&mut board, 8);
    maybe_move.unwrap().visualize();
    println!("\n{}", value);*/

}