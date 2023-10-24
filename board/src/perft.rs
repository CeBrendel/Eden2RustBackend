use std::time::{Duration, Instant};

use crate::board::Board;
use crate::moves::Move;

// FENs for testing
pub const PERFT_FEN_1: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";  // Perft: done to depth 6, MU-Test: ?
pub const PERFT_FEN_2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";  // Perft: done to depth 6, MU-Test: ?
pub const PERFT_FEN_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const PERFT_FEN_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";  // Perft: done to depth 6, MU-Test: ?

// Debugging prints?
const PRINT_CAPTURES: bool = false;
const PRINT_TERMINALS: bool = false;


pub trait InformedMove {
    fn is_capture(self: &Self) -> bool;
    fn is_en_passant(self: &Self) -> bool;
    fn is_castling(self: &Self) -> bool;
    fn is_promotion(self: &Self) -> bool;
    fn visualize(self: &Self);
}

pub trait PerftFunctionality {
    type Move: Copy + Clone + InformedMove;

    fn from_fen(fen: &str) -> Self;
    fn make_move(self: &mut Self, r#move: Self::Move);
    fn unmake_move(self: &mut Self);
    fn get_legal_moves(self: &Self) -> Vec<Self::Move>;
    fn visualize(self: &Self);
}

#[derive(Default)]
pub struct PerftInfo {
    duration: Duration,
    nodes_visited: usize,
    captures: usize,
    en_passant: usize,
    castles: usize,
    promotions: usize,
    terminals: usize/*,
    checks: usize,
    discovery_checks: usize,
    double_checks: usize,
    checkmates: usize*/
}

impl PerftInfo {
    pub fn visualize(self: &Self) {
        println!(
            "\
            \tNodes visited : {}.\n\
            \tThereof:\n\
            \t- Captures    : {},\n\
            \t- En-passants : {},\n\
            \t- Castles     : {},\n\
            \t- Promotions  : {},\n\
            \t- Terminals   : {},\n\
            \tDuration: {}ms.\
            ",
            self.nodes_visited, self.captures, self.en_passant, self.castles,
            self.promotions, self.terminals, self.duration.as_millis()
        )
    }
}

pub fn perft<T: PerftFunctionality>(board: &mut T, max_depth: u8, per_move: bool) {
    /*
    Simulate all possible sequences of (half)moves up until given depth and visualize results in
    console.
     */

    fn inner_perft<T: PerftFunctionality>(board: &mut T, remaining_depth: u8, perft_info: &mut PerftInfo) {

        // get legal moves
        let legal_moves = board.get_legal_moves();

        // loop through legal moves, make them and recurse
        let mut n_moves = 0;
        for r#move in legal_moves {
            n_moves += 1;

            if PRINT_CAPTURES && r#move.is_capture() {
                println!("Current board:");
                board.visualize();
                print!("Capture:");
                r#move.visualize();
                println!();
            }

            // update perft info
            if remaining_depth == 1 {
                perft_info.nodes_visited += 1;
                if r#move.is_capture() { perft_info.captures += 1; }
                if r#move.is_en_passant() { perft_info.en_passant += 1; }
                if r#move.is_castling() { perft_info.castles += 1; }
                if r#move.is_promotion() { perft_info.promotions += 1; }
            }

            if remaining_depth > 1 {
                // make move, recurse, unmake move
                board.make_move(r#move);
                inner_perft(board, remaining_depth - 1, perft_info);
                board.unmake_move();
            }
        }

        // no legal moves: terminal state
        if remaining_depth == 1 && n_moves == 0 {
            perft_info.terminals += 1;
            if PRINT_TERMINALS {
                println!("Terminal node:");
                board.visualize();
            }
        }
    }

    if per_move {
        for r#move in board.get_legal_moves() {
            // go to child
            board.make_move(r#move);

            // do perft of child and time
            let now = Instant::now();
            let mut perft_info_of_move = PerftInfo::default();
            inner_perft(board, max_depth - 1, &mut perft_info_of_move);
            perft_info_of_move.duration = now.elapsed();

            // back to parent
            board.unmake_move();

            // visualize result for child
            print!("\n");
            r#move.visualize();
            print!(":\n");
            perft_info_of_move.visualize();
        }
    } else {
        // do perft and time it
        let now = Instant::now();
        let mut perft_info = PerftInfo::default();
        inner_perft(board, max_depth, &mut perft_info);
        perft_info.duration = now.elapsed();

        // visualize
        perft_info.visualize();
    }
}
