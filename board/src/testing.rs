#![allow(dead_code)]

use std::time::{Duration, Instant};

use generic_magic::{Bool, False};

use crate::board::Board;


static PERFT_FENS: &[&str] = &[
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",  // Perft: done to depth 6, MU-Test: ?
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",  // Perft: done to depth 6, MU-Test: ?
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",  // Perft: done to depth 6, MU-Test: ?
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",  // Perft: ?, MU-Test: ?
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",  // Perft: ?, MU-Test: ?
];

static MATE_FENS: &[(&str, &str, i32)] = &[
    // in 5 plies
    ("r5rk/5p1p/5R2/4B3/8/8/7P/7K w - - 0 1", "f6a6", 30_000 - 5),  // Ra6, f6, Bxf6, Rg7, Rxa8#
    ("2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b - - 0 1", "b1g6", -30_000 + 5),  // Qg6+, Kg4, Qf5+, Kh5, Qh3#
    ("1k5r/pP3ppp/3p2b1/1BN1n3/1Q2P3/P1B5/KP3P1P/7q w - - 1 0", "c5a6", 30_000 - 5),  // Na6, Kxb7, Kxa6, Qb5#
    ("3r4/pR2N3/2pkb3/5p2/8/2B5/qP3PPP/4R1K1 w - - 1 0", "c3e5", 30_000 - 5),  // Be5, Kc5, Rc1, Bc4, b4#
    ("R6R/1r3pp1/4p1kp/3pP3/1r2qPP1/7P/1P1Q3K/8 w - - 1 0", "f4f5", 30_000 - 5),  // f5+, exf5, Qxh6+, gxh6, Rag8#
    ("4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1", "e5e1", -30_000 + 5),  // Re1, Rxe1, Rxe1, Qxe1, Qg2#

    // mate in 3 plies
    ("r2qk2r/pb4pp/1n2Pb2/2B2Q2/p1p5/2P5/2B2PPP/RN2R1K1 w - - 1 0", "f5g6", 30_000 - 3),  // Qg6+, hxg6, Bxg6#
    ("6k1/pp4p1/2p5/2bp4/8/P5Pb/1P3rrP/2BRRN1K b - - 0 1", "g2g1", -30_000 + 3),  // Rg1+, Kxg1, Rxf1#
    ("8/2k2p2/2b3p1/P1p1Np2/1p3b2/1P1K4/5r2/R3R3 b - - 0 1", "c6b5", -30_000 + 3),  // Bb5+, Nc4, Rd2#
    ("6k1/5p2/1p5p/p4Np1/5q2/Q6P/PPr5/3R3K w - - 1 0", "a3f8", 30_000 - 3),  // Qf8+, Kxf8, Rd8#
    ("r1b2k1r/ppppq3/5N1p/4P2Q/4PP2/1B6/PP5P/n2K2R1 w - - 1 0", "h5h6", 30_000 - 3),  // Qxh6+, Rxh6, Rg8#

    // mate in 2 plies (self made)
    ("r2qk2r/pb4pp/1n2PbQ1/2B5/p1p5/2P5/2B2PPP/RN2R1K1 b - - 2 1", "h7g6", 30_000 - 2),  // hxg6, Bxg6#
    ("6k1/pp4p1/2p5/2bp4/8/P5Pb/1P3r1P/2BRRNrK w - - 1 2", "h1g1", -30_000 + 2),  // Kxg1, Rxf1#
    ("8/2k2p2/6p1/Pbp1Np2/1p3b2/1P1K4/5r2/R3R3 w - - 1 2", "e5c4", -30_000 + 2),  // Nc4, Rd2#
    ("5Qk1/5p2/1p5p/p4Np1/5q2/7P/PPr5/3R3K b - - 2 1", "g8f8", 30_000 - 2),  // Kxf8, Rd8#
    ("r1b2k1r/ppppq3/5N1Q/4P3/4PP2/1B6/PP5P/n2K2R1 b - - 0 1", "h8h6", 30_000 - 2),  // Rxh6, Rg8#
];


