
/*
TODO:
    - move perft to separate module
    - transposition table via pointers? I don't like the TT in the search info and all of the lifetime stuff
*/

use crate::parsing::uci_loop;

mod parsing;
mod go;

#[allow(dead_code)]
fn perft() {
    /*
    - 0. without generics:
        - FEN2: 20.72
        - FEN4: 19.82
    - 1. with generics (IsCapt, IsEP. IsCast, IsProm) in make_move:
        - FEN2: 20.95
        - FEN4: 19.08
    - 2. with generics (IsCapt, IsEP. IsCast, IsProm, IsPS) in make_move:
        - FEN2: 21.74
        - FEN4: 20.09
    - 3. as 2. with (IsCapt, IsEP, IsCast, IsProm) in unmake_move:
        - FEN2: 21.81, 38.73, 38.73
        - FEN4: 20.17, 35.30, 32.09
    - 4. as 3. with surface level (IsWhite) in make_move/unmake_move:
        - FEN1: 5.41
        - FEN2: 8.98
        - FEN4: 35.30
        - FEN5: 9.99
        - FEN6: 9.16
    */

    use search::{alpha_beta::alpha_beta, minimax::minimax, transposition_table::TranspositionTable};
    use board::{board::Board, perft};


    let mut board_1 = Board::from_fen(perft::PERFT_FEN_1);
    let mut board_2 = Board::from_fen(perft::PERFT_FEN_2);
    let mut board_4 = Board::from_fen(perft::PERFT_FEN_4);
    let mut board_5 = Board::from_fen(perft::PERFT_FEN_5);
    let mut board_6 = Board::from_fen(perft::PERFT_FEN_6);

    /*perft::perft(&mut board_1, 6, false);
    perft::perft(&mut board_2, 6, false);
    perft::perft(&mut board_4, 6, false);
    perft::perft(&mut board_5, 5, false);
    perft::perft(&mut board_6, 6, false);*/

    println!("Starting!");

    println!("board_1");
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    minimax(&mut board_1, 3, transposition_table);
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    alpha_beta(&mut board_1, 3, transposition_table).visualize();

    println!("board_2");
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    minimax(&mut board_2, 3, transposition_table);
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    alpha_beta(&mut board_2, 3, transposition_table).visualize();

    println!("board_4");
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    minimax(&mut board_4, 3, transposition_table);
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    alpha_beta(&mut board_4, 3, transposition_table).visualize();

    println!("board_5");
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    minimax(&mut board_5, 3, transposition_table);
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    alpha_beta(&mut board_5, 3, transposition_table).visualize();

    println!("board_6");
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    minimax(&mut board_6, 3, transposition_table);
    let transposition_table: &mut TranspositionTable<Board> = &mut TranspositionTable::new();
    alpha_beta(&mut board_6, 3, transposition_table).visualize();
}

fn main() {
    uci_loop();

    // perft();
}