fn test_make_unmake<IsMaxDepth: Bool>(board: &mut Board, depth: u8) {
    // test whether make_move and unmake_move are inverse to each other

    if depth == 0 {
        return;
    }

    for r#move in board.get_legal_moves() {
        if IsMaxDepth::AS_BOOL {print!("At:"); r#move.visualize(); print!("\n")}

        let copy = board.clone();

        board.make_move(r#move);

        if board.white_king.tzcnt() >= 64 || board.black_king.tzcnt() >= 64 {
            println!("King went missing!!!!");
            for info in board.history.clone() {
                let r#move = info.r#move;
                println!();
                board.visualize();
                board.unmake_move();
                println!("\nAfter\n:");
                r#move.visualize();
            }
            board.visualize();

            assert!(false);
        }

        test_make_unmake::<False/*IsMaxDepth*/>(board, depth - 1);
        board.unmake_move();

        if *board != copy {
            println!("Discrepancy!");
            r#move.visualize();
            println!();

            if board.whites_turn != copy.whites_turn {println!("p");}
            if board.white_pawns != copy.white_pawns {println!("wp");}
            if board.black_pawns != copy.black_pawns {println!("bp");}
            if board.white_knights != copy.white_knights {println!("wn");}
            if board.black_knights != copy.black_knights {println!("bn");}
            if board.white_bishops != copy.white_bishops {println!("wb");}
            if board.black_bishops != copy.black_bishops {println!("bb");}
            if board.white_rooks != copy.white_rooks {println!("wr");}
            if board.black_rooks != copy.black_rooks {println!("br");}
            if board.white_queens != copy.white_queens {println!("wq");}
            if board.black_queens != copy.black_queens {println!("bq");}
            if board.white_king != copy.white_king {println!("wk");}
            if board.black_king != copy.black_king {println!("bk");}
            if board.white_mask != copy.white_mask {println!("wm");}
            if board.black_mask != copy.black_mask {println!("bm");}
            if board.occupation != copy.occupation {println!("occ"); board.occupation.visualize(); copy.occupation.visualize();}
            if board.square_piece_mapping != copy.square_piece_mapping {println!("spm");}
            if board.castle_permissions != copy.castle_permissions {println!("Castle");}
            if board.en_passant_square != copy.en_passant_square {println!("ep sq");}
            if board.zobrist_hash != copy.zobrist_hash {println!("Hash");}
            if board.fifty_move_counter != copy.fifty_move_counter {println!("50");}
            if board.history != copy.history {println!("History");}

            println!();
            board.visualize();
            println!();
            copy.visualize();
            println!();

            assert!(false);
        }
    }
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
            \tDuration: {}ms, i. e. {} MN/s.\
            ",
            self.nodes_visited, self.captures, self.en_passant, self.castles,
            self.promotions, self.terminals, self.duration.as_millis(),
            (self.nodes_visited as f64)/(1_000_000f64 * self.duration.as_secs() as f64)
        )
    }
}

fn perft(board: &mut Board, depth: u8) {
    /*
    Simulate all possible sequences of (half)moves up until given depth and visualize results in
    console.
    */

    // toggle debugging prints
    const PRINT_CAPTURES: bool = false;
    const PRINT_TERMINALS: bool = false;

    fn inner_perft(board: &mut Board, depth: u8, perft_info: &mut PerftInfo) {

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
            if depth == 1 {
                perft_info.nodes_visited += 1;
                if r#move.is_capture() { perft_info.captures += 1; }
                if r#move.is_en_passant() { perft_info.en_passant += 1; }
                if r#move.is_castling() { perft_info.castles += 1; }
                if r#move.is_promotion() { perft_info.promotions += 1; }
            }

            if depth > 1 {
                // make move, recurse, unmake move
                board.make_move(r#move);
                inner_perft(board, depth - 1, perft_info);
                board.unmake_move();
            }
        }

        // no legal moves: terminal state
        if depth == 1 && n_moves == 0 {
            perft_info.terminals += 1;
            if PRINT_TERMINALS {
                println!("Terminal node:");
                board.visualize();
            }
        }
    }

    // do perft
    for r#move in board.get_legal_moves() {

        // go to child
        board.make_move(r#move);

        // do perft of child and time
        let now = Instant::now();
        let mut perft_info_of_move = PerftInfo::default();
        inner_perft(board, depth-1, &mut perft_info_of_move);
        perft_info_of_move.duration = now.elapsed();

        // back to parent
        board.unmake_move();

        // visualize result for child
        print!("\n");
        r#move.visualize();
        print!(":\n");
        perft_info_of_move.visualize();
    }
}

#[cfg(test)]
mod tests {

    use generic_magic::True;
    use search::{minimax::minimax, alpha_beta::alpha_beta, transposition_table::TranspositionTable};

    use crate::{
        board::Board,
        moves::Move,
        testing::{
            PERFT_FENS,
            MATE_FENS,
            test_make_unmake,
            perft
        }
    };

    const MAKE_UNMAKE_DEPTH: u8 = 3;
    const MATE_SEARCH_DEPTH: u8 = 6;
    const COMPARE_DEPTH: u8 = 3;
    const PERFT_DEPTH: u8 = 3;


    #[test]
    fn test_make_unmake_multiple() {
        // test whether make_move and unmake_move are inverse to each (test on multiple boards)

        println!("Starting perft!");
        for &fen in PERFT_FENS {
            println!("FEN: {fen}\n");

            test_make_unmake::<True/*IsMaxDepth*/>(
                &mut Board::from_fen(fen),
                MAKE_UNMAKE_DEPTH
            );
        }
    }

    #[test]
    fn test_mate_multiple() {
        // test whether search reliably finds mate (in multiple positions)

        // TODO: Work through PVs and check TT in that way.

        println!("Starting!");
        for (fen, algebraic_move, evaluation) in MATE_FENS {
            println!("\nFEN: {fen}, move: {algebraic_move}");

            let mut board = Board::from_fen(fen);
            let r#move = Move::from_algebraic(algebraic_move, &board);
            let mut transposition_table: TranspositionTable<Board> = TranspositionTable::new();
            let info = alpha_beta(
                &mut board,
                MATE_SEARCH_DEPTH,
                &mut transposition_table
            );

            print!("Expected:");
            r#move.visualize();
            print!(" with score: {evaluation}");
            print!(", got:");
            info.best_move.unwrap().visualize();
            print!(" with score {}\n", info.evaluation);
            if info.best_move != Some(r#move) || info.evaluation != *evaluation {
                assert!(false);
            }
        }
    }

    /*#[test]
    fn compare_minimax_alpha_beta_multiple() {
        // Test whether minimax and alpha_beta return the same results

        println!("Starting!");
        for fen in PERFT_FENS {
            println!("FEN: {fen}\n");

            let mut board = Board::from_fen(fen);

            let mut transposition_table: TranspositionTable<Board> = TranspositionTable::new();
            minimax(&mut board, COMPARE_DEPTH, &mut transposition_table);

            let mut transposition_table: TranspositionTable<Board> = TranspositionTable::new();
            alpha_beta(&mut board, COMPARE_DEPTH, &mut transposition_table).visualize();
        }
    }*/

    #[test]
    fn perft_multiple() {
        /*
        Time speed of make_move, unmake_move and get_legal_moves.

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

        println!("Starting perft!");
        for fen in PERFT_FENS {
            println!("FEN: {fen}\n");

            perft(&mut Board::from_fen(fen), PERFT_DEPTH);
        }
    }
}
